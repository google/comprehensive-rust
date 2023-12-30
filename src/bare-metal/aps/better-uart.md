# A better UART driver

The PL011 actually has [a bunch more registers][1], and adding offsets to
construct pointers to access them is error-prone and hard to read. Plus, some of
them are bit fields which would be nice to access in a structured way.

| Offset | Register name | Width |
| ------ | ------------- | ----- |
| 0x00   | DR            | 12    |
| 0x04   | RSR           | 4     |
| 0x18   | FR            | 9     |
| 0x20   | ILPR          | 8     |
| 0x24   | IBRD          | 16    |
| 0x28   | FBRD          | 6     |
| 0x2c   | LCR_H         | 8     |
| 0x30   | CR            | 16    |
| 0x34   | IFLS          | 6     |
| 0x38   | IMSC          | 11    |
| 0x3c   | RIS           | 11    |
| 0x40   | MIS           | 11    |
| 0x44   | ICR           | 11    |
| 0x48   | DMACR         | 3     |

<details>

- There are also some ID registers which have been omitted for brevity.

</details>

[1]: https://developer.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers
