---
minutes: 10
existing course material:
- memory-management.md
- memory-management/manual.md
- memory-management/scope-based.md
- memory-management/garbage-collection.md
---

<!-- NOTES:
Short summary of how different languages handle memory management
-->
# Approaches to Memory Management

# Memory Management

Traditionally, languages have fallen into two broad categories:

* Full control via manual memory management: C, C++, Pascal, ...
* Full safety via automatic memory management at runtime: Java, Python, Go, Haskell, ...

Rust offers a new mix:

> Full control *and* safety via compile time enforcement of correct memory
> management.

It does this with an explicit ownership concept.

First, let's refresh how memory management works.
# Manual Memory Management

You allocate and deallocate heap memory yourself.

If not done with care, this can lead to crashes, bugs, security vulnerabilities, and memory leaks.

## C Example

You must call `free` on every pointer you allocate with `malloc`:

```c
void foo(size_t n) {
    int* int_array = malloc(n * sizeof(int));
    //
    // ... lots of code
    //
    free(int_array);
}
```

Memory is leaked if the function returns early between `malloc` and `free`: the
pointer is lost and we cannot deallocate the memory.
Worse, freeing the pointer twice, or accessing a freed pointer can lead to exploitable security vulnerabilities.
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
# Automatic Memory Management

An alternative to manual and scope-based memory management is automatic memory
management:

* The programmer never allocates or deallocates memory explicitly.
* A garbage collector finds unused memory and deallocates it for the programmer.

## Java Example

The `person` object is not deallocated after `sayHello` returns:

```java
void sayHello(Person person) {
  System.out.println("Hello " + person.getName());
}
```
