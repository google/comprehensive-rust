# Strings and Iterators

In this exercise, you are implementing a routing component of a web server. The
server is configured with a number of _path prefixes_ which are matched against
_request paths_. The path prefixes can contain a wildcard character which
matches a full segment. See the unit tests below.

Copy the following code to <https://play.rust-lang.org/> and make the tests
pass. Try avoiding allocating a `Vec` for your intermediate results:

```rust
// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

{{#include strings-iterators.rs:prefix_matches}}
    unimplemented!()
}

{{#include strings-iterators.rs:unit-tests}}
```
