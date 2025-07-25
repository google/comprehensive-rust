# Smart constructors

> TODO(timclicks): Think of a better type name; expand details

```rust,ignore
impl ForeignRefCount {
    fn new(...) {
        // ..
    }

    unsafe fn incr(&mut self) {
        // ...
    }

    unsafe fn decr(&mut self) {
        // ...
    }
}
```

```rust,ignore
impl ForeignRefCount {
    unsafe fn new_unchchecked(...) {
        // ..
    }

    fn incr(&mut self) {
        // ...
    }

    fn decr(&mut self) {
        // ...
    }
}
```

<details>

It is tedious to check invariants at every call during an object's life.
Instead, you can provide a `new_unchecked` method which provides an opportunity
for the invariants to be checked once and then later relied upon.

</details>
