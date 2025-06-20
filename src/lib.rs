#[macro_use(lazy_static)]
extern crate lazy_static;

use std::fmt;

pub mod google_log_list;
pub mod internal;
pub mod sct;
pub mod utils;

/// Errors that this library could produce.
#[derive(Debug)]
pub enum Error {
    /// Something strange happened.
    Unknown(String),

    /// Network IO error
    NetIO(reqwest::Error),

    /// The CT server provided us with invalid signature.
    InvalidSignature(String),

    /// Server responded with something bad (e.g. malformed JSON)
    MalformedResponseBody(String),

    /// Something's wrong with the certificate.
    BadCertificate(String),

    /// A malformed SCT is given.
    BadSct(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unknown(desc) => write!(f, "{}", desc),
            Error::NetIO(e) => write!(f, "Network IO error: {}", &e),
            Error::InvalidSignature(desc) => write!(f, "Invalid signature received: {}", &desc),
            Error::MalformedResponseBody(desc) => {
                write!(f, "Unable to parse server response: {}", &desc)
            }
            Error::BadCertificate(desc) => write!(
                f,
                "The certificate returned by the server has a problem: {}",
                &desc
            ),
            Error::BadSct(desc) => write!(f, "The SCT received is invalid: {}", desc),
        }
    }
}
