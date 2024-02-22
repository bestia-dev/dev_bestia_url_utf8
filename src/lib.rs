// dev_bestia_url_utf8 lib.rs

#![doc=include_str!("../README.md")]

// internal private modules
mod url_utf8_mod;

// region: public interface

use thiserror::Error;

/// thiserror enum
#[derive(Error, Debug)]
pub enum UrlUtf8Error {
    #[error("utf8 error")]
    Utf8Error {
        #[from]
        source: std::str::Utf8Error,
    },
}

// export/re-export public functions, traits,...

// the macro `url!()` is automatically exported at the root of the crate

pub use url_utf8_mod::UrlPartUtf8Decoded;
pub use url_utf8_mod::UrlUtf8EncodedString;

// endregion: public interface
