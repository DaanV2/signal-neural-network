use std::u8;


const MAX: usize = std::u8::MAX as usize;

pub struct Node {
    tf: [u8; MAX],
}

impl Node {
    pub const fn new() -> Node {
        Node {
            tf: [0 as u8; MAX]
        }
    }

    pub const fn tranform(&self, input: u8) -> u8 {
        return self.tf[input as usize]
    }
}