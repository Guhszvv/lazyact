use reqwest;
use semver;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub async fn update_checker() -> Result<(bool, String), Box<dyn Error>> {
    #[derive(Debug, Deserialize, Serialize)]
    struct Response {
        tag_name: String,
    }

    let current = semver::Version::parse("0.1.0")?;
    /*let resp = reqwest::get("https://api.github.com/repos/Guhszvv/lazyact/releases/latest")
        .await?
        .json::<Response>()
        .await?;
    */
    let test_version = "9999.0.0".to_string();
    let latest = semver::Version::parse(test_version.as_str())?;
    let has_update = latest > current;

    Ok((has_update, test_version))
}
