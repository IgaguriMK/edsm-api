use std::time::Duration;

use anyhow::Result;
use async_std::task::sleep;

use edsm_api::{system, systems};

/**** systems ****/

#[async_std::test]
async fn systems_system() -> Result<()> {
    let _ = systems::system(27).await?;

    test_sleep().await;
    Ok(())
}

/**** system ****/

#[async_std::test]
async fn system_bodies() -> Result<()> {
    let _ = system::bodies(27).await?;

    test_sleep().await;
    Ok(())
}

/**** util ****/

async fn test_sleep() {
    sleep(Duration::from_secs(5)).await;
}
