---
minutes: 5
---

# Worked example Implementing `Drop` for `!Unpin` types

```rust,editable,ignore
use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;

thread_local! {
    static BATCH_FOR_PROCESSING: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[derive(Debug)]
struct CustomString(String);

#[derive(Debug)]
struct SelfRef {
    data: CustomString,
    ptr: *const CustomString,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(data: &str) -> Pin<Box<SelfRef>> {
        let mut boxed = Box::pin(SelfRef {
            data: CustomString(data.to_owned()),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });

        let ptr: *const CustomString = &boxed.data;
        unsafe {
            Pin::get_unchecked_mut(Pin::as_mut(&mut boxed)).ptr = ptr;
        }
        boxed
    }
}

impl Drop for SelfRef {
    fn drop(&mut self) {
        // SAFETY: Safe because we are reading bytes from a String
        let payload = unsafe { std::ptr::read(&self.data) };
        BATCH_FOR_PROCESSING.with(|log| log.borrow_mut().push(payload.0));
    }
}

fn main() {
    let pinned = SelfRef::new("Rust 🦀");
    drop(pinned);

    BATCH_FOR_PROCESSING.with(|batch| {
        println!("Batch: {:?}", batch.borrow());
    });
}
```

<details>

This example uses the `Drop` trait to add data for some post-processing, such as
telemetry or logging.

**The Safety comment is incorrect.** `ptr::read` creates a bitwise copy, leaving
`self.data` in an invalid state. `self.data` will be dropped again at the end of
the method, which is a double free.

Ask the class for fix the code.

**Suggestion 0: Redesign**

Redesign the post-processing system to work without `Drop`.

**Suggestion 1: Clone**

Using `.clone()` is an obvious first choice, but it allocates memory.

```rust,ignore
impl Drop for SelfRef {
    fn drop(&mut self) {
        let payload = self.data.0.clone();
        BATCH_FOR_PROCESSING.with(|log| log.borrow_mut().push(payload));
    }
}
```

**Suggestion 2: ManuallyDrop**

Wrapping `CustomString` in `ManuallyDrop` prevents the (second) automatic drop
at the end of the `Drop` impl.

```rust,ignore
struct SelfRef {
    data: ManuallyDrop<CustomString>,
    ptr: *const CustomString,
    _pin: PhantomPinned,
}

// ...

impl Drop for SelfRef {
    fn drop(&mut self) {
        // SAFETY: self.data
        let payload = unsafe { ManuallyDrop::take(&mut self.data) };
        BATCH_FOR_PROCESSING.with(|log| log.borrow_mut().push(payload.0));
    }
}
```

</details>
