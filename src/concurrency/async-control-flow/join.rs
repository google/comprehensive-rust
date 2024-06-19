use futures::future::join_all;
use reqwest;

async fn size_of_page(url: &str) -> reqwest::Result<usize> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())
}

#[tokio::main]
async fn main() {
    let urls = ["https://rust-lang.org", "https://httpbin.org/ip", "BAD_URL"];
    let futures = urls.into_iter().map(size_of_page);
    let results = join_all(futures).await;
    for (url, result) in urls.into_iter().zip(results) {
        println!("{url}: {result:?}");
    }
}
