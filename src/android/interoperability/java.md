# Interoperability with Java

Java can load shared objects via
[Java Native Interface (JNI)](https://en.wikipedia.org/wiki/Java_Native_Interface).
The [`jni` crate](https://docs.rs/jni/) allows you to create a compatible
library.

First, we create a Rust function to export to Java:

_interoperability/java/src/lib.rs_:

```rust,compile_fail
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
