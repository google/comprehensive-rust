# Enums vs `dyn`

Enums and `dyn` provide two different approaches to dynamic polymorphism, and in
many cases both approaches can be used to solve the same problem. The main
trade-offs between the two are:

- Enums make "downcasting" to a subtype easy via pattern matching. Downcasting
  is possible with `dyn` but is more cumbersome.
- `dyn` allows for downstream code to introduce new types, whereas enums do not.

<details>

- When deciding whether to use an enum or `dyn`, there are two questions to ask:

  - Do I need to be able to downcast to the concrete subtype? Or do I primarily
    expect to go through a trait interface without needing to know the concrete
    type?

  - Do I know the full set of types up front, or do I need to allow downstream
    code to extend the set of types I will be handling?

</details>
