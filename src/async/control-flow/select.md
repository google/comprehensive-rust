# Select

## Run multiple futures concurrently until the first one resolves

### Equivalents:

- JS: `Promise.race`
- Python: `asyncio.new_event_loop().run_until_complete(asyncio.wait(task_set, return_when=asyncio.FIRST_COMPLETED))`

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
        sleep(Duration::from_secs(10)).await;
        cat_sender
            .send(String::from("Felix"))
            .await
            .expect("Failed to send cat.");
    });
    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await;
        dog_sender
            .send(String::from("Rex"))
            .await
            .expect("Failed to send cat.");
    });

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver)
        .await
        .expect("Failed to receive winner");

    assert_eq!(winner, Animal::Dog {name: String::from("Rex")});
}
```

<details>

* In this example, we have a race between a cat and a dog. `first_animal_to_finish_race` listens to both channels and will pick whichever arrives first. Since the dog takes 5 seconds, it wins against the cat that take 10 seconds.
* You can use `oneshot` channels in this example as the channels are supposed to receive only one `send`.
* You can try adding more contestants to the race and return a leaderboard. Also, you can add a deadline after which contestants get eliminated.

</details>
