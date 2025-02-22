# A Simple C Library

Let's first create a small C library:

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
