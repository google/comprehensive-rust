---
minutes: 0
---

# Aliasing XOR Mutability

We can use the mutual exclusion of `&T` and `&mut T` references for a single value to model some constraints.

```rust,editable
fn main() {
pub struct TransactionInterface(/* some kind of interior state */);

pub struct DatabaseConnection {
    transaction: TransactionInterface,
}

impl DatabaseConnection {
    pub fn new() -> Self { Self { transaction: TransactionInterface(/* again, pretend there's some interior state */) } }
    pub fn get_transaction(&self) -> &TransactionInterface { &self.transaction }
    pub fn commit(&mut self) {}
}

pub fn do_something_with_transaction(transaction: &TransactionInterface) {}

let mut db = DatabaseConnection::new();
let transaction = db.get_transaction();
do_something_with_transaction(transaction);
db.commit();
do_something_with_transaction(transaction); // 🛠️❌
}
```

<details>

- This example shows how we can use the "Aliasing XOR Mutability" rule when it comes to shared and mutable references to model safe access to transactions for a database. This is a loose sketch of such a model, and rust database frameworks you use may not look anything like this in practice.

- As laid out in [generalizing ownership]("./generalizing-ownership.md") we can look at the ways Mutable References and Shareable References interact to see if they fit with the invariants we want to uphold for an API.

- Here we want to be able to write to a transaction, which has some internal breaking of rust rules we don't concern ourselves with, before then committing that transaction.

- By having the "commit transaction" method take a mutable reference, regardless of if mutation is happening, the borrow checker prevents references to the internal transaction surface persisting between calls to that method.

- The transaction itself can be modelled with a shareable reference, not necessarily because the interior state stays the same while we use it but because this prevents using the "commit transaction" functionality until the transaction is "over."

<!-- Entirely reasonable to reframe the example off this contradiction, but I think it has pedagogical value regardless. -->
- Tangential: We could instead have the `get_transaction` method return a mutable reference off a mutable reference to self (`fn get_transaction(&mut self) -> &mut TransactionInterface`) but we're trying to show off the ways shareable and mutable references exclude each other here.

</details>