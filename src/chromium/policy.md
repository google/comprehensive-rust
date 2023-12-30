# Chromium Rust policy

Chromium does not yet allow first-party Rust except in rare cases as approved by
Chromium's
[Area Tech Leads](https://source.chromium.org/chromium/chromium/src/+/main:ATL_OWNERS).

Chromium's policy on third party libraries is outlined
[here](https://chromium.googlesource.com/chromium/src/+/main/docs/adding_to_third_party.md#rust) -
Rust is allowed for third party libraries under various circumstances, including
if they're the best option for performance or for security.

Very few Rust libraries directly expose a C/C++ API, so that means that nearly
all such libraries will require a small amount of first-party glue code.

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

> First-party Rust glue code for a particular third-party crate should normally
> be kept in `third_party/rust/<crate>/<version>/wrapper`.

Because of this, today's course will be heavily focused on:

- Bringing in third-party Rust libraries ("crates")
- Writing glue code to be able to use those crates from Chromium C++.

If this policy changes over time, the course will evolve to keep up.
