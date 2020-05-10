#![feature(bufreader_seek_relative)]

mod packet;
mod world;

#[tokio::main]
async fn main() {
    println!("Hello, gekraftet-server!");
}
