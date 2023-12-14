use config::{Config, File};
use tracing::log::info;

use super::axum_setting::AxumSetting;
use super::postgres_setting::PostgresSetting;

pub struct AppSetting {
    pub server: AxumSetting,
    pub database: PostgresSetting,
}

impl AppSetting {
    pub fn new() -> Self {
        let settings_file = File::with_name("src/settings/Settings");

        let settings = Config::builder()
            .add_source(settings_file)
            .build()
            .unwrap();

        info!("Loaded settings.");

        Self {
            server: AxumSetting {
                port: settings.get_int("axum.port").unwrap(),
            },
            database: PostgresSetting {
                url: settings.get_string("postgres.url").unwrap(),
            },
        }
    }
}
