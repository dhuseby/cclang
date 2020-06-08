use cclang::{
    CCLang::{
        self,
        Boolean,
        Dup,
        Pop
    },
    Machine,
    NullIO,
    Script
};

#[test]
pub fn stack_0() {
    let script = Script::from(vec![
        Boolean(true),
        Boolean(false),
        Pop
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
pub fn stack_1() {
    let script = Script::from(vec![
        Boolean(true),
        Dup
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 2 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }

    // the result should be a boolean with the value true
    match result.pop() {
        Some(Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

#[test]
pub fn stack_ser_0() {
    let script = Script::from(vec![
        Boolean(true),
        Boolean(false),
        Pop
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""TRUE FALSE POP""#);
}

#[test]
pub fn stack_ser_1() {
    let script = Script::from(vec![
        Boolean(true),
        Dup
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""TRUE DUP""#);
}

#[test]
pub fn stack_de_0() {
    let s1 = Script::from(vec![
        Boolean(true),
        Boolean(false),
        Pop
    ]);
    let s = r#""TRUE FALSE POP""#;
    let s2: Script<CCLang> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn stack_de_1() {
    let s1 = Script::from(vec![
        Boolean(true),
        Dup
    ]);
    let s = r#""TRUE DUP""#;
    let s2: Script<CCLang> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}


