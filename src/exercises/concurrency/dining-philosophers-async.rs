// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use tokio::time;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;


struct Fork;

/// We've already learnt that to avoid a deadlock, we have to break the
/// symmetry. So here we call the forks the first_fork and the second_fork.
/// The left fork is not necessarily the first fork.
struct Philosopher {
    name: String,
    first_fork: Arc<Mutex<Fork>>,
    second_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name)).await
            .unwrap();
    }

    async fn eat(&self) {
        // Pick up forks...
        let _first_lock = self.first_fork.lock().await;
        // Add a delay before picking the second fork to allow the execution
        // to transfer to another task
        time::sleep(time::Duration::from_millis(1)).await;
        let _second_lock = self.second_fork.lock().await;

        println!("{} is eating...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;

        // The locks are dropped here
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

#[tokio::main]
async fn main() {
    // Create forks
    let mut forks = vec![];
    (0..PHILOSOPHERS.len()).for_each(|_| forks.push(Arc::new(Mutex::new(Fork))));

    // Create philosophers
    let (philosophers, mut rx) = {
        let mut philosophers = vec![];
        let (tx, rx) = mpsc::channel(10);
        for (i, name) in PHILOSOPHERS.iter().enumerate() {
            let left_fork = forks[i].clone();
            let right_fork = forks[(i + 1) % PHILOSOPHERS.len()].clone();
            philosophers.push(Philosopher {
                name: name.to_string(),
                first_fork: if i % 2 == 0 { left_fork.clone() } else { right_fork.clone() },
                second_fork: if i % 2 == 0 { right_fork.clone() } else { left_fork.clone() },
                thoughts: tx.clone(),
            });
        }
        (philosophers, rx)
        // tx is dropped here, so we don't need to explicitly drop it later
    };

    // Make them think and eat
    for phil in philosophers {
        tokio::spawn(async move {
            for _ in 0..100 {
                phil.think().await;
                phil.eat().await;
            }
        });

    }

    // Output their thoughts
    while let Some(thought) = rx.recv().await {
        println!("Here is a thought: {thought}");
    }
}
