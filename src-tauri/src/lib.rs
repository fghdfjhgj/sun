
pub mod commands{
    use t3_lib_rust::other_flash_phone::{adb_devices_phone,
                                         fastboot_devices_phone,
                                         get_no_root_phone_data};
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    use t3_lib_rust::other_utils::*;
    #[tauri::command]
    fn greet(name: &str) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
    /// 检测设备连接状态
    ///
    /// 本函数通过执行`adb devices`和`fastboot devices`命令来检测设备是否以adb或fastboot模式连接。
    /// 根据设备连接状态返回相应的数值。
    ///
    /// 返回值:
    /// - 3: 设备既没有以adb模式连接，也没有以fastboot模式连接
    /// - 0: 设备以adb模式连接，但没有以fastboot模式连接
    /// - 1: 设备没有以adb模式连接，但以fastboot模式连接
    /// - 4: 未知错误
    #[tauri::command]
    fn detect_device() -> i32 {
        // 执行`adb devices`命令并检查是否成功
        let a = exec(str_to_cstr("adb devices".parse().unwrap())).success;
        // 执行`fastboot devices`命令并检查是否成功
        let b = exec(str_to_cstr("fastboot devices".parse().unwrap())).success;

        // 根据命令执行结果返回相应的数值
        if a == false && b == false {
            2
        } else if a == true && b == false {
            0
        } else if a == false && b == true {
            1
        }else {
            6
        }

    }
    #[tauri::command]
    fn get_phone_android_version(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.android_version).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_android_sdk_version(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.sdk_version).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_bootloader_if_start(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_bootloader).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_android_kernel_version(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.kernel_version).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_system_build_description(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_build_description).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_system_build_id(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_build_id).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_system_bbuild_version_security(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_build_version_security_patch).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_system_cpu_abi(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_cpu_abi).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_product_model(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_product_model).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_kernel_qemu(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_kernel_qemu).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_hardware(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_hardware).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_ro_product_brand(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_product_brand).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_ro_product_device(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_product_device).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_ro_product_manufacturer(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_product_manufacturer).expect("error")
        }
    }
    #[tauri::command]
    fn get_phone_modem_software_version(id:String)->String{
        let res=get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_modem_software_version).expect("error")
        }
    }



















    #[tauri::command]
    fn device_adb() ->String{
        cstring_to_string(adb_devices_phone()).expect("error")
    }
    #[tauri::command]
    fn device_fastboot() ->String{
        cstring_to_string(fastboot_devices_phone()).expect("error")
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,
            detect_device,
            device_adb,
            device_fastboot,
            get_phone_android_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
