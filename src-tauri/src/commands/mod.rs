use serde::{Deserialize, Serialize};

pub mod connect;
pub mod disconnect;
pub mod get_data;
pub mod get_settings;
pub mod get_table;
pub mod save_settings;
pub mod test_connection;

const SETTING_FILE: &str = "cloaty.json";

#[derive(Deserialize)]
pub struct Info {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    items_per_page: usize,
    fetch_limit: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Animation {
    enabled: bool,
    duration: usize,
    r#type: String,
    params: AnimationParams,
}

#[derive(Serialize, Deserialize)]
pub struct AnimationParams {
    fly: FlyParams,
}

#[derive(Serialize, Deserialize)]
pub struct FlyParams {
    in_x: isize,
    out_x: isize,
    in_y: isize,
    out_y: isize,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    database: Database,
    animation: Animation,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: Database {
                items_per_page: 50,
                fetch_limit: 100,
            },
            animation: Animation {
                enabled: true,
                duration: 200,
                r#type: "fade".to_string(),
                params: AnimationParams {
                    fly: FlyParams {
                        in_x: 100,
                        out_x: -100,
                        in_y: 0,
                        out_y: 0,
                    }
                }
            },
        }
    }
}
