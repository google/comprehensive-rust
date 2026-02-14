---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# With a raw pointer

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
#[derive(Debug)]
pub struct SelfReferentialBuffer {
    data: [u8; 1024],
    cursor: *mut u8,
}

impl SelfReferentialBuffer {
    pub fn new() -> Self {
        let mut buffer =
            SelfReferentialBuffer { data: [0; 1024], cursor: std::ptr::null_mut() };

        buffer.update_cursor();
        buffer
    }

    // Danger: must be called after every move
    pub fn update_cursor(&mut self) {
        self.cursor = self.data.as_mut_ptr();
    }

    pub fn read(&self, n_bytes: usize) -> &[u8] {
        unsafe {
            let start = self.data.as_ptr();
            let end = start.add(1024);
            let cursor = self.cursor as *const u8;

            assert!((start..=end).contains(&cursor), "cursor is out of bounds");

            let available = end.offset_from(cursor) as usize;
            let len = n_bytes.min(available);
            std::slice::from_raw_parts(cursor, len)
        }
    }

    pub fn write(&mut self, bytes: &[u8]) {
        unsafe {
            let start = self.data.as_mut_ptr();
            let end = start.add(1024);

            assert!((start..=end).contains(&self.cursor), "cursor is out of bounds");
            let available = end.offset_from(self.cursor) as usize;
            let len = bytes.len().min(available);

            std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.cursor, len);
            self.cursor = self.cursor.add(len);
        }
    }
}
```

<details>

Avoid spending too much time here.

Talking points:

- Emphasize that `unsafe` appears frequently. This is a hint that another design
  may be more appropriate.
- `unsafe` blocks lack safety comments. Therefore, this code is unsound.
- `unsafe` blocks are too broad. Good practice uses smaller `unsafe` blocks with
  specific behavior, specific preconditions and specific safety comments.

Questions:

Q: Should the `read()` and `write()` methods be marked as unsafe?\
A: Yes, because `self.cursor` will be a null pointer unless written to.

</details>
