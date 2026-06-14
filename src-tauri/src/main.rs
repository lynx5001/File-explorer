// The Tauri application entry point, handles window setup and lifecycle.
// All application-specific commands should be defined in `src-tauri/src/lib.rs`.

// Import commands from the `lib.rs` file (assuming the crate name is `accessible_file_explorer_app`)
use accessible_file_explorer_app::greet;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}