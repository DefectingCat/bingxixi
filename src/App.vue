<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { MMSState, useAppState } from "./store";
import { listen } from "@tauri-apps/api/event";
import { watch } from "vue";

function setSystemTheme() {
  const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");
  darkThemeMq.addEventListener("change", (e) => {
    if (e.matches) {
      document.body.setAttribute("theme-mode", "dark");
      document.documentElement.setAttribute("theme-mode", "dark");
    } else {
      document.body.removeAttribute("theme-mode");
      document.documentElement.removeAttribute("theme-mode");
    }
  });
}

// 检测当前平台，用于设置 macOS 与其他平台的标题栏样式
const { appState, setPlatform } = useAppState();
async function init() {
  const platform = await invoke<string>("platform");
  setPlatform(platform);

  // 启动时先获取下状态
  const mmsStore = await invoke<MMSState>("get_mms_store");
  appState.mms.logged = mmsStore.logged;
  appState.mms.cookie = mmsStore.cookie;

  // 全局监听 mms 的状态
  listen<MMSState>("mms_store", (event) => {
    appState.mms.logged = event.payload.logged;
    appState.mms.cookie = event.payload.cookie;
  });

  // 适配系统暗色模式
  if (!("theme" in localStorage)) {
    setSystemTheme();
    appState.themeMode = "auto";
  } else {
    appState.themeMode = localStorage.theme;
  }
}
init().catch(console.error);

watch(appState, () => {
  if (appState.themeMode === "auto") {
    setSystemTheme();
  }
});
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
@custom-variant dark (&:where([theme-mode=dark], [theme-mode=dark] *));

@theme {
  --color-mac-red: #ff5f58;
  --color-mac-yellow: #ffbd2e;
  --color-mac-green: #14c13f;
  --color-mac-red-hover: #e04e4a;
  --color-mac-yellow-hover: #e0a828;
  --color-mac-green-hover: #12a837;

  /* arco */
  --color-bg-1: #17171a;
  --color-bg-2: #232324;
  --color-text-1: hsla(0, 0%, 100%, 0.9);
  --color-text-2: hsla(0, 0%, 100%, 0.7);
}

html,
body {
  background-color: transparent;
  height: 100%;
  @apply dark:text-text-2;
}

#app {
  @apply bg-gray-100 dark:bg-bg-1 rounded-2xl h-full;
  @apply overflow-hidden;
  @apply flex flex-col;
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
