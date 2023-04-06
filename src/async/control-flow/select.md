# Select

A select operation waits until any of a set of futures is ready, and responds to
that future's result. In JavaScript, this is similar to `Promise.race`. In
Python, it compares to `asyncio.wait(task_set,
return_when=asyncio.FIRST_COMPLETED)`.

This is usually a macro, similar to match, with each arm of the form `pattern =
future => statement`. When the future is ready, the statement is executed with the
variable bound to the future's result.

```rust,editable,compile_fail
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq)]
enum Animal {
    Cat { name: String },
    Dog { name: String },
}

async fn first_animal_to_finish_race(
    mut cat_rcv: Receiver<String>,
    mut dog_rcv: Receiver<String>,
) -> Option<Animal> {
    tokio::select! {
        cat_name = cat_rcv.recv() => Some(Animal::Cat { name: cat_name? }),
        dog_name = dog_rcv.recv() => Some(Animal::Dog { name: dog_name? })
    }
}

#[tokio::main]
async fn main() {
    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);
    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        cat_sender
            .send(String::from("Felix"))
            .await
            .expect("Failed to send cat.");
    });
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        dog_sender
            .send(String::from("Rex"))
            .await
            .expect("Failed to send dog.");
    });

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver)
        .await
        .expect("Failed to receive winner");

    println!("Winner is {winner:?}");
}
```

<details>

* In this example, we have a race between a cat and a dog.
  `first_animal_to_finish_race` listens to both channels and will pick whichever
  arrives first. Since the dog takes 50ms, it wins against the cat that
  take 500ms seconds.

* You can use `oneshot` channels in this example as the channels are supposed to
  receive only one `send`.

* Try adding a deadline to the race, demonstrating selecting different sorts of
  futures.

* Note that `select!` consumes the futures it is given, and is easiest to use
  when every execution of `select!` creates new futures.

</details>
