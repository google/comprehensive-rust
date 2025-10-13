---
minutes: 10
---

# Using the Borrow checker to enforce Invariants

The logic of the borrow checker, while tied to "memory ownership", can be
abstracted away from this central use case to model other problems and prevent
API misuse.

```rust,editable
// Doors can be open or closed, and you need the right key to lock or unlock
// one. Modelled with a Shared key and Owned door.
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

<!-- TODO: link to typestate when that gets merged. -->

- The borrow checker has been used to prevent use-after-free and multiple
  mutable references up until this point, and we've used types to shape and
  restrict use of APIs already using
  [the Typestate pattern](../leveraging-the-type-system/typestate-pattern.md).

- This example uses the ownership & borrowing rules to model the locking and
  unlocking of a door. We can try to open a door with a key, but if it's the
  wrong key the door is still closed (here represented as an error) and the key
  persists regardless.

- The rules of the borrow checker exist to prevent developers from accessing,
  changing, and holding onto data in memory in unpredictable ways without being
  so restrictive that it would prevent _writing software_. The underlying
  logical system does not "know" what memory is. All it does is enforce a
  specific set of rules of how different operations affect what later operations
  are possible.

- Those rules can apply to many other cases: We can piggy-back onto the rules of
  the borrow checker to design APIs to be harder or impossible to misuse, even
  when there's little or no "memory safety" concerns in the problem domain. This
  section will walk through some of those different domains.

</details>
