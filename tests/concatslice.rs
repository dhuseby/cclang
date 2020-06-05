use bytes::{ BytesMut, BufMut };
use cclang::{
    AppIO,
    CCLang,
    Machine,
    Script,
    Encoding
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
*/

#[test]
pub fn concat() {
    let mut b = BytesMut::new();
    let d = hex::decode("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()).unwrap();
    b.put_slice(&d);

    let script = Script::from(vec![
        // push the expected merged binary
        CCL::Binary(b.freeze()),

        // decode and push the left binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the right binary
        CCL::Text("535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::Concat,
       
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
pub fn slice() {
    let mut b = BytesMut::new();
    let d = hex::decode("fde223e5919f671b".to_string()).unwrap();
    b.put_slice(&d);

    let script = Script::from(vec![
        // push the expected merged binary
        CCL::Binary(b.freeze()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // push the begin and end
        CCL::Index(0),
        CCL::Index(8),

        CCL::Slice,
       
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
pub fn concat_ser_0() {
    let mut b = BytesMut::new();
    let d = hex::decode("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()).unwrap();
    b.put_slice(&d);

    let script = Script::from(vec![
        // push the expected merged binary
        CCL::Binary(b.freeze()),

        // decode and push the left binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the right binary
        CCL::Text("535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::Concat,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a Hex DECODE 535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE CONCAT =""#);
}

#[test]
pub fn slice_ser_0() {
    let mut b = BytesMut::new();
    let d = hex::decode("fde223e5919f671b".to_string()).unwrap();
    b.put_slice(&d);

    let script = Script::from(vec![
        // push the expected merged binary
        CCL::Binary(b.freeze()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // push the begin and end
        CCL::Index(0),
        CCL::Index(8),

        CCL::Slice,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let s = serde_json::to_string(&script).unwrap();
    assert_eq!(s, r#""fde223e5919f671b fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a Hex DECODE 0 8 SLICE =""#);
}

#[test]
pub fn concat_de_0() {
    let s1 = Script::from(vec![
        // push the expected merged binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),

        // decode and push the left binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the right binary
        CCL::Text("535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        CCL::Concat,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let s = r#""fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a Hex DECODE 535a7e5315bf77a980b760d80de4e1a0c20487485cd7f7274480a4f3269aa9ef Hex DECODE CONCAT =""#;
    let s2: Script<CCL> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

#[test]
pub fn slice_de_0() {
    let s1 = Script::from(vec![
        // push the expected merged binary
        CCL::Text("fde223e5919f671b".to_string()),

        // decode and push the binary
        CCL::Text("fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // push the begin and end
        CCL::Index(0),
        CCL::Index(8),

        CCL::Slice,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let s = r#""fde223e5919f671b fde223e5919f671b0423ae3fa39f3f91992066b7f134323fbda965f7b903080a Hex DECODE 0 8 SLICE =""#;
    let s2: Script<CCL> = serde_json::from_str(s).unwrap();
    assert_eq!(s1, s2);
}

