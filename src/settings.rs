use config::{Config, File};

pub mod axum_setting;
pub mod postgres_setting;

use axum_setting::AxumSetting;
use postgres_setting::PostgresSetting;

pub struct AppSetting {
    pub server: AxumSetting,
    pub database: PostgresSetting,
}

impl AppSetting {
    pub fn new() -> Self {
        let settings_file = File::with_name("src/Settings");

        let settings = Config::builder()
            .add_source(settings_file)
            .build()
            .unwrap();

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
