# Threads

러스트의 스레드는 다른 언어의 스레드와 유사하게 동작합니다:
> Rust threads work similarly to threads in other languages:

```rust,editable
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}
```

* 스레드는 모두 데몬 스레드[^역주1]입니다. 따라서 주 스레드는 이를 기다리지 않습니다.
* 스레드의 패닉은 서로 독립적으로 발생합니다.
  * (스레드의) 패닉은 (주 스레드에게) 페이로드를 전달하고, 이는 `downcast_ref`로 풀어볼 수 있습니다.
> * Threads are all daemon threads, the main thread does not wait for them.
> * Thread panics are independent of each other.
>   * Panics can carry a payload, which can be unpacked with `downcast_ref`.

---
역주

- 다른언어의 스레드 === js만 했으면 헬게이트 열리는 장입니다(...)

[^역주1]: 데몬 스레드는 일반 스레드의 보조 스레드로 주 스레드가 종료되면 같이 종료되고, 백그라운드에서 낮은 우선순위로 동작합니다. 
    - 데몬: 사용자가 직접적으로 제어하지 않고 백그라운드에서 동작하는 프로그램.