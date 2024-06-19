use anyhow::Result;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};

#[derive(Debug)]
struct Work {
    input: u32,
    respond_on: oneshot::Sender<u32>,
}

async fn worker(mut work_queue: mpsc::Receiver<Work>) {
    let mut _iterations = 0;
    loop {
        tokio::select! {
            Some(work) = work_queue.recv() => {
                sleep(Duration::from_millis(10)).await; // Pretend to work.
                work.respond_on.send(work.input * 1000).unwrap();
                _iterations += 1;
            }
            // TODO: report number of iterations every 100ms
        }
    }
}

async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> Result<u32> {
    let (tx, rx) = oneshot::channel();
    work_queue.send(Work { input, respond_on: tx }).await?;
    Ok(rx.await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(10);
    tokio::spawn(worker(rx));
    for i in 0..100 {
        let resp = do_work(&tx, i).await?;
        println!("work result for iteration {i}: {resp}");
    }
    Ok(())
}
