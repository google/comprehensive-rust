# What a move is in Rust

Always a bitwise copy, even for types that do not implement `Copy`:

```rust
#[derive(Debug, Default)]
pub struct DynamicBuffer {
    data: Vec<u8>,
    position: usize,
};

pub fn move_and_inspect(x: DynamicBuffer) { println!("{x:?}"); }

pub fn main() {
   let a = DynamicBuffer::default();
   let mut b = a;
   b.data.push(b'R');
   b.data.push(b'U');
   b.data.push(b'S');
   b.data.push(b'T');
   move_and_inspect(b);
}
```

Generated [LLVM IR] for calling `move_and_expect()`:

```llvm
call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_12, ptr align 8 %b, i64 32, i1 false)
invoke void @move_and_inspect(ptr align 8 %_12)
```

- `memcpy` from variable `%b` to `%_12`
- Call to `move_and_inspect` with `%_12` (the copy)

<details>

Note that `DynamicBuffer` does not implement `Copy`.

Implication: a value's memory address is not stable.

To show movement as a bitwise copy, either
[open the code in the playground][LLVM IR] and look at the or
[the Compiler Explorer].

Optional for those who prefer assembly output:

The Compiler Explorer is useful for discussing the generated assembly and focus
the cursor assembly output in the `main` function on lines 128-136 (should be
highlighted in pink).

Relevant code generated output `move_and_inspect`:

```assembly
mov     rax, qword ptr [rsp + 16]
mov     qword ptr [rsp + 48], rax    
mov     rax, qword ptr [rsp + 24]
mov     qword ptr [rsp + 56], rax
movups  xmm0, xmmword ptr [rsp]
movaps  xmmword ptr [rsp + 32], xmm0
lea     rdi, [rsp + 32]
call    qword ptr [rip + move_and_inspect@GOTPCREL]
```

</details>

[LLVM IR]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6f587283e8e0ec02f1ea8e871fc9ac72
[The Compiler Explorer]: https://rust.godbolt.org/z/6o6nP7do4
