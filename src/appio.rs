use bytes::Bytes;
use std::clone::Clone;
use std::cmp::{ PartialEq, PartialOrd };
use std::fmt;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Whence {
    Start,
    Cur,
    End
}

impl fmt::Display for Whence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Whence::Start => write!(f, "START"),
            Whence::Cur => write!(f, "CUR"),
            Whence::End => write!(f, "END")
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Mode {
    R,
    RPlus,
    W,
    WPlus,
    A,
    APlus
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::R => write!(f, "r"),
            Mode::RPlus => write!(f, "r+"),
            Mode::W => write!(f, "w"),
            Mode::WPlus => write!(f, "w+"),
            Mode::A => write!(f, "a"),
            Mode::APlus => write!(f, "a+")
        }
    }
}

pub trait AppIO<T, U> 
    where T: Clone,
          U: Clone,
{
    fn open(&self, id: &U, mode: Mode) -> T;
    fn read(&self, h: &T, num: usize) -> Bytes;
    fn write(&self, h: &T, data: &Bytes) -> usize;
    fn seek(&self, h: &T, whence: Whence, num: isize);
    fn close(&self, h: &T);
}
