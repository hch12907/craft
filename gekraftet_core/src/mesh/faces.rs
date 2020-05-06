
#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    enabled: u8,
}

impl Face {
    pub const TOP:    u8 = 1;
    pub const BOTTOM: u8 = 2;
    pub const LEFT:   u8 = 4;
    pub const RIGHT:  u8 = 8;
    pub const FRONT:  u8 = 16;
    pub const BACK:   u8 = 32;

    pub fn from_bitfield(enabled: u8) -> Self {
        Self { enabled }
    }

    pub fn into_bitfield(self) -> u8 {
        self.enabled
    }

    pub fn all() -> Self {
        Self {
            enabled: 0b0011_1111
        }
    }

    pub fn empty() -> Self {
        Self {
            enabled: 0
        }
    }

    pub fn with_faces(bitfield: u8) -> Self {
        debug_assert!(bitfield <= 0b11_1111);

        Self {
            enabled: bitfield
        }
    }

    pub fn enable(&mut self, bitfield: u8) {
        debug_assert!(bitfield <= 0b11_1111);
        self.enabled |= bitfield;
    }

    pub fn disable(&mut self, bitfield: u8) {
        debug_assert!(bitfield <= 0b11_1111);
        self.enabled &= !bitfield;
    }

    pub fn intersects(&self, bitfield: u8) -> bool {
        (self.enabled & bitfield) != 0
    }
}
