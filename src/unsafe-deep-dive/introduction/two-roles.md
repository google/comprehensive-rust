---
minutes: 5
---

# The unsafe keyword has two roles

1. _Creating_ APIs with safety considerations

   - unsafe functions: `unsafe fn get_unchecked(&self) { ... }`
   - unsafe traits: `unsafe trait Send {}`

2. _Using_ APIs with safety considerations

   - invoking built-in unsafe operators: `unsafe { *ptr }`
   - calling unsafe functions: `unsafe { x.get_unchecked() }`
   - implementing unsafe traits: `unsafe impl Send for Counter {}`

<details>

Two roles:

1. **Creating** APIs with safety considerations and defining what needs to be
   considered
2. **Using** APIs with safety considerations and confirming that the
   consideration has been made

### Creating APIs with safety considerations

“First, the unsafe keyword enables you to create APIs that can break Rust’s
safety guarantees. Specifically, you need to use the unsafe keyword when
defining unsafe functions and unsafe traits.

“When used in this role, you’re informing users of your API that they need to be
careful.”

“The creator of the API should communicate what care needs to be taken. Unsafe
APIs are not complete without documentation about safety requirements.. Callers
need to know that they have satisfied any requirements, and that’s impossible if
they’re not written down.”

### Using APIs with safety considerations

“The unsafe keyword adopts its other role, using APIs, when it is used nearby to
a curly brace.

“When used in this role, the unsafe keyword means that the author has been
careful. They have verified that the code is safe and is providing an assurance
to others.”

“Unsafe blocks are most common. They allow you to invoke unsafe functions that
have been defined using the first role.

“Unsafe blocks also allow you to perform operations which the compiler knows are
unsafe, such as dereferencing a raw pointer.”

“You might also see the unsafe keyword being used to implement unsafe traits.

</details>
