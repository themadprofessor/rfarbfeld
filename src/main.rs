use std::env;
use std::fs::*;
use std::io::Read;
use std::error::Error;

fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut file = File::open(path).expect("Failed to open file!");
        check_magic(&mut file)?;
        {
            let mut deim_buf = [0; 8];
            let num = file.read(&mut deim_buf)
                .and_then(|x| if x < 8 {Err("Invalid amount read!")} else {Ok(x)})
                .expect("Failed to read dimensions");
        }
    }
}

fn check_magic(file: &mut File) -> Result<(), &str> {
    let mut magic_buff = [0; 8];
    let res = file.read(&mut magic_buff);
    if res.is_err() {
        return res.map_err(|err| err.description())
    } else {
        let num = res.unwrap();
        if num < 8 {
            Err("Invalid amount read!")
        } else if magic_buff != [0x66, 0x61, 0x72, 0x62, 0x66, 0x65, 0x6c, 0x64] {
            Err("Invalid farbeld file")
        }
    }
}