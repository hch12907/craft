use crate::maths::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub metadata: u16,
    pub id: u16,
}

impl Block {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            metadata: 0,
        }
    }
}
