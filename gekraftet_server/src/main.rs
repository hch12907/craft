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
                let mut mesg = Vec::new();
                match buffer.read_until(b'$', &mut mesg).await {
                    Ok(0) => return,
                    Ok(x) => println!("bytes received: {}", x),
                    Err(why) => println!("error while reading socket: {}", why),
                };
                mesg.pop();
                println!("message received: {}", String::from_utf8(mesg).unwrap())
            }
        });
    }
}
