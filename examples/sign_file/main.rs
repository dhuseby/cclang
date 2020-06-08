use bytes::Bytes;
use cclang::{
    CCLang,
    FileIO,
    MachineBuilder,
    Mode,
    Script,
    Signing
};
use semver::VersionReq;
use sodiumoxide::{
    self,
    crypto::sign::{
        SecretKey,
        sign_detached,
    }
};
use std::{
    fs::{
        OpenOptions
    },
    io::{
        self,
        Read,
        Write
    },
    path::PathBuf,
    str::FromStr
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "sign_file",
    version = "0.1",
    author = "Dave Huseby",
    about = "Sign/verify files using cclang",
)]
struct Opt {
    /// the subcommand operation
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Sign a file and output a CCLang signature
    #[structopt(name = "sign")]
    Sign {
        /// Path to file to sign
        #[structopt(parse(from_os_str))]
        data_file: PathBuf,

        /// Path to secret key file to sign the file with
        #[structopt(parse(from_os_str))]
        key_file: PathBuf,

        /// Path to file to save CCLang signature in. If no path is given then
        /// the signature is output to standard out.
        #[structopt(parse(from_os_str))]
        signature_file: Option<PathBuf>,

        /// Format for the signature serialization: text, json, or cbor
        #[structopt(short = "f", long = "format", default_value = "text")]
        format: String
    },

    /// Verify a CCLang signature
    #[structopt(name = "verify")]
    Verify {
        /// Path to the CCLang signature. If no path is given then the
        /// signature is read from standard in.
        #[structopt(parse(from_os_str))]
        signature_file: Option<PathBuf>,

        /// Format for the signature serialization: text, json, or cbor
        #[structopt(short = "f", long = "format", default_value = "text")]
        format: String
    }
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct SignatureBuilder {
    datafile: PathBuf,
    keyfile: PathBuf,
}

impl SignatureBuilder {
    fn new() -> Self {
        SignatureBuilder {
            datafile: PathBuf::default(),
            keyfile: PathBuf::default(),
        }
    }

    fn data_file(&mut self, path: &PathBuf) -> &mut Self {
        self.datafile = path.clone();
        self
    }

    fn key_file(&mut self, path: &PathBuf) -> &mut Self {
        self.keyfile = path.clone();
        self
    }

    fn sign(&self) -> Result<Script<CCLang>, Box<dyn std::error::Error>> {
        // read the data to sign
        let mut data_file = OpenOptions::new().read(true).open(self.datafile.as_path())?;
        let mut data: Vec<u8> = Vec::new();
        let _ = data_file.read_to_end(&mut data)?;

        // read the secret key
        let mut key_file = OpenOptions::new().read(true).open(self.keyfile.as_path())?;
        let mut key: Vec<u8> = Vec::new();
        let _ = key_file.read_to_end(&mut key)?;
        let sk = match SecretKey::from_slice(&key) {
            Some(sk) => sk,
            _ => {
                return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "could not load secret key")));
            }
        };

        // calculate the signature
        let signature = sign_detached(&data, &sk);
        
        // construct the signature script
        Ok(Script::from(vec![
            // version check
            CCLang::Text(VERSION.to_string()),
            CCLang::Version,

            CCLang::If,

            // push the signature
            CCLang::Binary(Bytes::copy_from_slice(signature.as_ref())),

            // push the public key
            CCLang::Binary(Bytes::copy_from_slice(sk.public_key().as_ref())),

            // read the file that was signed and push it
            CCLang::Text(self.datafile.to_string_lossy().to_string()),
            CCLang::Mode(Mode::from_str("rb").unwrap()),
            CCLang::Open,
            CCLang::Index(-1),
            CCLang::Read,
            CCLang::Close,

            // push the Ed25519 signing algorithm identifier
            CCLang::SigningId(Signing::Ed25519),

            // pop the identifier, message, pub key, and signature, verify and push boolean result
            CCLang::Verify,

            CCLang::Else,
            CCLang::Boolean(false),
            CCLang::Fi
        ]))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // init sodiumoxide
    sodiumoxide::init().unwrap();

    // parse the command line options
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Sign { data_file, key_file, signature_file, format } => {
            // create the signature
            let sig = SignatureBuilder::new()
                .data_file(&data_file)
                .key_file(&key_file)
                .sign()?;

            // figure out where we're writing it
            let mut sig_writer: Box<dyn Write> = match signature_file {
                Some(sigfile) => {
                    // open the file to save to
                    Box::new(OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .create(true)
                        .open(sigfile.as_path())?)
                },
                None => {
                    Box::new(io::stdout())
                }
            };

            // write the signature in whatever serialization format specified
            match format.as_str() {
                "text" => {
                    sig_writer.write_all(format!("{}", sig).as_bytes())?;
                },
                "json" => {
                    serde_json::to_writer(sig_writer, &sig)?;
                },
                "cbor" => {
                    serde_cbor::to_writer(sig_writer, &sig)?;
                },
                &_ => {
                    return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Unknown serialization format")));
                }
            }
            Ok(())
        },

        Command::Verify { signature_file, format } => {
            // figure out where we're reading from
            let mut sig_reader: Box<dyn Read> = match signature_file {
                Some(sigfile) => {
                    Box::new(OpenOptions::new()
                        .read(true)
                        .open(sigfile.as_path())?)
                },
                None => {
                    Box::new(io::stdin())
                }
            };

            // read the signature script from whichever format specified
            let sig: Script<CCLang> = match format.as_str() {
                "text" => {
                    let mut sig_data = Vec::new();
                    sig_reader.read_to_end(&mut sig_data)?;
                    let s = String::from_utf8(sig_data)?;
                    let sig_str = format!("\"{}\"", s);
                    serde_json::from_str(&sig_str)?
                },
                "json" => {
                    serde_json::from_reader(sig_reader)?
                },
                "cbor" => {
                    serde_cbor::from_reader(sig_reader)?
                },
                &_ => {
                    return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Unknown serialization format")));
                }
            };

            // execute the signature script doing version check
            let mut machine = MachineBuilder::new()
                .script(&sig)
                .version_req(&VersionReq::parse(format!(">= {}", VERSION).as_str()).unwrap())
                .build();
            let mut result = machine.execute(&FileIO).unwrap();

            // the result should be a boolean with the value true
            match result.pop() {
                Some(CCLang::Boolean(true)) => {
                    println!("Signature is VALID.");
                    Ok(())
                },
                _ => {
                    Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Signature is INVALID")))
                }
            }
        }
    }
}
