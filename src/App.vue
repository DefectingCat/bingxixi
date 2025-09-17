<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useAppState } from "./store";
import Titlebar from "./components/Titlebar.vue";

const appState = useAppState();
async function init() {
  const platform = await invoke<string>("platform");
  appState.setPlatform(platform);
}
init();

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at jttps://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <Suspense>
    <Titlebar />
  </Suspense>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <a-button type="primary" html-type="submit">Greet</a-button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped></style>
<style>
@import "tailwindcss";

html,
body {
  background-color: transparent;
  height: 100%;
}

#app {
  @apply bg-gray-100 dark:bg-gray-900 rounded-xl h-full;
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
}
@utility button-icon {
  @apply text-white opacity-0 transition-opacity duration-200 text-xs;
}
</style>
