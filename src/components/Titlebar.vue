<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import MacOS from "./Titlebar/MacOS.vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

const platform = await invoke<string>("platform");

const appWindow = getCurrentWindow();
const minimize = () => {
  try {
    appWindow.minimize();
  } catch (e) {
    console.error(e);
  }
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
  try {
    appWindow.hide();
  } catch (e) {
    console.error(e);
  }
};
</script>

<template>
  <div
    :class="[
      platform === 'windows' ? 'h-[50px]' : 'h-[30px]',
      'user-select-none w-full bg-gray-200 dark:bg-gray-800 bg-gray-100 dark:bg-gray-900 relative',
    ]"
  >
    <div
      class="h-full flex pointer-events-none absolute left-0 right-0 top-0 bottom-0"
    >
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
    <div data-tauri-drag-region class="h-full h-full"></div>
  </div>
</template>

<style scoped></style>
