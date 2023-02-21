# `HashMap`

Standard hash map with protection against HashDoS attacks:

```rust,editable
use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!("We know about {} books, but not Les Misérables.",
                 page_counts.len());
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown.")
        }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");
}
```

<details>

* `HashMap` is not defined in the prelude and needs to be brought into scope.
* Try the following lines of code. The first line will see if a book is in the hashmap and if not return an alternative value. The second line will insert the alternative value in the hashmap if the book is not found.

  ```rust,ignore
  let pc1 = page_counts
      .get("Harry Potter and the Sorcerer's Stone ")
      .unwrap_or(&336);
  let pc2 = page_counts
      .entry("The Hunger Games".to_string())
      .or_insert(374);
  ```
* Unlike `vec!`, there is unfortunately no standard `hashmap!` macro.
  * Although, since Rust 1.56, HashMap implements [`From<[(K, V); N]>`][1], which allows us to easily initialize a hash map from a literal array:

  ```rust,ignore
  let page_counts = HashMap::from([
    ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
    ("The Hunger Games".to_string(), 374),
  ]);
  ```

 * Alternatively HashMap can be built from any `Iterator` which yields key-value tuples.
* We are showing `HashMap<String, i32>`, and avoid using `&str` as key to make examples easier. Using references in collections can, of course, be done,
  but it can lead into complications with the borrow checker.
  * Try removing `to_string()` from the example above and see if it still compiles. Where do you think we might run into issues?

[1]: https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#impl-From%3C%5B(K%2C%20V)%3B%20N%5D%3E-for-HashMap%3CK%2C%20V%2C%20RandomState%3E

</details>
