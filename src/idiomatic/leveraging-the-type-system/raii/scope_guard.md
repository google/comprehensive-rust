# Scope Guards

A scope guard uses the `Drop` trait to ensure cleanup code runs automatically
when a scope exits — even if due to an error.

```rust,editable,compile_fail
use scopeguard::{ScopeGuard, guard};
use std::{
    fs::{self, File},
    io::Write,
};

fn download_successful() -> bool {
    true // change to false to simulate failure
}

fn main() {
    let path = "download.tmp";
    let mut file = File::create(path).expect("cannot create temporary file");

    // Set up cleanup immediately after file creation
    let cleanup = guard(path, |path| {
        println!("download failed, deleting: {:?}", path);
        let _ = fs::remove_file(path);
    });

    writeln!(file, "partial data...").unwrap();

    if download_successful() {
        // Download succeeded, keep the file
        let _path = ScopeGuard::into_inner(cleanup);
        println!("Download complete!");
    }
    // Otherwise, the guard runs and deletes the file
}
```

<details>

- This example simulates an HTTP download. We create a temporary file first,
  then use a scope guard to ensure that the file is deleted if the download
  fails.

- The guard is placed directly after creating the file, so even if `writeln!()`
  fails, the file will still be cleaned up. This ordering is essential for
  correctness.

- The guard's closure runs on scope exit unless defused with
  `ScopeGuard::into_inner`. In the success path, we defuse it to preserve the
  file.

- This pattern is useful when you want fallbacks or cleanup code to run
  automatically but only if success is not explicitly signaled.

- The `scopeguard` crate also supports cleanup strategies via the
  [`Strategy`](https://docs.rs/scopeguard/latest/scopeguard/trait.Strategy.html)
  trait. You can choose to run the guard on unwind only, or on success only, not
  just always.

## Manual Implementation Example

If you need a custom scope guard for a very specific task, you can implement one
manually. Here's a standalone example that mirrors the file deletion scenario
shown above: <TODO>

## Related Patterns

- Recall from the [Drop Bombs](./drop_bomb.md) chapter: drop bombs enforce that
  a resource is finalized. Scope guards take that further — they let you define
  automatic fallback behavior for cleanup when a resource is _not_ explicitly
  finalized.

- This pattern works well in combination with `Drop`, especially in fallible or
  multi-step operations where cleanup needs to be predictable, regardless of
  which step failed.

</details>
