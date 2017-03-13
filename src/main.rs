#![feature(test)]

extern crate byteorder;
extern crate test;

mod farbfeld;

use std::env;
use std::fs::*;
use std::io::Write;
use farbfeld::Farbfeld;

macro_rules! println_stderr {
    ($($arg:tt)*) => {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("Failed to write ro stderr");
    };
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        let file = File::open(path).expect("Failed to open file!");
        let mut img = Farbfeld::load(file).expect("Failed to load image!");
        //println!("Width: {} Height: {}", img.width(), img.height());
    } else {
        println_stderr!("No file given");
    }
}

