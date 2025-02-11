mod commands;

use crate::commands::commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,
            device_adb,
            detect_device,
            device_fastboot,
            get_phone_modem_software_version,
            get_phone_ro_product_manufacturer,
            get_phone_ro_product_device,
            get_phone_ro_product_brand,
            get_phone_hardware,
            get_phone_kernel_qemu,
            get_phone_product_model,
            get_phone_system_cpu_abi,
            get_phone_system_build_version_security,
            get_phone_system_build_id,
            get_phone_system_build_description,
            get_phone_android_kernel_version,
            get_phone_bootloader_if_start,
            get_phone_android_sdk_version,
            get_phone_android_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
