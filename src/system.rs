//! In-system informations API
//!
//! <https://www.edsm.net/ja/api-system-v1>

use edsm_dumps_model::model::body::Body;
use serde::Deserialize;
use serde_json::from_slice;
use surf::get;
use url::Url;

use crate::{Error, SystemSpecifier};

/// Get bodies in system.
pub async fn bodies(system: impl Into<SystemSpecifier<'_>>) -> Result<Bodies, Error> {
    let mut url =
        Url::parse("https://www.edsm.net/api-system-v1/bodies").expect("failed to parse base url");
    system.into().apply(&mut url);

    let bytes = get(url).recv_bytes().await?;

    if bytes == b"{}" {
        return Err(Error::EmptyResponse);
    }

    let v: Bodies = from_slice(&bytes)?;
    Ok(v)
}

/// Responce of [`bodies()`](fn.bodies.html).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bodies {
    /// System ID
    pub id: u64,
    /// In-game system ID
    pub id64: u64,
    /// System name
    pub name: String,
    /// EDSM page url
    pub url: String,
    /// body count in system
    pub body_count: u64,
    /// Bodies in system
    pub bodies: Vec<Body>,
}
