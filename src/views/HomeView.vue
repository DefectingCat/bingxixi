<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

async function login() {
  try {
    await invoke("login_mms");
  } catch (e) {
    console.error(e);
  }
}
</script>

<template>
  <div>Home</div>
  <a-button type="primary" @click="login"> Login to mms </a-button>
</template>
