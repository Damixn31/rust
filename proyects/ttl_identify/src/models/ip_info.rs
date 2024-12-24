use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IpInfo {
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub org: Option<String>,
    pub loc: Option<String>,
    pub postal: Option<String>,
    pub hostname: Option<String>,
}

impl IpInfo {
    pub fn display(&self) -> String {
        format!(
            "\n\tCiudad: {}\n\tRegión: {}\n\tPaís: {}\n\tOrganización: {}\n\tLocalizacion: {}\n\tPostal: {}\n\tHostname: {}",
            self.city
                .clone()
                .unwrap_or_else(|| "Desconocido".to_string()),
            self.region
                .clone()
                .unwrap_or_else(|| "Desconocido".to_string()),
            self.country
                .clone()
                .unwrap_or_else(|| "Desconocido".to_string()),
            self.org
                .clone()
                .unwrap_or_else(|| "Desconocido".to_string()),
            self.loc
                .clone()
                .unwrap_or_else(|| "Desconocido".to_string()),
            self.postal.clone().unwrap_or_else(|| "Desconocido".to_string()),
            self.hostname.clone().unwrap_or_else(|| "Desconocido".to_string())
        )
    }
}
