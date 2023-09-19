# Example

```rust,editable
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {  // No receiver, a static method
        Race { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {  // Exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {  // Shared and read-only borrowed access to self
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {  // Exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}
```

<details>
    
Key Points:
* All four methods here use a different method receiver.
  * You can point out how that changes what the function can do with the variable values and if/how it can be used again in `main`.
  * You can showcase the error that appears when trying to call `finish` twice.
* Note that although the method receivers are different, the non-static functions are called the same way in the main body. Rust enables automatic referencing and dereferencing when calling methods. Rust automatically adds in the `&`, `*`, `muts` so that that object matches the method signature.
* You might point out that `print_laps` is using a vector that is iterated over. We describe vectors in more detail in the afternoon. 

</details>
