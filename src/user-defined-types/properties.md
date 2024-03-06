# Properties table:

| Property                                     | Static                            | Constant     |
| -------------------------------------------- | --------------------------------- | ------------ |
| Has an address in memory                     | Yes                               | No (inlined) |
| Lives for the entire duration of the program | Yes                               | No           |
| Can be mutable                               | Yes (unsafe)                      | No           |
| Evaluated at compile time                    | Yes (initialised at compile time) | Yes          |
| Inlined wherever it is used                  | No                                | Yes          |

# More to Explore

Because `static` variables are accessible from any thread, they must be `Sync`.
Interior mutability is possible through a
[`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html), atomic or
similar.

Thread-local data can be created with the macro `std::thread_local`.