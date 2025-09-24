<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { MMSState, useAppState } from "./store";
import Titlebar from "./components/Titlebar.vue";
import { listen } from "@tauri-apps/api/event";

// 检测当前平台，用于设置 macOS 与其他平台的标题栏样式
const appState = useAppState();
async function init() {
  const platform = await invoke<string>("platform");
  appState.setPlatform(platform);

  // 启动时先获取下状态
  const mmsStore = await invoke<MMSState>("get_mms_store");
  appState.appState.mms.logged = mmsStore.logged;
  appState.appState.mms.cookie = mmsStore.cookie;

  // 全局监听 mms 的状态
  listen<MMSState>("mms_store", (event) => {
    appState.appState.mms.logged = event.payload.logged;
    appState.appState.mms.cookie = event.payload.cookie;
  });
}
init();
</script>

<template>
  <Suspense>
    <Titlebar />
  </Suspense>

  <router-view />
</template>

<style scoped></style>
<style>
@import "tailwindcss";

/* @custom-variant dark (&:where(.dark, .dark *)); */
@custom-variant dark (&:where([arco-theme=dark], [arco-theme=dark] *));

html,
body {
  background-color: transparent;
  height: 100%;
}

#app {
  @apply bg-gray-100 dark:bg-gray-900 rounded-2xl h-full;
  @apply overflow-hidden;
}

@theme {
  --color-mac-red: #ff5f58;
  --color-mac-yellow: #ffbd2e;
  --color-mac-green: #14c13f;
  --color-mac-red-hover: #e04e4a;
  --color-mac-yellow-hover: #e0a828;
  --color-mac-green-hover: #12a837;
}

@utility button-circle {
  @apply w-8 h-8 rounded-full transition-all duration-200 flex items-center justify-center shadow-[inset_0_1px_1px_0px_rgba(255,255,255,0.2)];
  @apply flex items-center justify-center;
}
@utility button-icon {
  @apply text-white opacity-0 transition-opacity duration-200 text-xs;
}

@plugin "./icons.mjs";
</style>
