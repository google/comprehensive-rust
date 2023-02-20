# Method Receiver

The `&self` above indicates that the method borrows the object immutably. There
are other possible receivers for a method:

* `&self`: borrows the object from the caller using a shared and immutable
  reference. The object can be used again afterwards.
* `&mut self`: borrows the object from the caller using a unique and mutable
  reference. The object can be used again afterwards.
* `self`: takes ownership of the object and moves it away from the caller. The
  method becomes the owner of the object. The object will be dropped (deallocated)
  when the method returns, unless its ownership is explicitly
  transmitted.
* `mut self`: same as above, but while the method owns the object, it can
  mutate it too. Complete ownership does not automatically mean mutability.
* No receiver: this becomes a static method on the struct. Typically used to
  create constructors which are called `new` by convention.

Beyond variants on `self`, there are also
[special wrapper types](https://doc.rust-lang.org/reference/special-types-and-traits.html)
allowed to be receiver types, such as `Box<Self>`.

<details>
  
Consider emphasizing "shared and immutable" and "unique and mutable". These constraints always come
together in Rust due to borrow checker rules, and `self` is no exception. It isn't possible to
reference a struct from multiple locations and call a mutating (`&mut self`) method on it.
  
</details>
