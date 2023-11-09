---
minutes: 15
existing course material:
- exercises/day-1/morning.md
- exercises/day-1/afternoon.md
- exercises/day-2/morning.md
- exercises/day-2/afternoon.md
- exercises/day-3/morning.md
- exercises/day-3/afternoon.md
---

# Exercise: Collatz conjecture


Compute number of steps before an integer `n` becomes `1` following two rules:

- If `n` is even, set `n = n/2`
- If `n` is odd, set `n  = 3 * n + 1`

Given

fn collatz(n: i32) -> u32 {
  todo!("Implement")
}

#[test]
fn test_collatz() {
  assert_eq!(collatz(1), 0);
  assert_eq!(collatz(5), 5);
  assert_eq!(collatz(50), 24);
}

fill in the collatz function.

# Day 1: Morning Exercises

In these exercises, we will explore two parts of Rust:

* Implicit conversions between types.

* Arrays and `for` loops.

<details>

A few things to consider while solving the exercises:

* Use a local Rust installation, if possible. This way you can get
  auto-completion in your editor. See the page about [Using Cargo] for details
  on installing Rust.

* Alternatively, use the Rust Playground.

The code snippets are not editable on purpose: the inline code snippets lose
their state if you navigate away from the page.

After looking at the exercises, you can look at the [solutions] provided.

[solutions]: solutions-morning.md

[Using Cargo]: ../../cargo.md

</details>
# Day 1: Afternoon Exercises

We will look at two things:

* The Luhn algorithm,

* An exercise on pattern matching.

<details>

After looking at the exercises, you can look at the [solutions] provided.

[solutions]: solutions-afternoon.md

</details>
# Day 2: Morning Exercises

We will look at implementing methods in two contexts:

* Storing books and querying the collection

* Keeping track of health statistics for patients

<details>

After looking at the exercises, you can look at the [solutions] provided.

[solutions]: solutions-morning.md

</details>
# Day 2: Afternoon Exercises

The exercises for this afternoon will focus on strings and iterators.

<details>

After looking at the exercises, you can look at the [solutions] provided.

[solutions]: solutions-afternoon.md

</details>
# Day 3: Morning Exercises

We will design a classical GUI library using traits and trait objects.

We will also look at enum dispatch with an exercise involving points and polygons.

<details>

After looking at the exercises, you can look at the [solutions] provided.

[solutions]: solutions-morning.md

</details>
# Day 3: Afternoon Exercises

Let us build a safe wrapper for reading directory content!

For this exercise, we suggest using a local dev environment instead
of the Playground. This will allow you to run your binary on your own machine.

To get started, follow the [running locally] instructions.

[running locally]: ../../cargo/running-locally.md

<details>

After looking at the exercise, you can look at the [solution] provided.

[solution]: solutions-afternoon.md

</details>
