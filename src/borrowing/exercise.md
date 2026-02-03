---
minutes: 20
---

# Exercise: Wizard's Inventory

In this exercise, you will manage a wizard's inventory using what you have
learned about borrowing and ownership.

- The wizard has a collection of spells. You need to implement functions to add
  spells to the inventory and to cast spells from them.

- Spells have a limited number of uses. When a spell has no uses left, it must
  be removed from the wizard's inventory.

```rust,editable,compile_fail
{{#include exercise.rs:setup}}

    // TODO: Implement `add_spell` to take ownership of a spell and add it to
    // the wizard's inventory.
    fn add_spell(..., spell: ...) {
        todo!()
    }

    // TODO: Implement `cast_spell` to borrow a spell from the inventory and
    // cast it. The wizard's mana should decrease by the spell's cost and the
    // number of uses for the spell should decrease by 1.
    //
    // If the wizard doesn't have enough mana, the spell should fail.
    // If the spell has no uses left, it is removed from the inventory.
    fn cast_spell(..., name: ...) {
        todo!()
    }
}

{{#include exercise.rs:main}}

{{#include exercise.rs:tests}}
```

<details>

- The goal of this exercise is to practice the core concepts of ownership and
  borrowing, specifically the rule that you cannot mutate a collection while
  holding a reference to one of its elements.
- `add_spell` should take ownership of a `Spell` and move it into the `Wizard`'s
  inventory.
- `cast_spell` is the core of the exercise. It needs to:
  1. Find the spell (by index or by reference).
  2. Check mana and decrement it.
  3. Decrement the spell's `uses`.
  4. Remove the spell if `uses == 0`.
- **Borrow Checker Conflict:** If students try to hold a reference to the spell
  (e.g., `let spell = &mut self.spells[i]`) and then call
  `self.spells.remove(i)` while that reference is still "alive" in the same
  scope, the borrow checker will complain. This is a great opportunity to show
  how to structure code to satisfy the borrow checker (e.g., by using indices or
  by ensuring the borrow ends before the mutation).

</details>
