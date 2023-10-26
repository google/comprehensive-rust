# Scope-Based Memory Management

Constructors and destructors let you hook into the lifetime of an object.

By wrapping a pointer in an object, you can free memory when the object is
destroyed. The compiler guarantees that this happens, even if an exception is
raised.

This is often called _resource acquisition is initialization_ (RAII) and gives
you smart pointers.

## C++ Example

```c++
void say_hello(std::unique_ptr<Person> person) {
  std::cout << "Hello " << person->name << std::endl;
}
```

* The `std::unique_ptr` object is allocated on the stack, and points to
  memory allocated on the heap.
* At the end of `say_hello`, the `std::unique_ptr` destructor will run.
* The destructor frees the `Person` object it points to.

Special move constructors are used when passing ownership to a function:

<!-- mdbook-xgettext: skip -->
```c++
std::unique_ptr<Person> person = find_person("Carla");
say_hello(std::move(person));
```
