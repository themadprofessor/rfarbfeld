use std::path::Path;
use std::io::{Read, BufReader};
use std::fs::File;

use nom::{be_u32, IResult};

mod pixel;
mod parser;
pub mod error;

use self::error::*;
pub use self::pixel::Pixel;

#[derive(Debug)]
pub struct Farbfeld {
    pixels: Vec<Pixel>,
    width: u32,
    height: u32
}

impl Farbfeld {
    pub fn new(width: u32, height: u32, pixels: Vec<Pixel>) -> Result<Farbfeld> {
        if ((width * height) as usize) != pixels.len() {
            Err(Error::from(ErrorKind::InvalidFarbfeldDimensions))
        } else {
            Ok(Farbfeld {
                width,
                height,
                pixels
            })
        }

    }

    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Farbfeld> {
        File::open(path)
            .map_err(|err| Error::from(ErrorKind::IoError(err)))
            .map(BufReader::new)
            .and_then(Farbfeld::from_read)
    }

    pub fn from_read<T: Read>(mut read: T) -> Result<Farbfeld> {
        let mut buff = Vec::new();
        read.read_to_end(&mut buff).map_err(ErrorKind::IoError)?;
        parser::i_to_res(parser::parse_farb(&buff))
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    pub fn row(&self, row: u32) -> Option<&[Pixel]> {
        if row >= self.height {
            None
        } else {
            let offset = (row * self.width) as usize;
            Some(&self.pixels[offset..offset + self.width as usize])
        }
    }

    pub fn width(&self) -> &u32 {
        &self.width
    }

    pub fn height(&self) -> &u32 {
        &self.height
    }

    pub fn into_raw(self) -> Vec<u16> {
        self.pixels.into_iter()
            .flat_map(|pixel| pixel.into_iter())
            .collect::<Vec<u16>>()
    }

    pub fn as_raw(&self) -> Vec<&u16> {
        self.pixels.iter()
            .flat_map(|pixel| pixel.iter())
            .collect::<Vec<&u16>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test::Bencher;

    #[test]
    fn test_invalid_new() {
        assert!(Farbfeld::new(10, 10, Vec::new()).is_err())
    }

    #[bench]
    fn bench_from_file(b: &mut Bencher) {
        b.iter(|| Farbfeld::from_file("test.ff").unwrap())
    }
}
