use gekraftet_core::world::Chunk;
use gekraftet_core::utils::PartialHeapArray;
use std::path::Path;
use std::fs::File;
use std::io::{ BufReader, Read, Error as IoError };

const REGION_SECTOR_SIZE: usize = 4096;
const REGION_ENTRY_COUNT: usize = 1024;

#[derive(Clone, Debug, PartialEq)]
struct ChunkLocation {
    sector_count: u8,
    sector_offset: u32,
}

#[derive(Clone, Debug)]
struct ChunkHeader {
    length: u32,
    compression_type: u8,
}

pub struct RegionHeader {
    locations: Box<[ChunkLocation; REGION_ENTRY_COUNT]>,
    timestamps: Box<[u32; REGION_ENTRY_COUNT]>,
}

pub struct Region {
    header: RegionHeader,
    data: Vec<(ChunkHeader, Box<[u8]>)>,
}

impl Region {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        let mut locations = PartialHeapArray::<_, REGION_ENTRY_COUNT>::new();
        let mut timestamps = PartialHeapArray::<_, REGION_ENTRY_COUNT>::new();
        let mut data_size = 0; // used later when allocating for `data`
        
        for _ in 0..REGION_ENTRY_COUNT {
            let mut bytes = [0u8; 4];
            reader.read_exact(&mut bytes)?;
            let location = ChunkLocation {
                sector_count: bytes[4],
                sector_offset: u32::from_be_bytes([
                    0, bytes[0], bytes[1], bytes[2]
                ]),
            };

            if location.sector_count > 0 {
                data_size += 1
            };

            locations.push(location).unwrap();
        }

        for _ in 0..REGION_ENTRY_COUNT {
            let mut bytes = [0u8; 4];
            reader.read_exact(&mut bytes)?;
            timestamps.push(u32::from_be_bytes(bytes)).unwrap();
        }

        let mut data = Vec::with_capacity(data_size);
        for loc in locations.iter() {
            // The first chunk sector starts at offset 2. (0 and 1 are occupied
            // by the region header).
            if loc.sector_count > 0 && loc.sector_offset >= 2 {
                reader.seek_relative((loc.sector_offset - 2) as i64 * REGION_SECTOR_SIZE as i64)?;
                let mut header_bytes = [0u8; 5];
                reader.read_exact(&mut header_bytes)?;

                let header = ChunkHeader {
                    length: u32::from_be_bytes([
                        header_bytes[0], header_bytes[1], 
                        header_bytes[2], header_bytes[3]
                    ]),

                    compression_type: header_bytes[4],
                };
                
                // We have read the first 5 bytes.
                let bytes_to_read = loc.sector_count as usize * REGION_SECTOR_SIZE - 5;
                let mut chunk_bytes = Vec::with_capacity(bytes_to_read);
                unsafe { chunk_bytes.set_len(bytes_to_read) };
                reader.read_exact(chunk_bytes.as_mut_slice())?;

                data.push((header, chunk_bytes.into_boxed_slice()));
            }
        }

        let region_header = RegionHeader {
            locations: locations.into_full_array().unwrap(),
            timestamps: timestamps.into_full_array().unwrap(),
        };

        Ok(Self {
            header: region_header,
            data,
        })
    }
}
