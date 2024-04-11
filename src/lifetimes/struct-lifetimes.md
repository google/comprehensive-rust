---
minutes: 5
---

# Lifetimes in Data Structures

If a data type stores borrowed data, it must be annotated with a lifetime:

```rust,editable
#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
}
```

<details>

- In the above example, the annotation on `Highlight` enforces that the data
  underlying the contained `&str` lives at least as long as any instance of
  `Highlight` that uses that data.
- If `text` is consumed before the end of the lifetime of `fox` (or `dog`), the
  borrow checker throws an error.
- Types with borrowed data force users to hold on to the original data. This can
  be useful for creating lightweight views, but it generally makes them somewhat
  harder to use.
- When possible, make data structures own their data directly.
- Some structs with multiple references inside can have more than one lifetime
  annotation. This can be necessary if there is a need to describe lifetime
  relationships between the references themselves, in addition to the lifetime
  of the struct itself. Those are very advanced use cases.

</details>
