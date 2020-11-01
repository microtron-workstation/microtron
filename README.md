![banner](https://i.imgur.com/1b9slCL.png)

## What's this?

`microtron` is a modular digital audio workstation which ties together audio modules into a process-based graph that can be used to produce and perform music. 

A module is defined as a struct implementing the `Module` trait. Encapsulating it within a `ModuleServer` turns it into a networking node able to communicate with the coordinator and other modules via UDP. On the other hand, modules can be composed within a single application by using the `Graph` and `Chain` types.

This repository contains the coordinator.

## Features

#### Process-Based Module Graph
The coordinator spawns processes and forwards packets holding audio buffers, MIDI events or parameter changes to their destinations using UDP connections. An implementor of the `Module` trait can be wrapped in a structure which enables it to communicate over the network and register itself with the coordinator, turning it into a process.

#### Application-Level Dataflow Structures
The `microtron_module` crate provides pre-built modules which enclose other modules in chain and graph structures, allowing for complex data-flow setups within a single process for digital instruments and effects.

#### Extensible Design
The coordinator is structurally similar to a microkernel in that it only handles the most essential work needed for the communication between network-enabled modules, leaving secondary things like audio rendering, graph synchronization as well as graphical, note and device interfaces to separate modules.

This makes it easy to switch out a module that is incompatible with your environment, for example if your code is running on a System-on-a-Chip where you might not be able to output sound like you usually would. *(The `no_std` support enabling this in the first place is coming soon)*

#### Save and Restore
Save a human-readable YAML snapshot of the current processes, their connections, the arrangement and other coordinator state so it can be loaded at a later point in time.

## Getting Started
As long as you have a recent Rust toolchain installed and your PATH environment variable contains the folder where Cargo installs binaries, you should be good to go.

### Compile and run in-folder
Use `cargo run` while hacking on the codebase, and compile with the release flag enabled (`cargo run --release`) if you need a build optimized for performance.

### Installation and Usage
Installing a copy of the Microtron coordinator onto your system can be done by simply running `cargo install` inside the root of the repository. After the command has completed successfully, verify the installation by entering `microtron` into the console. If all went well, you should be greeted by the coordinator and dropped into a blank Microtron setup.

## Contributing
As I'm still figuring this out myself, it might be a little early, but if you have an idea for a feature or a change and you want to see it integrated, please don't hesitate to create a pull request or project issue.

## License
**This project is licensed unter the MIT license.**
