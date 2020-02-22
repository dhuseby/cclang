use bytes::Bytes;
use cclang::{
    AppIO,
    Whence,
    Mode,
    CCLang,
    Machine,
    Script
};
use std::clone::Clone;
use std::cmp::{ PartialEq, PartialOrd };

#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullHandle {
    identifier: String,
    mode: Mode
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullIO;

impl AppIO<NullHandle, String> for NullIO {
    fn open(&self, id: &String, mode: Mode) -> NullHandle {
        NullHandle {
            identifier: id.to_string(),
            mode: mode
        }
    }

    fn read(&self, _h: &NullHandle, _num: usize) -> Bytes {
        Bytes::new()
    }

    fn write(&self, _h: &NullHandle, _data: &Bytes) -> usize {
        0
    }

    fn seek(&self, _h: &NullHandle, _whence: Whence, _num: isize) {}
    fn close(&self, _h: &NullHandle) {}
}

type CCL = CCLang<NullHandle, String, NullIO>;

#[test]
pub fn stack_0() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Boolean(false),
        CCL::Pop
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(CCL::Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn stack_1() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Dup
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 2 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(CCL::Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }

    // the result should be a boolean with the value true
    match result.pop() {
        Some(CCL::Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}


