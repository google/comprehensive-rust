# Elevator Operation

Elevators seem simple. You press a button, doors open, you wait, and you're at
the floor you requested. But implementing an elevator controller is surprisingly
difficult! This exercise involves building a simple elevator control that
operates in a simple simulator.

The overall design of this elevator uses the actor pattern: you will implement a
controller task that communicates with other components of the elevator system
by sending and receiving messages.

## Getting Started

Download the [exercise template](../../comprehensive-rust-exercises.zip) and look in the `elevator`
directory for the following files.

`src/main.rs`:

<!-- File src/main.rs -->

```rust,compile_fail
{{#include elevator/src/main.rs}}
```

`src/building.rs`:

<!-- File src/building.rs -->

```rust,compile_fail
{{#include elevator/src/building.rs}}
```

`src/driver.rs`:

<!-- File src/driver.rs -->

```rust,compile_fail
{{#include elevator/src/driver.rs}}
```

`src/controller.rs`:

<!-- File src/controller.rs -->

```rust,compile_fail
{{#include elevator/src/controller.rs}}
```

`Cargo.toml` (you shouldn't need to change this):

<!-- File Cargo.toml -->

```toml
{{#include elevator/Cargo.toml}}
```

Use `cargo run` to run the elevator simulation.

## Exercises

Begin by implementing a controller that can transport the passengers provided by
the simple driver. There is only one elevator, and passengers always go from
floor 0 to floor 2, one-by-one.

Once you have this done, make the problem more complex. Suggested tasks:

 * Make the driver more complex, with passengers arriving at random floors with
   random destinations at random times.

 * Create a building with more than one elevator, and adjust the controller to
   handle this efficiently.

 * Add additional events and metadata to analyze your controller's efficiency.
   What is the distribution of wait time for passengers? Is the result fair?

 * Modify the building to support a maximum passenger capacity for each
   elevator, and modify the controller to take this information into account.

 * Update the driver to simulate business traffic, with lots of passengers going
   up from the ground floor at the same time, and those passengers returning to
   the ground floor some time later. Can your controller adjust to these
   circumstances?

 * Modify the building to support "destination dispatch", where passengers
   signal their destination floor in the elevator lobby, before boarding the
   elevator.

 * If you are taking the course with other students, trade controllers or
   drivers with another student to see how robust your design is.

 * Build a textual or graphical display of the elevators as they run.
