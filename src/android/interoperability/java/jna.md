# Using JNA

Java Native Access [JNA](https://github.com/java-native-access/jna) is an Open Source wrapper around JNI. This overview is recommended for those already familiar with JNA or C.

First, we create a Rust function to export to Java:

_interoperability/java/jna/src/lib.rs_:

```rust,compile_fail
{{#include java/jna/src/lib.rs:hello}}
```

_interoperability/java/jna/Android.bp_:

```javascript
{{#include java/jna/Android.bp:libhello_jna}}
```

Finally, we can call this function from Java:

_interoperability/java/jna/HelloWorld.java_:

```java
{{#include java/jna/HelloWorld.java:HelloWorld}}
```

_interoperability/java/jna/Android.bp_:

```javascript
{{#include java/jna/Android.bp:helloworld_jna}}
```

Finally, you can build, sync, and run the binary:

```shell
{{#include ../build_all.sh:helloworld_jna}}
```
