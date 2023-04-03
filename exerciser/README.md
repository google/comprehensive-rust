# exerciser

This is a tool to generate templates for exercises from the Markdown source. Given a Markdown file
with one or more sections like:

````markdown
<!-- File src/main.rs -->

```rust,compile_fail
{{#include example/src/main.rs:main}}

fn some_more_code() {
    // TODO: Write some Rust code here.
}
```
````

You can run it like `cargo run my/markdown/file.md exercise-templates/example`, and it will create a
file `exercise-templates/example/src/main.rs` with the appropriate contents.
