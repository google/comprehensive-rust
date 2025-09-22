## Serializer: complete implementation

Looking back at our original desired flow:

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

We can now see this reflected directly in the types of our serializer:

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

The code for the full implementation of the `Serializer` and all its states can
be found in
[this Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=c9cbb831cd05fe9db4ce42713c83ca16).

<details>

- This pattern isn't a silver bullet. It still allows issues like:
  - Empty or invalid property names (which can be fixed using
    [the newtype pattern](../../newtype-pattern.md))
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
