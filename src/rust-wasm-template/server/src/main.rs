use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result};
use std::path::PathBuf;

#[get("/")]
async fn index() -> Result<NamedFile> {
    let path: PathBuf = "./files/index.html".parse()?;
    Ok(NamedFile::open(path)?.use_last_modified(true))
}

#[get("/files/{name}")]
async fn files(web::Path(name): web::Path<String>) -> Result<NamedFile> {
    let path: PathBuf = format!("./files/{}", name).parse()?;
    Ok(NamedFile::open(path)?.use_last_modified(true))
}

#[get("/font.otf")]
async fn font() -> Result<NamedFile> {
    let path: PathBuf = "./files/source-code-regular.otf".parse()?;
    Ok(NamedFile::open(path)?.use_last_modified(true))
}

#[get("/wasm/{name}")]
async fn serve_wasm(web::Path(name): web::Path<String>) -> Result<NamedFile> {
    let path: PathBuf = format!("./pkg/{}", name).parse()?;
    Ok(NamedFile::open(path)?.use_last_modified(true))
}

#[get("/wasm/snippets/{snippet_name}/{name}")]
async fn serve_wasm_snippet(
    web::Path((snippet_name, name)): web::Path<(String, String)>,
) -> Result<NamedFile> {
    let path: PathBuf = format!("./pkg/snippets/{}/{}", snippet_name, name).parse()?;
    Ok(NamedFile::open(path)?.use_last_modified(true))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(files)
            .service(serve_wasm)
            .service(serve_wasm_snippet)
            .service(font)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
