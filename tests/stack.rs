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
pub fn stack_0() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Boolean(false),
        CCL::Pop
    ]);
    let mut machine = Machine::from(script);
    let appio = NullIO;
    let mut result = machine.execute(&appio).unwrap();

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
    let mut machine = Machine::from(script);
    let appio = NullIO;
    let mut result = machine.execute(&appio).unwrap();

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

#[test]
pub fn stack_ser_0() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Boolean(false),
        CCL::Pop
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""TRUE FALSE POP""#);
}

#[test]
pub fn stack_ser_1() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Dup
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""TRUE DUP""#);
}

#[test]
pub fn stack_de_0() {
    let s1 = Script::from(vec![
        CCL::Boolean(true),
        CCL::Boolean(false),
        CCL::Pop
    ]);
    let s = r#""TRUE FALSE POP""#;
    let s2: Script<CCL> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn stack_de_1() {
    let s1 = Script::from(vec![
        CCL::Boolean(true),
        CCL::Dup
    ]);
    let s = r#""TRUE DUP""#;
    let s2: Script<CCL> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}


