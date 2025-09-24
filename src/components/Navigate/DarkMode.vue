<script setup lang="ts">
import { ref } from "vue";

const themeMode = ref<"light" | "dark" | "auto">("auto");
const handleTheme = (theme: "light" | "dark" | "auto") => {
  switch (theme) {
    case "light":
      themeMode.value = "light";
      document.body.setAttribute("arco-theme", "light");
      document.documentElement.setAttribute("arco-theme", "light");
      localStorage.theme = "light";
      break;
    case "dark":
      themeMode.value = "dark";
      document.body.setAttribute("arco-theme", "dark");
      document.documentElement.setAttribute("arco-theme", "dark");
      localStorage.theme = "dark";
      break;
    case "auto":
      themeMode.value = "auto";
      const darkMode = window.matchMedia(
        "(prefers-color-scheme: dark)",
      ).matches;
      document.body.setAttribute("arco-theme", darkMode ? "dark" : "light");
      document.documentElement.setAttribute(
        "arco-theme",
        darkMode ? "dark" : "light",
      );
      localStorage.removeItem("theme");
      break;
  }
};
</script>

<template>
  <a-dropdown trigger="hover" @select="handleTheme">
    <a-button type="text" class="">
      <template #icon>
        <div class="text-text-1 dark:text-text-2">
          <span
            class="icon-[fluent--weather-sunny-16-regular]"
            v-if="themeMode === 'light'"
          ></span>
          <span
            class="icon-[fluent--weather-moon-16-regular]"
            v-if="themeMode === 'dark'"
          ></span>
          <span
            class="icon-[fluent--device-meeting-room-16-regular]"
            v-if="themeMode === 'auto'"
          ></span>
        </div>
      </template>
    </a-button>
    <template #content>
      <a-doption value="light">
        <div class="flex items-center">
          <span class="icon-[fluent--weather-sunny-16-regular] mr-2"></span>
          <div>亮色模式</div>
        </div>
      </a-doption>
      <a-doption value="dark">
        <div class="flex items-center">
          <span class="icon-[fluent--weather-moon-16-regular] mr-2"></span>
          <div>暗色模式</div>
        </div>
      </a-doption>
      <a-doption value="auto">
        <div class="flex items-center">
          <span
            class="icon-[fluent--device-meeting-room-16-regular] mr-2"
          ></span>
          <div>跟随系统</div>
        </div>
      </a-doption>
    </template>
  </a-dropdown>
</template>

<style scoped></style>
