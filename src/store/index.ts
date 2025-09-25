import { defineStore } from "pinia";
import { reactive } from "vue";

export type AppState = {
  // 当前平台
  platform: string;
  // 暗色模式
  themeMode: "light" | "dark" | "auto";
  // mms 相关状态
  mss: MMSState;
};

export type MMSState = {
  logged: boolean;
  cookie: string;
};

export const useAppState = defineStore("appState", () => {
  const appState = reactive({
    platform: "unknown",
    themeMode: "auto",
    mms: {
      logged: false,
      cookie: "",
    },
  });

  function setPlatform(platform: string) {
    appState.platform = platform;
  }

  return { appState, setPlatform };
});
