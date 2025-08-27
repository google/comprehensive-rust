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
    pub fn new() -> Self { Self { transaction: TransactionInterface() } }
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

- Aliasing XOR Mutability is a constraint that lets us model a bunch of non-cpu-bound-race-condition related circumstances.

- This is an instance of the "Aliasing XOR Mutability" being used to articulate "You can do X or you can do Y, but not both" in the API.

- TODO: Namedropping other things to work with this.

</details>