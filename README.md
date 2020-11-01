![banner](https://i.imgur.com/1b9slCL.png)

## What's this?

`microtron` is a modular digital audio workstation which ties together audio modules into a process-based graph that can be used to produce and perform music. 

A module is defined as a struct implementing the `Module` trait. Encapsulating it within a `ModuleServer` turns it into a networking node able to communicate with the coordinator and other modules via UDP. On the other hand, modules can be composed within a single application by using the `Graph` and `Chain` types.

This repository contains the coordinator.

## Features

#### Process-based Module Graph
The coordinator spawns processes and forwards messages between them, like audio buffers, MIDI events or parameter changes. A struct implementing the `Module` trait can be wrapped in a structure that enables it to communicate over the network and register itself with the coordinator, turning it into a process.

#### Application-Level Dataflow Structures
The `microtron_module` crate provides pre-built modules which enclose other implementors of the `Module` trait in chain and graph structures, allowing for complex data-flow setups within a single application for digital instruments and effects.

#### Extensible Design
The coordinator is structurally similar to a microkernel in that it only handles the most essential work needed for the communication between network-enabled modules, leaving secondary things like audio rendering, graph synchronization as well as graphical, note and device interfaces to separate modules.

This makes it easy to switch out a module that is incompatible with your environment, for example if your code is running on a SoC where you might not be able to output sound like you usually would. *(The `no_std` support enabling this in the first place is in the works)*

#### Save and Restore
Save a human-readable YAML snapshot of the current processes, their connections, the arrangement and other coordinator data for later use.

## Installation

Make sure you have a Rust toolchain installed, then run the following command to compile and run the application:

```
cargo run --release
```

This will bind the packet forwarder to  [udp://localhost:32000](udp://localhost:32000), then you should be able to connect modules by sending a `Handshake` packet.
