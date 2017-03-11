use std::io::{Read, Cursor};
use std::fmt;
use std::fmt::{Display,Formatter};
use std::error;
use byteorder::{ReadBytesExt, BigEndian};

#[derive(Debug)]
pub struct FarbfeldErr {
    desc: String,
    super_err: Option<::std::io::Error>
}

#[derive(Debug)]
struct Pixel {
    red: u16,
    green: u16,
    blue: u16,
    alpha: u16
}

pub struct Farbfeld {
    pixels: Vec<Pixel>,
    width: u32,
    height: u32
}

impl error::Error for FarbfeldErr {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}

impl Display for FarbfeldErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let desc = &self.desc;
        let error = &self.super_err.as_ref();
        write!(f, "Failed to parse farbfeld data! {} {}", desc, &error.map_or("".to_string(), |err| format!("{}", err)))
    }
}

impl Pixel {
    fn new(buff: &Vec<u8>) -> Result<Pixel, FarbfeldErr> {
        let red_res = Cursor::new(buff[0..2].to_vec()).read_u16::<BigEndian>();
        let green_res = Cursor::new(buff[2..4].to_vec()).read_u16::<BigEndian>();
        let blue_res = Cursor::new(buff[4..6].to_vec()).read_u16::<BigEndian>();
        let alpha_res = Cursor::new(buff[6..8].to_vec()).read_u16::<BigEndian>();

        if red_res.is_ok() && green_res.is_ok() && blue_res.is_ok() && alpha_res.is_ok() {
            Ok(Pixel{
                red: red_res.unwrap(),
                green: green_res.unwrap(),
                blue: blue_res.unwrap(),
                alpha: alpha_res.unwrap()
            })
        } else {
            Err(FarbfeldErr{
                desc: format!("Could not read pixel value! Red Error: {}  Green Error: {}  \
                Blue Error: {}  Alpha Error: {}",
                              err_to_string(red_res),
                              err_to_string(green_res),
                              err_to_string(blue_res),
                              err_to_string(alpha_res)),
                super_err: None
            })
        }
    }
}

impl Farbfeld {
    pub fn load<T>(mut reader: T) -> Result<Farbfeld, FarbfeldErr> where T: Read {
        let empty = [0; 8];

        let mut buff:Vec<u8> = Vec::with_capacity(8);
        buff.extend_from_slice(&empty);
        let res = reader.read(&mut buff);
        res.map_err(|err| FarbfeldErr{desc: "Failed to read magic number!".to_string(), super_err: Some(err)})
            .and_then(|num| {
                if num < 8 {
                    Err(FarbfeldErr{desc: format!("Failed to read enough data for magic number! Read {} bytes.", num),
                        super_err: None})
                } else if buff != [0x66, 0x61, 0x72, 0x62, 0x66, 0x65, 0x6c, 0x64] {
                    Err(FarbfeldErr{desc: "Magic number indicated not farbfeld data!".to_string(),
                        super_err: None})
                } else {
                    buff.clear();
                    buff.extend_from_slice(&empty);
                    reader.read(&mut buff).map_err(|err|
                        FarbfeldErr{desc: "Failed to read dimensions".to_string(),
                            super_err: Some(err)})
                }
            })
        .and_then(|num| {
            if num < 8 {
                Err(FarbfeldErr{desc: format!("Failed to read enough data for dimensions! Read {} bytes.", num), super_err: None})
            } else {
                let width_res = Cursor::new(buff[0..4].to_vec()).read_u32::<BigEndian>();
                let height_res = Cursor::new(buff[4..8].to_vec()).read_u32::<BigEndian>();
                if width_res.is_ok() && height_res.is_ok() {
                    Ok((width_res.unwrap(), height_res.unwrap()))
                } else {
                    Err(FarbfeldErr{desc: format!("Could not parse dimensions! Width Error: {}  Height Error: {}",
                                     err_to_string(width_res),
                                     err_to_string(height_res)),
                        super_err: None})
                }
            }})
         .and_then(|dimensions| {
            let mut pixels = Vec::with_capacity((dimensions.0 * dimensions.1) as usize);
            while true {
                buff.clear();
                buff.extend_from_slice(&empty);

                let count_res = reader.read(&mut buff).map_err(|err|
                    FarbfeldErr{desc: "Failed to read data for pixels!".to_string(), super_err: Some(err)});
                if count_res.is_err(){
                    return count_res.map(|_| Farbfeld{width: 0, height: 0, pixels: Vec::new()})
                } else {
                    let count = count_res.unwrap();
                    if count == 0 {
                        break;
                    } else if count < 8 {
                        return Err(FarbfeldErr{desc: format!("Failed to read enough data! Read {} bytes.", count),
                            super_err: None})
                    }
                }

                let pixel_res = Pixel::new(&buff);
                if pixel_res.is_err() {
                    return pixel_res.map(|_| Farbfeld{width: 0, height: 0, pixels: Vec::new()})
                } else {
                    pixels.push(pixel_res.unwrap());
                }
            }

            Ok(Farbfeld{width: dimensions.0, height: dimensions.1, pixels: pixels})
        })
    }

    pub fn height(&mut self) -> u32 {
        self.height
    }

    pub fn width(&mut self) -> u32 {
        self.width
    }

    pub fn pixels(&mut self) -> &Vec<Pixel> {
        &self.pixels
    }
}

fn err_to_string<T, E:error::Error>(res: Result<T, E>) -> String {
    match res {
        Ok(_) => "".to_string(),
        Err(err) => format!("{}", err)
    }
}
