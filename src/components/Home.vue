<template>
  <div class="center-container">
    <button class="btn btn-on" :class="{ active: !isActive }" @click="toggleSwitch(true)">打开</button>
    <button class="btn btn-off" :class="{ active: isActive }" @click="toggleSwitch(false)">关闭</button>
  </div>
  <div class="instructions">
    <span class="action">打开：</span> 打开fn键锁定，允许使用普通的 F1-F12 键功能。
    <br>
    <span class="action">关闭：</span> 关闭fn键锁定，按下 F1-F12 将执行键盘的特殊功能。
  </div>
  <hr>
  <small class="sn">确认已经连接上了键盘,不行就多点几次,程序不用后台运行,点好关闭就行</small>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';



const isActive = ref(false);

const toggleSwitch = (state: boolean) => {
  isActive.value = state;
  let canFn = isActive.value;
  invoke("toggle_switch", { canFn: canFn })
    .then((res) => alert(res))
};


</script>

<style scoped>

/* 禁止选中文本 */
div,.sn{
    -webkit-touch-callout: none; /* iOS Safari */
    -webkit-user-select: none; /* Safari */
    -khtml-user-select: none; /* Konqueror HTML */
    -moz-user-select: none; /* Old versions of Firefox */
    -ms-user-select: none; /* Internet Explorer/Edge */
    user-select: none; /* Non-prefixed version, currently supported by Chrome and Opera */
  }
.center-container {
  display: flex;
  justify-content: center;
  align-items: center;
}

.instructions {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  font-size: 16px;
  line-height: 1.6;
  color: #333;
  margin-top: 33px;
}

.action {
  font-weight: bold;
  color: #0070c9;
  /* Apple's official blue color */
  margin-right: 5px;
}

.btn {
  padding: 10px 20px;
  border-radius: 5px;
  font-size: 16px;
  margin: 0 10px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.btn-on {
  background-color: #3aac3d;
  color: white;
}

.btn-off {
  background-color: #b12c22;
  color: white;
}

.active {
  opacity: 0.6;
}
.sn{
  color: #333;
  font-size: 10px;
  align-content: center;
}

</style>
