use bytes::{
    BufMut,
    BytesMut
};
use cclang::{
    CCLang::{
        Binary,
        Boolean,
        Decode,
        Encode,
        EncodingId,
        Equal,
        Text
    },
    Encoding,
    Machine,
    NullIO,
    Script
};

/* TEST DATA:
Hex:
msg: 8ada5f1dd1f839d1ce59adba31958d01a27480071d2ea624dff82f12681dcbd33c8bd88bee78d339f247b3c1269e4e71dc5daaa960364ebb08475f4b0a0a00ac
 pk: 0adb80d2fc4d74adb99059a596ba21706dada1e29fd855a664ce815f88e6b169
 sk: a8f595ca1dbc7b755257da0846fcd6beaabc001dc28a6cf1d01188098eaff6e20adb80d2fc4d74adb99059a596ba21706dada1e29fd855a664ce815f88e6b169
sig: a49b8805fd8230a4782ea1fad65945c7edc603aad9c6155d21f84aff501d82248c2f575d2406fcfeacb40d89e567dba15a3dc51a81d7ee98c625c6319facf509

Base64:
msg: itpfHdH4OdHOWa26MZWNAaJ0gAcdLqYk3/gvEmgdy9M8i9iL7njTOfJHs8Emnk5x3F2qqWA2TrsIR19LCgoArA==
 pk: CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=
 sk: qPWVyh28e3VSV9oIRvzWvqq8AB3Cimzx0BGICY6v9uIK24DS/E10rbmQWaWWuiFwba2h4p/YVaZkzoFfiOaxaQ==
sig: pJuIBf2CMKR4LqH61llFx+3GA6rZxhVdIfhK/1AdgiSML1ddJAb8/qy0DYnlZ9uhWj3FGoHX7pjGJcYxn6z1CQ==

Base64Url:
msg: itpfHdH4OdHOWa26MZWNAaJ0gAcdLqYk3_gvEmgdy9M8i9iL7njTOfJHs8Emnk5x3F2qqWA2TrsIR19LCgoArA==
 pk: CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=
 sk: qPWVyh28e3VSV9oIRvzWvqq8AB3Cimzx0BGICY6v9uIK24DS_E10rbmQWaWWuiFwba2h4p_YVaZkzoFfiOaxaQ==
sig: pJuIBf2CMKR4LqH61llFx-3GA6rZxhVdIfhK_1AdgiSML1ddJAb8_qy0DYnlZ9uhWj3FGoHX7pjGJcYxn6z1CQ==

Base58Bitcoin:
msg: 3n1re8FXBfyWATGbb6ri76dBQd3SH3VtzSyqrwXeXjxziz69mEdSKAa8MLJ31v6XHeaSao3Dq41TnrpkQis8q56B
 pk: jPCzTz1V1QBgR1JxyxWQKwiSkjvSxaQsoVQBNFke7YL
 sk: 4NvijY3mSxxHjAdrrGzh21ryNSVByzDuVwgV6ETqMhQtFs1zbmCYPa3PSiHSj12dtGiDFtPb39eAP4mYJVQ2C9Rr
sig: 4Ht2qwApkdkkdvXJSrFpe5vT3HBGWG2Zr3ZgWoyJ4XhQmCWccyEJZdNoQjjktpPo9SYy8Y45gaFNwTKAwJ9tQZix
*/

#[test]
pub fn decoding_hex() {
    let mut b = BytesMut::new();
    let data = hex::decode("0adb80d2fc4d74adb99059a596ba21706dada1e29fd855a664ce815f88e6b169".to_string()).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected binary
        Binary(b.freeze()),

        // decode and push the binary
        Text("0adb80d2fc4d74adb99059a596ba21706dada1e29fd855a664ce815f88e6b169".to_string()),
        EncodingId(Encoding::Hex),
        Decode,
       
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
pub fn decoding_base64() {
    let mut b = BytesMut::new();
    let data = base64::decode_config(&"CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=".to_string(), base64::STANDARD).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected binary
        Binary(b.freeze()),

        // decode and push the binary
        Text("CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=".to_string()),
        EncodingId(Encoding::Base64),
        Decode,
       
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
pub fn decoding_base64url() {
    let mut b = BytesMut::new();
    let data = base64::decode_config(&"CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=".to_string(), base64::URL_SAFE).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected binary
        Binary(b.freeze()),

        // decode and push the binary
        Text("CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=".to_string()),
        EncodingId(Encoding::Base64Url),
        Decode,
       
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
pub fn decoding_base58bitcoin() {
    let mut b = BytesMut::new();
    let data = bs58::decode("jPCzTz1V1QBgR1JxyxWQKwiSkjvSxaQsoVQBNFke7YL".to_string()).into_vec().unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected binary
        Binary(b.freeze()),

        // decode and push the binary
        Text("jPCzTz1V1QBgR1JxyxWQKwiSkjvSxaQsoVQBNFke7YL".to_string()),
        EncodingId(Encoding::Base58Bitcoin),
        Decode,
       
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
pub fn encoding_hex() {
    let mut b = BytesMut::new();
    let data = hex::decode("0adb80d2fc4d74adb99059a596ba21706dada1e29fd855a664ce815f88e6b169".to_string()).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected string
        Text("0adb80d2fc4d74adb99059a596ba21706dada1e29fd855a664ce815f88e6b169".to_string()),

        // encode and push the string
        Binary(b.freeze()),
        EncodingId(Encoding::Hex),
        Encode,
       
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
pub fn encoding_base64() {
    let mut b = BytesMut::new();
    let data = base64::decode_config(&"CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=".to_string(), base64::STANDARD).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected string
        Text("CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=".to_string()),

        // encode and push the string
        Binary(b.freeze()),
        EncodingId(Encoding::Base64),
        Encode,
       
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
pub fn encoding_base64url() {
    let mut b = BytesMut::new();
    let data = base64::decode_config(&"CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=".to_string(), base64::URL_SAFE).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected string
        Text("CtuA0vxNdK25kFmllrohcG2toeKf2FWmZM6BX4jmsWk=".to_string()),

        // encode and push the string
        Binary(b.freeze()),
        EncodingId(Encoding::Base64Url),
        Encode,
       
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
pub fn encoding_base58bitcoin() {
    let mut b = BytesMut::new();
    let data = bs58::decode("jPCzTz1V1QBgR1JxyxWQKwiSkjvSxaQsoVQBNFke7YL".to_string()).into_vec().unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected string
        Text("jPCzTz1V1QBgR1JxyxWQKwiSkjvSxaQsoVQBNFke7YL".to_string()),

        // encode and push the string
        Binary(b.freeze()),
        EncodingId(Encoding::Base58Bitcoin),
        Encode,
       
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

