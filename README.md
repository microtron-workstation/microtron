# Microtron

`microtron` is a modular digital audio workstation which ties together audio modules into a process-based graph that can be used to produce and perform music.

Any struct which implements the `Module` trait is considered an audio module. By encapsulating one within a `ModuleServer`, it can be turned into a networking node which communicates with the runtime and other modules via UDP. On the other hand, modules can be composed within a single application by using the `Graph` and `Chain` types.

## Getting Started

Make sure you have a Rust toolchain installed, then run the following command to compile and run the application:

```
cargo run --release
```

This will bind the packet forwarder to  `udp://localhost:32000`, then you should be able to connect modules by sending a `Handshake` packet.

## Features

#### `Process-based module graph`
The Microtron runtime maintains a directed acyclic graph of processes and forwards messages like audio buffers, MIDI events or parameter changes to their destinations via UDP.

#### `Save and restore projects`
The current configuration of the runtime, meaning the graph nodes, edges and other audio-related data such as sample rate and format can be saved to a file for later use.
