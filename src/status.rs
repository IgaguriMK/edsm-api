//! Game server status API
//!
//! <https://www.edsm.net/ja/api-status-v1>

use std::hash::Hash;

use chrono::{DateTime, Utc};
use log::debug;
use serde::Deserialize;
use serde_json::from_slice;
use surf::get;

use crate::dec::date_format;
use crate::{check_empty, Result};

/// Get game server's status.
/// # Example
///
/// ```
/// # fn main() -> anyhow::Result<()> { async_std::task::block_on(async {
/// #
/// let status = edsm_api::status::elite_server().await?;
///
/// println!("{}", status.message);
/// #
/// # async_std::task::sleep(std::time::Duration::from_secs(3)).await;
/// # Ok(()) }) }
/// ```
pub async fn elite_server() -> Result<EliteServer> {
    let url = "https://www.edsm.net/api-status-v1/elite-server";
    debug!("Requesting {}", url);
    let bytes = get(url).recv_bytes().await?;

    check_empty(&bytes)?;

    let v: EliteServer = from_slice(&bytes)?;
    Ok(v)
}

/// Responce of [`elite_server()`](fn.elite_server.html).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EliteServer {
    /// Datetime when we last checked the status from the Elite: Dangerous server.
    #[serde(with = "date_format")]
    pub last_update: DateTime<Utc>,
    /// Server status classes.
    #[serde(rename = "type")]
    pub typ: StatusType,
    /// Message from Elite: Dangerous server.
    pub message: String,
    /// Status code from Elite: Dangerous server.
    pub status: u64,
}

/// Server status classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatusType {
    /// Frontier server is running normally.
    Success,
    /// Server issue detected.
    Warning,
    /// No
    Danger,
}
