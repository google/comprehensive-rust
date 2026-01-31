---
minutes: 10
---

# Pitfall: Reaching too quickly for `dyn Trait`

```rust
use std::any::Any;

pub trait AddDyn: Any {
    fn add_dyn(&self, rhs: &dyn AddDyn) -> Box<dyn AddDyn>;
}

impl AddDyn for i32 {
    fn add_dyn(&self, rhs: &dyn AddDyn) -> Box<dyn AddDyn> {
        if let Some(downcast) = (rhs as &dyn Any).downcast_ref::<Self>() {
            Box::new(self + downcast)
        } else {
            Box::new(*self)
        }
    }
}

fn main() {
    let i: &dyn AddDyn = &42;
    let j: &dyn AddDyn = &64;
    let k: Box<dyn AddDyn> = i.add_dyn(j);
    dbg!((k.as_ref() as &dyn Any).is::<i32>());
    dbg!((k.as_ref() as &dyn Any).downcast_ref::<i32>());
}
```

<details>

- Coming from an OOP background, it's understandable to reach for this dynamic
  dispatch tool as early as possible.

- This is not the preferred way of doing things, trait objects put us in a
  situation where we're exchanging knowledge of a type that both the developer
  and compiler has for flexibility.

- The above example takes things to the absurd: If adding numbers were tied up
  in the dynamic dispatch process, it would be difficult to do anything at all.

  But dynamic dispatch is often hidden in many programming languages: here's it
  is more explicit.

  In the `i32` implementation of `AddDyn`, first we need to attempt to downcast
  the `rhs` argument to the same type as `i32`, silently failing if this isn't
  the case.

  Then we need to allocate the new value on the heap, because if we're keeping
  this in the world of dynamic dispatch then we need to do this.

  Once we've added two values together, if we want to view them we must downcast
  them again into a "real" type we can print out given the trait bounds tied up
  in the operation so far.

- Ask the class: Why can't we just add Display bounds in `main` to be able to
  print things as-is?

  Answer: Because add_dyn returns only a `dyn AddDyn`, we lose information about
  what the type implements between the argument type and return type. Even if
  the inputs implement `Display`, the return type does not.

- This leads to less performant code which is harder to understand

</details>
