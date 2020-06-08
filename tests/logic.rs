use bytes::Bytes;
use cclang::{
    CCLang::{
        self,
        Binary,
        Boolean,
        Equal,
        GreaterThan,
        GreaterThanEqual,
        Index,
        LessThan,
        LessThanEqual,
        NotEqual,
        Text
    },
    Machine,
    NullIO,
    Script
};

#[test]
pub fn equal_0() {
    let script = Script::from(vec![
        Index(0),
        Index(0),
        Equal
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn equal_1() {
    let script = Script::from(vec![
        Boolean(false),
        Index(0),
        Equal
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn equal_2() {
    let script = Script::from(vec![
        Boolean(true),
        Index(10),
        Equal
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn equal_3() {
    let script = Script::from(vec![
        Text(String::from("Hello!")),
        Text(String::from("Hello!")),
        Equal
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn equal_4() {
    let script = Script::from(vec![
        Binary(Bytes::from(&b"Hello!"[..])),
        Binary(Bytes::from(&b"Hello!"[..])),
        Equal
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn equal_ser_0() {
    let script = Script::from(vec![
        Index(0),
        Index(0),
        Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""0 0 =""#);
}

#[test]
pub fn equal_ser_1() {
    let script = Script::from(vec![
        Boolean(false),
        Index(0),
        Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""FALSE 0 =""#);
}

#[test]
pub fn equal_de_0() {
    let s1 = Script::from(vec![
        Index(0),
        Index(0),
        Equal
    ]);
    let s = r#""0 0 =""#;
    let s2: Script<CCLang> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn equal_de_1() {
    let s1 = Script::from(vec![
        Boolean(false),
        Index(0),
        Equal
    ]);
    let s = r#""FALSE 0 =""#;
    let s2: Script<CCLang> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn not_equal_0() {
    let script = Script::from(vec![
        Index(0),
        Index(1),
        NotEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn not_equal_1() {
    let script = Script::from(vec![
        Boolean(true),
        Boolean(false),
        NotEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn not_equal_2() {
    let script = Script::from(vec![
        Boolean(true),
        Index(0),
        NotEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn not_equal_3() {
    let script = Script::from(vec![
        Text(String::from("Hello!")),
        Text(String::from("World!")),
        NotEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn not_equal_4() {
    let script = Script::from(vec![
        Binary(Bytes::from(&b"Hello!"[..])),
        Binary(Bytes::from(&b"World!"[..])),
        NotEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_0() {
    let script = Script::from(vec![
        Index(0),
        Index(1),
        LessThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_1() {
    let script = Script::from(vec![
        Boolean(false),
        Boolean(true),
        LessThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_2() {
    let script = Script::from(vec![
        Index(0),
        Boolean(true),
        LessThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_3() {
    let script = Script::from(vec![
        Text(String::from("Hello!")),
        Text(String::from("World!")),
        LessThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_4() {
    let script = Script::from(vec![
        Binary(Bytes::from(&b"Hello!"[..])),
        Binary(Bytes::from(&b"World!"[..])),
        LessThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_equal_0() {
    let script = Script::from(vec![
        Index(0),
        Index(0),
        LessThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_equal_1() {
    let script = Script::from(vec![
        Boolean(false),
        Boolean(true),
        LessThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_equal_2() {
    let script = Script::from(vec![
        Index(0),
        Boolean(true),
        LessThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_equal_3() {
    let script = Script::from(vec![
        Text(String::from("Hello!")),
        Text(String::from("Hello!")),
        LessThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn less_than_equal_4() {
    let script = Script::from(vec![
        Binary(Bytes::from(&b"Hello!"[..])),
        Binary(Bytes::from(&b"World!"[..])),
        LessThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_0() {
    let script = Script::from(vec![
        Index(1),
        Index(0),
        GreaterThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_1() {
    let script = Script::from(vec![
        Boolean(true),
        Boolean(false),
        GreaterThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_2() {
    let script = Script::from(vec![
        Boolean(true),
        Index(0),
        GreaterThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_3() {
    let script = Script::from(vec![
        Text(String::from("World!")),
        Text(String::from("Hello!")),
        GreaterThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_4() {
    let script = Script::from(vec![
        Binary(Bytes::from(&b"World!"[..])),
        Binary(Bytes::from(&b"Hello!"[..])),
        GreaterThan
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_equal_0() {
    let script = Script::from(vec![
        Index(0),
        Index(0),
        GreaterThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_equal_1() {
    let script = Script::from(vec![
        Boolean(true),
        Boolean(false),
        GreaterThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_equal_2() {
    let script = Script::from(vec![
        Boolean(true),
        Index(0),
        GreaterThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_equal_3() {
    let script = Script::from(vec![
        Text(String::from("Hello!")),
        Text(String::from("Hello!")),
        GreaterThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn greater_than_equal_4() {
    let script = Script::from(vec![
        Binary(Bytes::from(&b"World!"[..])),
        Binary(Bytes::from(&b"Hello!"[..])),
        GreaterThanEqual
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}


