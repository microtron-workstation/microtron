![banner](https://i.imgur.com/NyLmOfr.png)

## Introduction

Microtron is a modular digital audio workstation which ties together audio modules into a process-based graph that can be used to produce and perform music.

This repository contains a piece of software called the **coordinator**, which acts as a central communication hub between all process-level modules. Compiled to machine code, it's referred to as `microtron`, and it takes care of two basic tasks:

#### Relaying packets from one process-level module to another

All arriving packets are sent through the coordinator to locate the right destination according to the internal graph representation, then sent out as quickly as possible again.

*We should probably also use a timecode of some sort to ensure the right order of sample buffers since UDP packets have the possibility of being out of order.*

#### Launching, maintaining and destroying instances of process-level modules

If a snapshot has been restored during the current session, the system programmatically launches and configures the process-level instances if they're not already running and correctly configured. Exiting the coordinator in this state involves a graceful shutdown by destroying the subprocess instances spawned earlier before Microtron terminates itself.

## Goals

#### A free, professional-grade digital audio workstation for everyone.

Yes, this is an ambitious one. Our primary mission is to do something different. We want to make high-quality digital audio production tools that are available to everyone free of charge, combatting the ever-growing commercialization especially present in the field of digital audio workstations and widely used audio plugin interfaces.

We need to have solid open-source digital signal processing infrastructure in place to implement a DAW that can keep up with the products of current industry leaders in regard to synthesizer and effect quality. 

What we also need is a *really* solid graphical user interface; one that is good enough to utilize the intense productivity boost gained from a well-designed user experience design optimized to evoke the state of psychological flow in the mind of the creator.

## Features

#### Process-Based Module Graph
The foundation of Microtron's signal processing logic, a single audio node, is realized as the `Module` trait. It represents a piece of code which continuously synthesizes or processes frames worth of audio. Integrating a module into a `ModuleSocket` equips it with UDP networking capabilities, thus enabling it to join the process-level graph maintained by the coordinator.

, and can be put to two uses: either joining the process-level graph inside a `ModuleServer`, or being composed with others in the application-level `Chain` and `Graph` structures. 

#### Application-Level Dataflow Structures
There are pre-built modules implementing different module structures, allowing modules to be composed with others in chain and graph structures that are local to the application itself, rather than to the process-level graph. Additionally, since the structural containers implement `Module` themselves, the application-level structures can theoretically be nested infinitely deep, allowing complex application-level dataflow structures to be used for digital instruments and effects.

#### Extensible Design
The coordinator is structurally similar to a microkernel in that it only handles the most fundamental work required for safe and reliable communication between process-level modules and leaves higher-level tasks like audio rendering, device interfaces, song arrangements and graphical user interfaces to external implementors.

This makes it easy to replace a part of the DAW if it's incompatible with your current environment. Theoretically, if your code was running on a System-on-a-Chip where direct output to the speakers wasn't possible, you could just replace the stock renderer with another module doing exactly what you need. Forwarding the audio to GPIO pins or something. 

*(And yes, support for `no_std` will hopefully arrive in the future.)*

#### Save and Restore
Save a human-readable YAML snapshot of the current processes, their connections, the arrangement and other coordinator state so that it can be loaded at a later point in time.

## Getting Started
As long as you have a recent Rust toolchain installed and your PATH environment variable contains the folder where Cargo installs binaries, you should be good to go.

### Installation and Usage
Installing a copy of the coordinator onto your system can be done by running the following Cargo command in your favorite terminal. 

```
cargo install microtron
```

After the command has completed, you can verify the installation by entering `microtron` into the console. If everything went according to plan, you should be greeted by the coordinator setting up an empty project and waiting for modules to connect.

### Compile and run locally
Use the following commands to compile and run the program without installing it onto your system right away. This is probably the right way to go if you're just trying out the application or hacking on the codebase a bit.

```
cargo run                    # Compile development build
cargo run --release          # Compile build with optimizations (may take much longer)
```

## Project Status

As of November 2020, this project is still very much in the alpha phase. 

**Please keep in mind that this is an experiment and in no way production-ready. It is very likely that the whole Microtron codebase will experience intense, earth-shattering change during its journey to 1.0.**


## Contributing

Since I'm still in the process of slowly figuring out the exact combination of nuts, bolts and duct tape required to make this project work more or less seamlessly, I'll probably need quite some time to even partially wrap my head around all the interesting concepts and dark corners of DSP engineering. If you have an idea for a feature or a change, or if you'd like to report an error, please don't hesitate to create a pull request or open an issue.

## License

**Licensed under the MIT license.**

## Disclaimer

The material embodied in this software is provided to you "as-is" and without warranty of any kind, express, implied or otherwise, including without limitation, any warranty of fitness for a particular purpose. In no event shall the developers of the Microtron project be liable to you or anyone else for any direct, special, incidental, indirect or consequential damages of any kind, or any damages whatsoever, including without limitation, loss of profit, loss of use, savings or revenue, or the claims of third parties, whether or not the developers of the Microtron project have been advised of the possibility of such loss, however caused and on any theory of liability, arising out of or in connection with the possession, use or performance of this software.
