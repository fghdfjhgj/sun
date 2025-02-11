const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}
async function showContent(pageId) {
  // 隐藏所有内容区域
  const contents = document.querySelectorAll('.content');
  contents.forEach(content => content.style.display = 'none');

  // 显示选中的内容区域
  await loadContent(pageId);
  document.getElementById(pageId).style.display = 'block';
}
async function detect_device(id) {
    await devices()
    await phone_data(id)
}
async function phone_data(id) {
    try {
        let res12="安卓版本:"+await invoke('get_phone_android_version',{id:id})
        let res13="SDK版本::"+await invoke('get_phone_android_sdk_version',{id:id})
        let a="设备是否解锁:"+await invoke('get_phone_bootloader_if_start',{id:id})
        let b="设备内核版本:"+await invoke('get_phone_android_kernel_version',{id:id})
        let c="设备品牌:"+await invoke('get_phone_ro_product_brand',{id:id})
        let d="设备型号:"+await invoke('get_phone_ro_product_model',{id:id})
        let e="设备制造商:"+await invoke('get_phone_ro_product_manufacturer',{id:id})
       let f="设备产品名称:"+ await invoke('get_phone_ro_product_device',{id:id})
        let g="设备架构:"+ await invoke('get_phone_system_cpu_abi',{id:id})
        let result = `${res12}\n${res13}\n${a}\n${b}\n${c}\n${d}\n${e}\n${f}\n${g}`;
        const outputBox = document.getElementById('phone_data');
        if (outputBox) {
            outputBox.innerHTML = result;
        } else {
            console.error('Element with ID "outputBox" not found.');
        }
    }catch (error){

    }
}





async function devices() {
    try {
        // 调用名为 'detect_device' 的命令，并等待其完成
        let res = await invoke("detect_device");

        // 根据 detect_device 的结果选择调用哪个命令或设置默认消息
        let result;
        if (res === 0) {
            result = await invoke('device_adb');
            console.log("ADB devices fetched successfully");
        } else if (res === 1) {


            result = await invoke('device_fastboot');
            console.log("Fastboot devices fetched successfully");
        } else if (res === 2) {
            result = "无法识别设备状态或设备未连接";
        } else {
            result = "未知的错误";
            console.warn(`Unexpected response from detect_device: ${res}`);
        }

        // 获取 HTML 元素 outputBox 并更新其内容
        const outputBox = document.getElementById('outputBox');
        if (outputBox) {
            outputBox.innerHTML = result;
        } else {
            console.error('Element with ID "outputBox" not found.');
        }
    } catch (error) {
        console.error('Failed to invoke commands:', error);
        const outputBox = document.getElementById('outputBox');
        if (outputBox) {
            outputBox.innerHTML = "Error fetching device list.";
        }
    }

}











































async function loadContent(pageId) {
  try {
      const response = await fetch(`${pageId}.html`);
      if (!response.ok) throw new Error(`Failed to fetch ${pageId}.html`);

      const html = await response.text();
      const parser = new DOMParser();
      const doc = parser.parseFromString(html, 'text/html');
      const contentDiv = doc.querySelector(`#${pageId}`);

      if (contentDiv) {
          // 清空并更新内容容器
          const contentContainer = document.getElementById('content-container');
          contentContainer.innerHTML = '';
          contentContainer.appendChild(contentDiv);
      } else {
          console.error(`Could not find element with ID ${pageId} in the fetched HTML.`);
      }
  } catch (error) {
      console.error(error);
  }
}
window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
