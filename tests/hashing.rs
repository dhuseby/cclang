use bytes::{ 
    BufMut,
    BytesMut
};
use cclang::{
    AppIO,
    CCLang,
    Encoding,
    Hashing,
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

/* TEST DATA
msg: fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef

SHA256: d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21

SHA512: 7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b
*/

#[test]
pub fn hashing_sha256() {
    let mut b = BytesMut::new();
    let data = hex::decode("d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21".to_string()).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected binary
        CCL::Binary(b.freeze()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::HashingId(Hashing::SHA256),
        CCL::Hash,
       
        // pop the decoded binary and the expected binary and compare
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
pub fn hashing_sha512() {
    let mut b = BytesMut::new();
    let data = hex::decode("7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b".to_string()).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected binary
        CCL::Binary(b.freeze()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::HashingId(Hashing::SHA512),
        CCL::Hash,
       
        // pop the decoded binary and the expected binary and compare
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
pub fn hashing_sha256_ser_0() {
    let script = Script::from(vec![
        // push the expected binary
        CCL::Text("d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21".to_string()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::HashingId(Hashing::SHA256),
        CCL::Hash,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21 fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE SHA256 HASH =""#);
}

#[test]
pub fn hashing_sha512_ser_0() {
    let script = Script::from(vec![
        // push the expected binary
        CCL::Text("7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b".to_string()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::HashingId(Hashing::SHA512),
        CCL::Hash,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE SHA512 HASH =""#);
}

#[test]
pub fn hashing_sha256_de_0() {
    let s1 = Script::from(vec![
        // push the expected binary
        CCL::Text("d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21".to_string()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::HashingId(Hashing::SHA256),
        CCL::Hash,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let s = r#""d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21 fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE SHA256 HASH =""#;
    let s2: Script<CCL> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn hashing_sha512_de_0() {
    let s1 = Script::from(vec![
        // push the expected binary
        CCL::Text("7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b".to_string()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::HashingId(Hashing::SHA512),
        CCL::Hash,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let s = r#""7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE SHA512 HASH =""#;
    let s2: Script<CCL> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

