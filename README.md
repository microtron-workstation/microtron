![banner](https://i.imgur.com/u3JtfYm.png)

## Introduction

`microtron` is an open-source audio engine that is intended to be used as a modular signal processing backend for the Microtron workstation.

Constantly maintaining a graph-like data structure that contains socket addresses and their relations enables us to figure out to which socket the messages received in our endless stream of packets should actually be sent to.

## Features

#### Process-Based Module Graph

Any piece of code that continuously synthesizes or processes frames worth of audio is considered a module. Integrating one into a `ModuleSocket` type equips it with UDP networking capabilities, enabling it to join the process-level graph maintained by the packet relay.

#### Save- and Restorable Snapshots

Save a human-readable YAML snapshot of the current processes, their connections and other relay state so that it can be restored at a later point in time.

## Goals

#### A free, professional-grade digital audio workstation for everyone.

Yes, this is an ambitious goal. Our mission is to do something new. We want to make high-quality digital audio production tools available to everyone free of charge, battling the ever-growing commercialization especially present in the field of digital audio workstations and widely used audio plugin interfaces.

We need to have solid open-source digital signal processing infrastructure in place to implement a DAW capable of keeping up with the current industry standard especially since many commercial digital audio workstations have been in development for a long time.

What we also need is a *really* solid graphical user interface; one that is good enough to utilize the productivity boost gained from well-made user interface and experience design. Optimally, we would like to evoke the state of psychological flow in the mind of the creator.

#### Stability and Speed

Being protected from a large number of hard-to-debug memory bugs in combination with the avoidance of `panic!()` calls whenever possible gives us a stable foundation on which we can build software that doesn't just crash out of nowhere, which is especially desirable if you're currently delivering a live performance to your fans.

Furthermore, we will explore if different types of optimizations like SIMD and GPU parallelization are useful for audio synthesis and processing, especially to complex modules like granular synthesizers and real-time harmonizers.

## Project Status

As of November 2020, this project is still very much in the alpha phase. 

**Please keep in mind that this is an experiment and in no way production-ready. It is very likely that the whole Microtron codebase will experience intense, earth-shattering changes during its journey to 1.0**

## Contributing

Since I'm still in the process of slowly figuring out the exact combination of nuts, bolts and duct tape required to make this project work, I'll probably need some time to wrap my head around all the interesting concepts and dark corners of digital signal processing I can find, about filter design, granular synthesis and *so* many other things. If you have an idea for a feature or a change, or you'd like to report an error, please don't hesitate to open an issue, create a pull request or send me a message.

## License

**Licensed under the MIT license.**

## Disclaimer

The material embodied in this software is provided to you "as-is" and without warranty of any kind, express, implied or otherwise, including without limitation, any warranty of fitness for a particular purpose. In no event shall the developers of the Microtron project be liable to you or anyone else for any direct, special, incidental, indirect or consequential damages of any kind, or any damages whatsoever, including without limitation, loss of profit, loss of use, savings or revenue, or the claims of third parties, whether or not the developers of the Microtron project have been advised of the possibility of such loss, however caused and on any theory of liability, arising out of or in connection with the possession, use or performance of this software.
