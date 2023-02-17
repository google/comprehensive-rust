# `no_std`

<table>
<tr>
<th>
  
`core`

</th>
<th>

`alloc`

</th>
<th>

`std`

</th>
</tr>
<tr valign="top">
<td>

* Slices, `&str`, `CStr`
* `NonZeroU8`...
* `Option`, `Result`
* `Display`, `Debug`, `write!`...
* `Iterator`
* `panic!`, `assert_eq!`...
* `NonNull` and all the usual pointer-related functions
* `Future` and `async`/`await`
* `fence`, `AtomicBool`, `AtomicPtr`, `AtomicU32`...
* `Duration`

</td>
<td>

* `Box`, `Cow`, `Arc`, `Rc`
* `Vec`, `BinaryHeap`, `BtreeMap`, `LinkedList`, `VecDeque`
* `String`, `CString`, `format!`

</td>
<td>

* `Error`
* `HashMap`
* `Mutex`, `Condvar`, `Barrier`, `Once`, `RwLock`, `mpsc`
* `File` and the rest of `fs`
* `println!`, `Read`, `Write`, `Stdin`, `Stdout` and the rest of `io`
* `Path`, `OsString`
* `net`
* `Command`, `Child`, `ExitCode`
* `spawn`, `sleep` and the rest of `thread`
* `SystemTime`, `Instant`

</td>
</tr>
</table>

<details>

* `HashMap` depends on RNG.
* `std` re-exports the contents of both `core` and `alloc`.

</details>
