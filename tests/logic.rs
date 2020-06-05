use bytes::Bytes;
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
pub fn equal_0() {
    let script = Script::from(vec![
        CCL::Index(0),
        CCL::Index(0),
        CCL::Equal
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
pub fn equal_1() {
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::Index(0),
        CCL::Equal
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
pub fn equal_2() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Index(10),
        CCL::Equal
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
pub fn equal_3() {
    let script = Script::from(vec![
        CCL::Text(String::from("Hello!")),
        CCL::Text(String::from("Hello!")),
        CCL::Equal
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
pub fn equal_4() {
    let script = Script::from(vec![
        CCL::Binary(Bytes::from(&b"Hello!"[..])),
        CCL::Binary(Bytes::from(&b"Hello!"[..])),
        CCL::Equal
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
pub fn equal_ser_0() {
    let script = Script::from(vec![
        CCL::Index(0),
        CCL::Index(0),
        CCL::Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""0 0 =""#);
}

#[test]
pub fn equal_ser_1() {
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::Index(0),
        CCL::Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""FALSE 0 =""#);
}

#[test]
pub fn equal_de_0() {
    let s1 = Script::from(vec![
        CCL::Index(0),
        CCL::Index(0),
        CCL::Equal
    ]);
    let s = r#""0 0 =""#;
    let s2: Script<CCL> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn equal_de_1() {
    let s1 = Script::from(vec![
        CCL::Boolean(false),
        CCL::Index(0),
        CCL::Equal
    ]);
    let s = r#""FALSE 0 =""#;
    let s2: Script<CCL> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn not_equal_0() {
    let script = Script::from(vec![
        CCL::Index(0),
        CCL::Index(1),
        CCL::NotEqual
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
pub fn not_equal_1() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Boolean(false),
        CCL::NotEqual
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
pub fn not_equal_2() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Index(0),
        CCL::NotEqual
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
pub fn not_equal_3() {
    let script = Script::from(vec![
        CCL::Text(String::from("Hello!")),
        CCL::Text(String::from("World!")),
        CCL::NotEqual
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
pub fn not_equal_4() {
    let script = Script::from(vec![
        CCL::Binary(Bytes::from(&b"Hello!"[..])),
        CCL::Binary(Bytes::from(&b"World!"[..])),
        CCL::NotEqual
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
pub fn less_than_0() {
    let script = Script::from(vec![
        CCL::Index(0),
        CCL::Index(1),
        CCL::LessThan
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
pub fn less_than_1() {
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::Boolean(true),
        CCL::LessThan
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
pub fn less_than_2() {
    let script = Script::from(vec![
        CCL::Index(0),
        CCL::Boolean(true),
        CCL::LessThan
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
pub fn less_than_3() {
    let script = Script::from(vec![
        CCL::Text(String::from("Hello!")),
        CCL::Text(String::from("World!")),
        CCL::LessThan
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
pub fn less_than_4() {
    let script = Script::from(vec![
        CCL::Binary(Bytes::from(&b"Hello!"[..])),
        CCL::Binary(Bytes::from(&b"World!"[..])),
        CCL::LessThan
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
pub fn less_than_equal_0() {
    let script = Script::from(vec![
        CCL::Index(0),
        CCL::Index(0),
        CCL::LessThanEqual
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
pub fn less_than_equal_1() {
    let script = Script::from(vec![
        CCL::Boolean(false),
        CCL::Boolean(true),
        CCL::LessThanEqual
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
pub fn less_than_equal_2() {
    let script = Script::from(vec![
        CCL::Index(0),
        CCL::Boolean(true),
        CCL::LessThanEqual
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
pub fn less_than_equal_3() {
    let script = Script::from(vec![
        CCL::Text(String::from("Hello!")),
        CCL::Text(String::from("Hello!")),
        CCL::LessThanEqual
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
pub fn less_than_equal_4() {
    let script = Script::from(vec![
        CCL::Binary(Bytes::from(&b"Hello!"[..])),
        CCL::Binary(Bytes::from(&b"World!"[..])),
        CCL::LessThanEqual
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
pub fn greater_than_0() {
    let script = Script::from(vec![
        CCL::Index(1),
        CCL::Index(0),
        CCL::GreaterThan
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
pub fn greater_than_1() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Boolean(false),
        CCL::GreaterThan
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
pub fn greater_than_2() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Index(0),
        CCL::GreaterThan
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
pub fn greater_than_3() {
    let script = Script::from(vec![
        CCL::Text(String::from("World!")),
        CCL::Text(String::from("Hello!")),
        CCL::GreaterThan
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
pub fn greater_than_4() {
    let script = Script::from(vec![
        CCL::Binary(Bytes::from(&b"World!"[..])),
        CCL::Binary(Bytes::from(&b"Hello!"[..])),
        CCL::GreaterThan
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
pub fn greater_than_equal_0() {
    let script = Script::from(vec![
        CCL::Index(0),
        CCL::Index(0),
        CCL::GreaterThanEqual
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
pub fn greater_than_equal_1() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Boolean(false),
        CCL::GreaterThanEqual
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
pub fn greater_than_equal_2() {
    let script = Script::from(vec![
        CCL::Boolean(true),
        CCL::Index(0),
        CCL::GreaterThanEqual
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
pub fn greater_than_equal_3() {
    let script = Script::from(vec![
        CCL::Text(String::from("Hello!")),
        CCL::Text(String::from("Hello!")),
        CCL::GreaterThanEqual
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
pub fn greater_than_equal_4() {
    let script = Script::from(vec![
        CCL::Binary(Bytes::from(&b"World!"[..])),
        CCL::Binary(Bytes::from(&b"Hello!"[..])),
        CCL::GreaterThanEqual
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


