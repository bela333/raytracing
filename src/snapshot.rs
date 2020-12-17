use std::{fs::File, io::Read};
use std::convert::TryInto;
#[derive(Clone)]
pub struct Snapshot{
    pub blocks: Vec<(i32, i32, i32)>
}

impl Snapshot{
    pub fn read(filename: &str) -> Self{
        let mut file = File::open(filename).unwrap(); //TODO: replace Unwrap
        let mut buf = [0u8;12];
        let mut blocks = Vec::new();
        while let Ok(()) = file.read_exact(&mut buf) {
            let x = i32::from_le_bytes(buf[0..4].try_into().unwrap());
            let y = i32::from_le_bytes(buf[4..8].try_into().unwrap());
            let z = i32::from_le_bytes(buf[8..12].try_into().unwrap());
            blocks.push((x, y, z));
        }
        Self{
            blocks
        }
    }
}