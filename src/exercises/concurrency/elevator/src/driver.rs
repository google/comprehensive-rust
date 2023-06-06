//! The driver controls when and where passengers arrive.

use crate::building::{Building, BuildingEvent, DriverCommand};
use tokio::sync::{broadcast, mpsc};

/// Create a new building to be driven by this driver.
pub fn make_building() -> Building {
    Building::new(3, 1)
}

/// Simulate people arriving at the ground floor and going to the first floor, one by one.
pub async fn driver(
    mut events_rx: broadcast::Receiver<BuildingEvent>,
    driver_cmd_tx: mpsc::Sender<DriverCommand>,
) {
    for _ in 0..3 {
        // A passenger has arrived..
        driver_cmd_tx
            .send(DriverCommand::PassengerArrived {
                at: 0,
                destination: 2,
            })
            .await
            .unwrap();

        // Wait until they are delivered..
        while let Ok(evt) = events_rx.recv().await {
            if let BuildingEvent::PassengerDelivered(_) = evt {
                break;
            }
        }
    }

    driver_cmd_tx.send(DriverCommand::Halt).await.unwrap();
}
