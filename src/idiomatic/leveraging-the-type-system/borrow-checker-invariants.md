---
minutes: 15
---

# Using the Borrow checker to enforce Invariants

The borrow checker, while added to enforce memory ownership, can model other
problems and prevent API misuse.

```rust,editable
/// Doors can be open or closed, and you need the right key to lock or unlock
/// one. Modelled with a Shared key and Owned door.
pub struct DoorKey {
    pub key_shape: u32,
}
pub struct LockedDoor {
    lock_shape: u32,
}
pub struct OpenDoor {
    lock_shape: u32,
}

fn open_door(key: &DoorKey, door: LockedDoor) -> Result<OpenDoor, LockedDoor> {
    if door.lock_shape == key.key_shape {
        Ok(OpenDoor { lock_shape: door.lock_shape })
    } else {
        Err(door)
    }
}

fn close_door(key: &DoorKey, door: OpenDoor) -> Result<LockedDoor, OpenDoor> {
    if door.lock_shape == key.key_shape {
        Ok(LockedDoor { lock_shape: door.lock_shape })
    } else {
        Err(door)
    }
}

fn main() {
    let key = DoorKey { key_shape: 7 };
    let closed_door = LockedDoor { lock_shape: 7 };
    let opened_door = open_door(&key, closed_door);
    if let Ok(opened_door) = opened_door {
        println!("Opened the door with key shape '{}'", key.key_shape);
    } else {
        eprintln!(
            "Door wasn't opened! Your key only opens locks with shape '{}'",
            key.key_shape
        );
    }
}
```

<details>

- We've seen the borrow checker prevent memory safety bugs (use-after-free, data
  races).

- We've also used types to shape and restrict APIs already using
  [the Typestate pattern](../leveraging-the-type-system/typestate-pattern.md).

- Language features are often introduced for a specific purpose.

  Over time, users may develop ways of using a feature in ways that were not
  predicted when they were introduced.

  In 2004, Java 5 introduced Generics with the
  [main stated purpose of enabling type-safe collections](https://jcp.org/en/jsr/detail?id=14).

  Since then, users and developers of the language expanded the use of generics
  to other areas of type-safe API design.
  <!-- TODO: Reference how this was adopted -->

  What we aim to do here is similar: Even though the borrow checker was
  introduced to prevent use-after-free and data races, it is just another API
  design tool. It can be used to model program properties that have nothing to
  do with preventing memory safety bugs.

- To use the borrow checker as a problem solving tool, we will need to "forget"
  that the original purpose of it is to prevent mutable aliasing in the context
  of preventing use-after-frees and data races.

  We should imagine working within situations where the rules are the same but
  the meaning is slightly different.

- This example uses ownership and borrowing are used to model the state of a
  physical door.

  `open_door` **consumes** a `LockedDoor` and returns a new `OpenDoor`. The old
  `LockedDoor` value is no longer available.

  If the wrong key is used, the door is left locked. It is returned as an `Err`
  case of the `Result`.

  It is a compile-time error to try and use a door that has already been opened.

- Similarly, `lock_door` consumes an `OpenDoor`, preventing closing the door
  twice at compile time.

- The rules of the borrow checker exist to prevent memory safety bugs, but the
  underlying logical system does not "know" what memory is.

  All the borrow checker does is enforce a specific set of rules of how users
  can order operations.

  This is just one case of piggy-backing onto the rules of the borrow checker to
  design APIs to be harder or impossible to misuse.

</details>
