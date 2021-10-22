// url_utf8_mod

// Cargo.toml:
// percent-encoding = "2.1.0"
// unwrap = "1.2.1"

// region: use statements

use core::str::FromStr;
use percent_encoding::{percent_decode_str, AsciiSet, CONTROLS};
use std::string::ToString;

use dev_bestia_string_utils::*;

// endregion: use statements

// region: url encoding

/// Constructor macro for UrlUtf8EncodedString  
#[macro_export]
macro_rules! url_u {
    // TODO: the macro could use repetition to avoid having 4 fn with different number of parameters.
    // 1 arguments, 0 fragment to encode
    ($literal:literal) => {
        // The macro will expand into the contents of this block.
        UrlUtf8EncodedString::new_0($literal)
    };
    // 2 arguments, 1 fragment to encode
    ($literal:expr,$part_1:expr) => {
        // The macro will expand into the contents of this block.
        crate::url_utf8_mod::UrlUtf8EncodedString::new_1($literal, $part_1)
    };
    // 3 arguments, 2 fragments to encode
    ($literal:expr,$part_1:expr,$part_2:expr) => {
        // The macro will expand into the contents of this block.
        crate::url_utf8_mod::UrlUtf8EncodedString::new_2($literal, $part_1, $part_2)
    };
    // 4 arguments, 3 fragments to encode
    ($literal:expr,$part_1:expr,$part_2:expr,$part_3:expr) => {
        // The macro will expand into the contents of this block.
        crate::url_utf8_mod::UrlUtf8EncodedString::new_3($literal, $part_1, $part_2, $part_3)
    };
    // 5 arguments, 4 fragments to encode
    ($literal:expr,$part_1:expr,$part_2:expr,$part_3:expr,$part_4:expr) => {
        // The macro will expand into the contents of this block.
        crate::url_utf8_mod::UrlUtf8EncodedString::new_4($literal, $part_1, $part_2, $part_3, $part_4)
    };
}

/// Type UrlUtf8EncodedString explicitly informs that the content has been url encoded.  
/// It contains a string with the whole url.  
/// The url is constructed with a special macro, where the dynamic parts are always encoded.  
/// It is impossible to force the developer to properly encode the static part of the url.  
/// But this special type is making this kind of errors difficult, obvious and traceable.  
#[derive(Clone, Debug)]
pub struct UrlUtf8EncodedString {
    /// private inaccessible field with encoded url  
    s: String,
}

impl UrlUtf8EncodedString {
    /// constructor with 0 dynamic fragment  
    pub fn new_0(literal: &str) -> UrlUtf8EncodedString {
        UrlUtf8EncodedString { s: s!(literal) }
    }
    /// constructor with 1 dynamic fragment  
    pub fn new_1(literal: &str, part_1: &str) -> UrlUtf8EncodedString {
        UrlUtf8EncodedString {
            s: literal.replacen("{}", &Self::encode_fragment(part_1), 1),
        }
    }
    /// constructor with 2 dynamic fragment  
    pub fn new_2(literal: &str, part_1: &str, part_2: &str) -> UrlUtf8EncodedString {
        UrlUtf8EncodedString {
            s: literal.replacen("{}", &Self::encode_fragment(part_1), 1).replacen("{}", &Self::encode_fragment(part_2), 1),
        }
    }
    /// constructor with 3 dynamic fragment  
    pub fn new_3(literal: &str, part_1: &str, part_2: &str, part_3: &str) -> UrlUtf8EncodedString {
        UrlUtf8EncodedString {
            s: literal
                .replacen("{}", &Self::encode_fragment(part_1), 1)
                .replacen("{}", &Self::encode_fragment(part_2), 1)
                .replacen("{}", &Self::encode_fragment(part_3), 1),
        }
    }
    /// constructor with 4 dynamic fragment  
    pub fn new_4(literal: &str, part_1: &str, part_2: &str, part_3: &str, part_4: &str) -> UrlUtf8EncodedString {
        UrlUtf8EncodedString {
            s: literal
                .replacen("{}", &Self::encode_fragment(part_1), 1)
                .replacen("{}", &Self::encode_fragment(part_2), 1)
                .replacen("{}", &Self::encode_fragment(part_3), 1)
                .replacen("{}", &Self::encode_fragment(part_4), 1),
        }
    }
    /// encode fragment / part - associated fn  
    pub fn encode_fragment(s: &str) -> String {
        // return
        s!(percent_encoding::utf8_percent_encode(s, FRAGMENT))
    }
}
impl ToString for UrlUtf8EncodedString {
    #[inline]
    /// returns encoded string (for use in html attributes)  
    fn to_string(&self) -> String {
        // return
        self.s.clone()
    }
}

// end region: url encoding

// region: url part decoding
/// https://url.spec.whatwg.org/#fragment-percent-encode-set  
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// the url must be utf 8. Only the 5 control characters are encoded.  
/// url has parts or fragments or segments delimited mostly by slash /  
/// every part must be encoded/decoded separately,  
/// to maintain the control character slash /  
#[derive(Clone, Debug)]
pub struct UrlPartUtf8Decoded {
    /// private inaccessible field - normal string - decoded  
    s: String,
}

impl UrlPartUtf8Decoded {
    /// Constructor from encoded str  
    /// Decodes the string. It can error.  
    fn new(s: &str) -> Result<Self, crate::UrlUtf8Error> {
        let s = s!(percent_decode_str(s).decode_utf8()?);
        Ok(UrlPartUtf8Decoded { s })
    }
    #[allow(unused)]
    /// rarely needed constructor from decoded (normal) string  
    pub fn new_from_decoded_string(s: &str) -> Self {
        UrlPartUtf8Decoded { s: s!(s) }
    }
    #[allow(unused)]
    /// rarely needed get encoded string  
    pub fn get_encoded_string(&self) -> String {
        UrlUtf8EncodedString::encode_fragment(&self.s)
    }
}
/// implementing FromStr because of path! in warp web server router  
/// it assumes that the original string is encoded  
impl FromStr for UrlPartUtf8Decoded {
    type Err = crate::UrlUtf8Error;
    #[inline]
    /// constructor, decodes the string from encoded str.  
    /// It can error.  
    /// It is used for path! in warp web server router.  
    fn from_str(s: &str) -> Result<Self, crate::UrlUtf8Error> {
        UrlPartUtf8Decoded::new(s)
    }
}
impl ToString for UrlPartUtf8Decoded {
    #[inline]
    /// returns decoded string (normal string)  
    fn to_string(&self) -> String {
        // return
        self.s.clone()
    }
}

// region: url part decoding

#[cfg(test)]
mod tests {
    use super::*;
    use unwrap::unwrap;

    #[test]
    fn test_decode_01() {
        let s = s!(unwrap!(UrlPartUtf8Decoded::new("a%20b%3Cc")));
        assert_eq!(&s, "a b<c");
    }

    #[test]
    fn test_encode_02() {
        let s = url_u!("/one/two/{}/", "a b<c>d'e\"f");
        let norm_str = s!(s);
        assert_eq!(&norm_str, "/one/two/a%20b%3Cc%3Ed\'e%22f/");
    }

    #[test]
    fn test_03() {
        let s = url_u!("/one/two/{}/{}/", "a b<ccc", ">ddd'e\"f");
        let norm_str = s!(s);
        assert_eq!(norm_str, "/one/two/a%20b%3Cccc/%3Eddd\'e%22f/");
    }

    #[test]
    fn test_04() {
        let s = url_u!("/one{}one/two/{}/{}/", "1 1 ", "a b<ccc", ">ddd'e\"f");
        let norm_str = s!(s);

        assert_eq!(norm_str, "/one1%201%20one/two/a%20b%3Cccc/%3Eddd\'e%22f/");
    }
    #[test]
    fn test_05() {
        let s = url_u!("/one{}one/two{}two/{}/{}/", "1 1 ", " 2 2", "a b<ccc", ">ddd'e\"f");
        let norm_str = s!(s);
        assert_eq!(norm_str, "/one1%201%20one/two%202%202two/a%20b%3Cccc/%3Eddd\'e%22f/");
    }
}
