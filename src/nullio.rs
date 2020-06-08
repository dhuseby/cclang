use crate::{
    AppIO,
    CCLang,
    Machine
};
use std::{
    clone::Clone,
    cmp::{
        PartialEq,
        PartialOrd
    },
    io
};

#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullIO;

impl AppIO<CCLang> for NullIO {
    fn open(&self, _m: &mut Machine<CCLang>) -> io::Result<()> { Ok(()) }
    fn read(&self, _m: &mut Machine<CCLang>) -> io::Result<()> { Ok(()) }
    fn write(&self, _m: &mut Machine<CCLang>) -> io::Result<()> { Ok(()) }
    fn seek(&self, _m: &mut Machine<CCLang>) -> io::Result<()> { Ok(()) }
    fn close(&self, _m: &mut Machine<CCLang>) -> io::Result<()> { Ok(()) }
}

