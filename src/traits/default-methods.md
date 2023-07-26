# Default Methods

Traits can implement behavior in terms of other trait methods:

```rust,editable
trait Equals {
    fn equals(&self, other: &Self) -> bool;
    fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equals(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equals(&b));
}
```

<details>

* Traits may specify pre-implemented (default) methods and methods that users are required to
  implement themselves. Methods with default implementations can rely on required methods.

* Move method `not_equals` to a new trait `NotEquals`.

* Make `Equals` a super trait for `NotEquals`.
    ```rust,editable,compile_fail
    trait NotEquals: Equals {
        fn not_equals(&self, other: &Self) -> bool {
            !self.equals(other)
        }
    }
    ```

* Provide a blanket implementation of `NotEquals` for `Equals`.
    ```rust,editable,compile_fail
    trait NotEquals {
        fn not_equals(&self, other: &Self) -> bool;
    }

    impl<T> NotEquals for T where T: Equals {
        fn not_equals(&self, other: &Self) -> bool {
            !self.equals(other)
        }
    }
    ```
  * With the blanket implementation, you no longer need `Equals` as a super trait for `NotEqual`.
    
</details>
