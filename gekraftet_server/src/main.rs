#![feature(bufreader_seek_relative)]

mod config;
mod packet;
mod world;

use config::Config;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = Config::try_read()?;
    let mut listener = TcpListener::bind((conf.ip, conf.port)).await?;
    
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("received connection: {}", addr);

        tokio::spawn(async move {
            let mut buffer = BufReader::new(stream);
            loop {
                packet::Packet::read_packet(&mut buffer).await.unwrap();
            }
        });
    }
}
