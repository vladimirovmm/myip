use anyhow::Result;
use futures::future::join_all;

async fn request_get_ip(url: &str) -> Result<String> {
    let type_ip = get_type_by_url(url);
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(_) => return Ok(format!("{type_ip}: -")),
    };
    let result_text = response.text().await?.trim().trim_matches('"').to_string();

    Ok(format!("{type_ip}: {result_text}"))
}

#[inline]
fn get_type_by_url(url: &str) -> String {
    { &url[8..12] }.to_string()
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
