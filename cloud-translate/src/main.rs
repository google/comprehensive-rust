use std::path::Path;

use anyhow::{anyhow, Context};
use google_cloud_auth::{token::Token, Config};
use polib::catalog::Catalog;
use polib::message::MessageBody;
use polib::po_file;
use reqwest::header::{self, HeaderMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextRequest<'a> {
    contents: Vec<&'a str>,
    mime_type: &'a str,
    source_language_code: &'a str,
    target_language_code: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextResponse {
    translations: Vec<Translation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Translation {
    translated_text: String,
}

async fn get_token() -> anyhow::Result<Token> {
    let scopes = [];
    let config = Config {
        audience: None,
        scopes: Some(&scopes),
    };

    let token_source = google_cloud_auth::create_token_source(config).await?;
    let token = token_source.token().await?;
    Ok(token)
}

fn create_client(gcp_project: &str) -> anyhow::Result<reqwest::Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "X-Goog-User-Project",
        header::HeaderValue::from_str(gcp_project)?,
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

async fn translate(
    client: &reqwest::Client,
    token: &Token,
    project: &str,
    msgids: &[&str],
    lang: &str,
) -> anyhow::Result<Vec<String>> {
    if msgids.is_empty() {
        // It is an error to send an empty request to Cloud Translate.
        return Ok(Vec::new());
    }

    let request = TranslateTextRequest {
        contents: msgids.into(),
        mime_type: "text/plain",
        source_language_code: "en_US",
        target_language_code: lang,
    };
    let url =
        format!("https://translate.googleapis.com/v3/projects/{project}:translateText");
    let bytes = client
        .post(&url)
        .bearer_auth(&token.access_token)
        .json(&request)
        .send()
        .await?
        .bytes()
        .await?;
    let response: TranslateTextResponse =
        serde_json::from_slice(&bytes).with_context(|| {
            format!("could not deserialize: {}", String::from_utf8_lossy(&bytes))
        })?;
    let msgstrs = response
        .translations
        .into_iter()
        .map(|t| t.translated_text)
        .collect::<_>();
    Ok(msgstrs)
}

fn load_catalog<P: AsRef<Path>>(path: P) -> anyhow::Result<Catalog> {
    po_file::parse(path.as_ref())
        .map_err(|err| anyhow!("{err}"))
        .with_context(|| {
            format!("Could not parse {} as PO file", path.as_ref().display())
        })
}

fn save_catalog<P: AsRef<Path>>(path: P, catalog: &Catalog) -> anyhow::Result<()> {
    let path = path.as_ref();
    if path.exists() {
        // po_file::write does not remove an existing file
        std::fs::remove_file(path)
            .with_context(|| format!("Removing {}", path.display()))?
    }
    polib::po_file::write(catalog, path)
        .with_context(|| format!("Writing catalog to {}", path.display()))?;
    Ok(())
}

async fn translate_catalog(
    catalog: &mut Catalog,
    client: &reqwest::Client,
    token: &Token,
    gcp_project: &str,
    max_char_count: usize,
) -> anyhow::Result<()> {
    let mut char_count = 0;
    let mut msg_indices = Vec::new();
    let mut msgids = Vec::new();
    for idx in 0..catalog.len() {
        let msg = catalog.get_message_by_index(idx).unwrap();
        if msg.is_translated() || msg.is_plural() {
            continue;
        }
        let msgid = msg.get_msgid().unwrap();
        if msgid.trim().is_empty() {
            continue;
        }
        char_count += msgid.chars().count();
        if char_count > max_char_count {
            let trunc = msgid.chars().take(20).collect::<String>();
            eprintln!("Stopping translation at message {idx}: {trunc}");
            break;
        }

        msg_indices.push(idx);
        msgids.push(msgid.as_str());
    }

    eprintln!(
        "Translating {} messages into {}",
        msg_indices.len(),
        catalog.metadata.language
    );
    let msgstrs = translate(
        client,
        token,
        gcp_project,
        &msgids,
        &catalog.metadata.language,
    )
    .await?;

    for (idx, msgstr) in msg_indices.into_iter().zip(msgstrs) {
        let mut msg = &mut catalog.messages[idx];
        match &mut msg.body {
            MessageBody::Singular(singular) => singular.msgstr = msgstr.clone(),
            MessageBody::Plural(_) => todo!("plural messages are not supported"),
        }
        msg.flags = String::from("fuzzy");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = std::env::args().collect::<Vec<_>>();
    match &args.as_slice() {
        &[_, gcp_project, po_file, max_char_count] => {
            let max_char_count = max_char_count.parse::<usize>()?;
            if max_char_count > 30000 {
                anyhow::bail!("You can only translate up to 30k characters at a time");
            }
            let token = get_token().await?;
            let client = create_client(gcp_project)?;

            let mut catalog = load_catalog(po_file)?;
            translate_catalog(&mut catalog, &client, &token, gcp_project, max_char_count)
                .await?;
            save_catalog(po_file, &catalog)?;
            Ok(())
        }
        _ => {
            anyhow::bail!("Usage: cloud-translate <gcp-project> <po-file> <char-count>");
        }
    }
}
