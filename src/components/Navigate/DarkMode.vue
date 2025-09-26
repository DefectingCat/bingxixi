<script setup lang="ts">
import { useAppState } from "../../store";

const { appState } = useAppState();
const handleTheme = (item: { value: "light" | "dark" | "auto" }) => {
  const theme = item.value;
  switch (theme) {
    case "light":
      appState.themeMode = "light";
      document.body.setAttribute("theme-mode", "light");
      document.documentElement.setAttribute("theme-mode", "light");
      localStorage.theme = "light";
      break;
    case "dark":
      appState.themeMode = "dark";
      document.body.setAttribute("theme-mode", "dark");
      document.documentElement.setAttribute("theme-mode", "dark");
      localStorage.theme = "dark";
      break;
    case "auto":
      appState.themeMode = "auto";
      const darkMode = window.matchMedia("(prefers-color-scheme: dark)");
      document.body.setAttribute(
        "theme-mode",
        darkMode.matches ? "dark" : "light",
      );
      document.documentElement.setAttribute(
        "theme-mode",
        darkMode.matches ? "dark" : "light",
      );
      localStorage.removeItem("theme");
      break;
  }
};
</script>

<template>
  <t-dropdown @click="handleTheme">
    <t-button variant="text" class="px-2!">
      <template #icon>
        <span
          class="icon-[fluent--weather-sunny-16-regular]"
          v-if="appState.themeMode === 'light'"
        ></span>
        <span
          class="icon-[fluent--weather-moon-16-regular]"
          v-if="appState.themeMode === 'dark'"
        ></span>
        <span
          class="icon-[fluent--device-meeting-room-16-regular]"
          v-if="appState.themeMode === 'auto'"
        ></span>
      </template>
    </t-button>
    <t-dropdown-menu>
      <t-dropdown-item value="light">
        <div class="flex items-center">
          <span class="icon-[fluent--weather-sunny-16-regular] mr-2"></span>
          <div>亮色模式</div>
        </div>
      </t-dropdown-item>
      <t-dropdown-item value="dark">
        <div class="flex items-center">
          <span class="icon-[fluent--weather-moon-16-regular] mr-2"></span>
          <div>暗色模式</div>
        </div>
      </t-dropdown-item>
      <t-dropdown-item value="auto">
        <div class="flex items-center">
          <span
            class="icon-[fluent--device-meeting-room-16-regular] mr-2"
          ></span>
          <div>跟随系统</div>
        </div>
      </t-dropdown-item>
    </t-dropdown-menu>
  </t-dropdown>
</template>

<style scoped></style>
