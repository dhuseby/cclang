use base64;
use bs58;
use bytes::{ BufMut, BytesMut, Bytes };
use crate::appio::{
    AppIO,
    Mode,
    Whence
};
use gsm::{
    Instruction,
    Machine,
    Stack
};
use hex;
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
use std::clone::Clone;
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Encoding {
    Hex,
    Base64,
    Base64Url,
    Base58Bitcoin,
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Encryption {
    XSalsa20Poly1305
}

impl fmt::Display for Encryption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Encryption::XSalsa20Poly1305 => write!(f, "XSalsa20Poly1305")
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Signing {
    Ed25519
}

impl fmt::Display for Signing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signing::Ed25519 => write!(f, "Ed25519")
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Hashing {
    SHA256,
    SHA512,
}

impl fmt::Display for Hashing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hashing::SHA256   => write!(f, "SHA256"),
            Hashing::SHA512   => write!(f, "SHA512"),
        }
    }
}

pub enum CCLang<T, U, V>
    where T: Clone + PartialEq + PartialOrd,
          U: Clone + PartialEq + PartialOrd,
          V: AppIO<T, U> + Clone + PartialEq + PartialOrd
{

    // data types
    Boolean(bool),
    Binary(Bytes),
    Text(String),
    EncodingId(Encoding),
    EncryptionId(Encryption),
    SigningId(Signing),
    HashingId(Hashing),
    IOHandle(T),
    IOIdentifier(U),
    IOIndex(isize),
    IOMode(Mode),
    IOWhence(Whence),

    // I/O
    Open(V),
    Read(V),
    Write(V),
    Seek(V),
    Close(V),

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

impl<T, U, V> CCLang<T, U, V>
    where T: Clone + PartialEq + PartialOrd,
          U: Clone + PartialEq + PartialOrd,
          V: AppIO<T, U> + Clone + PartialEq + PartialOrd,
{
    fn find_matching_elsefi(&self, m: &Machine<CCLang<T, U, V>>, i: usize) -> Option<IfMatch> {
        let mut ret = IfMatch { ifi: i, elsei: None, fii: 0 };
        let mut ip = ret.ifi + 1;
        loop {
            match m.get_instruction(ip) {
                Some(CCLang::If) => {
                    // this is an inner 'IF' that we need to find the end of.
                    let im = match self.find_matching_elsefi(m, ip) {
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
}

impl<T, U, V> fmt::Display for CCLang<T, U, V>
    where T: Clone + PartialEq + PartialOrd + fmt::Display,
          U: Clone + PartialEq + PartialOrd + fmt::Display,
          V: AppIO<T, U> + Clone + PartialEq + PartialOrd + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CCLang::Boolean(v) => write!(f, "{}", if *v { "TRUE" } else { "FALSE" }),
            CCLang::Binary(v) => write!(f, "{}", hex::encode(v.as_ref())),
            CCLang::Text(v) => write!(f, "{}", v),
            CCLang::EncodingId(encoding) => write!(f, "{}", encoding),
            CCLang::EncryptionId(encryption) => write!(f, "{}", encryption),
            CCLang::SigningId(signing) => write!(f, "{}", signing),
            CCLang::HashingId(hashing) => write!(f, "{}", hashing),
            CCLang::IOHandle(handle) => write!(f, "{}", handle),
            CCLang::IOIdentifier(id) => write!(f, "\"{}\"", id),
            CCLang::IOIndex(v) => write!(f, "{}", v),
            CCLang::IOMode(mode) => write!(f, "{}", mode),
            CCLang::IOWhence(whence) => write!(f, "{}", whence),
            CCLang::Open(_) => write!(f, "OPEN"),
            CCLang::Read(_) => write!(f, "READ"),
            CCLang::Write(_) => write!(f, "WRITE"),
            CCLang::Seek(_) => write!(f, "SEEK"),
            CCLang::Close(_) => write!(f, "CLOSE"),
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


impl<T, U, V> Clone for CCLang<T, U, V>
    where T: Clone + PartialEq + PartialOrd,
          U: Clone + PartialEq + PartialOrd,
          V: AppIO<T, U> + Clone + PartialEq + PartialOrd
{
    fn clone(&self) -> CCLang<T, U, V> {
        match self {
            CCLang::Boolean(v) => CCLang::Boolean(v.clone()),
            CCLang::Binary(v) => CCLang::Binary(v.clone()),
            CCLang::Text(v) => CCLang::Text(v.clone()),
            CCLang::EncodingId(encoding) => CCLang::EncodingId(encoding.clone()),
            CCLang::EncryptionId(encryption) => CCLang::EncryptionId(encryption.clone()),
            CCLang::SigningId(signing) => CCLang::SigningId(signing.clone()),
            CCLang::HashingId(hashing) => CCLang::HashingId(hashing.clone()),
            CCLang::IOHandle(handle) => CCLang::IOHandle(handle.clone()),
            CCLang::IOIdentifier(id) => CCLang::IOIdentifier(id.clone()),
            CCLang::IOIndex(v) => CCLang::IOIndex(v.clone()),
            CCLang::IOMode(mode) => CCLang::IOMode(mode.clone()),
            CCLang::IOWhence(whence) => CCLang::IOWhence(whence.clone()),
            CCLang::Open(io) => CCLang::Open(io.clone()),
            CCLang::Read(io) => CCLang::Read(io.clone()),
            CCLang::Write(io) => CCLang::Write(io.clone()),
            CCLang::Seek(io) => CCLang::Seek(io.clone()),
            CCLang::Close(io) => CCLang::Close(io.clone()),
            CCLang::Equal => CCLang::Equal,
            CCLang::NotEqual => CCLang::NotEqual,
            CCLang::LessThan => CCLang::LessThan,
            CCLang::LessThanEqual => CCLang::LessThanEqual,
            CCLang::GreaterThan => CCLang::GreaterThan,
            CCLang::GreaterThanEqual => CCLang::GreaterThanEqual,
            /*
            CCLang::BitOr => CCLang::BitOr,
            CCLang::BitAnd => CCLang::BitAnd,
            CCLang::BitXor => CCLang::BitXor,
            CCLang::BitNot => CCLang::BitNot,
            */
            CCLang::Encode => CCLang::Encode,
            CCLang::Decode => CCLang::Decode,
            CCLang::Encrypt => CCLang::Encrypt,
            CCLang::Decrypt => CCLang::Decrypt,
            CCLang::Sign => CCLang::Sign,
            CCLang::Verify => CCLang::Verify,
            CCLang::Hash => CCLang::Hash,
            CCLang::Concat => CCLang::Concat,
            CCLang::Slice => CCLang::Slice,
            CCLang::Dup => CCLang::Dup,
            CCLang::Pop => CCLang::Pop,
            CCLang::If => CCLang::If,
            CCLang::Else => CCLang::Else,
            CCLang::Fi => CCLang::Fi
        }
    }
}

impl<T, U, V> PartialEq for CCLang<T, U, V>
    where T: Clone + PartialEq + PartialOrd,
          U: Clone + PartialEq + PartialOrd,
          V: AppIO<T, U> + Clone + PartialEq + PartialOrd
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            CCLang::Boolean(l) => {
                match other {
                    CCLang::Boolean(r) => *l == *r,
                    CCLang::IOIndex(r) => {
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
            CCLang::IOHandle(l) => {
                match other {
                    CCLang::IOHandle(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::IOIdentifier(l) => {
                match other {
                    CCLang::IOIdentifier(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::IOIndex(l) => {
                match other {
                    CCLang::Boolean(r) => {
                        let lb = *l != 0;
                        lb == *r
                    },
                    CCLang::IOIndex(r) => *l == *r,
                    _ => false,
                }
            }
            CCLang::IOMode(l) => {
                match other {
                    CCLang::IOMode(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::IOWhence(l) => {
                match other {
                    CCLang::IOWhence(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Open(l) => {
                match other {
                    CCLang::Open(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Read(l) => {
                match other {
                    CCLang::Read(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Write(l) => {
                match other {
                    CCLang::Write(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Seek(l) => {
                match other {
                    CCLang::Seek(r) => *l == *r,
                    _ => false
                }
            },
            CCLang::Close(l) => {
                match other {
                    CCLang::Close(r) => *l == *r,
                    _ => false
                }
            },
            _ => self == other
        }
    }
}

impl<T, U, V> PartialOrd for CCLang<T, U, V>
    where T: Clone + PartialEq + PartialOrd,
          U: Clone + PartialEq + PartialOrd,
          V: AppIO<T, U> + Clone + PartialEq + PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            CCLang::Boolean(l) => {
                match other {
                    CCLang::Boolean(r) => l.partial_cmp(r),
                    CCLang::IOIndex(r) => {
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
            CCLang::IOHandle(l) => {
                if let CCLang::IOHandle(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::IOIdentifier(l) => {
                if let CCLang::IOIdentifier(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::IOIndex(l) => {
                match other {
                    CCLang::Boolean(r) => {
                        let ri = *r as isize;
                        l.partial_cmp(&ri)
                    },
                    CCLang::IOIndex(r) => l.partial_cmp(r),
                    _ => None
                }
            },
            CCLang::IOMode(l) => {
                if let CCLang::IOMode(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::IOWhence(l) => {
                if let CCLang::IOWhence(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::Open(l) => {
                if let CCLang::Open(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::Read(l) => {
                if let CCLang::Read(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::Write(l) => {
                if let CCLang::Write(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::Seek(l) => {
                if let CCLang::Seek(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            CCLang::Close(l) => {
                if let CCLang::Close(r) = other {
                    return l.partial_cmp(r);
                }
                None
            },
            _ => self.partial_cmp(other)
        }
    }
}

impl<T, U, V> Instruction<CCLang<T, U, V>> for CCLang<T, U, V>
    where T: Clone + PartialEq + PartialOrd,
          U: Clone + PartialEq + PartialOrd,
          V: AppIO<T, U> + Clone + PartialEq + PartialOrd
{
    fn execute(&self, m: &mut Machine<CCLang<T, U, V>>) -> Option<usize> {
        match self {
            CCLang::Boolean(_) |
            CCLang::Binary(_) |
            CCLang::Text(_) |
            CCLang::EncodingId(_) |
            CCLang::EncryptionId(_) |
            CCLang::SigningId(_) |
            CCLang::HashingId(_) |
            CCLang::IOHandle(_) |
            CCLang::IOIdentifier(_) |
            CCLang::IOIndex(_) |
            CCLang::IOMode(_) |
            CCLang::IOWhence(_) => {
                // just push the immediate constant onto the stack
                m.push(self.clone());
                m.next_ip()
            },
            CCLang::Equal => {
                let right = m.pop();
                let left = m.pop();
                m.push(CCLang::Boolean(left == right));
                m.next_ip()
            },
            CCLang::NotEqual => {
                let right = m.pop();
                let left = m.pop();
                m.push(CCLang::Boolean(left != right));
                m.next_ip()
            },
            CCLang::LessThan => {
                let right = m.pop();
                let left = m.pop();
                m.push(CCLang::Boolean(left < right));
                m.next_ip()
            },
            CCLang::LessThanEqual => {
                let right = m.pop();
                let left = m.pop();
                m.push(CCLang::Boolean(left <= right));
                m.next_ip()
            }
            CCLang::GreaterThan => {
                let right = m.pop();
                let left = m.pop();
                m.push(CCLang::Boolean(left > right));
                m.next_ip()
            },
            CCLang::GreaterThanEqual => {
                let right = m.pop();
                let left = m.pop();
                m.push(CCLang::Boolean(left >= right));
                m.next_ip()
            },
            /*
            CCLang::BitOr =>
            CCLang::BitAnd =>
            CCLang::BitXor =>
            CCLang::BitNot =>
            */
            CCLang::Decode => {
                if let CCLang::EncodingId(id) = m.pop() {
                    if let CCLang::Text(s) = m.pop() {
                        let mut b = BytesMut::new();
                        let data = match id {
                            Encoding::Hex => hex::decode(s).unwrap(),
                            Encoding::Base64 => base64::decode_config(&s, base64::STANDARD).unwrap(),
                            Encoding::Base64Url => base64::decode_config(&s, base64::URL_SAFE).unwrap(),
                            Encoding::Base58Bitcoin => bs58::decode(s).into_vec().unwrap()
                        };
                        b.put_slice(&data);
                        m.push(CCLang::Binary(b.freeze()));
                        return m.next_ip();
                    }
                }
                panic!()
            },
            CCLang::Encode => {
                if let CCLang::EncodingId(id) = m.pop() {
                    if let CCLang::Binary(b) = m.pop() {
                        let s = match id {
                            Encoding::Hex => hex::encode(b.as_ref()),
                            Encoding::Base64 => base64::encode_config(b.as_ref(), base64::STANDARD),
                            Encoding::Base64Url => base64::encode_config(b.as_ref(), base64::URL_SAFE),
                            Encoding::Base58Bitcoin => bs58::encode(b.as_ref()).into_string(),
                        };
                        m.push(CCLang::Text(s));
                        return m.next_ip();
                    }
                }
                panic!()
            },
            CCLang::Encrypt => {
                if let CCLang::EncryptionId(id) = m.pop() {
                    match id {
                        Encryption::XSalsa20Poly1305 => {
                            if let CCLang::Binary(n) = m.pop() {
                                if let CCLang::Binary(k) = m.pop() {
                                    if let CCLang::Binary(plaintext) = m.pop() {
                                        let nonce = xsalsa20poly1305::Nonce::from_slice(n.as_ref()).unwrap();
                                        let key = xsalsa20poly1305::Key::from_slice(k.as_ref()).unwrap();
                                        let mut b = BytesMut::new();
                                        let ciphertext = xsalsa20poly1305::seal(plaintext.as_ref(), &nonce, &key);
                                        b.put_slice(&ciphertext);
                                        m.push(CCLang::Binary(b.freeze()));
                                        return m.next_ip();
                                    }
                                }
                            }
                        }
                    }
                }
                panic!()
            },
            CCLang::Decrypt => {
                if let CCLang::EncryptionId(id) = m.pop() {
                    match id {
                        Encryption::XSalsa20Poly1305 => {
                            if let CCLang::Binary(n) = m.pop() {
                                if let CCLang::Binary(k) = m.pop() {
                                    if let CCLang::Binary(ciphertext) = m.pop() {
                                        let nonce = xsalsa20poly1305::Nonce::from_slice(n.as_ref()).unwrap();
                                        let key = xsalsa20poly1305::Key::from_slice(k.as_ref()).unwrap();
                                        let mut b = BytesMut::new();
                                        if let Ok(plaintext) = xsalsa20poly1305::open(ciphertext.as_ref(), &nonce, &key) {
                                            b.put_slice(&plaintext);
                                            m.push(CCLang::Binary(b.freeze()));
                                            return m.next_ip();
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
                if let CCLang::SigningId(id) = m.pop() {
                    match id {
                        Signing::Ed25519 => {
                            if let CCLang::Binary(sk) = m.pop() {
                                if let CCLang::Binary(msg) = m.pop() {
                                    let seckey = SecretKey::from_slice(sk.as_ref()).unwrap();
                                    let sig = sign_detached(&msg, &seckey);
                                    let mut signature = BytesMut::new();
                                    signature.put_slice(sig.as_ref());
                                    m.push(CCLang::Binary(signature.freeze()));
                                    return m.next_ip();
                                }
                            }
                        }
                    }
                }
                panic!()
            },
            CCLang::Verify => {
                if let CCLang::SigningId(id) = m.pop() {
                    match id {
                        Signing::Ed25519 => {
                            if let CCLang::Binary(msg) = m.pop() {
                                if let CCLang::Binary(pk) = m.pop() {
                                    if let CCLang::Binary(sig) = m.pop() {
                                        let signature = Signature::from_slice(sig.as_ref()).unwrap();
                                        let pubkey = PublicKey::from_slice(pk.as_ref()).unwrap();
                                        m.push(CCLang::Boolean(verify_detached(&signature, &msg, &pubkey)));
                                        return m.next_ip();
                                    }
                                }
                            }
                        }
                    }
                }
                panic!()
            },
            CCLang::Hash => {
                if let CCLang::HashingId(id) = m.pop() {
                    if let CCLang::Binary(b) = m.pop() {
                        let mut buf = BytesMut::new();
                        match id {
                            Hashing::SHA256 => buf.put_slice(sha256::hash(b.as_ref()).as_ref()),
                            Hashing::SHA512 => buf.put_slice(sha512::hash(b.as_ref()).as_ref())
                        }
                        m.push(CCLang::Binary(buf.freeze()));
                        return m.next_ip();
                    }
                }
                panic!()
            },
            CCLang::Concat => {
                if let CCLang::Binary(br) = m.pop() {
                    if let CCLang::Binary(bl) = m.pop() {
                        let mut buf = BytesMut::new();
                        buf.extend_from_slice(&bl);
                        buf.extend_from_slice(&br);
                        m.push(CCLang::Binary(buf.freeze()));
                        return m.next_ip();
                    }
                }
                panic!()
            },
            CCLang::Slice => {
                if let CCLang::IOIndex(end) = m.pop() {
                    if let CCLang::IOIndex(begin) = m.pop() {
                        if let CCLang::Binary(b) = m.pop() {
                            m.push(CCLang::Binary(b.slice(begin as usize..end as usize)));
                            return m.next_ip();
                        }
                    }
                }
                panic!()
            },
            CCLang::Open(io) => {
                if let CCLang::IOMode(mode) = m.pop() {
                    if let CCLang::IOIdentifier(id) = m.pop() {
                        m.push(CCLang::IOHandle(io.open(&id, mode)));
                        return m.next_ip();
                    }
                }
                panic!();
            },
            CCLang::Read(io) => {
                if let CCLang::IOIndex(num) = m.pop() {
                    if let CCLang::IOHandle(h) = m.pop() {
                        m.push(CCLang::Binary(io.read(&h, num as usize)));
                        m.push(CCLang::IOHandle(h));
                        return m.next_ip();
                    }
                }
                panic!();
            },
            CCLang::Write(io) => {
                if let CCLang::Binary(bin) = m.pop() {
                    if let CCLang::IOHandle(h) = m.pop() {
                        io.write(&h, &bin);
                        m.push(CCLang::IOHandle(h));
                        return m.next_ip();
                    }
                }
                panic!();
            },
            CCLang::Seek(io) => {
                if let CCLang::IOIndex(num) = m.pop() {
                    if let CCLang::IOWhence(whence) = m.pop() {
                        if let CCLang::IOHandle(h) = m.pop() {
                            io.seek(&h, whence, num);
                            m.push(CCLang::IOHandle(h));
                            return m.next_ip();
                        }
                    }
                }
                panic!();
            },
            CCLang::Close(io) => {
                if let CCLang::IOHandle(h) = m.pop() {
                    io.close(&h);
                    return m.next_ip();
                }
                panic!();
            },
            CCLang::Dup => {
                let stack: &mut Stack<CCLang<T, U, V>> = m.get_stack_mut();
                let top = match stack.pop() {
                    Some(t) => t,
                    _ => panic!()
                };
                stack.push(top.clone());
                stack.push(top.clone());
                m.next_ip()
            },
            CCLang::Pop => {
                let stack: &mut Stack<CCLang<T, U, V>> = m.get_stack_mut();
                let _ = stack.pop();
                m.next_ip()
            },
            CCLang::If => {
                // find the location of the matching 'ELSE' if any and 'FI'
                let ifm = match self.find_matching_elsefi(m, m.get_ip()) {
                    Some(ifefi) => ifefi,
                    None => return None
                };

                // get the Boolean from the stack
                if let CCLang::Boolean(b) = m.pop() {
                    if b {
                        // the boolean is true so continue with the code that is
                        // between this if and it's matching 'ELSE'
                        
                        // first record where we need to go after this block
                        m.push_frame(ifm.fii + 1);

                        // then tell the machine the correct next instruction
                        return m.next_ip();
                    } else {
                        // the boolean is false so skip to the instruction after
                        // the 'ELSE' if there is one, otherwise skip to after the
                        // 'FI'
                        let next_ip = match ifm.elsei {
                            Some(i) => {
                                // we're executing the 'ELSE' block so we need to
                                // push a frame with the correct next instruction
                                m.push_frame(ifm.fii + 1);

                                // set the next instruction pointer to the
                                // instruction after the 'ELSE'
                                i + 1
                            },

                            // No 'ELSE' clause so just skip to the instruction
                            // after the 'FI'. There is no need to record a frame.
                            None => ifm.fii + 1
                        };

                        return Some(next_ip);
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
                let next_ip = match m.pop_frame() {
                    Some(i) => i,
                    None => panic!()
                };

                return Some(next_ip);
            },
            CCLang::Fi => {
                // we finished executing an 'IF' or 'ELSE' block so pop the
                // frame and continue
                let next_ip = match m.pop_frame() {
                    Some(i) => i,
                    None => panic!()
                };

                return Some(next_ip);
            }
        }
    }
}
