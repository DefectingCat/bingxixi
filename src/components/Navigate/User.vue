<script setup lang="ts">
// 导航栏中的用户按钮
import { UserIcon } from "tdesign-icons-vue-next";
import { useAppState } from "../../store";

const appState = useAppState();
const logged = appState.appState.mms.logged;
</script>

<template>
  <t-popup placement="bottom">
    <t-button variant="text" class="px-2!">
      <template #icon>
        <span class="icon-[fluent--person-16-regular]"></span>
      </template>
    </t-button>
    <template #content>
      <UserInfo />
    </template>
  </t-popup>
</template>

<style scoped></style>
