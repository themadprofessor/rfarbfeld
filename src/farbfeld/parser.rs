use nom::{be_u32, be_u16, IResult};

use farbfeld::Pixel;
use farbfeld::Farbfeld;
use farbfeld::error::*;

named!(pub parse_pixel<Pixel>, do_parse!(
    red: be_u16 >>
    green: be_u16 >>
    blue: be_u16 >>
    alpha: be_u16 >>
    (Pixel::new(red, green, blue, alpha))
));

named!(pub parse_farb<Farbfeld>, do_parse!(
    tag!("farbfeld") >>
    width: be_u32 >>
    height: be_u32 >>
    pixels: many0!(flat_map!(take!(8), parse_pixel)) >>
    res: expr_res!(Farbfeld::new(width, height, pixels)) >>
    (res)
));

pub fn i_to_res<I, O>(res: IResult<I, O, u32>) -> Result<O> {
    match res {
        IResult::Incomplete(need) => Err(Error::from(ErrorKind::NotEnoughDataError(need))),
        IResult::Done(_, farb) => Ok(farb),
        IResult::Error(err) => Err(Error::from(ErrorKind::NomError(err)))
    }
}