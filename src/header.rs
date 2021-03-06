use std::fmt;

use base64;
use hyper::header::{Header, HeaderFormat};
use hyper::error::Result;
use hyper::Error;

#[derive(Debug, Clone)]
pub struct XRegistryAuth {
  body: String,
}

impl XRegistryAuth {
  pub fn new<S: Into<String>>(body: S) -> Self {
    Self { body: body.into() }
  }
}

impl Header for XRegistryAuth {
  fn header_name() -> &'static str {
    "X-Registry-Auth"
  }

  fn parse_header(raw: &[Vec<u8>]) -> Result<Self> {
    if raw.len() != 1 {
      return Err(Error::Header);
    }

    base64::decode(&raw[0])
      .map_err(|_| Error::Header)
      .and_then(|vec| String::from_utf8(vec).map_err(|_| Error::Header))
      .map(Self::new)
  }
}

impl HeaderFormat for XRegistryAuth {
  fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let b64 = base64::encode(&self.body);
    debug!("{}: {}", Self::header_name(), b64);
    write!(f, "{}", b64)
  }
}
