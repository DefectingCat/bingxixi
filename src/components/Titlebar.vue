<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import MacOS from "./Titlebar/MacOS.vue";
import Windows from "./Titlebar/Windows.vue";

const platform = await invoke<string>("platform");

const targetPlatform = (() => {
  switch (platform) {
    case "macos":
      return MacOS;
    case "windows":
      return Windows;
    default:
      return Windows;
  }
})();
</script>

<template>
  <div class="titlebar bg-gray-200 dark:bg-gray-800">
    <div
      class="left-0 right-0 top-0 bottom-0 absolute"
      data-tauri-drag-region
    ></div>
    <div class="h-full flex">
      <component :is="targetPlatform"></component>
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
