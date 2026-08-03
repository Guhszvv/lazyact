use reqwest;
use semver;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub async fn update_checker() -> Result<(bool, String), Box<dyn Error>> {
    #[derive(Debug, Deserialize, Serialize)]
    struct Response {
        tag_name: String,
    }
    let client = reqwest::Client::new();
    let current = semver::Version::parse(env!("CARGO_PKG_VERSION"))?;
    let resp = client
        .get("https://api.github.com/repos/Guhszvv/lazyact/releases/latest")
        .header(reqwest::header::USER_AGENT, "lazyact")
        .send()
        .await?
        .json::<Response>()
        .await?;
    let latest = semver::Version::parse(resp.tag_name.trim_start_matches('v'))?;
    let has_update = latest > current;

    Ok((has_update, latest.to_string()))
}
