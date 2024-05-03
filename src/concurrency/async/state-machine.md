---
minutes: 5
---

# State Machine

Rust transforms an async function or block to a hidden type that implements
`Future`, using a state machine to track the function's progress. The details of
this transform are complex, but it helps to have a schematic understanding of
what is happening.

```rust,editable
use futures::executor::block_on;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

async fn send(s: &str) {
    println!("{s}");
}

/*
async fn count_to(count: i32) {
    for i in 1..=count {
        send("tick").await;
    }
}
*/

fn count_to(count: i32) -> CountToFuture {
    CountToFuture { state: CountToState::Init, count, i: 0 }
}

struct CountToFuture {
    state: CountToState,
    count: i32,
    i: i32,
}

enum CountToState {
    Init,
    Sending(Pin<Box<dyn Future<Output = ()>>>),
}

impl std::future::Future for CountToFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match &mut self.state {
                CountToState::Init => {
                    self.i = 1;
                    self.state = CountToState::Sending(Box::pin(send("tick")));
                }
                CountToState::Sending(send_future) => {
                    match send_future.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(_) => {
                            self.i += 1;
                            if self.i > self.count {
                                return Poll::Ready(());
                            } else {
                                self.state =
                                    CountToState::Sending(Box::pin(send("tick")));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    block_on(count_to(5));
}
```

<details>

While this code will run, it is simplified from what the real state machine
would do. The important things to notice here are:

- Calling an async function does nothing but construct a value, ready to start
  on the first call to `poll`.
- All local variables are stored in the function's future struct, including an
  enum to identify where exection is currently suspended. The real generated
  state machine would not initialize `i` to 0.
- An `.await` in the async function is translated into a call to that async
  function, then polling the future it returns until it is `Poll::Ready`. The
  real generated state machine would not box this future.
- Execution continues eagerly until there's some reason to block. Try returning
  `Poll::Pending` in the `CountToState::Init` branch of the match, in hopes that
  `poll` will be called again with state `CountToState::Sending`. `block_on`
  will not do so!

</details>
