//! The controller directs the elevators to operate so that passengers
//! get to their destinations.

use crate::building::{BuildingCommand, BuildingEvent};
use tokio::sync::{broadcast, mpsc};

pub async fn controller(
    mut events_rx: broadcast::Receiver<BuildingEvent>,
    building_cmd_tx: mpsc::Sender<BuildingCommand>,
) {
    while let Ok(evt) = events_rx.recv().await {
        match evt {
            BuildingEvent::CallButtonPressed(at, _) => {
                building_cmd_tx
                    .send(BuildingCommand::GoToFloor(0, at))
                    .await
                    .unwrap();
            }
            BuildingEvent::FloorButtonPressed(_, destination) => {
                building_cmd_tx
                    .send(BuildingCommand::GoToFloor(0, destination))
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }
}
