# probe-rs, cargo-embed

[probe-rs](https://probe.rs/) is a handy toolset for embedded debugging, like OpenOCD but better
integrated.

* <abbr title="Serial Wire Debug">SWD</abbr> and JTAG via CMSIS-DAP, ST-Link and J-Link probes
* GDB stub and Microsoft <abbr title="Debug Adapter Protocol">DAP</abbr> server
* Cargo integration

`cargo-embed` is a cargo subcommand to build and flash binaries, log
<abbr title="Real Time Transfers">RTT</abbr> output and connect GDB. It's configured by an
`Embed.toml` file in your project directory.

<details>

* [CMSIS-DAP](https://arm-software.github.io/CMSIS_5/DAP/html/index.html) is an Arm standard
  protocol over USB for an in-circuit debugger to access the CoreSight Debug Access Port of various
  Arm Cortex processors. It's what the on-board debugger on the BBC micro:bit uses.
* ST-Link is a range of in-circuit debuggers from ST Microelectronics, J-Link is a range from
  SEGGER.
* The Debug Access Port is usually either a 5-pin JTAG interface or 2-pin Serial Wire Debug.
* probe-rs is a library which you can integrate into your own tools if you want to.
* The [Microsoft Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol/) lets
  VSCode and other IDEs debug code running on any supported microcontroller.
* cargo-embed is a binary built using the probe-rs library.
* RTT (Real Time Transfers) is a mechanism to transfer data between the debug host and the target
  through a number of ringbuffers.

</details>
