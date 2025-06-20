//! Things that are only useful if you are doing your own API calling.
//!
//! Note that the RFC calls inclusion proof "audit proof".

use crate::Error;
pub use digitally_signed_struct::*;
use std::time;

mod digitally_signed_struct;
pub mod openssl_ffi;

/// Construct a new [`reqwest::Client`] to be used with the
/// functions in this module. You don't necessary need to use this.
///
/// The client constructed will not store cookie or follow redirect.
pub fn new_http_client() -> Result<reqwest::blocking::Client, Error> {
    let mut def_headers = reqwest::header::HeaderMap::new();
    def_headers.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_static("rust-ctclient"),
    );
    match reqwest::blocking::Client::builder()
        .connect_timeout(time::Duration::from_secs(5))
        .gzip(true)
        .default_headers(def_headers)
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(r) => Ok(r),
        Err(e) => Err(Error::Unknown(format!("{}", &e))),
    }
}
