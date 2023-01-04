use std::vec::Vec;

#[derive(Debug)]
pub struct CPU {
        
}

impl CPU {
    pub fn load_elf(&self, bin: &Vec<u8>) -> &Self {
        self
    } 

    pub fn run(&self) -> &RegFile {
        
    }
}

pub fn new() -> CPU {
    CPU{}
}
