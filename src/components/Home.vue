<template>
  <div class="box">
    <div class="help-link">
      <div @click="showHelp">
        帮助
      </div>
      <div @click="showCmd">
        命令行用法
      </div>
    </div>
    <div class="switch-container">
      <button class="switch-btn on" :class="{ active: !isActive }" @click="toggleSwitch(true)">
        打开
        <span class="info-icon" @click.stop="showInfo('on')">?</span>
      </button>

      <button class="switch-btn off" :class="{ active: isActive }" @click="toggleSwitch(false)">
        关闭
        <span class="info-icon" @click.stop="showInfo('off')">?</span>
      </button>
    </div>

    <div v-if="showPopup" class="popup-overlay" @click="closePopup">
      <div class="popup-content" @click.stop>
        <h3>{{ popupTitle }}</h3>
        <p>{{ popupContent }}</p>
        <button @click="closePopup">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const isActive = ref(false);
const showPopup = ref(false);
const popupTitle = ref('');
const popupContent = ref('');

const toggleSwitch = (state: boolean) => {
  isActive.value = state;
  let canFn = isActive.value;
  invoke("toggle_switch", { canFn: canFn })
    .then((res) => {
      popupTitle.value = '执行结果';
      if (typeof res === 'string') {
        popupContent.value = res;
      } else {
        popupContent.value = '未知错误';
      }
      showPopup.value = true;
    })
};

const showInfo = (type: 'on' | 'off') => {
  popupTitle.value = type === 'on' ? '打开' : '关闭';
  popupContent.value = type === 'on'
    ? '打开fn键锁定，允许使用普通的 F1-F12 键功能。'
    : '关闭fn键锁定，按下 F1-F12 将执行键盘的特殊功能。';
  showPopup.value = true;
};

const showHelp = () => {
  popupTitle.value = '帮助信息';
  popupContent.value = '确认已经连接上了键盘,不行就多点几次,程序不用后台运行,点好关闭就行';
  showPopup.value = true;
};

const showCmd = () => {
  popupTitle.value = '命令行用法';
  popupContent.value = '用法:  setfn.exe -m  lock/unlock         \n           锁住:  setfn.exe -m  lock   (使用普通的f1f2)      \n     解锁:  setfn.exe -m unlock  (使用键盘特殊功能)';
  showPopup.value = true;
};

const closePopup = () => {
  showPopup.value = false;
};

onMounted(() => {
  // 检查系统是否处于暗黑模式
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    document.body.classList.add('dark-mode');
  }

  // 监听系统暗黑模式变化
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', e => {
    if (e.matches) {
      document.body.classList.add('dark-mode');
    } else {
      document.body.classList.remove('dark-mode');
    }
  });
});
</script>

<style scoped>
div,
body,
html {
  margin: 0;
  padding: 0;
}

.container {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.box {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.help-link {
  align-self: flex-start;
  width: 100%;
  margin-bottom: 20px;
  cursor: pointer;
  color: #084DCD;
  text-decoration: underline;
}

.switch-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  border-radius: 30px;
}

.switch-btn {
  width: 200px;
  padding: 15px;
  margin: 10px 0;
  border: none;
  border-radius: 50px;
  font-size: 18px;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
}

.switch-btn.on {
  background-color: #4CAF50;
  color: white;
}

.switch-btn.off {
  background-color: #f44336;
  color: white;
}

.switch-btn.active {
  opacity: 0.6;
}

.info-icon {
  position: absolute;
  right: 15px;
  top: 50%;
  transform: translateY(-50%);
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: help;
}

.popup-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  white-space: pre-line;
  z-index: 1000;
}

.popup-content {
  background-color: white;
  padding: 20px;
  border-radius: 5px;
  max-width: 80%;
  text-align: center;
  white-space: pre-line;
}

.popup-content button {
  margin-top: 10px;
  padding: 5px 10px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 3px;
  cursor: pointer;
}

/* 暗黑模式样式 */
@media (prefers-color-scheme: dark) {
  .container {
    background-color: #1a1a1a;
    color: #ffffff;
  }

  .help-link {
    color: #3CB1FB;
  }

  .popup-content {
    background-color: #2a2a2a;
    color: #ffffff;
  }

  .popup-content button {
    background-color: #4da3ff;
  }
}

/* 禁止选中文本 */
* {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
</style>