---
minutes: 15
---

# Scope Guards

A scope guard uses the `Drop` trait to run cleanup code automatically when a
scope exits, even during unwinding.

```rust,editable,compile_fail
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

- This example models a download workflow. We create a temporary file first,
  then use a scope guard to ensure that the file is deleted if the download
  fails.

- The `scopeguard` crate allows you to conveniently define a single-use
  `Drop`-based cleanup without defining a custom type with a custom `Drop`
  implementation.

- The guard is created directly after creating the file, so even if `writeln!()`
  fails, the file will still be cleaned up. This ordering is essential for
  correctness.

- The `guard()` creates a `ScopeGuard` instance. It a user-defined value (in
  this case, `path`) and the cleanup closure that later receives this value.

- The guard's closure runs on scope exit unless it is _defused_ with
  `ScopeGuard::into_inner` (removing the value so the guard does nothing on
  drop). In the success path, we call `into_inner` so the guard will not delete
  the file.

- A scope guard is similar to the `defer` feature in Go.

- This pattern is ideal for "cleanup on failure" scenarios, where a cleanup
  should run by default unless a success path is explicitly taken.

- This pattern is also useful when you don't control the cleanup strategy of the
  resource object. In this example, `File::drop()` closes the file but does not
  delete it, and we can't change the standard library to delete the file instead
  (nor should we, it is not a good idea anyway).

- The `scopeguard` crate also supports cleanup strategies via the
  [`Strategy`](https://docs.rs/scopeguard/latest/scopeguard/trait.Strategy.html)
  trait. You can choose to run the guard on unwind only, or on success only, not
  just always.

</details>
