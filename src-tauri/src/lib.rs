// This is the "Bridge" layer, exposing Rust functions as Tauri commands.
// All business logic should reside in the `engine` module.

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}