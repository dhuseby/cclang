use bytes::Bytes;
use cclang::{
    AppIO,
    Whence,
    Mode,
    CCLang,
    Machine,
    Script,
    Encoding,
    Signing
};
use std::clone::Clone;
use std::cmp::{ PartialEq, PartialOrd };

#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullHandle {
    identifier: String,
    mode: Mode
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct NullIO;

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
 * msg:
 * 7ccf1a3dd89255b11007df39110fa0e83b95030bf3b8b9113d3e0117a24770bc0bf4e61f780e949df0924ade33380dd000b42f394b9e7c0d3191d977df99e83f
 *  pk: 2eb9136429881b23cfdb02fba18422e2467ba0fa78527cf2d96c0791b2827a10
 *  sk: d2acb699a7e41806bdb3d4400a6ace771e5e6e079117fa941255014ea433e7b02eb9136429881b23cfdb02fba18422e2467ba0fa78527cf2d96c0791b2827a10
 * sig: df087999d4d9d01f97de110daf50dca0f422ebe624d20196820a0a97e49314c366dede0f4a3d869872c4d841910b14460a4c47fbb513f2bf82a7de9fc746a70b
 */

#[test]
pub fn signing_0() {
    let script = Script::from(vec![
        // decode and push the expected signature
        CCL::Text("df087999d4d9d01f97de110daf50dca0f422ebe624d20196820a0a97e49314c366dede0f4a3d869872c4d841910b14460a4c47fbb513f2bf82a7de9fc746a70b".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the message to be sign
        CCL::Text("7ccf1a3dd89255b11007df39110fa0e83b95030bf3b8b9113d3e0117a24770bc0bf4e61f780e949df0924ade33380dd000b42f394b9e7c0d3191d977df99e83f".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the secret key to sign with
        CCL::Text("d2acb699a7e41806bdb3d4400a6ace771e5e6e079117fa941255014ea433e7b02eb9136429881b23cfdb02fba18422e2467ba0fa78527cf2d96c0791b2827a10".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // push the Ed25519 signature algorithm identifier
        CCL::SigningId(Signing::Ed25519),

        // pop the identifier, secret key, and message, sign it and push the signature
        CCL::Sign,

        // pop the generated signature and the expected signature and check for equal, push a bool
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
pub fn verifying_0() {
    let script = Script::from(vec![
        // decode and push the signature
        CCL::Text("df087999d4d9d01f97de110daf50dca0f422ebe624d20196820a0a97e49314c366dede0f4a3d869872c4d841910b14460a4c47fbb513f2bf82a7de9fc746a70b".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the public key
        CCL::Text("2eb9136429881b23cfdb02fba18422e2467ba0fa78527cf2d96c0791b2827a10".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // decode and push the message
        CCL::Text("7ccf1a3dd89255b11007df39110fa0e83b95030bf3b8b9113d3e0117a24770bc0bf4e61f780e949df0924ade33380dd000b42f394b9e7c0d3191d977df99e83f".to_string()),
        CCL::EncodingId(Encoding::Hex),
        CCL::Decode,

        // push the Ed25519 signing algorithm identifier
        CCL::SigningId(Signing::Ed25519),

        // pop the identifier, message, pub key, and signature, verify and push boolean result
        CCL::Verify
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

