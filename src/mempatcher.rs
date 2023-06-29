use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::error;

pub struct IgnorableByte {
    ignore: bool,
    val: u8,
}

pub struct IgnorableBytePattern {
    bytes: Vec<IgnorableByte>,
}

impl IgnorableBytePattern {
    pub fn compare(&self, buf: Vec<u8>) -> bool {
        if self.bytes.len() != buf.len() { return false; }
        
        let mut index: usize = 0;
        for byte in &self.bytes {
            index += 1;

            if byte.ignore { continue; }
            if byte.val != buf[index] { return false; }
        }

        true
    }
}

pub fn patch(mut file: File, pattern: IgnorableBytePattern, patch: IgnorableBytePattern) -> Result<(),Box<dyn error::Error>>{

    file.rewind()?;
    
    loop {
        let mut buffer: Vec<u8> = vec![0u8; pattern.bytes.len()];
        let current_pos: u64 = file.stream_position()?;

        file.read_exact(&mut buffer)?;
        
        if pattern.compare(buffer) {
            file.seek(io::SeekFrom::Start(current_pos))?;

            let mut i: usize = 0;
            for byte in patch.bytes {
                i += 1;
                if byte.ignore { 
                    file.seek(io::SeekFrom::Current(1))?;
                }
                else {
                    file.write(&[byte.val])?;
                }
            }
            
            return Ok(());
        }

    }

    #[allow(unreachable_code)]
    {
        panic!("I shouldn't be here...");
    }
}