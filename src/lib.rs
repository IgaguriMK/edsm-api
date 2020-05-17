#![warn(missing_docs)]

//! EDSM HTTP API wrapper.

pub mod status;
pub mod system;
pub mod systems;

mod dec;

use thiserror::Error;
use url::Url;

/// API error type.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP client returns error.
    #[error("HTTP error: {0}")]
    HTTP(#[from] surf::Exception),
    /// API returned empty response (`{}`).
    #[error("empty response")]
    EmptyResponse,
    /// Failed to decode JSON.
    #[error("failed to decode JSON: {0}")]
    DecodeError(#[from] serde_json::Error),
}

/// Result type for edsm-api.
pub type Result<T> = std::result::Result<T, Error>;

/// A type specifies system.
///
/// The most of EDSM API allows specify system by-name and by-ID.
#[derive(Debug, Clone, Copy)]
pub enum SystemSpecifier<'a> {
    /// Specify system by name.
    Name(&'a str),
    /// Specify system by ID.
    Id(u64),
}

impl<'a> SystemSpecifier<'a> {
    fn apply(self, url: &mut Url) {
        let mut query_pairs = url.query_pairs_mut();

        match self {
            SystemSpecifier::Name(name) => {
                query_pairs.append_pair("systemName", name);
            }
            SystemSpecifier::Id(id) => {
                query_pairs.append_pair("systemId", &id.to_string());
            }
        }
    }
}

impl<'a> From<&'a str> for SystemSpecifier<'a> {
    fn from(v: &'a str) -> SystemSpecifier<'a> {
        SystemSpecifier::Name(v)
    }
}

impl<'a> From<u64> for SystemSpecifier<'a> {
    fn from(v: u64) -> SystemSpecifier<'a> {
        SystemSpecifier::Id(v)
    }
}

fn check_empty(bytes: &[u8]) -> Result<()> {
    if bytes == b"{}" || bytes == b"[]" {
        Err(Error::EmptyResponse)
    } else {
        Ok(())
    }
}
