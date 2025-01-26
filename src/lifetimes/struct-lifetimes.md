---
minutes: 5
---

# Lifetimes in Data Structures

If a data type stores borrowed data, it must be annotated with a lifetime:

```rust,editable
#[derive(Debug)]
enum HighlightColor {
    Pink,
    Yellow,
}

#[derive(Debug)]
struct Highlight<'document> {
    slice: &'document str,
    color: HighlightColor,
}

fn main() {
    let doc = String::from("The quick brown fox jumps over the lazy dog.");
    let noun = Highlight { slice: &doc[16..19], color: HighlightColor::Yellow };
    let verb = Highlight { slice: &doc[20..25], color: HighlightColor::Pink };
    // drop(doc);
    println!("{noun:?}");
    println!("{verb:?}");
}
```

<details>

- In the above example, the annotation on `Highlight` enforces that the data
  underlying the contained `&str` lives at least as long as any instance of
  `Highlight` that uses that data. A struct cannot live longer than the data it
  references.
- If `doc` is dropped before the end of the lifetime of `noun` or `verb`, the
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
