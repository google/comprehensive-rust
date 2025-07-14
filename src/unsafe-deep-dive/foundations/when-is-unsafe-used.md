---
minutes: 2
---

# When is unsafe used?

The unsafe keyword indicates that the programmer is responsible for upholding
Rust's safety guarantees.

The keyword has two roles:

- define pre-conditions must be satisfied
- verify that those defined pre-conditions are satisfied

## Further references

- [The unsafe keyword chapter of the Rust Reference](https://doc.rust-lang.org/reference/unsafe-keyword.html)

<details>

Places where pre-conditions can be defined (Role 1)

- [unsafe functions] (`unsafe fn { ... }`). Example: `get_unchecked` method on
  slices, which requires callers to verify that the index is in-bounds.
- unsafe traits (`unsafe trait`). Examples: [`Send`] and [`Sync`] marker traits
  in the standard library.

Places where pre-conditions must be satisfied (Role 2)

- unsafe blocks (`unafe { ... }`)
- implementing unsafe traits (`unsafe impl`)
- access external items (`unsafe extern`)
- adding
  [unsafe attributes](https://doc.rust-lang.org/reference/attributes.html) o an
  item. Examples: [`export_name`], [`link_section`] and [`no_mangle`]. Usage:
  `#[unsafe(no_mangle)]`

[unsafe functions]: https://doc.rust-lang.org/reference/unsafe-keyword.html#unsafe-functions-unsafe-fn
[unsafe traits]: https://doc.rust-lang.org/reference/unsafe-keyword.html#unsafe-traits-unsafe-trait
[`export_name`]: https://doc.rust-lang.org/reference/abi.html#the-export_name-attribute
[`link_section`]: https://doc.rust-lang.org/reference/abi.html#the-link_section-attribute
[`no_mangle`]: https://doc.rust-lang.org/reference/abi.html#the-no_mangle-attribute
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html

</details>
