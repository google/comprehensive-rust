---
minutes: 2
---

# Heterogeneous data with `dyn trait`

```rust
pub struct Lambda;

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "λ")
    }
}

pub struct Heterogeneous {
    pub collection: Vec<Box<dyn std::fmt::Display>>,
}

fn main() {
    let heterogeneous = Heterogeneous {
        collection: vec![
            Box::new(42u32),
            Box::new("Woah".to_string()),
            Box::new(Lambda),
        ],
    };
    for item in heterogeneous.collection {
        // We know "item" implements Display, but we know nothing else!
        println!("Display output: {}", item);
    }
}
```

<details>

- `dyn Trait`, being a dynamic dispatch tool, lets us store heterogeneous data
  in collections.

- In this example, we're storing types that all implement `std::fmt::Display`
  and printing all items in that collection to screen.

</details>
