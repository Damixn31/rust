use regex::Regex;

pub fn compile_regex_sshd() -> Option<Regex> {
    regex::Regex::new(r"Failed password for .* from (\d+\.\d+\.\d+\.\d+) port").ok()
}

pub fn compile_regex_mariadb() -> Option<Regex> {
    regex::Regex::new(r"Access denied for user '(.*)'@'(.*)'").ok()
}

pub fn compile_regex_connect_ips() -> Option<Regex> {
    regex::Regex::new(r"(\d+\.\d+\.\d+\.\d+):\d+").ok()
}
