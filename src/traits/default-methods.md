# Default Methods

Traits can implement behavior in terms of other trait methods:

```rust,editable
trait Equals {
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equal(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equal(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));
}
```

<details>

* Traits may specify pre-implemented (default) methods and methods that users are required to
  implement themselves. Methods with default implementations can rely on required methods.

* Move method `not_equal` to a new trait `NotEqual`.

* Make `Equals` a super trait for `NotEqual`.
    ```rust,editable,compile_fail
    trait NotEqual: Equals {
        fn not_equal(&self, other: &Self) -> bool {
            !self.equal(other)
        }
    }
    ```

* Provide a blanket implementation of `NotEqual` for `Equal`.
    ```rust,editable,compile_fail
    trait NotEqual {
        fn not_equal(&self, other: &Self) -> bool;
    }

    impl<T> NotEqual for T where T: Equals {
        fn not_equal(&self, other: &Self) -> bool {
            !self.equal(other)
        }
    }
    ```
  * With the blanket implementation, you no longer need `Equals` as a super trait for `NotEqual`.
    
</details>
