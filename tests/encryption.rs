use bytes::{
    BytesMut,
    BufMut
};
use cclang::{
    CCLang::{
        Binary,
        Boolean,
        Decode,
        Decrypt,
        EncodingId,
        Encrypt,
        EncryptionId,
        Equal,
        Text
    },
    Encoding,
    Encryption,
    Machine,
    NullIO,
    Script
};

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
        Binary(b.freeze()),

        // decode and push the plaintext
        Text("455a8ecfd265c6e4ce63e590679a6e68b1e34b3112cdfe3e655fa47c545ae3f4f13bc066d289ec1d59eda208578d0040ad69d37411ae044583ca2c844ebcc099".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the key
        Text("7e874bde68d5a1f99dc0675c22f4b94705b259b7e6033dc31e598b1f6cc330f7".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the nonce
        Text("a65af86b4856df7f655ff71132af566a736b91e24a11e114".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // push the encryption algorithm id
        EncryptionId(Encryption::XSalsa20Poly1305),

        Encrypt,
       
        // pop the encrypted binary and the expected binary and compare
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
pub fn decryption() {
    let mut b = BytesMut::new();
    let data = hex::decode("455a8ecfd265c6e4ce63e590679a6e68b1e34b3112cdfe3e655fa47c545ae3f4f13bc066d289ec1d59eda208578d0040ad69d37411ae044583ca2c844ebcc099".to_string()).unwrap();
    b.put_slice(&data);

    let script = Script::from(vec![
        // push the expected plaintext binary
        Binary(b.freeze()),

        // decode and push the ciphertext
        Text("64a5fa3599adffef7ca387345760900d1fdb95b74b572b4ac42150f29f11105f7258e5bc135427e9f3c9b1340882de656a4fe7d789e85f9c0b9156ea8bc28692f29d0ba4991fed9daf956d174f75e058".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the key
        Text("7e874bde68d5a1f99dc0675c22f4b94705b259b7e6033dc31e598b1f6cc330f7".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the nonce
        Text("a65af86b4856df7f655ff71132af566a736b91e24a11e114".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // push the encryption algorithm id
        EncryptionId(Encryption::XSalsa20Poly1305),

        Decrypt,
       
        // pop the encrypted binary and the expected binary and compare
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

