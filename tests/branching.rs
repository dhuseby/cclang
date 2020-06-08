use cclang::{
    CCLang::{
        Boolean,
        Else,
        Fi,
        If,
        Index
    },
    Machine,
    NullIO,
    Script
};

#[test]
fn simple_branching_0() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Boolean(true),
        If,
            Index(1),
        Else,
            Index(2),
        Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Index should have the value of 1
    match result.pop() {
        Some(Index(num)) => assert_eq!(num, 1),
        _ => panic!()
    }
}

#[test]
fn simple_branching_1() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Boolean(false),
        If,
            Index(1),
        Else,
            Index(2),
        Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 1 as usize);

    // the Index should have the value of 2
    match result.pop() {
        Some(Index(num)) => assert_eq!(num, 2),
        _ => panic!()
    }
}

#[test]
fn nested_branching_0() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Boolean(true),
        If,
            Index(1),
            Boolean(true),
            If,
                Index(3),
            Fi,
        Else,
            Index(2),
        Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the Index should have the value of 3
    match result.pop() {
        Some(Index(num)) => assert_eq!(num, 3),
        _ => panic!()
    }
}

#[test]
fn nested_branching_1() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Boolean(true),
        If,
            Index(1),
            Boolean(false),
            If,
                Index(3),
            Else,
                Index(4),
            Fi,
        Else,
            Index(2),
        Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the Index should have the value of 4
    match result.pop() {
        Some(Index(num)) => assert_eq!(num, 4),
        _ => panic!()
    }
}

#[test]
fn nested_branching_2() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Boolean(false),
        If,
            Index(1),
            Boolean(false),
            If,
                Index(3),
            Else,
                Index(4),
            Fi,
        Else,
            Index(2),
            Boolean(true),
            If,
                Index(3),
            Else,
                Index(4),
            Fi,
        Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the Index should have the value of 3
    match result.pop() {
        Some(Index(num)) => assert_eq!(num, 3),
        _ => panic!()
    }
}

#[test]
fn nested_branching_3() {
    // construct a simple if/else/fi script and load it into the machine
    let script = Script::from(vec![
        Boolean(false),
        If,
            Index(1),
            Boolean(false),
            If,
                Index(3),
            Else,
                Index(4),
            Fi,
        Else,
            Index(2),
            Boolean(false),
            If,
                Index(3),
            Else,
                Index(4),
            Fi,
        Fi
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&NullIO).unwrap();

    // there should be a single Index value on the stack
    assert_eq!(result.size(), 2 as usize);

    // the Index should have the value of 4
    match result.pop() {
        Some(Index(num)) => assert_eq!(num, 4),
        _ => panic!()
    }
}
