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

async function hide() {
  try {
    await invoke("hide_mms");
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
  <div>Home</div>
  <a-button type="primary" @click="login"> Login to mms </a-button>
  <a-button type="primary" @click="hide"> Hide mms </a-button>
  <a-button type="primary" @click="logout"> Logout mms </a-button>

  <div>{{ appState.appState.mms.logged }}</div>
</template>
