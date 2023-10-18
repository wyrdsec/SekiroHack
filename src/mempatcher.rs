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
            if byte.ignore { index += 1; continue; }
            if byte.val != buf[index] { return false; }
            index += 1;
        }

        true
    }
}

pub fn patch(mut file: &File, pattern: IgnorableBytePattern, patch: IgnorableBytePattern) -> Result<(),Box<dyn error::Error>>{

    file.rewind()?;
    
    loop {
        let mut buffer: Vec<u8> = vec![0u8; pattern.bytes.len()];
        let current_pos: u64 = file.stream_position()?;

        file.read_exact(&mut buffer)?;
        
        if pattern.compare(buffer) {
            file.seek(io::SeekFrom::Start(current_pos))?;

            for byte in patch.bytes {
                if byte.ignore { 
                    file.seek(io::SeekFrom::Current(1))?;
                }
                else {
                    file.write(&[byte.val])?;
                }
            }
            
            return Ok(());
        }

        file.seek(io::SeekFrom::Start(current_pos+1))?;

    }

    #[allow(unreachable_code)]
    {
        panic!("I shouldn't be here...");
    }
}

#[cfg(test)]
mod tests {
    use crate::mempatcher::*;
    use std::fs::File;
    use std::fs::{copy, remove_file};


    #[test]
    fn check_bytes_write() {    
        copy("testfiles/test.dat", "testfiles/test.dat.working").expect("Cannot copy testfiles/test.dat.");
        let mut file: File = File::options()
            .read(true)
            .write(true)
            .open("testfiles/test.dat.working")
            .expect("Could not open testfile");
        
        let pattern: IgnorableBytePattern = IgnorableBytePattern {
            bytes: vec![
                IgnorableByte { ignore: false, val: 32 },
                IgnorableByte { ignore: false, val: 33 },
                IgnorableByte { ignore: false, val: 34 },
                IgnorableByte { ignore: false, val: 35 },
                IgnorableByte { ignore: false, val: 36 },
                IgnorableByte { ignore: false, val: 37 },
            ]
        };

        let mempatch: IgnorableBytePattern = IgnorableBytePattern {
            bytes: vec![
                IgnorableByte { ignore: false, val: 38 },
                IgnorableByte { ignore: false, val: 39 },
                IgnorableByte { ignore: false, val: 40 },
                IgnorableByte { ignore: false, val: 41 },
                IgnorableByte { ignore: false, val: 42 },
                IgnorableByte { ignore: false, val: 43 },
            ]
        };

        let post_patch_expected_bytes: [u8; 6] = [38,39,40,41,42,43];
        
        let res = patch(&mut file, pattern, mempatch);

        match res {
            Ok(_) => {
                let mut cmp_buff: [u8; 6] = [0,0,0,0,0,0];
                file.seek(io::SeekFrom::Start(1024)).expect("Could not seek on working file.");
                file.read_exact(&mut cmp_buff).unwrap();

                for i in 0..cmp_buff.len() {
                    if cmp_buff[i] != post_patch_expected_bytes[i] {
                        panic!("Patch was not applied properly\nPatch contents: {:?}\nActual contents: {:?}", post_patch_expected_bytes, cmp_buff);
                    }
                }
                
            }
            Err(e) => { panic!("Caught error: {:?}", e); }
        }


        remove_file("testfiles/test.dat.working").expect("Could not delete test file.");
    }
}
