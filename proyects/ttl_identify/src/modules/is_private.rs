use std::net::IpAddr;

pub fn is_private_ip(ip: &str) -> bool {
    match ip.parse::<IpAddr>() {
        Ok(IpAddr::V4(v4)) => {
            v4.is_private() || v4.is_loopback() || v4.is_link_local() || v4.is_broadcast()
        }

        Ok(IpAddr::V6(v6)) => v6.segments()[0] & 0xffc0 == 0xfe80,
        Err(_) => false,
    }
}
