use bytes::{ BufMut, Bytes, BytesMut };
use cclang::{
    AppIO,
    Whence,
    Mode,
    CCLang,
    Machine,
    Script,
    Encoding,
};
use std::clone::Clone;
use std::cmp::{ PartialEq, PartialOrd };
use std::fmt;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullHandle {
    identifier: String,
    mode: Mode
}

impl fmt::Display for NullHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "NullHandle")
    }
}


#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullIO;

impl fmt::Display for NullIO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "NullIO")
    }
}

impl AppIO<NullHandle, String> for NullIO {
    fn open(&self, id: &String, mode: Mode) -> NullHandle {
        NullHandle {
            identifier: id.to_string(),
            mode: mode
        }
    }

    fn read(&self, _h: &NullHandle, _num: usize) -> Bytes {
        Bytes::new()
    }

    fn write(&self, _h: &NullHandle, _data: &Bytes) -> usize {
        0
    }

    fn seek(&self, _h: &NullHandle, _whence: Whence, _num: isize) {}
    fn close(&self, _h: &NullHandle) {}
}

type CCL = CCLang<NullHandle, String, NullIO>;

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
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

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
        CCL::IOIndex(0),
        CCL::IOIndex(8),

        CCL::Slice,
       
        // pop the decoded binary and the expected binary and compare
        CCL::Equal
    ]);
    let mut machine = Machine::<CCL>::from(&script);
    let mut result = machine.execute().unwrap();

    // should only be one item left on the stack
    assert_eq!(result.size(), 1 as usize);

    // the result should be a boolean with the value true
    match result.pop() {
        Some(CCL::Boolean(b)) => assert_eq!(b, true),
        _ => panic!()
    }
}

