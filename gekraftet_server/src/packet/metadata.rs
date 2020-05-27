use super::utils::*;
use std::collections::BTreeMap;
use tokio::io::{
    AsyncReadExt, 
    AsyncWriteExt,
    Error as IoError, 
    ErrorKind as IoErrorKind, 
    Result as IoResult
};

#[derive(Clone, Debug, PartialEq)]
pub enum MetadataNode {
    Byte(i8),
    Short(i16),
    Int(i32),
    Float(f32),
    Str(Box<str>),
    ItemStack(i16, i8, i16), // id, count, damage
    Entity(i32, i32, i32),
}

/// Metadata type used in some packets sent by the Minecraft client. More
/// information on this can be found on the [wiki](https://wiki.vg/Protocol&oldid=510#Metadata).
#[derive(Clone, Debug, PartialEq)]
pub struct Metadata {
    data: BTreeMap<i8, MetadataNode>,
}

impl Metadata {
    pub async fn read_from<I>(input: &mut I) -> IoResult<Self>
    where
        I: Unpin + AsyncReadExt,
    {
        let mut result = BTreeMap::new();

        let mut data_info = input.read_i8().await?;
        while data_info != 127 {
            let data_ty = data_info >> 5;
            let data_id = data_info & 0x1F;

            let value = match data_ty {
                0 => MetadataNode::Byte(input.read_i8().await?),
                1 => MetadataNode::Short(input.read_i16().await?),
                2 => MetadataNode::Int(input.read_i32().await?),
                3 => MetadataNode::Float(f32::from_bits(input.read_u32().await?)),
                4 => MetadataNode::Str(read_ucs2(input).await?),
                5 => {
                    let id = input.read_i16().await?;
                    let count = input.read_i8().await?;
                    let damage = input.read_i16().await?;
                    MetadataNode::ItemStack(id, count, damage)
                },
                6 => {
                    let info = [
                        input.read_i32().await?,
                        input.read_i32().await?,
                        input.read_i32().await?,
                    ];
                    MetadataNode::Entity(info[0], info[1], info[2])
                },
                x => Err(IoError::new(
                    IoErrorKind::InvalidData,
                    format!("invalid metadata type {}", x)
                ))?,
            };

            match result.insert(data_id, value) {
                Some(_) => Err(IoError::new(
                    IoErrorKind::InvalidData,
                    format!("invalid metadata index {}", data_id)
                ))?,
                _ => {},
            }

            data_info = input.read_i8().await?;
        }

        Ok(Self { data: result })
    }

    pub async fn write_to<I>(&self, input: &mut I) -> IoResult<()> 
    where
        I: AsyncWriteExt + Unpin
    {
        for (&key, value) in &self.data {
            match &value {
                MetadataNode::Byte(b) => {
                    input.write_i8((0 << 5) | key).await?;
                    input.write_i8(*b).await?;
                }

                MetadataNode::Short(s) => {
                    input.write_i8((1 << 5) | key).await?;
                    input.write_i16(*s).await?;
                }

                MetadataNode::Int(i) => {
                    input.write_i8((2 << 5) | key).await?;
                    input.write_i32(*i).await?;
                }

                MetadataNode::Float(f) => {
                    input.write_i8((3 << 5) | key).await?;
                    input.write_u32(f.to_bits()).await?;
                }

                MetadataNode::Str(s) => {
                    input.write_i8((4 << 5) | key).await?;
                    write_ucs2(input, s).await?;
                }

                MetadataNode::ItemStack(id, count, damage) => {
                    input.write_i8((5 << 5) | key).await?;
                    input.write_i16(*id).await?;
                    input.write_i8(*count).await?;
                    input.write_i16(*damage).await?;
                }

                MetadataNode::Entity(i0, i1, i2) => {
                    input.write_i8((5 << 5) | key).await?;
                    input.write_i32(*i0).await?;
                    input.write_i32(*i1).await?;
                    input.write_i32(*i2).await?;
                }
            }
        }
        
        input.write_i8(127).await?;
        Ok(())
    }
}
