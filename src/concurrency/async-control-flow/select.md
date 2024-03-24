---
minutes: 5
---

# Select

A select operation waits until any of a set of futures is ready, and responds to
that future's result. In JavaScript, this is similar to `Promise.race`. In
Python, it compares to
`asyncio.wait(task_set, return_when=asyncio.FIRST_COMPLETED)`.

Similar to a match statement, the body of `select!` has a number of arms, each
of the form `pattern = future => statement`. When a `future` is ready, its
return value is destructured by the `pattern`. The `statement` is then run with
the resulting variables. The `statement` result becomes the result of the
`select!` macro.

```rust,editable,compile_fail
{{#include select.rs}}
```

<details>

- In this example, we have a race between a cat and a dog.
  `first_animal_to_finish_race` listens to both channels and will pick whichever
  arrives first. Since the dog takes 50ms, it wins against the cat that take
  500ms.

- You can use `oneshot` channels in this example as the channels are supposed to
  receive only one `send`.

- Try adding a deadline to the race, demonstrating selecting different sorts of
  futures.

- Note that `select!` drops unmatched branches, which cancels their futures. It
  is easiest to use when every execution of `select!` creates new futures.

  - An alternative is to pass `&mut future` instead of the future itself, but
    this can lead to issues, further discussed in the pinning slide.

</details>
