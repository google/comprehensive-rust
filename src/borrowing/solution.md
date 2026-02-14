# Solution

```rust,editable
{{#include ../../third_party/rust-on-exercism/health-statistics.rs:solution}}
```

The solution explores how structs can capture references and how lifetimes tie
related data structures together:

- **Lifetimes in Structs:** `HealthReport` contains a reference
  `patient_name: &'a str`, which necessitates a lifetime parameter `'a`. This
  guarantees that a report cannot outlive the `User` it was created from.
- **Mutable Borrows:** `visit_doctor` takes `&mut self` to update the user's
  stats. Because it returns a `HealthReport` that borrows from `self`, the
  report's lifetime is tied to the duration of this borrow.
- **Idiomatic `Option` Handling:** `self.last_blood_pressure.map(...)` is used
  to concisely compute the change in blood pressure only when a prior value is
  available.

<details>

- **Lifetime Elision:** The signature
  `fn visit_doctor(&mut self, ...) -> HealthReport<'_>` uses anonymous
  lifetimes. The compiler expands this to indicate that the returned report
  borrows from `self`, effectively making the user inaccessible while the report
  is in scope.
- **Borrows and Mutation:** Inside `visit_doctor`, the report must be created
  _before_ the user's fields are updated, as creating a reference to `self.name`
  borrows `self`. In Rust, you cannot modify a value while it is borrowed.

</details>
