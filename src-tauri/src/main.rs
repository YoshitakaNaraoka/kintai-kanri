// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login,logout])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn login(window: tauri::Window) {
    window
        .emit("goto", Some(vec!["hello".to_string()]))
        .expect("failed to emit goto event");
}

#[tauri::command]
fn logout(window: tauri::Window) {
    // ホーム画面への遷移
    window
        .emit("goto", Some(vec!["home".to_string()]))
        .expect("failed to emit goto event");
}