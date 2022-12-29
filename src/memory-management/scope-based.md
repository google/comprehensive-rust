# Scope-Based Memory Management

생성자와 소멸자를 사용하여 객체의 수명에 연결(hook)할 수 있습니다. 
> Constructors and destructors let you hook into the lifetime of an object.

포인터를 객체에 래핑하여 객체가 소멸될 때 메모리를 해제 할 수 있습니다. 
컴파일러는 예외(exception)가 발생하더라도 이를 보장합니다. 
> By wrapping a pointer in an object, you can free memory when the object is
> destroyed. The compiler guarantees that this happens, even if an exception is raised.

이를 종종 RAII(Resource Acquisition Is Initialization, 리소스 획득은 초기화다)이라고 하며, RAII패턴의 스마트 포인터를 제공합니다.
> This is often called _resource acquisition is initialization_ (RAII) and gives
> you smart pointers.

## C++ Example

```c++
void say_hello(std::unique_ptr<Person> person) {
  std::cout << "Hello " << person->name << std::endl;
}
```

* `std::unique_ptr`객체는 스택에 할당되며, 힙에 할당된 메모리를 가리킵니다(point).
* `say_hello`함수가 끝나면 `std::unique_ptr`의 소멸자가 실행됩니다.
* 소멸자는 `Person` 객체가 가르키는(point) 곳을 해제합니다.

> * The `std::unique_ptr` object is allocated on the stack, and points to
>   memory allocated on the heap.
> * At the end of `say_hello`, the `std::unique_ptr` destructor will run.
> * The destructor frees the `Person` object it points to.

특별한 이동 생성자는 소유권을 함수로 전달할때 사용됩니다.
> Special move constructors are used when passing ownership to a function:

```c++
std::unique_ptr<Person> person = find_person("Carla");
say_hello(std::move(person));
```

--- 
역주
- C++ 지식이 짧아서 번역이 맞나 모르겠음.