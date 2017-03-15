use std::io::{Read, Cursor};
use std::fmt;
use std::fmt::{Display,Formatter};
use std::error;
use std::ops::{Index, IndexMut};
use byteorder::{ReadBytesExt, BigEndian};

#[derive(Debug)]
pub struct FarbfeldErr {
    desc: String,
    super_err: Option<::std::io::Error>
}

#[derive(Debug)]
pub struct Pixel {
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

    fn red(&self) -> u16 {
        self.red
    }

    fn green(&self) -> u16 {
        self.green
    }

    fn blue(&self) -> u16 {
        self.blue
    }

    fn alpha(&self) -> u16 {
        self.alpha
    }
}

impl Farbfeld {
    pub fn load<T>(mut reader: T) -> Result<Farbfeld, FarbfeldErr> where T: Read {
        let empty = [0; 8];

        let mut buff:Vec<u8> = Vec::with_capacity(8);
        buff.extend_from_slice(&empty);
        let res = reader.read(&mut buff);
        res.map_err(|err| FarbfeldErr{desc: "Failed to read magic number!".to_string(), super_err: Some(err)})
            .and_then(|num| check_magic(num, &mut buff, &mut reader, &empty))
            .and_then(|num| get_dimensions(num, &mut buff))
            .and_then(|dimensions| load_pixels(&mut reader, &mut buff, &dimensions, &empty)
                .map(|pixels| Farbfeld{width: dimensions.0, height: dimensions.1, pixels: pixels}))
    }

    pub fn get(&self, index: usize) -> Option<&Pixel> {
        self.pixels.get(index)
    }

    pub fn get_pos(&self, pos: [u32; 2]) -> Option<&Pixel> {
        self.get((self.width * pos[0] + pos[1]) as usize)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Pixel> {
        self.pixels.get_mut(index)
    }

    pub fn get_pos_mut(&mut self, pos: [u32; 2]) -> Option<&mut Pixel> {
        self.pixels.get_mut((self.width * pos[0] + pos[0]) as usize)
    }

    pub fn height(&mut self) -> u32 {
        self.height
    }

    pub fn width(&mut self) -> u32 {
        self.width
    }
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
        write!(f, "Failed to parse farbfeld data! {} {}", desc,
               &error.map_or("".to_string(), |err| format!("{}", err)))
    }
}

impl Index<[u32; 2]> for Farbfeld {
    type Output = Pixel;

    fn index(&self, index: [u32; 2]) -> &Self::Output {
        &self.pixels[(index[0] * self.width + index[1]) as usize]
    }
}

impl Index<usize> for Farbfeld {
    type Output = Pixel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

impl IndexMut<[u32; 2]> for Farbfeld {
    fn index_mut(&mut self, index: [u32; 2]) -> &mut Self::Output {
        &mut self.pixels[(index[0] * self.width + index[1]) as usize]
    }
}

impl IndexMut<usize> for Farbfeld {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

fn err_to_string<T, E:error::Error>(res: Result<T, E>) -> String {
    match res {
        Ok(_) => "".to_string(),
        Err(err) => format!("{}", err)
    }
}

fn load_pixels(reader: &mut Read, mut buff: &mut Vec<u8>, dimensions: &(u32, u32), empty: &[u8; 8]) -> Result<Vec<Pixel>, FarbfeldErr> {
    let mut pixels = Vec::with_capacity((dimensions.0 * dimensions.1) as usize);
    loop {
        buff.clear();
        buff.extend_from_slice(empty);

        let count_res = reader.read(&mut buff).map_err(|err|
            FarbfeldErr{desc: "Failed to read data for pixels!".to_string(), super_err: Some(err)});
        if count_res.is_err(){
            return count_res.map(|_| Vec::new())
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
            return Err(FarbfeldErr{desc: "Failed to parse pixel!".to_string(), super_err: None})
        } else {
            pixels.push(pixel_res.unwrap());
        }
    }

    pixels.shrink_to_fit();
    Ok(pixels)
}

fn check_magic(num: usize, mut buff: &mut Vec<u8>, mut reader: &mut Read, empty: &[u8;8]) -> Result<usize, FarbfeldErr> {
    if num < 8 {
        Err(FarbfeldErr{desc: format!("Failed to read enough data for magic number! Read {} bytes.", num),
            super_err: None})
    } else if buff.as_ref() != [0x66, 0x61, 0x72, 0x62, 0x66, 0x65, 0x6c, 0x64] {
        Err(FarbfeldErr{desc: "Magic number indicated not farbfeld data!".to_string(),
            super_err: None})
    } else {
        buff.clear();
        buff.extend_from_slice(empty);
        reader.read(&mut buff).map_err(|err|
            FarbfeldErr{desc: "Failed to read dimensions".to_string(),
                super_err: Some(err)})
    }
}

    fn get_dimensions(num: usize, mut buff: &mut Vec<u8>) -> Result<(u32, u32), FarbfeldErr> {
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
        }
}