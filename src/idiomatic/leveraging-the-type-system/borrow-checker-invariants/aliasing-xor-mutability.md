---
minutes: 15
---

# Mutually Exclusive References / "Aliasing XOR Mutability"

We can use the mutual exclusion of `&T` and `&mut T` references for a single
value to model some constraints.

```rust,editable
pub struct QueryResult;
pub struct DatabaseConnection {/* fields omitted */}

impl DatabaseConnection {
    pub fn new() -> Self {
        Self {}
    }
    pub fn results(&self) -> &[QueryResult] {
        &[] // fake results
    }
}

pub struct Transaction<'a> {
    connection: &'a mut DatabaseConnection,
}

impl<'a> Transaction<'a> {
    pub fn new(connection: &'a mut DatabaseConnection) -> Self {
        Self { connection }
    }
    pub fn query(&mut self, _query: &str) {
        // Send the query over, but don't wait for results.
    }
    pub fn commit(self) {
        // Finish executing the transaction and retrieve the results.
    }
}

fn main() {
    let mut db = DatabaseConnection::new();

    // The transaction `tx` mutably borrows `db`.
    let mut tx = Transaction::new(&mut db);
    tx.query("SELECT * FROM users");

    // This won't compile because `db` is already mutably borrowed.
    // let results = db.results(); // ❌🔨

    // The borrow of `db` ends when `tx` is consumed by `commit`.
    tx.commit();

    // Now it is possible to borrow `db` again.
    let results = db.results();
}
```

<details>

- Motivation: When working with a database API, a user might imagine that
  transactions are being committed "as they go" and try to read results in
  between queries being added to the transaction. This fundamental misuse of the
  API could lead to confusion as to why nothing is happening.

  While an obvious misunderstanding, situations such as this can happen in
  practice.

  Ask: Has anyone misunderstood an API by not reading the docs for proper use?

  Expect: Examples of early-career or in-university mistakes and
  misunderstandings.

  As an API grows in size and user base, a smaller percentage may have "total"
  knowledge of the system the API represents.

- This example shows how we can use Aliasing XOR Mutability prevent this kind of
  misuse

  This might happen if the user is working under the false assumption that the
  queries being written to the transaction happen "immediately" rather than
  being queued up and performed together.

- The constructor for the Transaction type takes a mutable reference to the
  database connection, which it holds onto that reference.

  The explicit lifetime here doesn't have to be intimidating, it just means
  "`Transaction` is outlived by the `DatabaseConnection` that was passed to it"
  in this case.

  The `mut` keyword in the type lets us determine that there is just one of
  these references present per variable of type `DatabaseConnection`.

- While a `Transaction` exists, we can't touch the `DatabaseConnection` variable
  that was created from it.

  Demonstrate: uncomment the `db.results()` line.

- This lifetime parameter for `Transaction` needs to come from somewhere, in
  this case it is derived from the lifetime of the owned `DatabaseConnection`
  from which an exclusive reference is being passed.

- As laid out in [generalizing ownership](generalizing-ownership.md) and
  [the opening slide for this section](../borrow-checker-invariants.md) we can
  look at the ways Mutable References and Shareable References interact to see
  if they fit with the invariants we want to uphold for an API.

- Note: The query results not being public and placed behind a getter function
  lets us enforce the invariant "users can only look at query results if they
  are not also writing to a transaction."

  If they're publicly available to the user outside of the definition module
  then this invariant can be invalidated.

</details>
