use tokio::io::{AsyncReadExt, AsyncWriteExt, Error as IoError, ErrorKind, Result as IoResult};

pub async fn read_utf8<I>(input: &mut I) -> IoResult<Box<str>>
where
    I: AsyncReadExt + Unpin,
{
    let len = input.read_i16().await?;
    let mut bytes = Vec::with_capacity(len as usize);

    input.take(len as u64).read_to_end(&mut bytes).await?;

    let result = String::from_utf8(bytes)
        .map_err(|e| IoError::new(ErrorKind::InvalidData, e))?
        .into_boxed_str();

    Ok(result)
}

pub async fn read_ucs2<I>(input: &mut I) -> IoResult<Box<str>>
where
    I: AsyncReadExt + Unpin,
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

pub async fn write_utf8<I, R>(input: &mut I, val: R) -> IoResult<()>
where
    I: AsyncWriteExt + Unpin,
    R: AsRef<str>
{
    let bytes = val.as_ref().as_bytes();
    input.write_i16(bytes.len() as i16).await?;
    input.write_all(bytes).await?;
    Ok(())
}

pub async fn write_ucs2<I, R>(input: &mut I, val: R) -> IoResult<()>
where
    I: AsyncWriteExt + Unpin,
    R: AsRef<str>
{
    let code_point = val.as_ref().encode_utf16().collect::<Vec<_>>();
    input.write_i16(code_point.len() as i16).await?;
    for x in code_point.iter() {
        input.write_u16(*x).await?;
    }
    Ok(())
}
