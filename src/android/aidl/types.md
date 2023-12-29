# Working With AIDL Types

AIDL types translate into the appropriate idiomatic Rust type:

* Primitive types map (mostly) to idiomatic Rust types.
* Collection types like slices, `Vec`s and string types are supported.
* References to AIDL objects can be sent between file handles.
* File handles and parcelables are fully supported.
