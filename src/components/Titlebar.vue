<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const platform = await invoke<string>("platform");
console.log(platform);
</script>

<template>
  <div class="titlebar bg-gray-200 dark:bg-gray-800">
    <div data-tauri-drag-region></div>
    <div class="controls">
      <button id="titlebar-minimize" title="minimize" class="hover:bg-gray-300">
        <!-- https://api.iconify.design/mdi:window-minimize.svg -->
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path fill="currentColor" d="M19 13H5v-2h14z" />
        </svg>
      </button>
      <button id="titlebar-maximize" title="maximize" class="hover:bg-gray-300">
        <!-- https://api.iconify.design/mdi:window-maximize.svg -->
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />
        </svg>
      </button>
      <button id="titlebar-close" title="close" class="hover:bg-gray-300">
        <!-- https://api.iconify.design/mdi:close.svg -->
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path
            fill="currentColor"
            d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z"
          />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  height: 30px;
  user-select: none;
  display: grid;
  grid-template-columns: auto max-content;
  width: 100%;
}
.titlebar > .controls {
  display: flex;
}
.titlebar button {
  appearance: none;
  padding: 0;
  margin: 0;
  border: none;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  transition: background-color 0.2s;
  cursor: pointer;
}
</style>
