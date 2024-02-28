use super::{Settings, SETTING_FILE};

#[tauri::command]
pub async fn get_settings() -> Result<serde_json::Value, String> {
    let settings = match std::fs::read_to_string(SETTING_FILE) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                let def = Settings::default();
                let str = serde_json::to_string(&def).unwrap();

                _ = std::fs::write(SETTING_FILE, &str);
                return serde_json::to_value(&def).map_err(|e| e.to_string());
            }
            _ => return Err(e.to_string()),
        },
    };

    Ok(serde_json::from_str(&settings).map_err(|e| e.to_string())?)
}
