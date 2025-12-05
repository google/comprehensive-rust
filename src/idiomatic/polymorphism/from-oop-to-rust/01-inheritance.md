---
minutes: 5
---

# Inheritance in OOP languages

```cpp
#include <iostream>
using namespace std;

// Base class
class Vehicle {
public:
    void accellerate() { }
    void brake() { }
};

// Inheriting class
class Car : public Vehicle {
public:
    void honk() { }
};

int main() {
    Car myCar;                  // Create a Car object
    myCar.accellerate();        // Inherited method
    myCar.honk();               // Car's own method
    myCar.brake();              // Inherited method
    return 0;
}
```

- Here to remind students what inheritance is.

- Inheritance is a mechanism where a "child" type gains the fields and methods
  of the "parent" types it is inheriting from.

- Methods are able to be overridden as-needed by the inheriting type.

- Can call methods of inherited-from classes with `super`.
