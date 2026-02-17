<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Chromium Rust policy

Chromium's Rust policy can be found
[here](https://source.chromium.org/chromium/chromium/src/+/main:docs/rust.md;l=22).
Rust can be used for both first-party and third-party code.

Using Rust for pure first-party code looks like this:

```bob
"C++"                           Rust
.- - - - - - - - - -.           .- - - - - - - - - - -.
:                   :           :                     :
: Existing Chromium :           :  Chromium Rust      :
: "C++"             :           :  code               :
: +---------------+ :           : +----------------+  :
: |               | :           : |                |  :
: |         o-----+-+-----------+-+->              |  :
: |               | : Language  : |                |  :
: +---------------+ : boundary  : +----------------+  :
:                   :           :                     :
`- - - - - - - - - -'           `- - - - - - - - - - -'
```

The third-party case is also common. You will typically also need a small amount
of first-party glue code, because very few Rust libraries directly expose a
C/C++ API.

```bob
"C++"                           Rust
.- - - - - - - - - -.           .- - - - - - - - - - - - - - - - - - - - - - -.
:                   :           :                                             :
: Existing Chromium :           :  Chromium Rust              Existing Rust   :
: "C++"             :           :  "wrapper"                  crate           :
: +---------------+ :           : +----------------+          +-------------+ :
: |               | :           : |                |          |             | :
: |         o-----+-+-----------+-+->            o-+----------+-->          | :
: |               | : Language  : |                | Crate    |             | :
: +---------------+ : boundary  : +----------------+ API      +-------------+ :
:                   :           :                                             :
`- - - - - - - - - -'           `- - - - - - - - - - - - - - - - - - - - - - -'
```

The scenario of using a third-party crate is the more complex one, so today's
course will focus on:

- Bringing in third-party Rust libraries ("crates")
- Writing glue code to be able to use those crates from Chromium C++. (The same
  techniques are used when working with first-party Rust code).
