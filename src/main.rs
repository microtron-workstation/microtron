extern crate microtron;

pub mod message;
pub mod types;

use std::error::Error;
use std::env;
use std::collections::HashMap;
use tokio;
use tokio::net::UdpSocket;
use colored::*;

use microtron::Runtime;

fn print_ok(message: String, description: Option<String>) {
    println!("{}   {}\n", " OK ".green().bold().reversed(), message.bold());

    if let Some(text) = description {
        let lines = text.split("\n");

        for line in lines {
            println!("       {}", line);
        }

        println!("");
    }
}

fn print_err(message: String, description: Option<String>) {
    println!("{}  {}\n", " ERR ".red().bold().reversed(), message.bold());

    if let Some(text) = description {
        let lines = text.split("\n");

        for line in lines {
            println!("       {}", line);
        }

        println!("");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:32000".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    self::print_ok(
        format!("Packet forwarder listening on udp://{}", socket.local_addr()?),
        Some(format!("You can now go ahead and connect modules.")),
    );

    let peers = HashMap::new();

    let server = Runtime {
        socket,
        peers,
        counter: 0,
        buf: vec![0; 1024],
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
