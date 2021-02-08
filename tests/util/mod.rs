use dotenv::dotenv;
use palettebot_lib::config::Config;

pub fn setup_config() -> Config {
    dotenv().ok();

    let mut config = Config::new();
    config.read_env();

    config
}
