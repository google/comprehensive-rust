---
minutes: 15
---

# Mutually Exclusive References, or "Aliasing XOR Mutability"

We can use the mutual exclusion of `&T` and `&mut T` references for a single
value to model some constraints.

```rust,editable,compile_fail
pub struct Transaction(/* specifics omitted */);
pub struct QueryResult(String);

pub struct DatabaseConnection {
    transaction: Transaction,
    query_results: Vec<QueryResult>,
}

impl DatabaseConnection {
    pub fn new() -> Self {
        Self {
            transaction: Transaction(/* again, specifics omitted */),
            query_results: vec![],
        }
    }
    pub fn get_transaction(&mut self) -> &mut Transaction {
        &mut self.transaction
    }
    pub fn results(&self) -> &[QueryResult] {
        &self.query_results
    }
    pub fn commit(&mut self) {
        /* Work omitted, including sending/clearing the transaction */
        println!("Transaction committed!")
    }
}

pub fn do_something_with_transaction(transaction: &mut Transaction) {}

fn main() {
    let mut db = DatabaseConnection::new();
    let mut transaction = db.get_transaction();
    do_something_with_transaction(transaction);
    let assumed_the_transactions_happened_immediately = db.results(); // ❌🔨
    do_something_with_transaction(transaction);
    // Works, as the lifetime of "transaction" as a reference ended above.
    let assumed_the_transactions_happened_immediately_again = db.results();
    db.commit();
}
```

<details>

- Aliasing XOR Mutability means "we can have multiple immutable references, a
  single mutable reference, but not both."

- This example shows how we can use the mutual exclusion of these kinds of
  references to dissuade a user from reading query results while using a
  transaction API.

  This might happen if the user is working under the false assumption that the
  queries being written to the transaction happen "immediately" rather than
  being queued up and performed together.

- By borrowing one field of a struct via a method that returns a mutable /
  exclusive reference we prevent access to the other fields of that struct under
  a shared / non-exclusive reference until the lifetime of that borrow ends.

  Note: This has to be via a method, as the compiler can reason about borrowing
  different fields in mutable/shared ways simultaneously if that borrowing is
  done manually.

  Demonstrate:

  - Change the instances of `db.get_transaction()` and `db.results()` to manual
    borrows (`&mut db.transaction` and `&db.query_results` respectively) to show
    the difference in what the borrow checker allows.

  - Put the non-`main` part of this example in a module to reiterate that this
    manual access is not possible across module boundaries.

- As laid out in [generalizing ownership](generalizing-ownership.md) we can look
  at the ways Mutable References and Shareable References interact to see if
  they fit with the invariants we want to uphold for an API.

- In this case, having the query results not public and placed behind a getter
  function, we can enforce the invariant "users of this API are not looking at
  the query results at the same time as they are writing to a transaction."

- The "don't look at query results while building a transaction" invariant can
  still be circumvented, how so?

  - The user could access the transaction solely through `db.get_transaction()`,
    leaving the lifetime too temporary to prevent access to `db.results()`.

  - How could we avoid this by working in other concepts from "Leveraging the
    Type System"?

</details>
