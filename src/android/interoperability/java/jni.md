# Using JNI

Java natively can load objects into Java with [Java Native Interface
(JNI)](https://en.wikipedia.org/wiki/Java_Native_Interface). The [`jni`
crate](https://docs.rs/jni/) allows you to create a compatible library.

First, we create a Rust function to export to Java:

_interoperability/java/jni/src/lib.rs_:

```rust,compile_fail
{{#include java/jni/src/lib.rs:hello}}
```

_interoperability/java/jni/Android.bp_:

```javascript
{{#include java/jni/Android.bp:libhello_jni}}
```

Finally, we can call this function from Java:

_interoperability/java/jni/HelloWorld.java_:

```java
{{#include java/jni/HelloWorld.java:HelloWorld}}
```

_interoperability/java/jni/Android.bp_:

```javascript
{{#include java/jni/Android.bp:helloworld_jni}}
```

Finally, you can build, sync, and run the binary:

```shell
{{#include ../build_all.sh:helloworld_jni}}
```
