use bytes::{ 
    BufMut,
    BytesMut
};
use cclang::{
    CCLang::{
        self,
        Binary,
        Boolean,
        Decode,
        EncodingId,
        Equal,
        Hash,
        HashingId,
        Text
    },
    Encoding,
    Hashing,
    Machine,
    NullIO,
    Script
};

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
        Binary(b.freeze()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        HashingId(Hashing::SHA256),
        Hash,
       
        // pop the decoded binary and the expected binary and compare
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
pub fn hashing_sha512() {
    let mut b = BytesMut::new();
    let data = hex::decode("7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b".to_string()).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected binary
        Binary(b.freeze()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        HashingId(Hashing::SHA512),
        Hash,
       
        // pop the decoded binary and the expected binary and compare
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
pub fn hashing_sha256_ser_0() {
    let script = Script::from(vec![
        // push the expected binary
        Text("d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21".to_string()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        HashingId(Hashing::SHA256),
        Hash,
       
        // pop the decoded binary and the expected binary and compare
        Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21 fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE SHA256 HASH =""#);
}

#[test]
pub fn hashing_sha512_ser_0() {
    let script = Script::from(vec![
        // push the expected binary
        Text("7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b".to_string()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        HashingId(Hashing::SHA512),
        Hash,
       
        // pop the decoded binary and the expected binary and compare
        Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE SHA512 HASH =""#);
}

#[test]
pub fn hashing_sha256_de_0() {
    let s1 = Script::from(vec![
        // push the expected binary
        Text("d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21".to_string()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        HashingId(Hashing::SHA256),
        Hash,
       
        // pop the decoded binary and the expected binary and compare
        Equal
    ]);
    let s = r#""d19242361d4e1faacb8f7561b7fc2eaf02b09bb9a449377d944a0e0142851b21 fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE SHA256 HASH =""#;
    let s2: Script<CCLang> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn hashing_sha512_de_0() {
    let s1 = Script::from(vec![
        // push the expected binary
        Text("7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b".to_string()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        HashingId(Hashing::SHA512),
        Hash,
       
        // pop the decoded binary and the expected binary and compare
        Equal
    ]);
    let s = r#""7ccd257b67b0ec6b68a68640575494cfec9792ade654fbb4f8fddf05c80bc183eff14c0056e9db0d52faf03aca9c671c63147bf6c8e8ef8beb75548ed7409c5b fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE SHA512 HASH =""#;
    let s2: Script<CCLang> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

