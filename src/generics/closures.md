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
    let mul_5 = |x| x * 5;

    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("mul_5: {}", apply_with_log(mul_5, 20));
}
```

<details>

If you have an `FnOnce`, you may only call it once. It might consume captured values.

An `FnMut` might mutate captured values, so you can call it multiple times but not concurrently.

An `Fn` neither consumes nor mutates captured values, or perhaps captures nothing at all, so it can
be called multiple times concurrently.

`FnMut` is a subtype of `FnOnce`. `Fn` is a subtype of `FnMut` and `FnOnce`. I.e. you can use an
`FnMut` wherever an `FnOnce` is called for, and you can use an `Fn` wherever an `FnMut` or `FnOnce`
is called for.

`move` closures only implement `FnOnce`.

</details>
