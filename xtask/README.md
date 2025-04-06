# xtask

The purpose of the xtask binary is to enable cross platform task automation
within the project (somewhat similar to how `npm run` is used in Node.js
projects to run scripts). Please see
[cargo xtask](https://github.com/matklad/cargo-xtask) for more information.

To add support for a new task, add a new arm to the `match` in the `execute_task` function, and add a new handler function that contains the logic.
