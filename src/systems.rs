//! Serarch systems API
//!
//! <https://www.edsm.net/en/api-v1>

use edsm_dumps_model::model::body::StarSubType;
use edsm_dumps_model::model::system::Coords;
use log::debug;
use serde::Deserialize;
use serde_json::from_slice;
use surf::get;
use url::Url;

use crate::{check_empty, Result, SystemSpecifier};

/// Get system's information.
///
/// # Example
///
/// ```
/// # fn main() -> anyhow::Result<()> { async_std::task::block_on(async {
/// #
/// let system_info = edsm_api::systems::system(27).await?;
///
/// assert_eq!(&system_info.name, "Sol");
/// #
/// # async_std::task::sleep(std::time::Duration::from_secs(3)).await;
/// # Ok(()) }) }
/// ```
pub async fn system(system: impl Into<SystemSpecifier<'_>>) -> Result<SystemInfo> {
    let mut url =
        Url::parse("https://www.edsm.net/api-v1/system").expect("failed to parse base url");
    system.into().apply(&mut url);

    url.query_pairs_mut()
        .append_pair("showId", "1")
        .append_pair("showCoordinates", "1")
        .append_pair("showPermit", "1")
        .append_pair("showInformation", "1")
        .append_pair("showPrimaryStar", "1")
        .append_pair("includeHidden", "1");

    debug!("Requesting {}", url);
    let bytes = get(&url).recv_bytes().await?;

    check_empty(&bytes)?;

    let v: SystemInfo = from_slice(&bytes)?;
    Ok(v)
}

/// Responce of [`elite_server()`](fn.elite_server.html).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    /// System's ID
    pub id: u64,
    /// In-game system ID
    pub id64: u64,
    /// System's name
    pub name: String,
    /// Coords of system
    pub coords: Coords,
    /// `true` if system is locked.
    pub coords_locked: bool,
    /// `true` if permit is required.
    pub require_permit: bool,
    /// Information of primary star
    pub primary_star: PrimaryStar,
}

/// Information of primary star in `SystemInfo`
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryStar {
    /// Primary star's name
    pub name: String,
    /// Type of star
    #[serde(rename = "type")]
    pub typ: StarSubType,
    /// `true` if star is scoopable
    pub is_scoopable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[async_std::test]
    async fn system_not_found() {
        let res = system("Not Exist XX-X x0").await;

        if let Err(Error::EmptyResponse) = res {
            // Ok
        } else {
            panic!("Invalid result: {:?}", res);
        }

        async_std::task::sleep(std::time::Duration::from_secs(3)).await;
    }
}
