# Using JNA

Java Native Access [JNA](https://github.com/java-native-access/jna) is an Open Source wrapper around JNI. Please use their Readme for project setup.
*Note, unlike the `JNI` Example, this Example is for Windows.*

First, we create a Rust function to export to Java:

_interoperability/java/jna/src/lib.rs_:

```rust,compile_fail
{{#include java/jna/src/lib.rs:hello}}
```

Finally, we can call this function from Java:

_interoperability/java/jna/HelloWorld.java_:

```java
{{#include java/jna/HelloWorld.java:HelloWorld}}
```

Finally, you can build, sync, and run the binary:

```shell
{{#include ../build_all.sh:helloworld_jna}}
```
