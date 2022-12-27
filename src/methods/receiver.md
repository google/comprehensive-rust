# Method Receiver

The `&self` above indicates that the method borrows the object immutably. There
are other possible receivers for a method:

* `&self`: borrows the object from the caller using a shared and immutable
  reference. The object can be used again afterwards.
* `&mut self`: borrows the object from the caller using a unique and mutable
  reference. The object can be used again afterwards.
* `self`: takes ownership of the object and moves it away from the caller. The
  method becomes the owner of the object. The object will be drop (deallocated)
  when the method returns, unless it’s ownership is explicitly
  transmitted.
* No receiver: this becomes a static method on the struct. Typically used to
  create constructors which are called `new` by convention.

There are even more types, e.g., `Box<Self>`.
