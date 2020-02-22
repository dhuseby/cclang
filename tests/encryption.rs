use bytes::{ BufMut, Bytes, BytesMut };
use cclang::{
    AppIO,
    Whence,
    Mode,
    CCLang,
    Machine,
    Script,
    Encoding,
    Encryption
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

/* TEST DATA:
msg: 455a8ecfd265c6e4ce63e590679a6e68b1e34b3112cdfe3e655fa47c545ae3f4f13bc066d289ec1d59eda208578d0040ad69d37411ae044583ca2c844ebcc099
key: 7e874bde68d5a1f99dc0675c22f4b94705b259b7e6033dc31e598b1f6cc330f7
nonce: a65af86b4856df7f655ff71132af566a736b91e24a11e114
box: 64a5fa3599adffef7ca387345760900d1fdb95b74b572b4ac42150f29f11105f7258e5bc135427e9f3c9b1340882de656a4fe7d789e85f9c0b9156ea8bc28692f29d0ba4991fed9daf956d174f75e058
*/

#[test]
pub fn encryption() {
    let mut b = BytesMut::new();
    let data = hex::decode("64a5fa3599adffef7ca387345760900d1fdb95b74b572b4ac42150f29f11105f7258e5bc135427e9f3c9b1340882de656a4fe7d789e85f9c0b9156ea8bc28692f29d0ba4991fed9daf956d174f75e058".to_string()).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected ciphertext binary
        CCL::Binary(b.freeze()),

        // decode and push the plaintext
        CCL::Text("455a8ecfd265c6e4ce63e590679a6e68b1e34b3112cdfe3e655fa47c545ae3f4f13bc066d289ec1d59eda208578d0040ad69d37411ae044583ca2c844ebcc099".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the key
        CCL::Text("7e874bde68d5a1f99dc0675c22f4b94705b259b7e6033dc31e598b1f6cc330f7".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the nonce
        CCL::Text("a65af86b4856df7f655ff71132af566a736b91e24a11e114".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // push the encryption algorithm id
        CCL::EncryptionId(Encryption::XSalsa20Poly1305),

        CCL::Encrypt,
       
        // pop the encrypted binary and the expected binary and compare
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
pub fn decryption() {
    let mut b = BytesMut::new();
    let data = hex::decode("455a8ecfd265c6e4ce63e590679a6e68b1e34b3112cdfe3e655fa47c545ae3f4f13bc066d289ec1d59eda208578d0040ad69d37411ae044583ca2c844ebcc099".to_string()).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected plaintext binary
        CCL::Binary(b.freeze()),

        // decode and push the ciphertext
        CCL::Text("64a5fa3599adffef7ca387345760900d1fdb95b74b572b4ac42150f29f11105f7258e5bc135427e9f3c9b1340882de656a4fe7d789e85f9c0b9156ea8bc28692f29d0ba4991fed9daf956d174f75e058".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the key
        CCL::Text("7e874bde68d5a1f99dc0675c22f4b94705b259b7e6033dc31e598b1f6cc330f7".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the nonce
        CCL::Text("a65af86b4856df7f655ff71132af566a736b91e24a11e114".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // push the encryption algorithm id
        CCL::EncryptionId(Encryption::XSalsa20Poly1305),

        CCL::Decrypt,
       
        // pop the encrypted binary and the expected binary and compare
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

