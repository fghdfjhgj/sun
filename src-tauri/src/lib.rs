use t3_lib_rust::other_flash_phone::{adb_devices_phone, fastboot_devices_phone};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use t3_lib_rust::other_utils::*;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn detect_device() -> bool {
    exec(str_to_cstr("adb devices".parse().unwrap())).success

}
#[tauri::command]
fn devices_adb() ->String{
    cstring_to_string(adb_devices_phone()).expect("error")
}
#[tauri::command]
fn devices_fastboot() ->String{
    cstring_to_string(fastboot_devices_phone()).expect("error")
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,detect_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
