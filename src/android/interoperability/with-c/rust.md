# Calling Rust

We can now call this from a C binary:

_interoperability/rust/libanalyze/analyze.h_

```c
{{#include rust/libanalyze/analyze.h:analyze_numbers}}
```

_interoperability/rust/analyze/main.c_

```c
{{#include rust/analyze/main.c:main}}
```

_interoperability/rust/analyze/Android.bp_

```javascript
{{#include rust/analyze/Android.bp}}
```

Build, push, and run the binary on your device:

```shell
{{#include ../../build_all.sh:analyze_numbers}}
```
