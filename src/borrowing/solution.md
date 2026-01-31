# Solution

```rust,editable
{{#include ../../third_party/rust-on-exercism/health-statistics.rs:solution}}
```

- **Lifetimes in Structs:** `HealthReport` has a lifetime parameter `'a` because
  it contains a reference `patient_name: &'a str`. This ensures that the report
  cannot outlive the `User` it refers to.
- **Mutable Reference (`&mut self`):** `visit_doctor` modifies the `User`
  struct, so it must take `&mut self`.
- **Lifetime Elision:** The return type `HealthReport<'_>` indicates that the
  output lifetime is tied to the input lifetime of `self`. Explicitly, this
  would be `fn visit_doctor<'a>(&'a mut self, ...) -> HealthReport<'a>`.
- **Option combinators:** We use `self.last_blood_pressure.map(...)` to
  convenienty calculate the blood pressure change if the previous measurement
  exists.

<details>

- Explain that `HealthReport` borrows from `User`. While `report` exists, `User`
  is borrowed (mutably, because it came from `visit_doctor`), so we cannot use
  `User` for anything else until `report` is dropped.
- Note the cast to `i32` for blood pressure calculation to allow for negative
  changes.

</details>
