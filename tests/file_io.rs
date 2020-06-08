use bytes::{
    BytesMut,
    BufMut
};
use cclang::{
    CCLang::{
        self,
        Binary,
        Close,
        Handle,
        Index,
        Mode,
        Open,
        Read,
        Seek,
        Text,
        Whence,
        Write
    },
    Machine,
    FileIO,
    Script
};
use gsm;
use hex;
use rand;
use std::{
    fs,
    str::FromStr
};

#[test]
fn open_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Text("LICENSE".to_string()),
        Mode(gsm::Mode::from_str("r").unwrap()),
        Open
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an IOHandle
    match result.pop() {
        Some(Handle(_)) => {},
        _ => panic!()
    }
}

#[test]
fn open_close_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Text("LICENSE".to_string()),
        Mode(gsm::Mode::from_str("r").unwrap()),
        Open,
        Close
    ]);
    let mut machine = Machine::from(script);
    let result = machine.execute(&FileIO).unwrap();

    // there shouldn't be anything on the stack
    assert_eq!(result.size(), 0 as usize);
}

#[test]
fn read_text_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Text("LICENSE".to_string()),
        Mode(gsm::Mode::from_str("r").unwrap()),
        Open,
        Index(128),
        Read,
        Close
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an Text of length 128
    match result.pop() {
        Some(Text(s)) => {
            assert_eq!(s.len(), 128);
        },
        _ => panic!()
    }
}

#[test]
fn read_binary_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Text("LICENSE".to_string()),
        Mode(gsm::Mode::from_str("rb").unwrap()),
        Open,
        Index(128),
        Read,
        Close
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an Binary of length 128
    match result.pop() {
        Some(Binary(b)) => {
            assert_eq!(b.len(), 128);
        },
        _ => panic!()
    }
}

#[test]
fn seek_text_file() {
    // construct the script and load it into the machine
    let script = Script::from(vec![
        Text("LICENSE".to_string()),
        Mode(gsm::Mode::from_str("r").unwrap()),
        Open,
        Index(-31),
        Whence(gsm::Whence::End),
        Seek,
        Index(11),
        Read,
        Close
    ]);
    let mut machine = Machine::from(script);
    let mut result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 1 as usize);

    // the item on the stack should be an Text of length 128
    match result.pop() {
        Some(Text(s)) => {
            assert_eq!(s.len(), 11);
            assert_eq!(s.as_str(), "limitations");
        },
        _ => panic!()
    }
}

#[test]
fn write_text_file() {
    let x = rand::random::<u16>();
    let fname = format!("test-{}.txt", x);
    let data = "When in the Course of human events...".to_string();
    let len = data.len() as u64;

    // construct the script and load it into the machine
    let script = Script::from(vec![
        Text(fname.clone()),
        Mode(gsm::Mode::from_str("w").unwrap()),
        Open,
        Text(data),
        Write,
        Close
    ]);
    let mut machine = Machine::from(script);
    let result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 0 as usize);

    let meta = fs::metadata(&fname).unwrap();
    assert!(meta.is_file());
    assert_eq!(meta.len(), len);
    fs::remove_file(&fname).unwrap();
}

#[test]
fn write_binary_file() {
    let x = rand::random::<u16>();
    let fname = format!("test-{}.bin", x);
    let mut b = BytesMut::new();
    let data = hex::decode("0adb80d2fc4d74adb99059a596ba21706dada1e29fd855a664ce815f88e6b169").unwrap();
    let len = data.len() as u64;
    b.put(data.as_ref());

    // construct the script and load it into the machine
    let script = Script::from(vec![
        Text(fname.clone()),
        Mode(gsm::Mode::from_str("wb").unwrap()),
        Open,
        Binary(b.freeze()),
        Write,
        Close
    ]);
    let mut machine = Machine::from(script);
    let result = machine.execute(&FileIO).unwrap();

    // there should only be one item on the stack
    assert_eq!(result.size(), 0 as usize);

    let meta = fs::metadata(&fname).unwrap();
    assert!(meta.is_file());
    assert_eq!(meta.len(), len);
    fs::remove_file(&fname).unwrap();
}

#[test]
fn de_0() {
    let x = rand::random::<u16>();
    let fname = format!("script-{}.txt", x);

    let s1 = Script::from(vec![
        Text(fname.clone()),
        Mode(gsm::Mode::from_str("w").unwrap()),
        Open,
        Text("blah".to_string()),
        Write,
        Close
    ]);
    let s = format!("\"{} w OPEN blah WRITE CLOSE\"", &fname);
    let s2: Script<CCLang> = serde_json::from_str(&s).unwrap();

    assert_eq!(s1, s2);

    let mut machine = Machine::from(s2);
    let result = machine.execute(&FileIO).unwrap();

    // the stack should be empty
    assert_eq!(result.size(), 0 as usize);

    let meta = fs::metadata(&fname).unwrap();
    assert!(meta.is_file());
    assert_eq!(meta.len(), 4);
    fs::remove_file(&fname).unwrap();
}

#[test]
fn ser_0() {
    let x = rand::random::<u16>();
    let fname = format!("script-{}.txt", x);
    
    let script = Script::from(vec![
        Text(fname.clone()),
        Mode(gsm::Mode::from_str("w").unwrap()),
        Open,
        Text("blah".to_string()),
        Write,
        Close
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, format!("\"{} w OPEN blah WRITE CLOSE\"", &fname));

    let mut machine = Machine::from(script);
    let result = machine.execute(&FileIO).unwrap();

    // the stack should be empty
    assert_eq!(result.size(), 0 as usize);

    let meta = fs::metadata(&fname).unwrap();
    assert!(meta.is_file());
    assert_eq!(meta.len(), 4);
    fs::remove_file(&fname).unwrap();
}
