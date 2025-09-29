<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useAppState } from "../store";

const appState = useAppState();

async function login() {
  try {
    await invoke("login_mms");
  } catch (e) {
    console.error(e);
  }
}

async function logout() {
  try {
    await invoke("logout_mms");
  } catch (e) {
    console.error(e);
  }
}
</script>

<template>
  <div>首页</div>
  <t-button type="primary" @click="login"> Login to mms </t-button>
  <t-button type="primary" @click="logout"> Logout mms </t-button>

  <div>{{ appState.appState.mms.logged }}</div>
</template>
