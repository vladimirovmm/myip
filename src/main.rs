use std::{net::IpAddr, str::FromStr};

use anyhow::Result;
use futures::future::join_all;

async fn request_get_ip(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let result_text = response.text().await?.trim().trim_matches('"').to_string();

    let ip = IpAddr::from_str(&result_text)?;

    let prefix = match ip {
        IpAddr::V4(_) => "ipv4",
        IpAddr::V6(_) => "ipv6",
    };

    Ok(format!("{prefix}: {ip}"))
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Ваш IP:");

    let requests: Vec<_> = [
        "https://ipv4-internet.yandex.net/api/v0/ip",
        "https://ipv6-internet.yandex.net/api/v0/ip",
    ]
    .into_iter()
    .map(request_get_ip)
    .collect();

    for address in join_all(requests).await {
        let output = match address {
            Ok(ip_address) => ip_address,
            Err(err) => format!("Error: {err}"),
        };

        println!("{output}");
    }

    Ok(())
}