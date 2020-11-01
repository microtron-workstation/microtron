![banner](https://i.imgur.com/NyLmOfr.png)

## Introduction

This repository is part of the Microtron ecosystem and contains a piece of software called the **coordinator**. It provides a central communication hub for all process-level modules which can also spawn subprocesses when needed. The *process-level* descriptor here refers to modules that are active on the UDP network, whereas the natural habitat of *application-level* modules lies within a process-level module. Grouped and entangled in chain and graph structures, they can form the engine of a synthesizer, a vocoder plugin or just about anything audio-related you could think of.

The project compiles to a binary called `microtron` which takes care of two basic tasks:

#### Relaying packets from one process-level module to another

Depending on their type, all arriving packets are either handled directly by the coordinator, or they are processed to determine their destination according to the internal representation of the process graph, after which they are quickly sent.

*We should probably use a timecode of some sort here to ensure the right order of sample buffers, since UDP does not guarantee the correct ordering of arriving packets. For our case, maybe even a simple solution like counting to 1024, then wrapping back around and attaching this number to every packet would allow us to make sure that all packets are processed in the order they were sent.*

#### Launching, maintaining and destroying instances of process-level modules

When a project file is loaded, the system launches all the process-level modules defined in the project, configures them to be in the correct state and builds a process graph. It's possible for a portion of the graph to be dead if something went wrong, such as a missing process or one that threw an error code. 

As soon as we're warmed up, things become smoother again. "Maintaining" of a bunch of process-level modules more or less just means using them, and because Rust helps a lot with memory safety, we don't really need to worry about them crashing.

Sending an exit packet to a coordinator which is currently giving birth will momentarily suspend the shutdown, try to gracefully shut down all subprocesses and then resume as soon as there are none left.

## Goals

#### A free, professional-grade digital audio workstation for everyone.

Yes, this is an ambitious goal. Our mission is to do something new. We want to make high-quality digital audio production tools available to everyone free of charge, battling the ever-growing commercialization especially present in the field of digital audio workstations and widely used audio plugin interfaces.

We need to have solid open-source digital signal processing infrastructure in place to implement a DAW capable of keeping up with the current industry standard especially since many commercial digital audio workstations have been around for a long time.

What we also need is a *really* solid graphical user interface; one that is good enough to utilize the intense productivity boost gained from a UI/UX design optimized to evoke the state of psychological flow in the mind of the creator.

## Features

#### Process-Based Module Graph

The foundation of Microtron's signal processing logic, a single audio node, is realized as the `Module` trait. It represents a piece of code which continuously synthesizes or processes frames worth of audio. Integrating a module into a `ModuleSocket` equips it with UDP networking capabilities, thus enabling it to join the process-level graph maintained by the coordinator.

#### Application-Level Dataflow Structures

There are pre-built modules implementing different module structures, allowing modules to be composed with others in chain and graph structures that are local to the application itself, rather than to the process-level graph. Additionally, since the structural containers implement the `Module` trait themselves, the application-level structures can theoretically be nested infinitely deep, allowing complex application-level dataflow structures to be used for digital instruments and effects.

#### Extensible Design

The coordinator is structurally similar to a microkernel as it only handles the most fundamental work required for safe and reliable communication between process-level modules and leaves the implementation of higher-level tasks like audio rendering, device interfaces, song arrangements and graphical user interfaces to external modules.

This makes it easy to replace a part of the DAW if it's incompatible with your current environment. Theoretically, if your code was running on a System-on-a-Chip where direct output to the speakers wasn't possible, you could just replace the stock renderer with another module doing exactly what you need. Forwarding the audio to GPIO pins or something. 

*(And yes, support for `no_std` will hopefully arrive sometime in the future.)*

#### Save and Restore

Save a human-readable YAML snapshot of the current processes, their connections, the arrangement and other coordinator state so that it can be loaded at a later point in time.

## Getting Started

As long as you have a recent Rust toolchain installed and your PATH environment variable contains the folder where Cargo installs binaries, you should be good to go.

### Installation and Usage

Install a copy of the coordinator onto your system by running the following Cargo command in your terminal emulator of choice:

```
cargo install microtron
```

After the command has completed, you can verify the installation by entering `microtron` into the console. If everything went according to plan, you should be greeted by the coordinator setting up an empty project and waiting for modules to connect.

### Compile and run locally

Alternatively, you can use the following commands to compile and run the program without installing it onto your system right away. This is probably the right way to go if you just want to have a look at the application or hack on the codebase a bit.

```
cargo run                    # Compile development build
cargo run --release          # Compile build with optimizations (may take much longer)
```

## Project Status

As of November 2020, this project is still *very* much in the alpha phase. 

**Please keep in mind that this is an experiment and in no way production-ready. It is very likely that the whole Microtron codebase will experience intense, earth-shattering changes during their journey to 1.0**


## Contributing

Since I'm still in the process of slowly figuring out the exact combination of nuts, bolts and duct tape required to make this project work more or less seamlessly, I'll probably need quite some time to even partially wrap my head around all the interesting concepts and dark corners of digital signal processing I can find, about filter design, granular synthesis and *so many* other things. If you have an idea for a feature or a change, or you'd like to report an error, please don't hesitate to open an issue, creating a pull request or sending me a message.

## License

**Licensed under the MIT license.**

## Disclaimer

The material embodied in this software is provided to you "as-is" and without warranty of any kind, express, implied or otherwise, including without limitation, any warranty of fitness for a particular purpose. In no event shall the developers of the Microtron project be liable to you or anyone else for any direct, special, incidental, indirect or consequential damages of any kind, or any damages whatsoever, including without limitation, loss of profit, loss of use, savings or revenue, or the claims of third parties, whether or not the developers of the Microtron project have been advised of the possibility of such loss, however caused and on any theory of liability, arising out of or in connection with the possession, use or performance of this software.
