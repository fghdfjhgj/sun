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
async function detectDevice() {
    let res=await invoke("detect_device");
    if (res===true){
        
    }else{

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
