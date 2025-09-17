<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import MacOS from "./Titlebar/MacOS.vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

const platform = await invoke<string>("platform");

const appWindow = getCurrentWindow();
const minimize = () => {
  appWindow.minimize();
};
const maximize = async () => {
  try {
    const isMaximized = await appWindow.isMaximized();
    if (isMaximized) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
  } catch (e) {
    console.error(e);
  }
};
const close = () => {
  // appWindow.close();
  appWindow.hide();
};
</script>

<template>
  <div class="titlebar bg-gray-200 dark:bg-gray-800 relative">
    <div
      class="left-0 right-0 top-0 bottom-0 absolute"
      data-tauri-drag-region
    ></div>
    <div class="h-full flex">
      <MacOS
        @minimize="minimize"
        @maximize="maximize"
        @close="close"
        v-if="platform === 'macos'"
      />
      <Windows
        @minimize="minimize"
        @maximize="maximize"
        @close="close"
        v-else
      />
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  height: 30px;
  user-select: none;
  width: 100%;
}
</style>
