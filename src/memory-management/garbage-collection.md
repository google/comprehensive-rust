# Automatic Memory Management

수동, 스코프기반 메모리 관리의 대안인 자동 메모리 관리 입니다:
* 개발자는 메모리를 명시적으로 할당/해제 하지 않습니다. 
* 가비지 컬렉터(GC)는 개발자 대신 사용되지 않는 메모리를 찾아 해제합니다.

> An alternative to manual and scope-based memory management is automatic memory management:
> 
> * The programmer never allocates or deallocates memory explicitly.
> * A garbage collector finds unused memory and deallocates it for the programmer.

## Java Example

`person`객체는 `sayHello`함수 반환 후에도 해제되지 않습니다. (GC가 나중에 알아서 해제합니다.)
> The `person` object is not deallocated after `sayHello` returns:

```java
void sayHello(Person person) {
  System.out.println("Hello " + person.getName());
}
```
