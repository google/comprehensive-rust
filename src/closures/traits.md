---
minutes: 10
---

# Closure traits

Closures or lambda expressions have types which cannot be named. However, they
implement special [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html),
[`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html), and
[`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) traits:

The special types `fn(..) -> T` refer to function pointers - either the address
of a function, or a closure that captures nothing.

```rust,editable
fn apply_and_log(
    func: impl FnOnce(&'static str) -> String,
    func_name: &'static str,
    input: &'static str,
) {
    println!("Calling {func_name}({input}): {}", func(input))
}

fn main() {
    let suffix = "-itis";
    let add_suffix = |x| format!("{x}{suffix}");
    apply_and_log(&add_suffix, "add_suffix", "senior");
    apply_and_log(&add_suffix, "add_suffix", "appenix");

    let mut v = Vec::new();
    let mut accumulate = |x| {
        v.push(x);
        v.join("/")
    };
    apply_and_log(&mut accumulate, "accumulate", "red");
    apply_and_log(&mut accumulate, "accumulate", "green");
    apply_and_log(&mut accumulate, "accumulate", "blue");

    let take_and_reverse = |prefix| {
        let mut acc = String::from(prefix);
        acc.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
        acc
    };
    apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");
}
```

<details>

An `Fn` (e.g. `add_suffix`) neither consumes nor mutates captured values. It can
be called needing only a shared reference to the closure, which means the
closure can be executed repeatedly and even concurrently.

An `FnMut` (e.g. `accumulate`) might mutate captured values. The closure object
is accessed via exclusive reference, so it can be called repeatedly but not
concurrently.

If you have an `FnOnce` (e.g. `take_and_reverse`), you may only call it once.
Doing so consumes the closure and any values captured by move.

`FnMut` is a subtype of `FnOnce`. `Fn` is a subtype of `FnMut` and `FnOnce`.
I.e. you can use an `FnMut` wherever an `FnOnce` is called for, and you can use
an `Fn` wherever an `FnMut` or `FnOnce` is called for.

When you define a function that takes a closure, you should take `FnOnce` if you
can (i.e. you call it once), or `FnMut` else, and last `Fn`. This allows the
most flexibility for the caller.

In contrast, when you have a closure, the most flexible you can have is `Fn`
(which can be passed to a consumer of any of the 3 closure traits), then
`FnMut`, and lastly `FnOnce`.

The compiler also infers `Copy` (e.g. for `add_suffix`) and `Clone` (e.g.
`take_and_reverse`), depending on what the closure captures. Function pointers
(references to `fn` items) implement `Copy` and `Fn`.

</details>
