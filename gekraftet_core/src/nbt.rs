use std::collections::HashMap;
use std::convert::TryFrom;
use std::string::FromUtf8Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DepthOver512,
    InvalidList,
    InvalidTag(u8),
    InvalidUtf8(FromUtf8Error),
    TagsOfSameName,
    TagUnnamed,
    UnexpectedEof,
}

#[derive(Debug)]
pub struct NamedBinaryTag {
    root: HashMap<String, ValueData>,
}

#[derive(Debug)]
pub enum ValueData {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    Str(String),
    List(u8, Vec<ValueData>),
    Compound(HashMap<String, ValueData>),
}

impl NamedBinaryTag {
    pub fn new() -> Self {
        Self { 
            root: HashMap::new() 
        }
    }

    pub fn from_binary(value: &[u8]) -> Result<Self> {
        let mut root = Self::new();
        root.root = Self::parse_compound(value, 0)?.1
            .into_compound()
            .unwrap();

        Ok(root)
    }

    fn parse_byte(bytes: &[u8]) -> Result<(usize, ValueData)> {
        let result = bytes.get(0).ok_or(Error::UnexpectedEof)?;
        Ok((1, ValueData::Byte(*result as i8)))
    }

    fn parse_short(bytes: &[u8]) -> Result<(usize, ValueData)> {
        let result = bytes.get(0..2).ok_or(Error::UnexpectedEof)?;
        let short = i16::from_be_bytes([result[0], result[1]]);
        Ok((2, ValueData::Short(short)))
    }

    fn parse_int(bytes: &[u8]) -> Result<(usize, ValueData)> {
        let result = bytes.get(0..4).ok_or(Error::UnexpectedEof)?;
        let int = i32::from_be_bytes(<[u8; 4]>::try_from(result).unwrap());
        Ok((2, ValueData::Int(int)))
    }

    fn parse_long(bytes: &[u8]) -> Result<(usize, ValueData)> {
        let result = bytes.get(0..8).ok_or(Error::UnexpectedEof)?;
        let long = i64::from_be_bytes(<[u8; 8]>::try_from(result).unwrap());
        Ok((2, ValueData::Long(long)))
    }

    fn parse_float(bytes: &[u8]) -> Result<(usize, ValueData)> {
        let result = bytes.get(0..4).ok_or(Error::UnexpectedEof)?;
        let float = f32::from_be_bytes(<[u8; 4]>::try_from(result).unwrap());
        Ok((2, ValueData::Float(float)))
    }
    
    fn parse_double(bytes: &[u8]) -> Result<(usize, ValueData)> {
        let result = bytes.get(0..8).ok_or(Error::UnexpectedEof)?;
        let double = f64::from_be_bytes(<[u8; 8]>::try_from(result).unwrap());
        Ok((2, ValueData::Double(double)))
    }

    fn parse_string(bytes: &[u8]) -> Result<(usize, ValueData)> {
        let len = bytes.get(0..2).ok_or(Error::UnexpectedEof)?;
        let len = u16::from_be_bytes([len[0], len[1]]) as usize;
        
        let str_bytes = bytes.get(3..3 + len).ok_or(Error::UnexpectedEof)?;
        let result = String::from_utf8(Vec::from(str_bytes))
            .map(|s| ValueData::Str(s))
            .map_err(|err| Error::InvalidUtf8(err))?;
        
        Ok((len, result))
    }

    fn parse_byte_array(bytes: &[u8]) -> Result<(usize, ValueData)> {
        let len = bytes.get(0..4).ok_or(Error::UnexpectedEof)?;
        let len = u32::from_be_bytes(<[u8; 4]>::try_from(len).unwrap()) as usize;
        
        let bytes = bytes.get(3..3 + len).ok_or(Error::UnexpectedEof)?;
        let result = unsafe {
            let bytes = bytes.as_ptr() as *const i8;
            let i8_bytes = std::slice::from_raw_parts(bytes, len);
            ValueData::ByteArray(Vec::from(i8_bytes))
        };

        
        Ok((len, result))
    }

    fn parse_list(bytes: &[u8], depth: u16) -> Result<(usize, ValueData)> {
        let tag_id = *bytes.get(0).ok_or(Error::UnexpectedEof)?;
        let len = bytes.get(1..5).ok_or(Error::UnexpectedEof)?;
        let len = u32::from_be_bytes(<[u8; 4]>::try_from(len).unwrap()) as usize;
        
        let mut offset = 0;
        let mut values = Vec::with_capacity(len);
        for i in 0..len {
            let (len, data) = match tag_id {
                0 => Err(Error::InvalidList)?,
                
                1 => Self::parse_byte(&bytes[5 + i..])?,
                2 => Self::parse_short(&bytes[5 + i * 2..])?,
                3 => Self::parse_int(&bytes[5 + i * 4..])?,
                4 => Self::parse_long(&bytes[5 + i * 8..])?,
                5 => Self::parse_float(&bytes[5 + i * 4..])?,
                6 => Self::parse_double(&bytes[5 + i * 8..])?,
                7 => Self::parse_byte_array(&bytes[5 + offset..])?,
                8 => Self::parse_string(&bytes[5 + offset..])?,
                9 => Self::parse_list(&bytes[5 + offset..], depth + 1)?,
                10 => Self::parse_compound(&bytes[5 + offset..], depth + 1)?,

                x => Err(Error::InvalidTag(x))?,
            };

            offset += len;
            values.push(data);
        };

        Ok((offset + 5, ValueData::List(tag_id, values)))
    }

    fn parse_compound(bytes: &[u8], depth: u16) -> Result<(usize, ValueData)> {
        let mut offset = 0;
        let mut result = HashMap::new();

        if depth > 512 {
            Err(Error::DepthOver512)?
        }

        while let Some(current_byte) = bytes.get(offset) {
            offset += 1;

            let (len, key, value) = match current_byte {
                0 => break,
                
                x => {
                    let (len_name, name) = Self::parse_string(&bytes[offset..])?;
                    let name = name.into_string().unwrap();
                    offset += len_name;

                    let (len, data) = match *x {
                        1 => Self::parse_byte(&bytes[offset..])?,
                        2 => Self::parse_short(&bytes[offset..])?,
                        3 => Self::parse_int(&bytes[offset..])?,
                        4 => Self::parse_long(&bytes[offset..])?,
                        5 => Self::parse_float(&bytes[offset..])?,
                        6 => Self::parse_double(&bytes[offset..])?,
                        7 => Self::parse_byte_array(&bytes[offset..])?,
                        8 => Self::parse_string(&bytes[offset..])?,
                        9 => Self::parse_list(&bytes[offset..], depth + 1)?,
                        10 => Self::parse_compound(&bytes[offset..], depth + 1)?,

                        invalid => Err(Error::InvalidTag(invalid))?,
                    };

                    (len, name, data)
                },
            };
            
            offset += len;
            
            match result.insert(key, value) {
                Some(x) => Err(Error::TagsOfSameName)?,
                None => {},
            };
        };

        Ok((offset, ValueData::Compound(result)))
    }
}

macro_rules! impl_getter {
    ($func:ident, $variant:path, $ret:ty) => {
        pub fn $func(&self) -> Option<&$ret> {
            match self {
                $variant(x) => Some(x),
                _ => None,
            }
        }
    };

    (mut $func:ident, $variant:path, $ret:ty) => {
        pub fn $func(&mut self) -> Option<&mut $ret> {
            match self {
                $variant(x) => Some(x),
                _ => None,
            }
        }
    };

    (into $func:ident, $variant:path, $ret:ty) => {
        pub fn $func(self) -> Option<$ret> {
            match self {
                $variant(x) => Some(x),
                _ => None,
            }
        }
    };
}

impl ValueData {
    impl_getter!(byte, Self::Byte, i8);
    impl_getter!(short, Self::Short, i16);
    impl_getter!(int, Self::Int, i32);
    impl_getter!(long, Self::Long, i64);
    impl_getter!(float, Self::Float, f32);
    impl_getter!(double, Self::Double, f64);
    impl_getter!(byte_array, Self::ByteArray, [i8]);
    impl_getter!(string, Self::Str, str);
    impl_getter!(compound, Self::Compound, HashMap<String, Self>);

    impl_getter!(mut byte_mut, Self::Byte, i8);
    impl_getter!(mut short_mut, Self::Short, i16);
    impl_getter!(mut int_mut, Self::Int, i32);
    impl_getter!(mut long_mut, Self::Long, i64);
    impl_getter!(mut float_mut, Self::Float, f32);
    impl_getter!(mut double_mut, Self::Double, f64);
    impl_getter!(mut byte_array_mut, Self::ByteArray, [i8]);
    impl_getter!(mut string_mut, Self::Str, str);
    impl_getter!(mut compound_mut, Self::Compound, HashMap<String, Self>);
    
    impl_getter!(into into_byte, Self::Byte, i8);
    impl_getter!(into into_short, Self::Short, i16);
    impl_getter!(into into_int, Self::Int, i32);
    impl_getter!(into into_long, Self::Long, i64);
    impl_getter!(into into_float, Self::Float, f32);
    impl_getter!(into into_double, Self::Double, f64);
    impl_getter!(into into_byte_array, Self::ByteArray, Vec<i8>);
    impl_getter!(into into_string, Self::Str, String);
    impl_getter!(into into_compound, Self::Compound, HashMap<String, Self>);

    pub fn list(&self) -> Option<(u8, &Vec<Self>)> {
        match self {
            Self::List(tag, data) => Some((*tag, data)),
            _ => None,
        }
    }

    pub fn list_mut(&mut self) -> Option<(u8, &mut Vec<Self>)> {
        match self {
            Self::List(tag, data) => Some((*tag, data)),
            _ => None,
        }
    }

    pub fn into_list(self) -> Option<(u8, Vec<Self>)> {
        match self {
            Self::List(tag, data) => Some((tag, data)),
            _ => None,
        }
    }
}
