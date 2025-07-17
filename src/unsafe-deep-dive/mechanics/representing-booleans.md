# Example: Representing Booleans

To Boolean values must match a precise representation to avoid undefined
behavior.

<table>
    <tr>
        <th>Bit pattern</th><th>Rust type</th>
    </tr>
    <tr>
        <td><code class="hljs">00000001</code></td><td><code class="hljs">true</code></td>
    </tr>
    <tr>
        <td><code class="hljs">00000000</code></td><td><code class="hljs">false</code></td>
    </tr>
    <tr>
        <td>Other patterns</td><td>Undefined</td>
    </tr>
</table>

You have two tasks in this exercise.

- First, create Rust struct that represents a Boolean value and a function that
  create a value of your type from `u8` with no overhead cost while ensuring
  that undefined behavior is impossible.
- Secondly, review someone else's implementation.

Starter code:

```rust
struct Boolean(u8);
```
