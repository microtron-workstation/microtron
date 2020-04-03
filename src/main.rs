extern crate microtron;

pub mod message;
pub mod types;

use std::error::Error;
use std::env;
use std::collections::HashMap;
use tokio;
use tokio::net::UdpSocket;
use colored::*;

use microtron::Server;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    info!(format!("Module server listening on {}", socket.local_addr()?));

    let peers = HashMap::new();

    let server = Server {
        socket,
        peers,
        counter: 0,
        buf: vec![0; 1024],
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
