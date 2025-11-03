---
minutes: 15
---

# Mutually Exclusive References / "Aliasing XOR Mutability"

We can use the mutual exclusion of `&T` and `&mut T` references to prevent data
from being used before it is ready.

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

- Motivation: In this database API queries are kicked off for asynchronous
  execution and the results are only available once the whole transaction is
  finished.

  A user might think that queries are executed immediately, and try to read
  results before they are made available. This API misuse could make the app
  read incomplete or incorrect data.

  While an obvious misunderstanding, situations such as this can happen in
  practice.

  Ask: Has anyone misunderstood an API by not reading the docs for proper use?

  Expect: Examples of early-career or in-university mistakes and
  misunderstandings.

  As an API grows in size and user base, a smaller percentage of users has deep
  knowledge of the system the API represents.

- This example shows how we can use Aliasing XOR Mutability to prevent this kind
  of misuse.

- The code might read results before they are ready if the programmer assumes
  that the queries execute immediately rather than kicked off for asynchronous
  execution.

- The constructor for the `Transaction` type takes a mutable reference to the
  database connection, and stores it in the returned `Transaction` value.

  The explicit lifetime here doesn't have to be intimidating, it just means
  "`Transaction` is outlived by the `DatabaseConnection` that was passed to it"
  in this case.

  The reference is mutable to completely lock out the `DatabaseConnection` from
  other usage, such as starting further transactions or reading the results.

- While a `Transaction` exists, we can't touch the `DatabaseConnection` variable
  that was created from it.

  Demonstrate: uncomment the `db.results()` line. Doing so will result in a
  compile error, as `db` is already mutably borrowed.

- Note: The query results not being public and placed behind a getter function
  lets us enforce the invariant "users can only look at query results if there
  is no active transactions."

  If the query results were placed in a public struct field, this invariant
  could be violated.

</details>
