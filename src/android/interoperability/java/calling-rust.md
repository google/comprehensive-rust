<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Calling Rust from Java

We can now call the Rust function from Java:

_interoperability/java/HelloWorld.java_:

```java
{{#include HelloWorld.java:HelloWorld}}
```

Add the Java binary to your `Android.bp` file:

_interoperability/java/Android.bp_:

```javascript
{{#include Android.bp:helloworld_jni}}
```

Finally, build, sync, and run the binary:

```shell
{{#include ../../build_all.sh:helloworld_jni}}
```
