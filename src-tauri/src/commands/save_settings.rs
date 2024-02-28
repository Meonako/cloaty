use super::SETTING_FILE;

#[tauri::command]
pub async fn save_settings(settings: &str) -> Result<(), String> {
    std::fs::write(SETTING_FILE, settings).map_err(|e| e.to_string())?;
    Ok(())
}