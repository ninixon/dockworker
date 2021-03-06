//! Docker
#![doc(html_root_url = "https://ghmlee.github.io/rust-docker/doc")]

extern crate base64;
extern crate byteorder;
extern crate dirs;
#[macro_use]
extern crate failure;
extern crate hyper;
#[macro_use]
extern crate log;
#[cfg(windows)]
extern crate named_pipe;
extern crate nix;
#[cfg(feature = "openssl")]
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_aux;
#[macro_use]
extern crate serde_json;
extern crate tar;
#[cfg(unix)]
extern crate unix_socket;
extern crate url;

mod header;
mod docker;
mod error;
mod hyper_client;
mod http_client;
pub mod models;
mod swarm;
#[cfg(unix)]
mod unix;
pub(crate) mod api;

pub use error::{Result, Error};
pub use docker::Docker;
