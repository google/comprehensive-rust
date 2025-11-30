# Scope Guards

A scope guard uses the `Drop` trait to ensure cleanup code runs automatically
when a scope exits — even if due to an error.

```rust,editable
use scopeguard::{ScopeGuard, guard};
use std::fs::{self, File};
use std::io::Write;

fn download_successful() -> bool {
    // [...]
    true
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
        let path = ScopeGuard::into_inner(cleanup);
        println!("Download '{path}' complete!");
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

</details>
