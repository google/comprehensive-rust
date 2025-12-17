---
minutes: 5
---

# With Offset

```rust,editable
#[derive(Debug)]
pub struct SelfReferentialBuffer {
    data: [u8; 1024],
    position: usize,
}

impl SelfReferentialBuffer {
    pub fn new() -> Self {
        SelfReferentialBuffer { data: [0; 1024], position: 0 }
    }

    pub fn read(&self, n_bytes: usize) -> &[u8] {
        let available = self.data.len().saturating_sub(self.position);
        let len = n_bytes.min(available);
        &self.data[self.position..self.position + len]
    }

    pub fn write(&mut self, bytes: &[u8]) {
        let available = self.data.len().saturating_sub(self.position);
        let len = bytes.len().min(available);
        self.data[self.position..self.position + len].copy_from_slice(&bytes[..len]);
        self.position += len;
    }
}
```

<details>

In Rust, it's more idiomatic to use an offset variable and to create references
on-demand.

</details>
