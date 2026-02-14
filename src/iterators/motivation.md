---
minutes: 3
---

<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Motivating Iterators

If you want to iterate over the contents of an array, you'll need to define:

- Some state to keep track of where you are in the iteration process, e.g. an
  index.
- A condition to determine when iteration is done.
- Logic for updating the state of iteration each loop.
- Logic for fetching each element using that iteration state.

In a C-style for loop you declare these things directly:

```c,editable
for (int i = 0; i < array_len; i += 1) {
    int elem = array[i];
}
```

In Rust we bundle this state and logic together into an object known as an
"iterator".

<details>

- This slide provides context for what Rust iterators do under the hood. We use
  the (hopefully) familiar construct of a C-style `for` loop to show how
  iteration requires some state and some logic, that way on the next slide we
  can show how an iterator bundles these together.

- Rust doesn't have a C-style `for` loop, but we can express the same thing with
  `while`:
  ```rust,editable
  # // Copyright 2024 Google LLC
  # // SPDX-License-Identifier: Apache-2.0
  #
  let array = [2, 4, 6, 8];
  let mut i = 0;
  while i < array.len() {
      let elem = array[i];
      i += 1;
  }
  ```

## More to Explore

There's another way to express array iteration using `for` in C and C++: You can
use a pointer to the front and a pointer to the end of the array and then
compare those pointers to determine when the loop should end.

```c,editable
for (int *ptr = array; ptr < array + len; ptr += 1) {
    int elem = *ptr;
}
```

If students ask, you can point out that this is how Rust's slice and array
iterators work under the hood (though implemented as a Rust iterator).

</details>
