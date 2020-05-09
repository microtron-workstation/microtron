# Microtron

`microtron` is a modular digital audio workstation which ties together audio modules into a process-based graph that can be used to produce and perform music.

At the heart of the system is the `Module` trait, which allows modules to be launched stand-alone as a server, or be composed with others using the `ModuleChain` and `ModuleGraph` structures.

## Getting Started

Make sure you have a Rust toolchain installed, then run the following command to compile and run the application:

```
cargo run --release
```

This will bind the packet forwarder to  `udp://localhost:32000`, then you should be able to connect modules by sending a `Handshake` packet.
