pub mod commands {
    use t3_lib_rust::other_flash_phone::{adb_devices_phone,
                                         fastboot_devices_phone,
                                         get_no_root_phone_data};
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    use t3_lib_rust::other_utils::*;

    /// 向前端发送问候信息
    ///
    /// 本函数接收一个字符串参数`name`，并返回一个问候语。
    ///
    /// 参数:
    /// - `name`: 要问候的人的名字
    ///
    /// 返回值:
    /// - 包含问候语的字符串
    ///
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
        } else {
            6
        }
    }

    /// 获取手机的Android版本
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的Android版本。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含Android版本的字符串
    #[tauri::command]
    fn get_phone_android_version(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.android_version).expect("error")
        }
    }

    /// 获取手机的Android SDK版本
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的Android SDK版本。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含Android SDK版本的字符串
    #[tauri::command]
    fn get_phone_android_sdk_version(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.sdk_version).expect("error")
        }
    }

    /// 获取手机的Bootloader启动状态
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的Bootloader启动状态。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含Bootloader启动状态的字符串
    #[tauri::command]
    fn get_phone_bootloader_if_start(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_bootloader).expect("error")
        }
    }

    /// 获取手机的内核版本
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的内核版本。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含内核版本的字符串
    #[tauri::command]
    fn get_phone_android_kernel_version(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.kernel_version).expect("error")
        }
    }

    /// 获取手机的系统构建描述
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的系统构建描述。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含系统构建描述的字符串
    #[tauri::command]
    fn get_phone_system_build_description(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_build_description).expect("error")
        }
    }

    /// 获取手机的系统构建ID
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的系统构建ID。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含系统构建ID的字符串
    #[tauri::command]
    fn get_phone_system_build_id(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_build_id).expect("error")
        }
    }

    /// 获取手机的系统构建安全补丁版本
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的系统构建安全补丁版本。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含系统构建安全补丁版本的字符串
    #[tauri::command]
    fn get_phone_system_build_version_security(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_build_version_security_patch).expect("error")
        }
    }

    /// 获取手机的CPU ABI
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的CPU ABI。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含CPU ABI的字符串
    #[tauri::command]
    fn get_phone_system_cpu_abi(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_cpu_abi).expect("error")
        }
    }

    /// 获取手机的产品型号
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的产品型号。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含产品型号的字符串
    #[tauri::command]
    fn get_phone_product_model(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_product_model).expect("error")
        }
    }

    /// 获取手机的内核QEMU状态
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的内核QEMU状态。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含内核QEMU状态的字符串
    #[tauri::command]
    fn get_phone_kernel_qemu(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_kernel_qemu).expect("error")
        }
    }

    /// 获取手机的硬件信息
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的硬件信息。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含硬件信息的字符串
    #[tauri::command]
    fn get_phone_hardware(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_hardware).expect("error")
        }
    }

    /// 获取手机的产品品牌
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的产品品牌。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含产品品牌的字符串
    #[tauri::command]
    fn get_phone_ro_product_brand(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_product_brand).expect("error")
        }
    }

    /// 获取手机的产品设备名称
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的产品设备名称。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含产品设备名称的字符串
    #[tauri::command]
    fn get_phone_ro_product_device(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_product_device).expect("error")
        }
    }

    /// 获取手机的产品制造商
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的产品制造商。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含产品制造商的字符串
    #[tauri::command]
    fn get_phone_ro_product_manufacturer(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_product_manufacturer).expect("error")
        }
    }

    /// 获取手机的Modem软件版本
    ///
    /// 本函数接收一个字符串参数`id`，表示设备的唯一标识符，并返回该设备的Modem软件版本。
    ///
    /// 参数:
    /// - `id`: 设备的唯一标识符
    ///
    /// 返回值:
    /// - 包含Modem软件版本的字符串
    #[tauri::command]
    fn get_phone_modem_software_version(id: String) -> String {
        let res = get_no_root_phone_data(str_to_cstr(id));
        unsafe {
            let data_ref = &mut *res;
            cstring_to_string(data_ref.ro_modem_software_version).expect("error")
        }
    }

    /// 获取通过ADB连接的设备列表
    ///
    /// 本函数执行`adb devices`命令，并返回连接的设备列表。
    ///
    /// 返回值:
    /// - 包含设备列表的字符串
    #[tauri::command]
    fn device_adb() -> String {
        cstring_to_string(adb_devices_phone()).expect("error")
    }

    /// 获取通过Fastboot连接的设备列表
    ///
    /// 本函数执行`fastboot devices`命令，并返回连接的设备列表。
    ///
    /// 返回值:
    /// - 包含设备列表的字符串
    #[tauri::command]
    fn device_fastboot() -> String {
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
            get_phone_android_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
