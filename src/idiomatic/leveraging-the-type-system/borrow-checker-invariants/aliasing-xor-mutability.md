---
minutes: 0
---

# Mutually Exclusive References, or "Aliasing XOR Mutability"

We can use the mutual exclusion of `&T` and `&mut T` references for a single value to model some constraints.

```rust,editable
pub struct Transaction(/* some kind of interior state */);
pub struct QueryResult(String);

pub struct DatabaseConnection {
    transaction: Transaction,
    query_results: Vec<QueryResult>,
}

impl DatabaseConnection {
    pub fn new() -> Self { Self { transaction: Transaction(/* again, pretend there's some interior state */), query_results: vec![] } }
    pub fn get_transaction(&mut self) -> &mut Transaction { &mut self.transaction }
    pub fn results(&self) -> &[QueryResult] { &self.query_results }
    pub fn commit(&mut self) { println!("Transaction committed!") }
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

- Aliasing XOR Mutability means "we can have multiple immutable references, a single mutable reference, but not both."

- This example shows how we can use the mutual exclusion of these kinds of references to dissuade a user from reading query results while using a transaction API, something that might happen if the user is working under the false assumption that the queries being written to the transaction happen "immediately" rather than being queued up and performed together.

- By borrowing one field of a struct under a mutable / exclusive reference we prevent access to the other fields of that struct under a shared / non-exclusive reference until the lifetime of that borrow ends.

- As laid out in [generalizing ownership](generalizing-ownership.md) we can look at the ways Mutable References and Shareable References interact to see if they fit with the invariants we want to uphold for an API.

- In this case, having the query results not public and placed behind a getter function, we can enforce the invariant "users of this API are not looking at the query results at the same time as they are writing to a transaction."

<!-- Setup for Exercises -->
<details>
<summary>
The "don't look at query results while building a transaction" invariant can still be circumvented, how so?
</summary>
    <ul>
    <li>
    The user could access the transaction solely through `db.get_transaction()`, leaving the lifetime too temporary to prevent access to `db.results()`.
    </li>
    <li>
    How could we avoid this by working in other concepts from "Leveraging the Type System"?
    </li>
    </ul>
</details>

</details>