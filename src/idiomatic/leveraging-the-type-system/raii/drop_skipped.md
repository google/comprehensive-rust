# Drop can be skipped

There are cases where destructors will not run.

```rust,editable
#[derive(Debug)]
struct Foo<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> Drop for Foo<T> {
    fn drop(&mut self) {
        println!("{self:?}::drop() called");

        // panic!("{self:?}::drop() panics");

        println!("{self:?}::drop() finished, with inner: {:?}", self.0);
    }
}

fn main() {
    let value = Foo(Foo(Foo(())));

    std::process::exit(0);

    // std::mem::forget(value);

    // panic!("main() panics with value: {value:?}");
}
```

<details>

- In the version that calls
  [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html),
  `Foo::drop` is never run because `exit` terminates the process immediately
  without unwinding.

  - You can prevent accidental use of `exit` by denying the
    [`clippy::exit`](https://rust-lang.github.io/rust-clippy/stable/index.html#exit)
    lint.

- If you remove the `std::process::exit(0)` line, the stack will unwind normally
  and each `drop()` method will run in turn.

- Try uncommenting the
  [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html) call.
  What do you think will happen?

  Forgetting a value intentionally _leaks_ it — the memory is never reclaimed,
  but this is still memory-safe in Rust. Since the value is never dropped, its
  destructor does not run.

  [`Box::leak`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak)
  is another example of intentional leaking, often used to create data that
  lives for the remainder of the process.

- Remove the `mem::forget` call, then uncomment the `panic!` below it. What do
  you expect now?

  With the default `panic=unwind` setting, the stack still unwinds and
  destructors run, even when the panic starts in `main`.

  - With
    [`panic=abort`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic),
    no unwinding takes place.

- Finally, uncomment the `panic!` inside `Foo::drop` and run it. Ask the class:
  which destructors run before the abort?

</details>
