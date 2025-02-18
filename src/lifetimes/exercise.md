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

## Messages

A proto message is encoded as a series of fields, one after the next. Each is
implemented as a "tag" followed by the value. The tag contains a field number
(e.g., `2` for the `id` field of a `Person` message) and a wire type defining
how the payload should be determined from the byte stream. These are combined
into a single integer, as decoded in `unpack_tag` below.

## Varint

Integers, including the tag, are represented with a variable-length encoding
called VARINT. Luckily, `parse_varint` is defined for you below.

## Wire Types

Proto defines several wire types, only two of which are used in this exercise.

The `Varint` wire type contains a single varint, and is used to encode proto
values of type `int32` such as `Person.id`.

The `Len` wire type contains a length expressed as a varint, followed by a
payload of that number of bytes. This is used to encode proto values of type
`string` such as `Person.name`. It is also used to encode proto values
containing sub-messages such as `Person.phones`, where the payload contains an
encoding of the sub-message.

## Exercise

The given code also defines callbacks to handle `Person` and `PhoneNumber`
fields, and to parse a message into a series of calls to those callbacks.

What remains for you is to implement the `parse_field` function and the
`ProtoMessage` trait for `Person` and `PhoneNumber`.

<!-- compile_fail because the stubbed out code has type inference errors. -->

```rust,editable,compile_fail
{{#include exercise.rs:preliminaries }}


{{#include exercise.rs:parse_field }}
        _ => todo!("Based on the wire type, build a Field, consuming as many bytes as necessary.")
    };
    todo!("Return the field, and any un-consumed bytes.")
}

{{#include exercise.rs:parse_message }}

{{#include exercise.rs:message_phone_number_type}}

{{#include exercise.rs:message_person_type}}

// TODO: Implement ProtoMessage for Person and PhoneNumber.

{{#include exercise.rs:tests }}
```

<details>

- In this exercise there are various cases where protobuf parsing might fail,
  e.g. if you try to parse an `i32` when there are fewer than 4 bytes left in
  the data buffer. In normal Rust code we'd handle this with the `Result` enum,
  but for simplicity in this exercise we panic if any errors are encountered. On
  day 4 we'll cover error handling in Rust in more detail.

</details>
