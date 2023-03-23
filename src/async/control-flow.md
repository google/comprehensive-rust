# Futures Control Flow

Futures can be combined together to produce concurrent compute flow graphs.
We will demonstrate here a non-exhaustive array of common operations:

## Run a group of futures concurrently until they all resolve: `join_all`

### Equivalents:

- JS: `Promise.all`
- Python: `asyncio.gather`

```rust,editable,compile_fail
use anyhow::Result;
use futures::future;
use reqwest;
use std::collections::HashMap;

async fn size_of_page(url: &str) -> Result<usize> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())
}

#[tokio::main]
async fn main() {
    let urls: [&str; 4] = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
        "BAD_URL",
    ];
    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_sizes_dict: HashMap<&str, Result<usize>> =
        urls.into_iter().zip(results.into_iter()).collect();
    println!("{:?}", page_sizes_dict);
}
```

## Run multiple futures concurrently until the first one resolves: `select`

### Equivalents:

- JS: `Promise.race`
- Python: `asyncio.new_event_loop().run_until_complete(asyncio.wait(task_set, return_when=asyncio.FIRST_COMPLETED))`

```rust,editable,compile_fail
use anyhow::Result;
use tokio::sync::mpsc::{self, Receiver, Sender};
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

async fn simulate_animals_race(
    sender: Sender<String>,
    duration: Duration,
    name: String,
) -> Result<()> {
    sleep(duration).await;
    sender.send(name).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);
    tokio::spawn(simulate_animals_race(
        cat_sender,
        Duration::from_secs(10),
        String::from("Felix"),
    ));
    tokio::spawn(simulate_animals_race(
        dog_sender,
        Duration::from_secs(5),
        String::from("Rex"),
    ));

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver)
        .await
        .expect("Failed to receive winner");

    println!("The winner of the race is {:?}", winner);

    assert_eq!(
        winner,
        Animal::Dog {
            name: String::from("Rex")
        }
    );
}
```

## Iterate over multiple channels with a timeout: `select` with `pin`

```rust,editable,compile_fail
use anyhow::Result;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq)]
enum AnimalRacePerformance {
    Cat { name: String },
    Dog { name: String },
}

async fn race_finish_line(
    mut cat_rcv: Receiver<String>,
    mut dog_rcv: Receiver<String>,
    timeout: Duration,
) -> Option<Vec<AnimalRacePerformance>> {
    let mut performances: Vec<AnimalRacePerformance> = Vec::new();
    let timeout_sleep = sleep(timeout);
    // See the chapter about pin.
    tokio::pin!(timeout_sleep);

    loop {
        tokio::select! {
            cat_name = cat_rcv.recv() => performances.push(
                AnimalRacePerformance::Cat { name: cat_name? }),
            dog_name = dog_rcv.recv() => performances.push(
                AnimalRacePerformance::Dog { name: dog_name? }),
            _ = timeout_sleep.as_mut() => break
        }
    }
    Some(performances)
}

async fn simulate_one_animal_race(
    sender: Sender<String>,
    duration: Duration,
    name: String,
) -> Result<()> {
    sleep(duration).await;
    sender.send(name).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);

    // Asked Bard to generate cat and dogs names.
    let cat_names_and_time = [
        ("Leo", 9),
        ("Milo", 3),
        ("Luna", 13),
        ("Oliver", 5),
        ("Charlie", 11),
    ];
    let dog_names_and_time = [
        ("Scout", 14),
        ("Rex", 4),
        ("Teddy", 20),
        ("Ollie", 1),
        ("Finn", 2),
    ];

    let finish_line_future = race_finish_line(cat_receiver, dog_receiver, Duration::from_secs(6));

    for (cat_name, duration_secs) in cat_names_and_time {
        tokio::spawn(simulate_one_animal_race(
            cat_sender.clone(),
            Duration::from_secs(duration_secs),
            cat_name.to_string(),
        ));
    }

    for (dog_name, duration_secs) in dog_names_and_time {
        tokio::spawn(simulate_one_animal_race(
            dog_sender.clone(),
            Duration::from_secs(duration_secs),
            dog_name.to_string(),
        ));
    }

    println!(
        "{:?}",
        finish_line_future
            .await
            .expect("Failed to collect finish line")
    );
    // [Dog { name: "Ollie" }, Dog { name: "Finn" }, Cat { name: "Milo" }, Dog { name: "Rex" }, Cat { name: "Oliver" }]
}
```
