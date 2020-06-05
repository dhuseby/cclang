use cclang::{
    AppIO,
    CCLang,
    Machine,
    Script
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
struct NullIO;

impl AppIO<CCL> for NullIO {
    fn open(&self, _m: &mut Machine<CCL>) -> io::Result<()> { Ok(()) }
    fn read(&self, _m: &mut Machine<CCL>) -> io::Result<()> { Ok(()) }
    fn write(&self, _m: &mut Machine<CCL>) -> io::Result<()> { Ok(()) }
    fn seek(&self, _m: &mut Machine<CCL>) -> io::Result<()> { Ok(()) }
    fn close(&self, _m: &mut Machine<CCL>) -> io::Result<()> { Ok(()) }
}

type CCL = CCLang;

#[test]
fn simple_branching_0() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::If,
            CCL::Index(1),
        CCL::Else,
            CCL::Index(2),
        CCL::Fi
    ]);
    let mut machine = Machine::from(script);
    let appio = NullIO;
    let mut result = machine.execute(&appio).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Index should have the value of 1
    match result.pop() {
        Some(CCL::Index(num)) => assert_eq!(num, 1),
        _ => panic!()
    }
}

#[test]
fn simple_branching_1() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::If,
            CCL::Index(1),
        CCL::Else,
            CCL::Index(2),
        CCL::Fi
    ]);
    let mut machine = Machine::from(script);
    let appio = NullIO;
    let mut result = machine.execute(&appio).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Index should have the value of 2
    match result.pop() {
        Some(CCL::Index(num)) => assert_eq!(num, 2),
        _ => panic!()
    }
}

#[test]
fn nested_branching_0() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::If,
            CCL::Index(1),
            CCL::Boolean(true),
            CCL::If,
                CCL::Index(3),
            CCL::Fi,
        CCL::Else,
            CCL::Index(2),
        CCL::Fi
    ]);
    let mut machine = Machine::from(script);
    let appio = NullIO;
    let mut result = machine.execute(&appio).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the Index should have the value of 3
    match result.pop() {
        Some(CCL::Index(num)) => assert_eq!(num, 3),
        _ => panic!()
    }
}

#[test]
fn nested_branching_1() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::If,
            CCL::Index(1),
            CCL::Boolean(false),
            CCL::If,
                CCL::Index(3),
            CCL::Else,
                CCL::Index(4),
            CCL::Fi,
        CCL::Else,
            CCL::Index(2),
        CCL::Fi
    ]);
    let mut machine = Machine::from(script);
    let appio = NullIO;
    let mut result = machine.execute(&appio).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the Index should have the value of 4
    match result.pop() {
        Some(CCL::Index(num)) => assert_eq!(num, 4),
        _ => panic!()
    }
}

#[test]
fn nested_branching_2() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::If,
            CCL::Index(1),
            CCL::Boolean(false),
            CCL::If,
                CCL::Index(3),
            CCL::Else,
                CCL::Index(4),
            CCL::Fi,
        CCL::Else,
            CCL::Index(2),
            CCL::Boolean(true),
            CCL::If,
                CCL::Index(3),
            CCL::Else,
                CCL::Index(4),
            CCL::Fi,
        CCL::Fi
    ]);
    let mut machine = Machine::from(script);
    let appio = NullIO;
    let mut result = machine.execute(&appio).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the Index should have the value of 3
    match result.pop() {
        Some(CCL::Index(num)) => assert_eq!(num, 3),
        _ => panic!()
    }
}

#[test]
fn nested_branching_3() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::If,
            CCL::Index(1),
            CCL::Boolean(false),
            CCL::If,
                CCL::Index(3),
            CCL::Else,
                CCL::Index(4),
            CCL::Fi,
        CCL::Else,
            CCL::Index(2),
            CCL::Boolean(false),
            CCL::If,
                CCL::Index(3),
            CCL::Else,
                CCL::Index(4),
            CCL::Fi,
        CCL::Fi
    ]);
    let mut machine = Machine::from(script);
    let appio = NullIO;
    let mut result = machine.execute(&appio).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the Index should have the value of 4
    match result.pop() {
        Some(CCL::Index(num)) => assert_eq!(num, 4),
        _ => panic!()
    }
}
