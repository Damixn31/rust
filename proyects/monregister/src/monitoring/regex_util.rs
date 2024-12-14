pub fn compile_regex() -> Option<regex::Regex> {
    regex::Regex::new(r"Failed password for .* from (\d+\.\d+\.\d+\.\d+) port").ok()
}
