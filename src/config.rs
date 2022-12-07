use serde::Deserialize;
use std::process::exit;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    ad_user: String,
    with_data: bool,
    base_url: String,
    hdr_url: String,
    proxy_user: String,
}
impl Config {
    pub fn build() -> Config {
        let data = ConfigFile::read();
        let user_id = String::from("my_ad_user");
        let ad_user = format!("{}\\{}", data.userdir, user_id);
        let base_url = format!("{}/{}", data.host, data.proxy_prefix);
        let hdr_url = match data.hdr_url {
            Some(value) => value,
            None => data.host,
        };
        Config {
            ad_user,
            with_data: data.with_data,
            base_url,
            hdr_url,
            proxy_user: data.proxy_user,
        }
    }
}
/// This struct will temporarily contain all data loaded from config file.
/// It will be transformed and consumed by Config::build()
#[derive(Deserialize, Debug)]
struct ConfigFile {
    userdir: String,
    with_data: bool,
    host: String,
    hdr_url: Option<String>,
    proxy_prefix: String,
    proxy_user: String,
}

/// Reads the config file into struct ConfigFile
/// if there is a problem => panic!
impl ConfigFile {
    fn read() -> ConfigFile {
        let bytes = std::fs::read_to_string("./config.toml").unwrap_or_else(|error| {
            eprintln!("Error loading config file (config.toml): {}", error);
            exit(1);
        });

        match toml::from_str(&bytes) {
            Ok(cfg) => cfg,
            Err(err_msg) => {
                eprintln!(
                    "Parse error: {} \nPlease make sure, that your config.toml is correct",
                    err_msg
                );
                exit(1);
            }
        }
    }
}
