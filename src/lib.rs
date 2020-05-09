extern crate serde;

pub mod message;
pub mod types;

use std::net::SocketAddr;
use std::io;
use std::collections::HashMap;
use tokio;
use tokio::net::UdpSocket;

use self::message::{Packet, Message};
use self::types::{ID};

pub struct Runtime {
    pub socket:  UdpSocket,
    pub peers:   HashMap<ID, SocketAddr>,
    pub counter: ID,
    pub buf:     Vec<u8>,
}

impl Runtime {
    pub async fn run(mut self) -> Result<(), io::Error> {
        loop {
            // Wait to receive a datagram on the UDP socket.
            let (bytes_read, peer) = self.socket.recv_from(&mut self.buf).await?;

            // Try to deserialize the bytes into a Packet.
            let packet: Packet = bincode::deserialize(&self.buf[.. bytes_read]).unwrap();

            match packet.message {
                Message::Connect => {
                    self.peers.insert(self.counter, peer);
                    self.counter += 1;
                    self.send_packet(
                        Packet {
                            from: 0,
                            to:   self.counter,
                            message: Message::ConnectionEstablished,
                        },
                        peer,
                    ).await?;
                },

                Message::Buffer { .. } => {

                }

                Message::ConnectionEstablished => {},
                _ => {},
            }
        }
    }

    async fn send_packet(&mut self, packet: Packet, peer: SocketAddr) -> Result<usize, io::Error> {
        self.socket.send_to(&bincode::serialize(&packet).unwrap(), &peer).await
    }
}

macro_rules! info {
    ($msg:expr) => {
        println!("{}   {}", "[info]".cyan().bold(), $msg);
    };
}

macro_rules! error {
    ($msg:expr) => {
        println!("{}  {}", "[error]".bright_red().bold(), $msg.bold());
    };
}
