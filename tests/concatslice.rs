use bytes::{
    BytesMut,
    BufMut
};
use cclang::{
    CCLang::{
        self,
        Binary,
        Boolean,
        Concat,
        Decode,
        EncodingId,
        Equal,
        Index,
        Slice,
        Text
    },
    Encoding,
    Machine,
    NullIO,
    Script
};

/* TEST DATA
msg: fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef
*/

#[test]
pub fn concat() {
    let mut b = BytesMut::new();
    let d = hex::decode("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()).unwrap();
    b.put_slice(&d);

    let script = Script::from(vec![
        // push the expected merged binary
        Binary(b.freeze()),

        // decode and push the left binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the right binary
        Text("535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        Concat,
       
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
pub fn slice() {
    let mut b = BytesMut::new();
    let d = hex::decode("fde223e5919f671b".to_string()).unwrap();
    b.put_slice(&d);

    let script = Script::from(vec![
        // push the expected merged binary
        Binary(b.freeze()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // push the begin and end
        Index(0),
        Index(8),

        Slice,
       
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
pub fn concat_ser_0() {
    let mut b = BytesMut::new();
    let d = hex::decode("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()).unwrap();
    b.put_slice(&d);

    let script = Script::from(vec![
        // push the expected merged binary
        Binary(b.freeze()),

        // decode and push the left binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the right binary
        Text("535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        Concat,
       
        // pop the decoded binary and the expected binary and compare
        Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a Hex DECODE 535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE CONCAT =""#);
}

#[test]
pub fn slice_ser_0() {
    let mut b = BytesMut::new();
    let d = hex::decode("fde223e5919f671b".to_string()).unwrap();
    b.put_slice(&d);

    let script = Script::from(vec![
        // push the expected merged binary
        Binary(b.freeze()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // push the begin and end
        Index(0),
        Index(8),

        Slice,
       
        // pop the decoded binary and the expected binary and compare
        Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""fde223e5919f671b Hex DECODE fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a Hex DECODE 0 8 SLICE =""#);
}

#[test]
pub fn concat_de_0() {
    let s1 = Script::from(vec![
        // push the expected merged binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),

        // decode and push the left binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the right binary
        Text("535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        Concat,
       
        // pop the decoded binary and the expected binary and compare
        Equal
    ]);
    let s = r#""fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a Hex DECODE 535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE CONCAT =""#;
    let s2: Script<CCLang> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn slice_de_0() {
    let s1 = Script::from(vec![
        // push the expected merged binary
        Text("fde223e5919f671b".to_string()),

        // decode and push the binary
        Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // push the begin and end
        Index(0),
        Index(8),

        Slice,
       
        // pop the decoded binary and the expected binary and compare
        Equal
    ]);
    let s = r#""fde223e5919f671b fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a Hex DECODE 0 8 SLICE =""#;
    let s2: Script<CCLang> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

