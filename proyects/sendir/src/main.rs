use sendir::{app, config::Config, details_use::UsageInfo};

fn main() {
    let config = Config::new_conf();

    let usage_info = UsageInfo::new("sendir");
    usage_info.print_usege();

    //let dir = "directorio local";
    app::run(config).ok();
}
