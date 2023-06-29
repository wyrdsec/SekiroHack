use std::{fs, fmt, error, path::PathBuf, io::{self, BufRead}};

#[derive(Debug, Clone)]
struct SekiroNotFound;

impl fmt::Display for SekiroNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find sekiro process in /proc directory.")
    }
}

impl error::Error for SekiroNotFound{
    fn description(&self) -> &str {
        ""
    }
}


struct SekiroPatcher {
    pid: u32,
    mem_file: fs::File,
}

impl SekiroPatcher {

    pub fn new(pid_s: String) -> Result<Self,Box<dyn error::Error>> {
        let pid: u32 = pid_s.parse().expect("Could not convert PID to integer...");
        let file = fs::File::open(PROC_PATH.to_owned() + "/" + &pid_s + "/mem")?;
        Ok(SekiroPatcher { pid: pid, mem_file: file })
    }

    pub fn patch_resolution(height: u32, width: u32) -> Result<(),Box<dyn error::Error>> {
        todo!();
    }

    pub fn patch_fps(fps: u32) -> Result<(),Box<dyn error::Error>> {
        todo!();
    }

    pub fn disable_dragonrot() -> Result<(),Box<dyn error::Error>> {
        todo!();
    }

    fn patch_speedfix() -> Result<(),Box<dyn error::Error>> {
        todo!();
    }
}


const PROC_PATH: &'static str = "/proc";
const PROC_NAME: &'static str = "SekiroHack\n";


fn is_sekiro(path: &PathBuf) -> bool {
    if let Ok(f) = fs::File::open(path) {
        let mut lines = io::BufReader::new(f);
        let mut line = String::new();

        match lines.read_line(&mut line) {
            Ok(_) => {},
            Err(_) => { return false }
        }

        if line.ends_with(PROC_NAME) {
            true
        }
        else {
            false
        }
    }
    else {
        false
    }
}


pub fn find_sekiro() -> Result<String, Box<dyn error::Error>> {
    let proc_dir = fs::read_dir(PROC_PATH)?;

    for proc_entry in proc_dir {
        if let Ok(dir_ent) = proc_entry {
            let filetype = dir_ent.file_type()?;

            if filetype.is_dir() {
                let status_path = dir_ent.path().join("status");
                if is_sekiro(&status_path) {
                    let filename = dir_ent.file_name().into_string().unwrap();
                    return Ok(filename);
                }
            }
        } 
    }

    Err(Box::new(SekiroNotFound))
}

// Returns if successful
pub fn sekiro_main() -> bool {
    match find_sekiro() {
        Ok(pid) => { println!("{}", pid) },
        Err(e) => { println!("Error: {}", e); return false;}
    }

    true
}