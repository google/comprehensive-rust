---
minutes: 30
---

# Exercise: Protobuf Parsing

In this exercise, you will build a parser for the
[protobuf binary encoding](https://protobuf.dev/programming-guides/encoding/).
Don't worry, it's simpler than it seems! This illustrates a common parsing
pattern, passing slices of data. The underlying data itself is never copied.

Fully parsing a protobuf message requires knowing the types of the fields,
indexed by their field numbers. That is typically provided in a `proto` file. In
this exercise, we'll encode that information into `match` statements in
functions that get called for each field.

We'll use the following proto:

```proto
message PhoneNumber {
  optional string number = 1;
  optional string type = 2;
}

message Person {
  optional string name = 1;
  optional int32 id = 2;
  repeated PhoneNumber phones = 3;
}
```

A proto message is encoded as a series of fields, one after the next. Each is
implemented as a "tag" followed by the value. The tag contains a field number
(e.g., `2` for the `id` field of a `Person` message) and a wire type defining
how the payload should be determined from the byte stream.

Integers, including the tag, are represented with a variable-length encoding
called VARINT. Luckily, `parse_varint` is defined for you below. The given code
also defines callbacks to handle `Person` and `PhoneNumber` fields, and to parse
a message into a series of calls to those callbacks.

What remains for you is to implement the `parse_field` function and the
`ProtoMessage` trait for `Person` and `PhoneNumber`.

<!-- compile_fail because `mdbook test` does not allow use of `thiserror` -->

```rust,editable,compile_fail
{{#include exercise.rs:preliminaries }}


{{#include exercise.rs:parse_field }}
        _ => todo!("Based on the wire type, build a Field, consuming as many bytes as necessary.")
    };
    todo!("Return the field, and any un-consumed bytes.")
}

{{#include exercise.rs:parse_message }}

{{#include exercise.rs:message_types}}

// TODO: Implement ProtoMessage for Person and PhoneNumber.

{{#include exercise.rs:main }}
```
