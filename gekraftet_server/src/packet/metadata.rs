use super::utils::read_ucs2;
use std::collections::BTreeMap;
use tokio::io::{
    AsyncReadExt, 
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
}
