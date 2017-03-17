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
    fn new(buff: &[u8; 8]) -> Result<Pixel, FarbfeldErr> {
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
        let mut buff = [0; 8];

        if let Some(err) = check_count(reader.read(&mut buff), 8) {
            return Err(FarbfeldErr{
                desc: format!("Failed to read magic number data! Caused by: {}", err),
                super_err: None})
        }
        if let Some(err) = check_magic(&buff) {
            return Err(err)
        }

        if let Some(err) = check_count(reader.read(&mut buff), 8) {
            return Err(FarbfeldErr{
                desc: format!("Failed to read dimension data! Caused by: {}", err),
                super_err: None})
        }
        let dimen_res = get_dimensions(&buff);
        let dimensions = match dimen_res {
            Ok((width, height)) => (width, height),
            Err(err) => return Err(err)
        };

        load_pixels(&mut reader, &dimensions).map(|pixels| Farbfeld{
            pixels: pixels,
            width: dimensions.0,
            height: dimensions.1
        })
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

fn check_count(count_res: ::std::io::Result<usize>, count_req: usize) -> Option<FarbfeldErr> {
    if count_res.is_err() {
        return Some(FarbfeldErr{
            desc: "Failed to read data!".to_string(),
            super_err: Some(count_res.unwrap_err())})
    } else {
        let count = count_res.unwrap();
        if count < count_req {
            return Some(FarbfeldErr{desc:
                format!("Failed to read correct amount of data! Read {} bytes.", count),
                super_err: None})
        } else {
            None
        }
    }
}

fn err_to_string<T, E:error::Error>(res: Result<T, E>) -> String {
    match res {
        Ok(_) => String::new(),
        Err(err) => format!("{}", err)
    }
}

fn load_pixels(reader: &mut Read, dimensions: &(u32, u32)) -> Result<Vec<Pixel>, FarbfeldErr> {
    let mut pixels = Vec::with_capacity((dimensions.0 * dimensions.1) as usize);

    loop {
        let mut buff = [0; 8];
        let count_res = reader.read(&mut buff)
            .map_err(|err| FarbfeldErr{desc: "Failed to read data!".to_string(), super_err: Some(err)})
            .and_then(|num| {
                if num < 8 && num != 0 {
                    Err(FarbfeldErr{
                        desc: format!("Failed to read enough data for pixels! Read {} bytes.", num),
                        super_err: None})
                } else {
                    Ok(num)
                }
            });
        if count_res.is_err() {
            return Err(count_res.unwrap_err())
        } else if count_res.unwrap() == 0 {
            break;
        }

        let pixel_res = Pixel::new(&buff);
        if pixel_res.is_err() {
            return Err(pixel_res.unwrap_err())
        } else {
            pixels.push(pixel_res.unwrap())
        }
    }

    pixels.shrink_to_fit();
    Ok::<Vec<Pixel>, FarbfeldErr>(pixels)
}

fn check_magic(buff: &[u8; 8]) -> Option<FarbfeldErr> {
    if buff == &[0x66, 0x61, 0x72, 0x62, 0x66, 0x65, 0x6c, 0x64] {
        None
    } else {
        Some(FarbfeldErr{desc: "Invalid farbeld file! Incorrect magic number!".to_string(),
            super_err: None})
    }
}

fn get_dimensions(buff: &[u8; 8]) -> Result<(u32, u32), FarbfeldErr> {
    let width_res = Cursor::new(&buff[0..4]).read_u32::<BigEndian>();
    let height_res = Cursor::new(&buff[4..8]).read_u32::<BigEndian>();
    if width_res.is_ok() && height_res.is_ok() {
        Ok((width_res.unwrap(), height_res.unwrap()))
    } else {
        Err(FarbfeldErr{desc: format!("Couldn't parse dimensions! Width Error: {} Height Error: {}",
                                      err_to_string(width_res),
                                      err_to_string(height_res)),
            super_err: None})
    }
}

mod test {
    extern crate test;

    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use test::Bencher;

    #[bench]
    fn ld(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("test.ff").expect("Failed to open file!");
            Farbfeld::load(BufReader::new(file)).unwrap()
        })
    }
}