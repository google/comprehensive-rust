# Golden Rule – Callsite Clarity & Readability

```rust,editable
/// The magic function. Important for tax reasons.
fn my_magic(i: u32) -> f32 {
    i as f32 + 2.
}

/// The x function. Foundational for infrastructure reasons.
fn x(i: f32) -> String {
    format!("{:.2}", (i / 3.).fract())
}

/// Our business logic relies on this calculation for tax reasons,
/// regulatory reasons, or critical infrastructure reasons. So if you
/// see it do be careful about changing how it's handled!
fn taxes_and_infrastructure(input: u32) -> String {
    format!("{:.2}", ((input as f32 + 2.) / 3.).fract())
}

fn main() {
    println!("{}", x(my_magic(128)));
    println!("{}", taxes_and_infrastructure(128));
}
```

<details>

- Writing new code is often easier than reading code, so how can we make reading
  code easier? By making what is happening at a callsite of functions _as clear
  and readable as possible_ before a reader might start referring to the
  documentation of the functions and types at play.

- When reading a codebase, regardless of if you own it or not, you are far more
  likely to end up reading _calls_ of functions than the definitions of those
  functions themselves.

  This is true across languages, but the communities around Rust (including the
  development of the standard library) has settled on a set of things to value
  to keep the process of _reading code_ reliable in certain contexts.

- Ask before running: which function is more readable here, and why?

</details>
