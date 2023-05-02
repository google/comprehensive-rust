use building::BuildingEvent;
use tokio::sync::broadcast;

mod building;
mod controller;
mod driver;

#[tokio::main]
async fn main() {
    let building = driver::make_building();
    let (building_task, events_rx, building_cmd_tx, driver_cmd_tx) = building.start();

    tokio::spawn(print_events(events_rx.resubscribe()));
    tokio::spawn(driver::driver(events_rx.resubscribe(), driver_cmd_tx));
    tokio::spawn(controller::controller(events_rx, building_cmd_tx));
    building_task.await.unwrap();
}

async fn print_events(mut events_rx: broadcast::Receiver<BuildingEvent>) {
    while let Ok(evt) = events_rx.recv().await {
        println!("BuildingEvent::{:?}", evt);
    }
}
