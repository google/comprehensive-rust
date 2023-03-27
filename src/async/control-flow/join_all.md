# join_all

Futures can be combined together to produce concurrent compute flow graphs.

## Run a group of futures concurrently until they all resolve: `join_all`

### Equivalents:

- JS: `Promise.all`
- Python: `asyncio.gather`

```rust,editable,compile_fail
use anyhow::Result;
use futures::future;
use reqwest;
use std::collections::HashMap;

async fn size_of_page(url: &str) -> Result<usize> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())
}

#[tokio::main]
async fn main() {
    let urls: [&str; 4] = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
        "BAD_URL",
    ];
    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_sizes_dict: HashMap<&str, Result<usize>> =
        urls.into_iter().zip(results.into_iter()).collect();
    println!("{:?}", page_sizes_dict);
}
```

<details>

* `join_all` should soon be stabilized as part of the standard library in `std::future`.
* For multiple futures of disjoint types, you can use `join!` but you must know how many futures you will have at compile time.
* You can also combine `join_all` with `join!` for instance to join all requests to an http service as well as a database query.
* The risk of `join` is that one of the future could never resolve, this would cause your program to stall. 
* Try adding a timeout to the future.

</details>

