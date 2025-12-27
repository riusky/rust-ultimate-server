// 这是一个自动化脚本，用于从特定网站收集theme代码片段 https://tweakcn.com/editor/theme
(async function() {
  // 配置参数
  const config = {
    initialDelay: 1800,    // 初始等待时间(ms)
    betweenClicks: 1400,   // 点击间隔(ms)
    debug: true           // 是否显示调试信息
  };

  // 元素选择器
  const selectors = {
    initialButton: '#radix-_r_24_-content-controls > div > div.border-b > div > button.inline-flex.items-center.gap-2.whitespace-nowrap.text-sm.font-medium.transition-colors.focus-visible\\:outline-none.focus-visible\\:ring-1.focus-visible\\:ring-ring.disabled\\:pointer-events-none.disabled\\:opacity-50.\\[\\&_svg\\]\\:pointer-events-none.\\[\\&_svg\\]\\:size-4.\\[\\&_svg\\]\\:shrink-0.hover\\:bg-accent.hover\\:text-accent-foreground.px-4.py-2.group.relative.w-full.justify-between.md\\:min-w-56.h-14.rounded-none',
    menuItems: '[cmdk-item]',
    codeButton: '#_r_u_ > div > div > div > div.flex.items-center.gap-1 > button:nth-child(12)',
    codeContent: '[id^="radix-_r_"][id*="-content-index\\.css"] pre code',
    closeButton: '#radix-_r_1f_ > button'
  };

  // 收集的数据
  const collectedData = [];

  try {
    // 获取所有可点击项目
    const openInitialMenu = async () => {
      const button = document.querySelector(selectors.initialButton);
      if (!button) throw new Error('初始按钮未找到');
      
      button.click();
      if (config.debug) console.log('已点击初始按钮，等待菜单加载...');
      await delay(config.initialDelay);
      
      const menu = document.querySelector('[cmdk-group-items]');
      if (!menu) throw new Error('下拉菜单未加载');
      
      return document.querySelectorAll(selectors.menuItems);
    };

    // 获取代码内容
const getCodeContent = async () => {
  const codeButton = document.querySelector(selectors.codeButton);
  if (!codeButton) throw new Error('代码按钮未找到');
  
  codeButton.click();
  if (config.debug) console.log('已点击代码按钮，等待弹窗加载...');
  await delay(config.initialDelay);
  
  // 尝试多种方式获取代码内容
  let codeElement = document.querySelector(selectors.codeContent);
  
  // 如果第一种选择器没找到，尝试备用选择器
  if (!codeElement) {
    codeElement = document.querySelector('pre code');
  }
  
  if (!codeElement) {
    // 如果还是没找到，检查整个pre元素的内容
    const preElement = document.querySelector('[id^="radix-_r_"][id*="-content-index"] pre');
    if (preElement) {
      return preElement.innerText;
    }
    throw new Error('代码内容未找到');
  }
  
  const content = codeElement.innerText;
  
  // 关闭弹窗
  const closeButton = document.querySelector(selectors.closeButton);
  if (!closeButton) throw new Error('关闭按钮未找到');
  
  closeButton.click();
  if (config.debug) console.log('已关闭代码弹窗');
  await delay(config.betweenClicks);
  
  return content;
};

    // 辅助函数：延迟
    const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
    
    // 主流程
    const items = await openInitialMenu();
    if (items.length === 0) throw new Error('未找到可点击的项目');
    
    if (config.debug) console.log(`找到 ${items.length} 个项目，开始处理...`);
    
    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      
      // 点击菜单项
      item.click();
      const itemName = item.textContent.trim();
      if (config.debug) console.log(`处理 ${i + 1}/${items.length}: ${itemName}`);
      await delay(config.betweenClicks);
      
      // 获取代码内容
      try {
        const codeContent = await getCodeContent();
        collectedData.push({ name: itemName, code: codeContent });
        
        if (config.debug) {
          console.log(`获取到代码内容:\n${codeContent.substring(0, 50)}...`);
          console.log('当前收集的数据:', collectedData);
        }
      } catch (err) {
        console.error(`处理 ${itemName} 时出错:`, err.message);
      }
      
      // 重新打开菜单
      //if (i < items.length - 1) {
      //  await openInitialMenu();
      //}
    }
    
    console.log('所有项目处理完成！收集的数据:', collectedData);
    
    // 这里可以添加数据保存逻辑，例如：
    // downloadAsFile(collectedData, 'code-collection.json');
    
  } catch (error) {
    console.error('执行出错:', error.message);
  }

  // 可选：将数据保存为文件
  function downloadAsFile(data, filename) {
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }
})();