use nom::{be_u16, IResult};

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Pixel {
    red: u16,
    green: u16,
    blue: u16,
    alpha: u16
}

impl Pixel {
    pub fn new<T>(red: T, green: T, blue: T, alpha: T) -> Pixel where T: Into<u16> {
        Pixel {
            red: red.into(),
            green: green.into(),
            blue: blue.into(),
            alpha: alpha.into()
        }
    }

    pub fn red(&self) -> &u16 {
        &self.red
    }

    pub fn green(&self) -> &u16 {
        &self.green
    }

    pub fn blue(&self) -> &u16 {
        &self.blue
    }

    pub fn alpha(&self) -> &u16 {
        &self.alpha
    }

    pub fn red_mut(&mut self) -> &mut u16 {
        &mut self.red
    }

    pub fn green_mut(&mut self) -> &mut u16 {
        &mut self.green
    }

    pub fn blue_mut(&mut self) -> &mut u16 {
        &mut self.blue
    }

    pub fn alpha_mut(&mut self) -> &mut u16 {
        &mut self.alpha
    }
}

impl From<[u16; 4]> for Pixel {
    fn from(i: [u16; 4]) -> Self {
        Pixel {
            red: i[0],
            green: i[1],
            blue: i[2],
            alpha: i[3]
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::*;

    use test::Bencher;

    #[test]
    fn test_parse() {
        assert_eq!(Pixel::from([10_u16, 20_u16, 30_u16, 40_u16]), Pixel::new(10_u16, 20_u16, 30_u16, 40_u16));
    }
}