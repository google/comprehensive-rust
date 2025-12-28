---
minutes: 15
---

# Problem solving: Break Down the Problem

```rust
// Problem: implementing a GUI API

// Question: What's the minimum useful behavior for a drawing API?
pub trait DrawApi {
    fn arc(&self, center: [f32; 2], radius: f32, start_angle: f32, end_angle: f32);
    fn line(&self, start: [f32; 2], end: [f32; 2]);
}

pub struct TextDraw;

impl DrawApi for TextDraw {
    fn arc(&self, center: [f32; 2], radius: f32, start_angle: f32, end_angle: f32) {
        println!("arc of radius ")
    }

    fn line(&self, start: [f32; 2], end: [f32; 2]) { /* ... */
    }
}

// Question: What's a good API for users?

pub trait Draw {
    fn draw<T: DrawApi>(&self, surface: &mut T);
}

pub struct Rect {
    start: [f32; 2],
    end: [f32; 2],
}

impl Draw for Rect {
    fn draw<T: DrawApi>(&self, surface: &mut T) {
        surface.line([self.start[0], self.start[1]], [self.end[0], self.start[1]]);
        surface.line([self.end[0], self.start[1]], [self.end[0], self.end[1]]);
        surface.line([self.end[0], self.end[1]], [self.start[0], self.end[1]]);
        surface.line([self.start[0], self.end[1]], [self.start[0], self.start[1]]);
    }
}
```

<details>
- You're already adept at breaking down problems, but you're likely used to reaching for OOP-style methods.

This isn't a drastic change, it just requires re-ordering the way you approach
things.

- Try to solve the problem with either Generics & Traits or Enums first.

  Does the problem require a specific set of types? An enum may be the cleanest
  way of solving this problem.

  Does the problem really care about the specifics of the types involved, or can
  behavior be focused on?

- Organize your problem solving around finding a minimum viable amount of
  knowledge to implement something.

  Does a trait already exist for this use case? If so, use it!

- If you really do need heterogeneous collections, use them! They exist in rust
  as a tool for a reason.

  Be aware of the XY problem: a problem may seem most easily addressable by one
  solution, but it might not tackle the root cause and could lead to new
  difficult problems popping up in the future.

  That is, be certain that dynamic dispatch with trait objects is what you need
  before you commit to using them.

  Be certain that traits are what you need before you commit to using them.

</details>
