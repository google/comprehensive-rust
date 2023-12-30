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

#![allow(dead_code)]

// ANCHOR: solution
// ANCHOR: event
#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    // ANCHOR_END: event
    /// A button was pressed.
    ButtonPressed(Button),

    /// The car has arrived at the given floor.
    CarArrived(Floor),

    /// The car's doors have opened.
    CarDoorOpened,

    /// The car's doors have closed.
    CarDoorClosed,
}

/// A floor is represented as an integer.
type Floor = i32;

// ANCHOR: direction
/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}
// ANCHOR_END: direction

/// A user-accessible button.
#[derive(Debug)]
enum Button {
    /// A button in the elevator lobby on the given floor.
    LobbyCall(Direction, Floor),

    /// A floor button within the car.
    CarFloor(Floor),
}

// ANCHOR: car_arrived
/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    // ANCHOR_END: car_arrived
    Event::CarArrived(floor)
}

// ANCHOR: car_door_opened
/// The car doors have opened.
fn car_door_opened() -> Event {
    // ANCHOR_END: car_door_opened
    Event::CarDoorOpened
}

// ANCHOR: car_door_closed
/// The car doors have closed.
fn car_door_closed() -> Event {
    // ANCHOR_END: car_door_closed
    Event::CarDoorClosed
}

// ANCHOR: lobby_call_button_pressed
/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    // ANCHOR_END: lobby_call_button_pressed
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

// ANCHOR: car_floor_button_pressed
/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    // ANCHOR_END: car_floor_button_pressed
    Event::ButtonPressed(Button::CarFloor(floor))
}

// ANCHOR: main
fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
// ANCHOR_END: main
