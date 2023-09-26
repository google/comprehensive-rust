# Implicit Conversions

Rust will not automatically apply _implicit conversions_ between types ([unlike
C++][3]). You can see this in a program like this:

<!-- mdbook-xgettext: skip -->
```rust,editable,compile_fail
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}
```

The Rust integer types all implement the [`From<T>`][1] and [`Into<T>`][2]
traits to let us convert between them. The `From<T>` trait has a single `from()`
method and similarly, the `Into<T>` trait has a single `into()` method.
Implementing these traits is how a type expresses that it can be converted into
another type.

The standard library has an implementation of `From<i8> for i16`, which means
that we can convert a variable `x` of type `i8` to an `i16` by calling 
`i16::from(x)`. Or, simpler, with `x.into()`, because `From<i8> for i16`
implementation automatically create an implementation of `Into<i16> for i8`.

The same applies for your own `From` implementations for your own types, so it is
sufficient to only implement `From` to get a respective `Into` implementation automatically.

1. Execute the above program and look at the compiler error.

2. Update the code above to use `into()` to do the conversion.

3. Change the types of `x` and `y` to other things (such as `f32`, `bool`,
   `i128`) to see which types you can convert to which other types. Try
   converting small types to big types and the other way around. Check the
   [standard library documentation][1] to see if `From<T>` is implemented for
   the pairs you check.

[1]: https://doc.rust-lang.org/std/convert/trait.From.html
[2]: https://doc.rust-lang.org/std/convert/trait.Into.html
[3]: https://en.cppreference.com/w/cpp/language/implicit_conversion
