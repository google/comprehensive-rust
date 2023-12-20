# Service Implementation

We can now implement the AIDL service:

*birthday_service/src/lib.rs*:

```rust,ignore
{{#include birthday_service/src/lib.rs:IBirthdayService}}
```

*birthday_service/Android.bp*:

```javascript
{{#include birthday_service/Android.bp:libbirthdayservice}}
```

<details>

* Point out the path to the generated `IBirthdayService` trait, and explain why
  each of the segments is necessary.
* TODO: What does the `binder::Interface` trait do? Are there methods to
  override? Where source?

</details>
