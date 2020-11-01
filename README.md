![banner](https://i.imgur.com/NyLmOfr.png)

<> (https://i.imgur.com/1b9slCL.png)

## Introduction

Microtron is a modular digital audio workstation which ties together audio modules into a process-based graph that can be used to produce and perform music.

This repository contains a piece of software called the **coordinator**, which acts as a central communication hub between all process-level modules. Compiled to machine code, it's referred to as the `microtron` binary, and it has two basic responsibilities:

#### Relaying packets from one process-level module to another

All arriving packets are sent through the coordinator to locate the right destination according to the internal graph representation, then sent out as quickly as possible again.

*We should probably also use a timecode of some sort to ensure the right order of sample buffers since UDP packets have the possibility of being out of order.*

#### Launching, maintaining and destroying instances of process-level modules

If a snapshot has been restored during the current session, the system programmatically launches and configures the process-level instances if they're not already running and correctly configured. Exiting the coordinator in this state involves a graceful shutdown by destroying the subprocess instances spawned earlier before Microtron terminates itself.

## Features

#### Process-Based Module Graph
The foundation of Microtron's digital signal processing logic, a single block, is realized as the `Module` trait. It represents a piece of code synthesizing or processing frames worth of audio, and can be put to two uses: either joining the process-level graph inside a `ModuleServer`, or being composed with others in the application-level `Chain` and `Graph` structures. Since those are modules too, they can be used recursively.

#### Application-Level Dataflow Structures
The `microtron_module` crate provides pre-built modules implementing audio chains and graphs as well as utility modules like gain control and mixers, allowing for complex dataflow setups within a single application for digital instruments and effects.

#### Extensible Design
The coordinator is structurally similar to a microkernel in that it only handles the most fundamental work required for safe and reliable communication between process-level modules, leaving higher-level tasks like audio rendering, device interfaces and song arrangement to external implementors.

This makes it easy to replace a module which is incompatible with your environment, for example if your code was running on a System-on-a-Chip where you might not be able to output sound like you usually would. *(Yes, support for `no_std` will arrive in the future.)*

#### Save and Restore
Save a human-readable YAML snapshot of the current processes, their connections, the arrangement and other coordinator state so it can be loaded at a later point in time.

## Getting Started
As long as you have a recent Rust toolchain installed and your PATH environment variable contains the folder where Cargo installs binaries, you should be good to go.

### Compile and run in-folder
Use the following commands to compile while hacking on the codebase or trying without installing it onto the system right away:

```
cargo run                    # Compile development build
cargo run --release          # Compile build with optimizations (may take much longer)
```

### Installation and Usage
Installing a copy of the Microtron coordinator onto your system can be done by simply running the following command inside the root of the repository:

```cargo install``` 

After the command has completed successfully, you can verify the installation by entering `microtron` into the console. If everything went fine, you should be greeted by the coordinator setting up a blank Microtron setup.

## Project Status

As of November 2020, this project is still very much in the alpha phase. 
**Keep in mind that this project may experience intense, earth-shattering change on its way to the first stable release.**

## Contributing
As I'm still figuring this out myself, it might be a little early for this, but if you have an idea for a feature or a change, or if you'd like to report a malfunction, please don't hesitate to create a pull request or open an issue.

## License
**Licensed unter the MIT license.**
