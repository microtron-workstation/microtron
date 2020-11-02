![banner](https://i.imgur.com/u3JtfYm.png)

## Introduction

`microtron` is a high-performance asynchronous packet relay that forms the foundation of inter-modular data transmission in the Microtron system.

## Features

#### Process-Based Module Graph

The relay server constantly maintains a graph of all connected modules and their connections. Keep in mind that this is just a graph-like data structure containing a few socket addresses and encoding how they are linked. We use this to figure out where the data received in our endless stream of packets should actually go.

Any piece of code which continuously synthesizes or processes frames worth of audio is considered a module. Integrating one into a `ModuleSocket` type equips it with UDP networking capabilities, enabling it to join the process-level graph maintained by the packet relay.

#### Low Latency, High Throughput

The simple design employed by the relay server combined with a highly performant asynchronous runtime provided by `tokio` enables it to deliver all messages as quickly as possible, even under the stress of a high load.

#### Save and Restore

Save a human-readable YAML snapshot of the current processes, their connections and other relay state so that it can be restored at a later point in time.

## Getting Started

As long as you have a recent Rust toolchain installed and your PATH environment variable contains the folder where Cargo installs binaries, you should be good to go.

### Installation and Usage

Install a copy of the coordinator onto your system by running the following Cargo command in your terminal emulator of choice:

```
cargo install microtron
```

After the command has completed, you can verify the installation by entering `microtron` into the console. If everything went according to plan, you should be greeted by the coordinator setting up an empty project and waiting for modules to connect.

### Local compilation and execution

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
