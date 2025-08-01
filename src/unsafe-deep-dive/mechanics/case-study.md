# Case Study: Lesser-known parts of std::mem

As a group, we'll study some parts of Rust's memory management functionality:

- `std::mem::TransmuteFrom` trait and its `Assume` struct
- `std::mem::discriminant`
- `std::mem::forget_unsized`
- `std::mem::MaybeUninit<T>`

<details>

Split learners into small groups and ask them to look into the implementation of
one of the types above.

You may need to show learner how to view the source code of the standard
library.

After a few minutes, they should be able to answer the following questions:

- What is the purpose of the function/type/trait?
- How does the documentation describe the safety contract, if any? Can that
  documentation be improved?
- Were there any interesting parts in its implementation?

</details>
