import { defineStore } from "pinia";
import { reactive } from "vue";
import { MMSUser } from "../api";

export type AppState = {
  // 当前平台
  platform: string;
  // 暗色模式
  themeMode: "light" | "dark" | "auto";
  // mms 相关状态
  mms: MMSState;
  // mms 用户信息
  mmsUser: MMSUser | null;
};

export type MMSState = {
  logged: boolean;
  cookie: string;
};

export const useAppState = defineStore("appState", () => {
  const appState = reactive<AppState>({
    platform: "unknown",
    themeMode: "auto",
    mms: {
      logged: false,
      cookie: "",
    },
    mmsUser: null,
  });

  function setPlatform(platform: string) {
    appState.platform = platform;
  }

  return { appState, setPlatform };
});
