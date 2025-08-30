## Typestate Pattern with Generics

By combining typestate modeling with generics, we can express a wider range of
valid states and transitions without duplicating logic. This approach is
especially useful when the number of states grows or when multiple states share
behavior but differ in structure.

```rust
# use std::fmt::Write as _;
#
struct Serializer<S> {
    // [...]
    # indent: usize,
    # buffer: String,
    # state: S,
}

struct Root;
struct Struct<S>(S);
struct List<S>(S);
struct Property<S>(S);

impl Serializer<Root> {
    fn new() -> Self {
        // [...]
        # Self {
        #     indent: 0,
        #     buffer: String::new(),
        #     state: Root,
        # }
    }

    fn serialize_struct(mut self, name: &str) -> Serializer<Struct<Root>> {
        // [...]
        # writeln!(self.buffer, "{name} {{").unwrap();
        # Serializer {
        #     indent: self.indent + 1,
        #     buffer: self.buffer,
        #     state: Struct(self.state),
        # }
    }

    fn finish(self) -> String {
        // [...]
        # self.buffer
    }
}

impl<S> Serializer<Struct<S>> {
    fn serialize_property(mut self, name: &str) -> Serializer<Property<Struct<S>>> {
        // [...]
        # write!(self.buffer, "{}{name}: ", " ".repeat(self.indent * 2)).unwrap();
        # Serializer {
        #     indent: self.indent,
        #     buffer: self.buffer,
        #     state: Property(self.state),
        # }
    }

    fn finish_struct(mut self) -> Serializer<S> {
        // [...]
        # self.indent -= 1;
        # writeln!(self.buffer, "{}}}", " ".repeat(self.indent * 2)).unwrap();
        # Serializer {
        #     indent: self.indent,
        #     buffer: self.buffer,
        #     state: self.state.0,
        # }
    }
}

impl<S> Serializer<Property<Struct<S>>> {
    fn serialize_struct(mut self, name: &str) -> Serializer<Struct<Struct<S>>> {
        // [...]
        # writeln!(self.buffer, "{name} {{").unwrap();
        # Serializer {
        #     indent: self.indent + 1,
        #     buffer: self.buffer,
        #     state: Struct(self.state.0),
        # }
    }

    fn serialize_list(mut self) -> Serializer<List<Struct<S>>> {
        // [...]
        # writeln!(self.buffer, "[").unwrap();
        # Serializer {
        #     indent: self.indent + 1,
        #     buffer: self.buffer,
        #     state: List(self.state.0),
        # }
    }

    fn serialize_string(mut self, value: &str) -> Serializer<Struct<S>> {
        // [...]
        # writeln!(self.buffer, "{value},").unwrap();
        # Serializer {
        #     indent: self.indent,
        #     buffer: self.buffer,
        #     state: self.state.0,
        # }
    }
}

impl<S> Serializer<List<S>> {
    fn serialize_struct(mut self, name: &str) -> Serializer<Struct<List<S>>> {
        // [...]
        # writeln!(self.buffer, "{}{name} {{", " ".repeat(self.indent * 2)).unwrap();
        # Serializer {
        #     indent: self.indent + 1,
        #     buffer: self.buffer,
        #     state: Struct(self.state),
        # }
    }

    fn serialize_string(mut self, value: &str) -> Self {
        // [...]
        # writeln!(self.buffer, "{}{value},", " ".repeat(self.indent * 2)).unwrap();
        # self
    }

    fn finish_list(mut self) -> Serializer<S> {
        // [...]
        # self.indent -= 1;
        # writeln!(self.buffer, "{}]", " ".repeat(self.indent * 2)).unwrap();
        # Serializer {
        #     indent: self.indent,
        #     buffer: self.buffer,
        #     state: self.state.0,
        # }
    }
}

fn main() {
    # #[rustfmt::skip]
    let serializer = Serializer::new()
        .serialize_struct("Foo")
            .serialize_property("bar")
            .serialize_struct("Bar")
                .serialize_property("baz")
                .serialize_list()
                    .serialize_string("abc")
                    .serialize_struct("Baz")
                        .serialize_property("partial")
                        .serialize_string("def")
                        .serialize_property("empty")
                        .serialize_struct("Empty")
                        .finish_struct()
                    .finish_struct()
                .finish_list()
            .finish_struct()
        .finish_struct();

    let output = serializer.finish();

    println!("{output}");

    // These will all fail at compile time:

    // Serializer::new().serialize_list();
    // Serializer::new().serialize_string("foo");
    // Serializer::new().serialize_struct("Foo").serialize_string("bar");
    // Serializer::new().serialize_struct("Foo").serialize_list();
    // Serializer::new().serialize_property("foo");
}
```

<details>

- The full code for this example is available
  [in the playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=48b106089ca600453f3ed00a0a31af26).

- By using generics to track the parent context, we can construct arbitrarily
  nested serializers that enforce valid transitions between struct, list, and
  property states.

- This lets us build a recursive structure while preserving control over what
  methods are accessible in each state.

- Methods common to all states can be implemented for any `S` in
  `Serializer<S>`.

- These marker types (e.g., `List<S>`) incur no memory or runtime overhead, as
  they hold no data other than a possible Zero-Sized Type. Their sole purpose is
  to enforce correct API usage by leveraging the type system.

- Here's how the flow maps to a state machine:

```bob
    +-----------+   +---------+------------+-----+
    |           |   |         |            |     |
    V           |   V         |            V     |
                +                                |
serializer --> structure --> property --> list +-+

    |           |   ^           |          ^
    V           |   |           |          |
                |   +-----------+          |
  String        |                          |
                +--------------------------+
```

- And this is reflected directly in the types of our serializer:

```bob
                                                         +------+
                                                 finish  |      |
                           serialize             struct  V      |
                           struct
+---------------------+ --------------> +-----------------------------+ <---------------+
| Serializer [ Root ] |                 | Serializer [ Struct [ S ] ] |                 |
+---------------------+ <-------------- +-----------------------------+ <-----------+   |
                          finish struct                                             |   |
         |                                  |     serialize   |                     |   |
         |                       +----------+     property    V          serialize  |   |
         |                       |                                       string or  |   |
finish   |                       |    +-------------------------------+  struct     |   |
         V                       |    | Serializer [ Property [ S ] ] | ------------+   |
                         finish  |    +-------------------------------+                 |
     +--------+          struct  |                                                      |
     | String |                  |                serialize   |                         |
     +--------+                  |                list        V                         |
                                 |                                         finish       |
                                 |        +---------------------------+    list         |
                                 +------> | Serializer [ List [ S ] ] | ----------------+
                                          +---------------------------+
                                                  serialize
                                                  list or string  ^
                                              |   or finish list  |
                                              +-------------------+
```

- Of course, this pattern isn't a silver bullet. It still allows issues like:
  - Empty or invalid property names (which can be fixed using
    [the newtype pattern](../newtype-pattern.md))
  - Duplicate property names (which could be tracked in `Struct<S>` and handled
    via `Result`)

- If validation failures occur, we can also change method signatures to return a
  `Result`, allowing recovery:

  ```rust,compile_fail
  struct PropertySerializeError<S> {
      kind: PropertyError,
      serializer: Serializer<Struct<S>>,
  }

  impl<S> Serializer<Struct<S>> {
      fn serialize_property(
          self,
          name: &str,
      ) -> Result<Serializer<Property<Struct<S>>>, PropertySerializeError<S>> {
          /* ... */
      }
  }
  ```

- While this API is powerful, it’s not always ergonomic. Production serializers
  typically favor simpler APIs and reserve the typestate pattern for enforcing
  critical invariants.

- One excellent real-world example is
  [`rustls::ClientConfig`](https://docs.rs/rustls/latest/rustls/client/struct.ClientConfig.html#method.builder),
  which uses typestate with generics to guide the user through safe and correct
  configuration steps.

</details>
