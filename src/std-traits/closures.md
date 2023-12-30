---
minutes: 20
---

# Closures

Closures or lambda expressions have types which cannot be named. However, they
implement special [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html),
[`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html), and
[`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) traits:

```rust,editable
fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn main() {
    let add_3 = |x| x + 3;
    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("add_3: {}", apply_with_log(add_3, 20));

    let mut v = Vec::new();
    let mut accumulate = |x: i32| {
        v.push(x);
        v.iter().sum::<i32>()
    };
    println!("accumulate: {}", apply_with_log(&mut accumulate, 4));
    println!("accumulate: {}", apply_with_log(&mut accumulate, 5));

    let multiply_sum = |x| x * v.into_iter().sum::<i32>();
    println!("multiply_sum: {}", apply_with_log(multiply_sum, 3));
}
```

<details>

An `Fn` (e.g. `add_3`) neither consumes nor mutates captured values, or perhaps
captures nothing at all. It can be called multiple times concurrently.

An `FnMut` (e.g. `accumulate`) might mutate captured values. You can call it
multiple times, but not concurrently.

If you have an `FnOnce` (e.g. `multiply_sum`), you may only call it once. It
might consume captured values.

`FnMut` is a subtype of `FnOnce`. `Fn` is a subtype of `FnMut` and `FnOnce`.
I.e. you can use an `FnMut` wherever an `FnOnce` is called for, and you can use
an `Fn` wherever an `FnMut` or `FnOnce` is called for.

When you define a function that takes a closure, you should take `FnOnce` if you
can (i.e. you call it once), or `FnMut` else, and last `Fn`. This allows the
most flexibility for the caller.

In contrast, when you have a closure, the most flexible you can have is `Fn` (it
can be passed everywhere), then `FnMut`, and lastly `FnOnce`.

The compiler also infers `Copy` (e.g. for `add_3`) and `Clone` (e.g.
`multiply_sum`), depending on what the closure captures.

By default, closures will capture by reference if they can. The `move` keyword
makes them capture by value.

```rust,editable
fn make_greeter(prefix: String) -> impl Fn(&str) {
    return move |name| println!("{} {}", prefix, name);
}

fn main() {
    let hi = make_greeter("Hi".to_string());
    hi("there");
}
```

</details>
