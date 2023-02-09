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
        println!("We've know about {} books, but not Les Misérables.",
                 page_counts.len());
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown.")
        }
    }
}
```


<details>

* The HashMap is not defined in the prelude and needs to be brought into scope.
* In a hashmap, types with the `Copy` trait (`i32`) are copied into the hashmap, and owned values (`String`) are moved into the hashmap.
* Try the following lines of code. The first line will see if a book is in the hashmap and if not return an alternative value. The second line will insert the alternative value in the hashmap if the book is not found.

  ```rust,ignore
  let pc1 = page_counts
      .get("Harry Potter and the Sorcerer's Stone ")
      .unwrap_or(&336);
  let pc2 = page_counts
      .entry("The Hunger Games".to_string())
      .or_insert(374);
  ```

</details>
