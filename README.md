![banner](https://i.imgur.com/1b9slCL.png)

## Introduction

Microtron is a modular digital audio workstation which ties together audio modules into a process-based graph that can be used to produce and perform music.

This repository contains a piece of software called *the coordinator*, which provides the `microtron` binary, and it has two basic responsibilities: relay packets between modules, and launch, maintain and destroy instances of process-level modules if a snapshot has been restored during the current session.

The most important piece of the system is formed by the `Module` trait, which represents a piece of code that synthesizes or processes frames worth of audio. Enabling a module to join the process-level graph can be done by inserting it into a `ModuleServer` and launching the server. Additionally, the abstraction allows for the implementation of complex digital signal processing logic within an audio process by organizing modules into recursive `Chain` and `Graph` structures.

## Features

#### Process-Based Module Graph
The coordinator spawns processes and forwards packets holding audio buffers, MIDI events or parameter changes to their destinations using UDP connections. An implementor of the `Module` trait can be wrapped in a structure which enables it to communicate over the network and register itself with the coordinator, turning it into a process.

#### Application-Level Dataflow Structures
The `microtron_module` crate provides pre-built modules which enclose other modules in chain and graph structures, allowing for complex data-flow setups within a single process for digital instruments and effects.

#### Extensible Design
The coordinator is structurally similar to a microkernel in that it only handles the most essential work needed for the communication between network-enabled modules, leaving higher-level tasks like audio rendering, graph synchronization as well as graphical, note and device interfaces to separate modules.

This makes it easy to replace a module incompatible with your environment, for example if your code is running on a System-on-a-Chip where you might not be able to output sound like you usually would. *(The `no_std` support enabling this in the first place is coming soon)*

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

## Contributing
As I'm still figuring this out myself, it might be a little early for this, but if you have an idea for a feature or a change, or if you'd like to report a malfunction, please don't hesitate to create a pull request or open an issue.

## License
**This project is licensed unter the MIT license.**
