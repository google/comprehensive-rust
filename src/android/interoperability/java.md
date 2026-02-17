<!--
Copyright 2022 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Interoperability with Java

Java can load shared objects via
[Java Native Interface (JNI)](https://en.wikipedia.org/wiki/Java_Native_Interface).
The [`jni` crate](https://docs.rs/jni/) allows you to create a compatible
library.

First, we create a Rust function to export to Java:

_interoperability/java/src/lib.rs_:

```rust,compile_fail
# // Copyright 2022 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include java/src/lib.rs:hello}}
```

_interoperability/java/Android.bp_:

```javascript
{{#include java/Android.bp:libhello_jni}}
```

We then call this function from Java:

_interoperability/java/HelloWorld.java_:

```java
{{#include java/HelloWorld.java:HelloWorld}}
```

_interoperability/java/Android.bp_:

```javascript
{{#include java/Android.bp:helloworld_jni}}
```

Finally, you can build, sync, and run the binary:

```shell
{{#include ../build_all.sh:helloworld_jni}}
```

<details>

- The `unsafe(no_mangle)` attribute instructs Rust to emit the
  `Java_HelloWorld_hello` symbol exactly as written. This is important so that
  Java can recognize the symbol as a `hello` method on the `HelloWorld` class.

  - By default, Rust will mangle (rename) symbols so that a binary can link in
    two versions of the same Rust crate.

</details>
