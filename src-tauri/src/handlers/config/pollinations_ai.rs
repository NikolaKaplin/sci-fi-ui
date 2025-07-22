use serde_json::json;
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn save_api_token(
    app: tauri::AppHandle,
    token: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store("settings.json")?; // Используем стандартное преобразование ошибок

    store.set("api_token", json!(token));
    store.save()?;

    Ok(())
}

#[tauri::command]
pub fn get_api_token(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let store = match app.store("settings.json") {
        Ok(store) => store,
        Err(e) => return Err(format!("Failed to access store: {}", e)),
    };

    match store.get("api_token") {
        Some(value) => {
            if let Some(token) = value.as_str() {
                Ok(Some(token.to_string()))
            } else {
                Err("Invalid token format in store".to_string())
            }
        }
        None => Ok(None),
    }
}
