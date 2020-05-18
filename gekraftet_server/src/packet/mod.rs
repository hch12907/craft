mod data;
mod id;
mod receiver;

pub use data::*;
pub use id::*;

use tokio::io::{ AsyncReadExt, Result as IoResult };

pub struct Packet {
    data: PacketData,
}

impl Packet {
    pub async fn read_packet<I>(input: &mut I) -> IoResult<()> 
        where I: AsyncReadExt + Unpin + tokio::io::AsyncWriteExt
    {
        let packet_id = PacketId::from_packet_id(input.read_u8().await?);
        let packet = match packet_id {
            Some(PacketId::KeepAlive) => PacketData::read_keep_alive(input).await?,
            Some(PacketId::LoginRequest) => PacketData::read_login_request(input).await?,
            Some(PacketId::Handshake) => PacketData::read_handshake(input).await?,

            _ => PacketData::read_generic(input).await?,
        };

        println!("{:?}", packet);

        Ok(())
    }
}
