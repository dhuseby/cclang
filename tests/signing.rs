use cclang::{
    CCLang::{
        Boolean,
        Decode,
        EncodingId,
        Equal,
        Sign,
        SigningId,
        Text,
        Verify
    },
    Encoding,
    Machine,
    NullIO,
    Script,
    Signing
};

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
        Text("df087999d4d9d01f97de110daf50dca0f422ebe624d20196820a0a97e49314c366dede0f4a3d869872c4d841910b14460a4c47fbb513f2bf82a7de9fc746a70b".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the message to be sign
        Text("7ccf1a3dd89255b11007df39110fa0e83b95030bf3b8b9113d3e0117a24770bc0bf4e61f780e949df0924ade33380dd000b42f394b9e7c0d3191d977df99e83f".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the secret key to sign with
        Text("d2acb699a7e41806bdb3d4400a6ace771e5e6e079117fa941255014ea433e7b02eb9136429881b23cfdb02fba18422e2467ba0fa78527cf2d96c0791b2827a10".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // push the Ed25519 signature algorithm identifier
        SigningId(Signing::Ed25519),

        // pop the identifier, secret key, and message, sign it and push the signature
        Sign,

        // pop the generated signature and the expected signature and check for equal, push a bool
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
pub fn verifying_0() {
    let script = Script::from(vec![
        // decode and push the signature
        Text("df087999d4d9d01f97de110daf50dca0f422ebe624d20196820a0a97e49314c366dede0f4a3d869872c4d841910b14460a4c47fbb513f2bf82a7de9fc746a70b".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the public key
        Text("2eb9136429881b23cfdb02fba18422e2467ba0fa78527cf2d96c0791b2827a10".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // decode and push the message
        Text("7ccf1a3dd89255b11007df39110fa0e83b95030bf3b8b9113d3e0117a24770bc0bf4e61f780e949df0924ade33380dd000b42f394b9e7c0d3191d977df99e83f".to_string()),
        EncodingId(Encoding::Hex),
        Decode,

        // push the Ed25519 signing algorithm identifier
        SigningId(Signing::Ed25519),

        // pop the identifier, message, pub key, and signature, verify and push boolean result
        Verify
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

