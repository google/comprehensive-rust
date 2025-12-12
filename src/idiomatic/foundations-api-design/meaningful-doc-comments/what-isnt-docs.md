---
minutes: 5
---

Names and Signatures are not full documentation

```rust
// bad  
/// Returns a future that resolves when operation completes.  
fn syncToServer() -> Future<Bool>  

// good  
/// Sends local edits to the server, overwriting concurrent edits  
/// if any happened.  
fn syncToServer() -> Future<Bool>  
// bad  
/// Returns an error if sending the email fails.  
fn send(&self, email: Email) -> Result<(), Error>  

// good  
/// Queues the email for background delivery and returns immediately.  
/// Returns an error immediately if the email is malformed.  
fn send(&self, email: Email) -> Result<(), Error>
```

<details>

- Motivation: API designers can over-commit to the idea that a function name and
  signature is enough documentation.

It's better than nothing, but it's worse than good documentation.

- Again, names and types are _part_ of the documentation. They are not always
  the full story!

- Consider the behavior of functions that are not covered by the name, parameter
  names, or signature of that function.

  In the example on the slide it is not obvious that `syncToServer()` could
  overwrite something (leading to a data loss), so document that.

  In the email example, it is not obvious that the function can return success
  and still fail to deliver the email.

- Use comments to disambiguate. Nuanced behaviors, behaviors that users of an
  API could trip up on, should be documented.

  For example, consider a remove() method on a business entity: There are many
  ways to remove an entity!

  Is it removing the entity from the database? From the parent collection in
  memory (unlink vs erase)?

  If it is removing the data in the database, is the data actually being
  deleted, or merely marked as deleted, but still recoverable (soft vs hard
  delete)?

</details>
