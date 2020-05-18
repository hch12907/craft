use super::PacketData;
use tokio::io::{ AsyncReadExt, Error as IoError, ErrorKind, Result as IoResult };

async fn read_utf8<I>(input: &mut I) -> IoResult<Box<str>>
    where I: AsyncReadExt + Unpin
{
    let len = input.read_i16().await?;
    let mut bytes = Vec::with_capacity(len as usize);
    
    input.take(len as u64).read_to_end(&mut bytes).await?;
    
    let result = String::from_utf8(bytes)
        .map_err(|e| IoError::new(ErrorKind::InvalidData, e))?
        .into_boxed_str();
    
    Ok(result)
}

async fn read_ucs2<I>(input: &mut I) -> IoResult<Box<str>>
    where I: AsyncReadExt + Unpin
{
    let len = input.read_i16().await?;
    let mut code_units = Vec::with_capacity(len as usize);
    
    for _ in 0..len {
        code_units.push(input.read_u16().await?);
    }

    let result = String::from_utf16(code_units.as_ref())
        .map_err(|e| IoError::new(ErrorKind::InvalidData, e))?
        .into_boxed_str();
    
    Ok(result)
}

impl PacketData {
    pub(super) async fn read_keep_alive<I>(_input: &mut I) -> IoResult<Self> 
        where I: AsyncReadExt
    {
        Ok(PacketData::KeepAlive)
    }

    pub(super) async fn read_login_request<I>(input: &mut I) -> IoResult<Self> 
        where I: AsyncReadExt + Unpin
    {
        let id = input.read_i32().await?;
        let username = read_ucs2(input).await?;
        let seed = 0; input.read_u64().await?;
        let dimension = 0; input.read_u8().await?;
        
        Ok(PacketData::LoginRequest {
            id,
            username,
            seed,
            dimension
        })
    }

    pub(super) async fn read_handshake<I>(input: &mut I) -> IoResult<Self> 
        where I: AsyncReadExt + Unpin + tokio::io::AsyncWriteExt
    {
        let username_or_hash = read_ucs2(input).await?;
        input.write_i8(2).await?;
        input.write_i16(1).await?;
        input.write_u16(45).await?;
        Ok(PacketData::Handshake {
            username_or_hash
        })
    }

    pub(super) async fn read_generic<I>(input: &mut I) -> IoResult<Self> 
        where I: AsyncReadExt + Unpin
    {
        Ok(PacketData::DisconnectOrKick {
            reason: "Feature unimplemented".into()
        })
    }
}
