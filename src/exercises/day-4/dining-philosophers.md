# Dining Philosophers

식사하는 철학자 문제는 동시성에 있어서 고전적인 문제입니다:

> 5명의 철학자가 원탁에서 식사를 하고 있습니다. 
> 철학자는 원탁에서 자신의 자리에 앉아있습니다.
> 포크는 각 접시 사이에 있습니다. 
> 제공되는 요리를 먹기 위해서는 2개의 포크를 모두 사용해야합니다.
> 철학자는 생각을 하다가 배가 고프면 자신의 좌우의 포크를 들어 요리를 먹습니다.
> 철학자는 요리를 먹은 후에는 포크를 다시 자리에 내려놓습니다.
> 철학자는 자신의 좌,우에 포크가 있을때만 요리를 먹을 수 있습니다.
> 따라서 두개의 포크는 오직 자신의 좌,우 철학자가 생각을 할 때만 사용할 수 있습니다.

이번 훈련에서는 [로컬 카고 설치](../../cargo/running-locally.md)가 필요합니다.
아래 코드를 복사해서 `src/main.rs`에 붙여놓고 빈 부분을 채우고, `cargo run` 커맨드로 테스트 해서 교착상태(데드락)이 되지 않는지 확인합니다:

> The dining philosophers problem is a classic problem in concurrency:
> 
> > Five philosophers dine together at the same table. Each philosopher has their
> > own place at the table. There is a fork between each plate. The dish served is
> > a kind of spaghetti which has to be eaten with two forks. Each philosopher can
> > only alternately think and eat. Moreover, a philosopher can only eat their
> > spaghetti when they have both a left and right fork. Thus two forks will only
> > be available when their two nearest neighbors are thinking, not eating. After
> > an individual philosopher finishes eating, they will put down both forks.
> 
> You will need a local [Cargo installation](../../cargo/running-locally.md) for
> this exercise. Copy the code below to `src/main.rs` file, fill out the blanks,
> and test that `cargo run` does not deadlock:

```rust,compile_fail
{{#include dining-philosophers.rs:Philosopher}}
    // left_fork: ...
    // right_fork: ...
    // thoughts: ...
}

{{#include dining-philosophers.rs:Philosopher-think}}

{{#include dining-philosophers.rs:Philosopher-eat}}
        // Pick up forks...
{{#include dining-philosophers.rs:Philosopher-eat-end}}
    // Create forks

    // Create philosophers

    // Make them think and eat

    // Output their thoughts
}
```
---
역주
- 로컬에서 `cargo new dining-philosopher`로 프로젝트를 생성해서 위 코드 붙여넣고 시작하면 됩니다.
- 데드락 회피 알고리즘 자체를 테스트 하는건 아니니 구글 서칭 go. 
    - 역자는 위치에 따라(홀짝) 드는 포크 순서를 바꾸는 방식으로 했습니다. 
