use reqwest::Client;
use std::error::Error;

use crate::models::ip_info::IpInfo;

pub async fn get_ip_info(ip: &str, token: &str) -> Result<IpInfo, Box<dyn Error>> {
    let url = format!("https://ipinfo.io/{}/json?token={}", ip, token);

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let info = response.json::<IpInfo>().await?;
        Ok(info)
    } else {
        Err(format!("Error al consultar ipinfo: Codigo {}", response.status()).into())
    }
}
