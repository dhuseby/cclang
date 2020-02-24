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
use std::fmt;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullHandle {
    identifier: String,
    mode: Mode
}

impl fmt::Display for NullHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "NullHandle")
    }
}


#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullIO;

impl fmt::Display for NullIO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "NullIO")
    }
}

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
fn simple_branching_0() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::If,
            CCL::IOIndex(1),
        CCL::Else,
            CCL::IOIndex(2),
        CCL::Fi
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // there should be a single IOIndex value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the IOIndex should have the value of 1
    match result.pop() {
        Some(CCL::IOIndex(num)) => assert_eq!(num, 1),
        _ => panic!()
    }
}

#[test]
fn simple_branching_1() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::If,
            CCL::IOIndex(1),
        CCL::Else,
            CCL::IOIndex(2),
        CCL::Fi
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // there should be a single IOIndex value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the IOIndex should have the value of 2
    match result.pop() {
        Some(CCL::IOIndex(num)) => assert_eq!(num, 2),
        _ => panic!()
    }
}

#[test]
fn nested_branching_0() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::If,
            CCL::IOIndex(1),
            CCL::Boolean(true),
            CCL::If,
                CCL::IOIndex(3),
            CCL::Fi,
        CCL::Else,
            CCL::IOIndex(2),
        CCL::Fi
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // there should be a single IOIndex value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the IOIndex should have the value of 3
    match result.pop() {
        Some(CCL::IOIndex(num)) => assert_eq!(num, 3),
        _ => panic!()
    }
}

#[test]
fn nested_branching_1() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::If,
            CCL::IOIndex(1),
            CCL::Boolean(false),
            CCL::If,
                CCL::IOIndex(3),
            CCL::Else,
                CCL::IOIndex(4),
            CCL::Fi,
        CCL::Else,
            CCL::IOIndex(2),
        CCL::Fi
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // there should be a single IOIndex value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the IOIndex should have the value of 4
    match result.pop() {
        Some(CCL::IOIndex(num)) => assert_eq!(num, 4),
        _ => panic!()
    }
}

#[test]
fn nested_branching_2() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::If,
            CCL::IOIndex(1),
            CCL::Boolean(false),
            CCL::If,
                CCL::IOIndex(3),
            CCL::Else,
                CCL::IOIndex(4),
            CCL::Fi,
        CCL::Else,
            CCL::IOIndex(2),
            CCL::Boolean(true),
            CCL::If,
                CCL::IOIndex(3),
            CCL::Else,
                CCL::IOIndex(4),
            CCL::Fi,
        CCL::Fi
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // there should be a single IOIndex value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the IOIndex should have the value of 3
    match result.pop() {
        Some(CCL::IOIndex(num)) => assert_eq!(num, 3),
        _ => panic!()
    }
}

#[test]
fn nested_branching_3() {

    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::If,
            CCL::IOIndex(1),
            CCL::Boolean(false),
            CCL::If,
                CCL::IOIndex(3),
            CCL::Else,
                CCL::IOIndex(4),
            CCL::Fi,
        CCL::Else,
            CCL::IOIndex(2),
            CCL::Boolean(false),
            CCL::If,
                CCL::IOIndex(3),
            CCL::Else,
                CCL::IOIndex(4),
            CCL::Fi,
        CCL::Fi
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // there should be a single IOIndex value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the IOIndex should have the value of 4
    match result.pop() {
        Some(CCL::IOIndex(num)) => assert_eq!(num, 4),
        _ => panic!()
    }
}
