//! The building simulates floors and elevators.

use tokio::sync::{broadcast, mpsc};
use tokio::task;
use tokio::time;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
}

/// A passenger is a person with a destination floor in mind.
#[derive(Debug)]
struct Passenger {
    destination: FloorId,
}

/// FloorId identifies a floor. These are zero-based integers.
pub type FloorId = usize;

/// Floor represents the current status of a floor in the building.
#[derive(Default, Debug)]
struct Floor {
    passengers: Vec<Passenger>,
}

/// ElevatorId identifies an elevator in the building. These are zero-based integers.
pub type ElevatorId = usize;

/// Elevator represents the current status of an elevator in the building.
#[derive(Default, Debug)]
struct Elevator {
    /// Floor the elevator is currently on. In the simulation the elevator
    /// transports instantaneously from one floor to the next in a single
    /// simulation tick.
    position: FloorId,
    /// Destination floor for the elevator, if any. This can change at any time.
    destination: Option<FloorId>,
    /// Passengers currently on the elevator.
    passengers: Vec<Passenger>,
    /// True if the elevator is stopped with the doors open. The elevator
    /// will not move with the doors open, but they will close at the next
    /// tick of the simulation.
    doors_open: bool,
}

/// A BuildingEvent is an event that occurs in the building.
#[derive(Debug, Clone)]
pub enum BuildingEvent {
    /// A passenger has pressed a floor button in the elevator.
    FloorButtonPressed(ElevatorId, FloorId),
    /// A passenger on the given floor has pressed the call button.
    CallButtonPressed(FloorId, Direction),
    /// The elevator has arrived at the given floor. If this is the
    /// elevator's destination, then it will stop open its doors.
    AtFloor(ElevatorId, FloorId),
    /// A passenger has been delivered to their desired floor.
    PassengerDelivered(FloorId),
}

/// A BuildingCommand tells the building what to do.
#[derive(Debug)]
pub enum BuildingCommand {
    /// Set the elevator's destination. The elevator will close its doors
    /// if necessary and then begin moving toward this floor.
    GoToFloor(ElevatorId, FloorId),
}

/// A DriverCommand is a message from the driver to change the state of
/// the building.
#[derive(Debug)]
pub enum DriverCommand {
    /// A passenger has arrived and is waiting for an elevator. The passenger will automatically
    /// press the relevant call button, board the elevator when it arrives, press their floor
    /// button, and depart when the doors open on their destination floor.
    PassengerArrived { at: FloorId, destination: FloorId },

    /// Halt all activity in the building and end the building task.
    Halt,
}

/// Building manages the current status of the building.
#[derive(Debug)]
pub struct Building {
    floors: Vec<Floor>,
    elevators: Vec<Elevator>,
}

impl Building {
    pub fn new(num_floors: usize, num_elevators: usize) -> Self {
        let mut floors = vec![];
        for _ in 0..num_floors {
            floors.push(Floor::default());
        }
        let mut elevators = vec![];
        for _ in 0..num_elevators {
            elevators.push(Elevator::default());
        }
        Self { floors, elevators }
    }

    /// Start the building. The resulting channels are used to communicate
    /// with the building
    pub fn start(
        self,
    ) -> (
        task::JoinHandle<()>,
        broadcast::Receiver<BuildingEvent>,
        mpsc::Sender<BuildingCommand>,
        mpsc::Sender<DriverCommand>,
    ) {
        let (events_tx, events_rx) = broadcast::channel(10);
        let (building_cmd_tx, building_cmd_rx) = mpsc::channel(10);
        let (driver_cmd_tx, driver_cmd_rx) = mpsc::channel(10);
        let task = tokio::spawn(self.run(events_tx, building_cmd_rx, driver_cmd_rx));
        (task, events_rx, building_cmd_tx, driver_cmd_tx)
    }

    async fn run(
        mut self,
        events_tx: broadcast::Sender<BuildingEvent>,
        mut building_cmd_rx: mpsc::Receiver<BuildingCommand>,
        mut driver_cmd_rx: mpsc::Receiver<DriverCommand>,
    ) {
        let mut ticker = time::interval(time::Duration::from_millis(100));
        loop {
            tokio::select! {
                Some(BuildingCommand::GoToFloor(el, fl)) = building_cmd_rx.recv() => {
                    self.elevators[el].destination = Some(fl);
                }
                Some(cmd) = driver_cmd_rx.recv() => {
                    match cmd {
                        DriverCommand::PassengerArrived{at, destination} => {
                            self.new_passenger(&events_tx, at, destination).await;
                        }
                        DriverCommand::Halt => return,
                    }
                }
                _ = ticker.tick() => self.move_elevators(&events_tx).await
            }
        }
    }

    /// Move the elevators toward their destinations.
    async fn move_elevators(&mut self, events_tx: &broadcast::Sender<BuildingEvent>) {
        for el in 0..self.elevators.len() {
            let elevator = &mut self.elevators[el];

            // If the elevator's doors are open, close them and wait for the next tick.
            if elevator.doors_open {
                elevator.doors_open = false;
                continue;
            }

            // If the elevator has somewhere to go, move toward it.
            if let Some(dest) = elevator.destination {
                if dest > elevator.position {
                    elevator.position += 1;
                }
                if dest < elevator.position {
                    elevator.position -= 1;
                }
                events_tx
                    .send(BuildingEvent::AtFloor(el, elevator.position))
                    .unwrap();

                // If the elevator has reached its destination, open
                // the doors and let passengers get on and off.
                if elevator.position == dest {
                    elevator.destination = None;
                    elevator.doors_open = true;
                    self.exchange_passengers(&events_tx, el).await;
                }
            }
        }
    }

    /// Handle a new passenger arriving at the given floor.
    async fn new_passenger(
        &mut self,
        events_tx: &broadcast::Sender<BuildingEvent>,
        at: FloorId,
        destination: FloorId,
    ) {
        println!("Passenger arrived at {} going to {}", at, destination);
        if at == destination {
            events_tx
                .send(BuildingEvent::PassengerDelivered(destination))
                .unwrap();
            return;
        }

        self.floors[at].passengers.push(Passenger { destination });
        let dir = if at < destination {
            Direction::Up
        } else {
            Direction::Down
        };
        events_tx
            .send(BuildingEvent::CallButtonPressed(at, dir))
            .unwrap();
    }

    /// The doors for the given elevator are open, so take on and discharge passengers.
    async fn exchange_passengers(
        &mut self,
        events_tx: &broadcast::Sender<BuildingEvent>,
        el: ElevatorId,
    ) {
        let elevator = &mut self.elevators[el];
        let fl = elevator.position;

        // Handle passengers leaving the elevator at their floor.
        let (this_floor, other_floors): (Vec<Passenger>, Vec<Passenger>) = elevator
            .passengers
            .drain(..)
            .partition(|px| px.destination == fl);
        for px in this_floor {
            events_tx
                .send(BuildingEvent::PassengerDelivered(px.destination))
                .unwrap();
        }
        elevator.passengers = other_floors;

        // Handle passengers entering the elevator.
        for px in self.floors[fl].passengers.drain(..) {
            events_tx
                .send(BuildingEvent::FloorButtonPressed(el, px.destination))
                .unwrap();
            elevator.passengers.push(px);
        }
    }
}
