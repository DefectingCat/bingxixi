import { defineStore } from "pinia";
import { reactive } from "vue";

export type AppState = {
  platform: string;
};

export const useAppState = defineStore("appState", () => {
  const appState = reactive({
    platform: "unknown",
  });

  function setPlatform(platform: string) {
    appState.platform = platform;
  }

  return { appState, setPlatform };
});
