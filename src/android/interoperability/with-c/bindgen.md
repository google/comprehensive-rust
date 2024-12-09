# Using Bindgen

The [bindgen](https://rust-lang.github.io/rust-bindgen/introduction.html) tool
can auto-generate bindings from a C header file.

First create a small C library:

_interoperability/bindgen/libbirthday.h_:

```c
{{#include bindgen/libbirthday.h:card}}
```

_interoperability/bindgen/libbirthday.c_:

```c
{{#include bindgen/libbirthday.c:print_card}}
```

Add this to your `Android.bp` file:

_interoperability/bindgen/Android.bp_:

```javascript
{{#include bindgen/Android.bp:libbirthday}}
```

Create a wrapper header file for the library (not strictly needed in this
example):

_interoperability/bindgen/libbirthday_wrapper.h_:

```c
{{#include bindgen/libbirthday_wrapper.h:include}}
```

You can now auto-generate the bindings:

_interoperability/bindgen/Android.bp_:

```javascript
{{#include bindgen/Android.bp:libbirthday_bindgen}}
```

Finally, we can use the bindings in our Rust program:

_interoperability/bindgen/Android.bp_:

```javascript
{{#include bindgen/Android.bp:print_birthday_card}}
```

_interoperability/bindgen/main.rs_:

```rust,compile_fail
{{#include bindgen/main.rs:main}}
```

Build, push, and run the binary on your device:

```shell
{{#include ../../build_all.sh:print_birthday_card}}
```

Finally, we can run auto-generated tests to ensure the bindings work:

_interoperability/bindgen/Android.bp_:

```javascript
{{#include bindgen/Android.bp:libbirthday_bindgen_test}}
```

```shell
{{#include ../../build_all.sh:libbirthday_bindgen_test}}
```

<details>

- The Android build rules will automatically call `bindgen` for you behind the
  scenes.

- Notice that the Rust code in `main` is still hard to write. It is good
  practice to encapsulate the output of `bindgen` in a Rust library which
  exposes a safe interface to caller.

</details>
