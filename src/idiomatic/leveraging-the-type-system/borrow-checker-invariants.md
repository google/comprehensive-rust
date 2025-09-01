---
minutes: 10
---

# Using the Borrow checker to enforce Invariants

The logic of the borrow checker, while tied to "memory ownership", can be abstracted away from this central use case to model other problems and prevent API misuse.

```rust,editable
fn main() {
// Doors can be open or closed, and you need the right key to lock or unlock one.
// Modelled with Shared Key and Owned Door. Nothing to do with "memory safety"!
pub struct DoorKey { pub key_shape: u32 }
pub struct ClosedDoor { lock_shape: u32 }
pub struct OpenDoor { lock_shape: u32 }

fn open_door(key: &DoorKey, door: ClosedDoor) -> Result<OpenDoor, ClosedDoor> {
    if door.lock_shape == key.key_shape {
        Ok(OpenDoor{lock_shape: door.lock_shape})
    } else {
        Err(door)
    }
}

fn close_door(key: &DoorKey, door: OpenDoor) -> Result<ClosedDoor, OpenDoor> {
    if door.lock_shape == key.key_shape {
        Ok(ClosedDoor{lock_shape: door.lock_shape})
    } else {
        Err(door)
    }
}

let key = DoorKey{ key_shape: 7 };
let closed_door = ClosedDoor{ lock_shape: 7};
let opened_door = open_door(&key, closed_door);
if let Ok(opened_door) = opened_door {
    println!("Opened the door with key shape '{}'", key.key_shape);
} else {
    eprintln!("Door wasn't opened! Your key only opens locks with shape '{}'", key.key_shape);
}
}
```

<details>

- The borrow checker has been used to prevent use-after-free and multiple mutable references up until this point.

- This example uses the ownership & borrowing rules to model the opening and closing of a door. We can try to open a door with a key, but if it's the wrong key the door is still closed (here represented as an error.)

- The rules of the borrow checker fundamentally exist to prevent developers from accessing, changing, and holding onto data in memory in unpredictable ways without preventing _writing software_. The underlying logical system does not "know" what memory is. All it does is enforce a specific set of rules of how different operations affect what possible later operations are.

- Those rules can apply to many other cases, so we can piggy-back onto the rules of the borrow checker to design APIs to be harder or impossible to misuse. Even when there's little or no actual "memory safety" concerns in the problem domain.

- This section will walk through some of those different domains.

- Interior, private mutability or reference counting in data types may let an API designer shift the meaning of ownership to a different (but analogous) set of semantics as they need to, even to the point where some API designers have managed to model self-referential types this way.

</details>