use base64;
use bs58;
use bytes::{
    BufMut,
    BytesMut,
    Bytes
};
use gsm::{
    AppIO,
    Instruction,
    Machine
};
use hex;
use semver::Version;
use serde::{
    de,
    Deserialize,
    Deserializer
};
use sodiumoxide::crypto::hash::{
    sha256,
    sha512
};
use sodiumoxide::crypto::secretbox::{
    xsalsa20poly1305
};
use sodiumoxide::crypto::sign::{
    PublicKey,
    SecretKey,
    Signature,
    sign_detached,
    verify_detached
};
use std::{
    any::Any,
    cmp::Ordering,
    fmt,
    rc::Rc
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Encoding {
    Hex,
    Base64,
    Base64Url,
    Base58Bitcoin,
}

struct EncodingVisitor;

impl<'de> de::Visitor<'de> for EncodingVisitor {
    type Value = Encoding;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encoding token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let l = v.to_lowercase();
        match l.as_str() {
            "hex" => Ok(Encoding::Hex),
            "base64" => Ok(Encoding::Base64),
            "base64url" => Ok(Encoding::Base64Url),
            "base58bitcoin" => Ok(Encoding::Base58Bitcoin),
            &_ => Err(E::custom(format!("failed to parse '{}'", l)))
        }
    }
}

impl<'de> Deserialize<'de> for Encoding {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Encoding, D::Error> {
        d.deserialize_any(EncodingVisitor)
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Encoding::Hex       => write!(f, "Hex"),
            Encoding::Base64    => write!(f, "Base64"),
            Encoding::Base64Url => write!(f, "Base64Url"),
            Encoding::Base58Bitcoin => write!(f, "Base58Bitcoin")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Encryption {
    XSalsa20Poly1305
}

struct EncryptionVisitor;

impl<'de> de::Visitor<'de> for EncryptionVisitor {
    type Value = Encryption;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encryption token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let l = v.to_lowercase();
        match l.as_str() {
            "xsalsa20poly1305" => Ok(Encryption::XSalsa20Poly1305),
            &_ => Err(E::custom(format!("failed to parse '{}'", l)))
        }
    }
}

impl<'de> Deserialize<'de> for Encryption {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Encryption, D::Error> {
        d.deserialize_any(EncryptionVisitor)
    }
}

impl fmt::Display for Encryption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Encryption::XSalsa20Poly1305 => write!(f, "XSalsa20Poly1305")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Signing {
    Ed25519
}

struct SigningVisitor;

impl<'de> de::Visitor<'de> for SigningVisitor {
    type Value = Signing;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signing token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let l = v.to_lowercase();
        match l.as_str() {
            "ed25519" => Ok(Signing::Ed25519),
            &_ => Err(E::custom(format!("failed to parse '{}'", l)))
        }
    }
}

impl<'de> Deserialize<'de> for Signing {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Signing, D::Error> {
        d.deserialize_any(SigningVisitor)
    }
}

impl fmt::Display for Signing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signing::Ed25519 => write!(f, "Ed25519")
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Hashing {
    SHA256,
    SHA512,
}

struct HashingVisitor;

impl<'de> de::Visitor<'de> for HashingVisitor {
    type Value = Hashing;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hashing token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let l = v.to_lowercase();
        match l.as_str() {
            "sha256" => Ok(Hashing::SHA256),
            "sha512" => Ok(Hashing::SHA512),
            &_ => Err(E::custom(format!("failed to parse '{}'", l)))
        }
    }
}

impl<'de> Deserialize<'de> for Hashing {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Hashing, D::Error> {
        d.deserialize_any(HashingVisitor)
    }
}

impl fmt::Display for Hashing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hashing::SHA256   => write!(f, "SHA256"),
            Hashing::SHA512   => write!(f, "SHA512"),
        }
    }
}

#[derive(Clone)]
pub enum CCLang
{
    Version,

    // data types
    Boolean(bool),
    Binary(Bytes),
    Text(String),
    EncodingId(Encoding),
    EncryptionId(Encryption),
    SigningId(Signing),
    HashingId(Hashing),

    // I/O data types
    Index(isize),
    Handle(Rc<dyn Any>),
    Whence(gsm::Whence),
    Mode(gsm::Mode),

    // I/O
    Open,
    Read,
    Write,
    Seek,
    Close,

    // logical comparison
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    // logical operations
    /*
    BitOr,
    BitAnd,
    BitXor,
    BitNot,
    */

    // conversion operations
    Encode,
    Decode,

    // encryption
    Encrypt,
    Decrypt,

    // signing
    Sign,
    Verify,

    // hashing
    Hash,

    // data maniupation
    Concat,
    Slice,

    // stack operations
    Dup,
    Pop,

    // flow control
    If,
    Else,
    Fi
}

struct IfMatch {
    ifi: usize,
    elsei: Option<usize>,
    fii: usize
}

fn find_matching_elsefi(m: &Machine<CCLang>, i: usize) -> Option<IfMatch> {
    let mut ret = IfMatch { ifi: i, elsei: None, fii: 0 };
    let mut ip = ret.ifi + 1;
    loop {
        match m.geti(ip) {
            Some(CCLang::If) => {
                // this is an inner 'IF' that we need to find the end of.
                let im = match find_matching_elsefi(m, ip) {
                    Some(inner) => inner,
                    None => return None
                };

                // skip to the index just after the closing 'FI'
                ip = im.fii + 1;
            },

            Some(CCLang::Else) => {
                // this is an 'ELSE' to our starting 'IF' so we just record the
                // index in the result and move on.
                ret.elsei = Some(ip);
                ip += 1;
            },

            Some(CCLang::Fi) => {
                // we found our matching 'FI' so return the result
                ret.fii = ip;
                return Some(ret);
            },

            // skip over instructions that aren't if/else/fi
            Some(_) => {
                ip += 1;
            },

            // if we don't get an instruction we've reached the end of the
            // script without finding the matching 'FI' so return None to
            // signal the error.
            None => {
                return None;
            }
        }
    }
}

struct CCLangVisitor;

impl<'de> de::Visitor<'de> for CCLangVisitor {
    type Value = CCLang;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CCLang token")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let encodingv = EncodingVisitor;
        let encryptionv = EncryptionVisitor;
        let signingv = SigningVisitor;
        let hashingv = HashingVisitor;
        let mv = gsm::ModeVisitor;
        let wv = gsm::WhenceVisitor;

        if let Ok(e) = encodingv.visit_str::<E>(v) {
            return Ok(CCLang::EncodingId(e));
        } else if let Ok(e) = encryptionv.visit_str::<E>(v) {
            return Ok(CCLang::EncryptionId(e));
        } else if let Ok(s) = signingv.visit_str::<E>(v) {
            return Ok(CCLang::SigningId(s));
        } else if let Ok(h) = hashingv.visit_str::<E>(v) {
            return Ok(CCLang::HashingId(h));
        } else if let Ok(m) = mv.visit_str::<E>(v) {
            return Ok(CCLang::Mode(m));
        } else if let Ok(w) = wv.visit_str::<E>(v) {
            return Ok(CCLang::Whence(w));
        } else {
            let l = v.to_lowercase();
            match l.as_str() {
                "cclang" => return Ok(CCLang::Version),
                "true" => return Ok(CCLang::Boolean(true)),
                "false" => return Ok(CCLang::Boolean(false)),
                "open" => return Ok(CCLang::Open),
                "read" => return Ok(CCLang::Read),
                "write" => return Ok(CCLang::Write),
                "seek" => return Ok(CCLang::Seek),
                "close" => return Ok(CCLang::Close),
                "=" => return Ok(CCLang::Equal),
                "!=" => return Ok(CCLang::NotEqual),
                "<" => return Ok(CCLang::LessThan),
                "<=" => return Ok(CCLang::LessThanEqual),
                ">" => return Ok(CCLang::GreaterThan),
                ">=" => return Ok(CCLang::GreaterThanEqual),
                "encode" => return Ok(CCLang::Encode),
                "decode" => return Ok(CCLang::Decode),
                "encrypt" => return Ok(CCLang::Encrypt),
                "decrypt" => return Ok(CCLang::Decrypt),
                "sign" => return Ok(CCLang::Sign),
                "verify" => return Ok(CCLang::Verify),
                "hash" => return Ok(CCLang::Hash),
                "concat" => return Ok(CCLang::Concat),
                "slice" => return Ok(CCLang::Slice),
                "dup" => return Ok(CCLang::Dup),
                "pop" => return Ok(CCLang::Pop),
                "if" => return Ok(CCLang::If),
                "else" => return Ok(CCLang::Else),
                "fi" => return Ok(CCLang::Fi),
                &_ => {
                    match v.parse::<isize>() {
                        Ok(i) => return Ok(CCLang::Index(i)),
                        _ => {
                            return Ok(CCLang::Text(v.to_string()));
                        }
                    }
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for CCLang {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<CCLang, D::Error> {
        d.deserialize_any(CCLangVisitor)
    }
}

impl fmt::Debug for CCLang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CCLang::Version => write!(f, "CCLANG"),
            CCLang::Boolean(v) => write!(f, "Boolean({})", if *v { "TRUE" } else { "FALSE" }),
            CCLang::Binary(v) => write!(f, "Binary({})", hex::encode(v.as_ref())),
            CCLang::Text(v) => write!(f, "Text({})", v),
            CCLang::EncodingId(encoding) => write!(f, "EncodingId({})", encoding),
            CCLang::EncryptionId(encryption) => write!(f, "EncryptionId({})", encryption),
            CCLang::SigningId(signing) => write!(f, "SigningId({})", signing),
            CCLang::HashingId(hashing) => write!(f, "HashingId({})", hashing),
            CCLang::Index(v) => write!(f, "Index({})", v),
            CCLang::Handle(_) => write!(f, "Handle"),
            CCLang::Whence(w) => write!(f, "Whence({})", w),
            CCLang::Mode(m) => write!(f, "Mode({})", m),
            CCLang::Open => write!(f, "OPEN"),
            CCLang::Read => write!(f, "READ"),
            CCLang::Write => write!(f, "WRITE"),
            CCLang::Seek => write!(f, "SEEK"),
            CCLang::Close => write!(f, "CLOSE"),
            CCLang::Equal => write!(f, "="),
            CCLang::NotEqual => write!(f, "!="),
            CCLang::LessThan => write!(f, "<"),
            CCLang::LessThanEqual => write!(f, "<="),
            CCLang::GreaterThan => write!(f, ">"),
            CCLang::GreaterThanEqual => write!(f, ">="),
            /*
            CCLang::BitOr => write!(f, "|"),
            CCLang::BitAnd => write!(f, "&"),
            CCLang::BitXor => write!(f, "^"),
            CCLang::BitNot => write!(f, "~"),
            */
            CCLang::Encode => write!(f, "ENCODE"),
            CCLang::Decode => write!(f, "DECODE"),
            CCLang::Encrypt => write!(f, "ENCRYPT"),
            CCLang::Decrypt => write!(f, "DECRYPT"),
            CCLang::Sign => write!(f, "SIGN"),
            CCLang::Verify => write!(f, "VERIFY"),
            CCLang::Hash => write!(f, "HASH"),
            CCLang::Concat => write!(f, "CONCAT"),
            CCLang::Slice => write!(f, "SLICE"),
            CCLang::Dup => write!(f, "DUP"),
            CCLang::Pop => write!(f, "POP"),
            CCLang::If => write!(f, "IF"),
            CCLang::Else => write!(f, "ELSE"),
            CCLang::Fi => write!(f, "FI")
        }
    }
}

impl fmt::Display for CCLang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CCLang::Version => write!(f, "CCLANG"),
            CCLang::Boolean(v) => write!(f, "{}", if *v { "TRUE" } else { "FALSE" }),
            CCLang::Binary(v) => write!(f, "{} {} {}", hex::encode(v.as_ref()), CCLang::EncodingId(Encoding::Hex), CCLang::Decode),
            CCLang::Text(v) => write!(f, "{}", v),
            CCLang::EncodingId(encoding) => write!(f, "{}", encoding),
            CCLang::EncryptionId(encryption) => write!(f, "{}", encryption),
            CCLang::SigningId(signing) => write!(f, "{}", signing),
            CCLang::HashingId(hashing) => write!(f, "{}", hashing),
            CCLang::Index(v) => write!(f, "{}", v),
            CCLang::Handle(_) => panic!("cannot serialize Handle"),
            CCLang::Whence(w) => write!(f, "{}", w),
            CCLang::Mode(m) => write!(f, "{}", m),
            CCLang::Open => write!(f, "OPEN"),
            CCLang::Read => write!(f, "READ"),
            CCLang::Write => write!(f, "WRITE"),
            CCLang::Seek => write!(f, "SEEK"),
            CCLang::Close => write!(f, "CLOSE"),
            CCLang::Equal => write!(f, "="),
            CCLang::NotEqual => write!(f, "!="),
            CCLang::LessThan => write!(f, "<"),
            CCLang::LessThanEqual => write!(f, "<="),
            CCLang::GreaterThan => write!(f, ">"),
            CCLang::GreaterThanEqual => write!(f, ">="),
            /*
            CCLang::BitOr => write!(f, "|"),
            CCLang::BitAnd => write!(f, "&"),
            CCLang::BitXor => write!(f, "^"),
            CCLang::BitNot => write!(f, "~"),
            */
            CCLang::Encode => write!(f, "ENCODE"),
            CCLang::Decode => write!(f, "DECODE"),
            CCLang::Encrypt => write!(f, "ENCRYPT"),
            CCLang::Decrypt => write!(f, "DECRYPT"),
            CCLang::Sign => write!(f, "SIGN"),
            CCLang::Verify => write!(f, "VERIFY"),
            CCLang::Hash => write!(f, "HASH"),
            CCLang::Concat => write!(f, "CONCAT"),
            CCLang::Slice => write!(f, "SLICE"),
            CCLang::Dup => write!(f, "DUP"),
            CCLang::Pop => write!(f, "POP"),
            CCLang::If => write!(f, "IF"),
            CCLang::Else => write!(f, "ELSE"),
            CCLang::Fi => write!(f, "FI")
        }
    }
}

impl PartialEq for CCLang {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CCLang::Version => { match other { CCLang::Version => true, _ => false } },
            CCLang::Boolean(l) => {
                match other {
                    CCLang::Boolean(r) => *l == *r,
                    CCLang::Index(r) => {
                        let rb = *r != 0;
                        *l == rb
                    },
                    _ => false
                }
            },
            CCLang::Binary(l) => {
                match other {
                    CCLang::Binary(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Text(l) => {
                match other {
                    CCLang::Text(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::EncodingId(l) => {
                match other {
                    CCLang::EncodingId(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::EncryptionId(l) => {
                match other {
                    CCLang::EncryptionId(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::SigningId(l) => {
                match other {
                    CCLang::SigningId(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::HashingId(l) => {
                match other {
                    CCLang::HashingId(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Handle(_) => {
                true
            },
            CCLang::Index(l) => {
                match other {
                    CCLang::Boolean(r) => {
                        let lb = *l != 0;
                        lb == *r
                    },
                    CCLang::Index(r) => {
                        *l == *r
                    },
                    _ => false,
                }
            }
            CCLang::Mode(l) => {
                match other {
                    CCLang::Mode(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Whence(l) => {
                match other {
                    CCLang::Whence(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Open => { match other { CCLang::Open => true, _ => false } },
            CCLang::Read => { match other { CCLang::Read => true, _ => false } },
            CCLang::Write => { match other { CCLang::Write => true, _ => false } },
            CCLang::Seek => { match other { CCLang::Seek => true, _ => false } },
            CCLang::Close => { match other { CCLang::Close => true, _ => false } },
            CCLang::Equal => { match other { CCLang::Equal => true, _ => false } },
            CCLang::NotEqual => { match other { CCLang::NotEqual => true, _ => false } },
            CCLang::LessThan => { match other { CCLang::LessThan => true, _ => false } },
            CCLang::LessThanEqual => { match other { CCLang::LessThanEqual => true, _ => false } },
            CCLang::GreaterThan => { match other { CCLang::GreaterThan => true, _ => false } },
            CCLang::GreaterThanEqual => { match other { CCLang::GreaterThanEqual => true, _ => false } },
            /*
            CCLang::BitOr => { match other { CCLang::BitOr => true, _ => false } },
            CCLang::BitAnd => { match other { CCLang::BitAnd => true, _ => false } },
            CCLang::BitXor => { match other { CCLang::BitXor => true, _ => false } },
            CCLang::BitNot => { match other { CCLang::BitNot => true, _ => false } },
            */
            CCLang::Encode => { match other { CCLang::Encode => true, _ => false } },
            CCLang::Decode => { match other { CCLang::Decode => true, _ => false } },
            CCLang::Encrypt => { match other { CCLang::Encrypt => true, _ => false } },
            CCLang::Decrypt => { match other { CCLang::Decrypt => true, _ => false } },
            CCLang::Sign => { match other { CCLang::Sign => true, _ => false } },
            CCLang::Verify => { match other { CCLang::Verify => true, _ => false } },
            CCLang::Hash => { match other { CCLang::Hash => true, _ => false } },
            CCLang::Concat => { match other { CCLang::Concat => true, _ => false } },
            CCLang::Slice => { match other { CCLang::Slice => true, _ => false } },
            CCLang::Dup => { match other { CCLang::Dup => true, _ => false } },
            CCLang::Pop => { match other { CCLang::Pop => true, _ => false } },
            CCLang::If => { match other { CCLang::If => true, _ => false } },
            CCLang::Else => { match other { CCLang::Else => true, _ => false } },
            CCLang::Fi => { match other { CCLang::Fi => true, _ => false } }
        }
    }
}

impl PartialOrd for CCLang {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            CCLang::Boolean(l) => {
                match other {
                    CCLang::Boolean(r) => l.partial_cmp(r),
                    CCLang::Index(r) => {
                        let rb = *r != 0;
                        l.partial_cmp(&rb)
                    },
                    _ => None
                }
            },
            CCLang::Binary(l) => {
                if let CCLang::Binary(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::Text(l) => {
                if let CCLang::Text(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::EncodingId(l) => {
                if let CCLang::EncodingId(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::EncryptionId(l) => {
                if let CCLang::EncryptionId(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::SigningId(l) => {
                if let CCLang::SigningId(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::HashingId(l) => {
                if let CCLang::HashingId(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::Handle(_) => {
                Some(Ordering::Equal)
            },
            CCLang::Index(l) => {
                match other {
                    CCLang::Boolean(r) => {
                        let ri = *r as isize;
                        l.partial_cmp(&ri)
                    },
                    CCLang::Index(r) => l.partial_cmp(r),
                    _ => None
                }
            },
            CCLang::Mode(l) => {
                if let CCLang::Mode(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::Whence(l) => {
                if let CCLang::Whence(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            _ => self.partial_cmp(other)
        }
    }
}

impl Instruction<CCLang> for CCLang {
    fn execute(&self, ip: usize, m: &mut Machine<CCLang>, io: &dyn AppIO<CCLang>) {
        match self {
            CCLang::Handle(_) => panic!(),
            CCLang::Version => {
                if let Some(CCLang::Text(s)) = m.pop() {
                    if let Ok(v) = Version::parse(&s) {
                        m.push(CCLang::Boolean(m.version_check(&v)));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!();
            },
            CCLang::Boolean(_) |
            CCLang::Binary(_) |
            CCLang::Text(_) |
            CCLang::EncodingId(_) |
            CCLang::EncryptionId(_) |
            CCLang::SigningId(_) |
            CCLang::HashingId(_) |
            CCLang::Index(_) |
            CCLang::Whence(_) |
            CCLang::Mode(_) => {
                // just push the immediate constant onto the stack
                m.push(self.clone());
                m.pushr(ip + 1);
            },
            CCLang::Equal => {
                if let Some(right) = m.pop() {
                    if let Some(left) = m.pop() {
                        m.push(CCLang::Boolean(left == right));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!();
            },
            CCLang::NotEqual => {
                if let Some(right) = m.pop() {
                    if let Some(left) = m.pop() {
                        m.push(CCLang::Boolean(left != right));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!();
            },
            CCLang::LessThan => {
                if let Some(right) = m.pop() {
                    if let Some(left) = m.pop() {
                        m.push(CCLang::Boolean(left < right));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!();
            },
            CCLang::LessThanEqual => {
                if let Some(right) = m.pop() {
                    if let Some(left) = m.pop() {
                        m.push(CCLang::Boolean(left <= right));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!();
            }
            CCLang::GreaterThan => {
                if let Some(right) = m.pop() {
                    if let Some(left) = m.pop() {
                        m.push(CCLang::Boolean(left > right));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!();
            },
            CCLang::GreaterThanEqual => {
                if let Some(right) = m.pop() {
                    if let Some(left) = m.pop() {
                        m.push(CCLang::Boolean(left >= right));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!();
            },
            /*
            CCLang::BitOr =>
            CCLang::BitAnd =>
            CCLang::BitXor =>
            CCLang::BitNot =>
            */
            CCLang::Decode => {
                if let Some(CCLang::EncodingId(id)) = m.pop() {
                    if let Some(CCLang::Text(s)) = m.pop() {
                        let mut b = BytesMut::new();
                        let data = match id {
                            Encoding::Hex => hex::decode(s).unwrap(),
                            Encoding::Base64 => base64::decode_config(&s, base64::STANDARD).unwrap(),
                            Encoding::Base64Url => base64::decode_config(&s, base64::URL_SAFE).unwrap(),
                            Encoding::Base58Bitcoin => bs58::decode(s).into_vec().unwrap()
                        };
                        b.put_slice(&data);
                        m.push(CCLang::Binary(b.freeze()));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!()
            },
            CCLang::Encode => {
                if let Some(CCLang::EncodingId(id)) = m.pop() {
                    if let Some(CCLang::Binary(b)) = m.pop() {
                        let s = match id {
                            Encoding::Hex => hex::encode(b.as_ref()),
                            Encoding::Base64 => base64::encode_config(b.as_ref(), base64::STANDARD),
                            Encoding::Base64Url => base64::encode_config(b.as_ref(), base64::URL_SAFE),
                            Encoding::Base58Bitcoin => bs58::encode(b.as_ref()).into_string(),
                        };
                        m.push(CCLang::Text(s));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!()
            },
            CCLang::Encrypt => {
                if let Some(CCLang::EncryptionId(id)) = m.pop() {
                    match id {
                        Encryption::XSalsa20Poly1305 => {
                            if let Some(CCLang::Binary(n)) = m.pop() {
                                if let Some(CCLang::Binary(k)) = m.pop() {
                                    if let Some(CCLang::Binary(plaintext)) = m.pop() {
                                        let nonce = xsalsa20poly1305::Nonce::from_slice(n.as_ref()).unwrap();
                                        let key = xsalsa20poly1305::Key::from_slice(k.as_ref()).unwrap();
                                        let mut b = BytesMut::new();
                                        let ciphertext = xsalsa20poly1305::seal(plaintext.as_ref(), &nonce, &key);
                                        b.put_slice(&ciphertext);
                                        m.push(CCLang::Binary(b.freeze()));
                                        m.pushr(ip + 1);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
                panic!()
            },
            CCLang::Decrypt => {
                if let Some(CCLang::EncryptionId(id)) = m.pop() {
                    match id {
                        Encryption::XSalsa20Poly1305 => {
                            if let Some(CCLang::Binary(n)) = m.pop() {
                                if let Some(CCLang::Binary(k)) = m.pop() {
                                    if let Some(CCLang::Binary(ciphertext)) = m.pop() {
                                        let nonce = xsalsa20poly1305::Nonce::from_slice(n.as_ref()).unwrap();
                                        let key = xsalsa20poly1305::Key::from_slice(k.as_ref()).unwrap();
                                        let mut b = BytesMut::new();
                                        if let Ok(plaintext) = xsalsa20poly1305::open(ciphertext.as_ref(), &nonce, &key) {
                                            b.put_slice(&plaintext);
                                            m.push(CCLang::Binary(b.freeze()));
                                            m.pushr(ip + 1);
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                panic!()
            },
            CCLang::Sign => {
                if let Some(CCLang::SigningId(id)) = m.pop() {
                    match id {
                        Signing::Ed25519 => {
                            if let Some(CCLang::Binary(sk)) = m.pop() {
                                if let Some(CCLang::Binary(msg)) = m.pop() {
                                    let seckey = SecretKey::from_slice(sk.as_ref()).unwrap();
                                    let sig = sign_detached(&msg, &seckey);
                                    let mut signature = BytesMut::new();
                                    signature.put_slice(sig.as_ref());
                                    m.push(CCLang::Binary(signature.freeze()));
                                    m.pushr(ip + 1);
                                    return;
                                }
                            }
                        }
                    }
                }
                panic!()
            },
            CCLang::Verify => {
                if let Some(CCLang::SigningId(id)) = m.pop() {
                    match id {
                        Signing::Ed25519 => {
                            if let Some(CCLang::Binary(msg)) = m.pop() {
                                if let Some(CCLang::Binary(pk)) = m.pop() {
                                    if let Some(CCLang::Binary(sig)) = m.pop() {
                                        let signature = Signature::from_slice(sig.as_ref()).unwrap();
                                        let pubkey = PublicKey::from_slice(pk.as_ref()).unwrap();
                                        m.push(CCLang::Boolean(verify_detached(&signature, &msg, &pubkey)));
                                        m.pushr(ip + 1);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
                panic!()
            },
            CCLang::Hash => {
                if let Some(CCLang::HashingId(id)) = m.pop() {
                    if let Some(CCLang::Binary(b)) = m.pop() {
                        let mut buf = BytesMut::new();
                        match id {
                            Hashing::SHA256 => buf.put_slice(sha256::hash(b.as_ref()).as_ref()),
                            Hashing::SHA512 => buf.put_slice(sha512::hash(b.as_ref()).as_ref())
                        }
                        m.push(CCLang::Binary(buf.freeze()));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!()
            },
            CCLang::Concat => {
                if let Some(CCLang::Binary(br)) = m.pop() {
                    if let Some(CCLang::Binary(bl)) = m.pop() {
                        let mut buf = BytesMut::new();
                        buf.extend_from_slice(&bl);
                        buf.extend_from_slice(&br);
                        m.push(CCLang::Binary(buf.freeze()));
                        m.pushr(ip + 1);
                        return;
                    }
                }
                panic!()
            },
            CCLang::Slice => {
                if let Some(CCLang::Index(end)) = m.pop() {
                    if let Some(CCLang::Index(begin)) = m.pop() {
                        if let Some(CCLang::Binary(b)) = m.pop() {
                            m.push(CCLang::Binary(b.slice(begin as usize..end as usize)));
                            m.pushr(ip + 1);
                            return;
                        }
                    }
                }
                panic!()
            },
            CCLang::Open => {
                if let Err(e) = io.open(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            CCLang::Read => {
                if let Err(e) = io.read(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            CCLang::Write => {
                if let Err(e) = io.write(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            CCLang::Seek => {
                if let Err(e) = io.seek(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            CCLang::Close => {
                if let Err(e) = io.close(m) {
                    panic!(format!("{}", e));
                }
                m.pushr(ip + 1);
            },
            CCLang::Dup => {
                if let Some(top) = m.pop() {
                    m.push(top.clone());
                    m.push(top.clone());
                    m.pushr(ip + 1);
                    return;
                }
                panic!();
            },
            CCLang::Pop => {
                if let Some(_) = m.pop() {
                    m.pushr(ip + 1);
                    return;
                }
                panic!();
            },
            CCLang::If => {
                // find the location of the matching 'ELSE' if any and 'FI'
                if let Some(ifm) = find_matching_elsefi(m, ip) {
                    // get the Boolean from the stack
                    if let Some(CCLang::Boolean(b)) = m.pop() {
                        if b {
                            // the boolean is true so continue with the code that is
                            // between this if and it's matching 'ELSE'

                            // first record where we need to go after this block
                            m.pushr(ifm.fii + 1);

                            // then tell the machine the correct next instruction
                            m.pushr(ip + 1);
                            return;
                        } else {
                            // the boolean is false so skip to the instruction after
                            // the 'ELSE' if there is one, otherwise skip to after the
                            // 'FI'
                            let next_ip = match ifm.elsei {
                                Some(i) => {
                                    // we're executing the 'ELSE' block so we need to
                                    // push a frame with the correct next instruction
                                    m.pushr(ifm.fii + 1);

                                    // set the next instruction pointer to the
                                    // instruction after the 'ELSE'
                                    i + 1
                                },

                                // No 'ELSE' clause so just skip to the instruction
                                // after the 'FI'. There is no need to record a frame.
                                None => ifm.fii + 1
                            };

                            m.pushr(next_ip);
                            return;
                        }
                    }
                }
                panic!()
            },
            CCLang::Else => {
                // we see an 'ELSE' so this can only be because we previously
                // encoutered in 'IF' and the boolean was true and the
                // if/else/fi block had an else. the right thing to do here is
                // to pop the frame from the machine and skip to the next
                // instruction pointer.
                if let Some(next_ip) = m.popr() {
                    m.pushr(next_ip);
                    return;
                }
                panic!();
            },
            CCLang::Fi => {
                // we finished executing an 'IF' or 'ELSE' block so pop the
                // frame and continue
                if let Some(next_ip) = m.popr() {
                    m.pushr(next_ip);
                    return;
                }
                panic!();
            }
        }
    }
}
