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

  Forgetting a value intentionally _leaks_ it. Leaking is still memory safe, but
  it prevents the destructor from ever running, so `drop()` will _not_ be
  called.

  [`Box::leak`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak)
  is another example of intentional leaking, often used to create data that
  lives for the remainder of the process.

- Undo the leak and uncomment the `panic!` just below it. What do you expect
  now?

  The stack will still unwind and the destructors will still run, even when the
  panic originates in `main`.

  - This is only true when compiling with the default `panic=unwind` config
    profile.

    With
    [`panic=abort`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic),
    no unwinding takes place.

- Finally, consider what happens when you also uncomment the `panic!` inside
  `Foo::drop`.

</details>
