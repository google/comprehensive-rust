// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: setup
use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
// ANCHOR_END: setup

// ANCHOR: extract_links
fn extract_links(response: Response) -> Result<Vec<Url>, Error> {
    let base_url = response.url().to_owned();
    let document = response.text()?;
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    let mut valid_urls = Vec::new();
    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(url) => valid_urls.push(url),
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }

    Ok(valid_urls)
}
// ANCHOR_END: extract_links

fn check_links(url: Url) -> Result<Vec<Url>, Error> {
    println!("Checking {url}");

    let response = get(url.to_owned())?;

    if !response.status().is_success() {
        return Ok(vec![url.to_owned()]);
    }

    let links = extract_links(response)?;
    for link in &links {
        println!("{link}, {:?}", link.domain());
    }

    let mut failed_links = Vec::new();
    for link in links {
        if link.domain() != url.domain() {
            println!("Checking external link: {link}");
            let response = get(link.clone())?;
            if !response.status().is_success() {
                println!("Error on {url}: {link} failed: {}", response.status());
                failed_links.push(link);
            }
        } else {
            println!("Checking link in same domain: {link}");
            failed_links.extend(check_links(link)?)
        }
    }

    Ok(failed_links)
}

fn main() {
    let start_url = Url::parse("https://www.google.org").unwrap();
    match check_links(start_url) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}
