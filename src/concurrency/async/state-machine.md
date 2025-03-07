---
minutes: 7
---

# State Machine

Rust transforms an async function or block to a hidden type that implements
`Future`, using a state machine to track the function's progress. The details of
this transform are complex, but it helps to have a schematic understanding of
what is happening.

```rust,editable,compile_fail
use futures::executor::block_on;
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

async fn send(s: String) -> usize {
    println!("{}", s);
    s.len()
}

/*
async fn example(x: i32) -> usize {
    let double_x = x*2;
    let mut count = send(format!("x = {x}")).await;
    count += send(format!("double_x = {double_x}")).await;
    count
}
*/

fn example(x: i32) -> ExampleFuture {
    ExampleFuture::Init { x }
}

#[pin_project(project=ExampleFutureProjected)]
enum ExampleFuture {
    Init {
        x: i32,
    },
    FirstSend {
        double_x: i32,
        #[pin]
        fut: Pin<Box<dyn Future<Output = usize>>>,
    },
    SecondSend {
        count: usize,
        #[pin]
        fut: Pin<Box<dyn Future<Output = usize>>>,
    },
}

impl std::future::Future for ExampleFuture {
    type Output = usize;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.as_mut().project() {
                ExampleFutureProjected::Init { x } => {
                    let double_x = *x * 2;
                    let fut = Box::pin(send(format!("x = {x}")));
                    *self = ExampleFuture::FirstSend { double_x, fut };
                }
                ExampleFutureProjected::FirstSend { double_x, mut fut } => {
                    match fut.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(count) => {
                            let fut =
                                Box::pin(send(format!("double_x = {double_x}")));
                            *self = ExampleFuture::SecondSend { count, fut };
                        }
                    }
                }
                ExampleFutureProjected::SecondSend { count, mut fut } => {
                    match fut.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(tmp) => {
                            *count += tmp;
                            return Poll::Ready(*count);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    println!("result: {}", block_on(example(5)));
}
```

<details>

While this code will run, it is simplified from what the real state machine
would do. The important things to notice here are:

- Calling an async function does nothing but construct a value, ready to start
  on the first call to `poll`.
- All local variables are stored in the function's future struct, including an
  enum to identify where execution is currently suspended. The real generated
  state machine would not initialize `i` to 0.
- An `.await` in the async function is translated into a call to that async
  function, then polling the future it returns until it is `Poll::Ready`. The
  real generated state machine would contain the future type defined by `send`,
  but that cannot be expressed in Rust syntax.
- Execution continues eagerly until there's some reason to block. Try returning
  `Poll::Pending` in the `ExampleState::Init` branch of the match, in hopes that
  `poll` will be called again with state `ExampleState::Sending`. `block_on`
  will not do so!

</details>
