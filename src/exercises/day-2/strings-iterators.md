# Strings and Iterators

In this exercise, you are implementing a routing component of a web server. The
server is configured with a number of _path prefixes_ which are matched against
_request paths_. The path prefixes can contain a wildcard character which
matches a full segment. See the unit tests below.

Copy the following code to <https://play.rust-lang.org/> and make the tests
pass. Try avoiding allocating a `Vec` for your intermediate results:


```rust
{{#include strings-iterators.rs:prefix_matches}}
pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    println!("Use parameters {prefix} and {request_path}");
    unimplemented!()
}

{{#include strings-iterators.rs:unit-tests}}
```
