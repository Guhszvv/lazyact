use reqwest;
use semver;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub async fn update_checker() -> Result<(bool, String), Box<dyn Error>> {
    #[derive(Debug, Deserialize, Serialize)]
    struct Response {
        tag_name: String,
    }

    let current = semver::Version::parse(env!("CARGO_PKG_VERSION"))?;
    let resp = reqwest::get("https://api.github.com/repos/Guhszvv/lazyact/releases/latest")
        .await?
        .json::<Response>()
        .await?;
    let latest = semver::Version::parse(resp.tag_name.as_str())?;
    let has_update = latest > current;

    Ok((has_update, latest.to_string()))
}
