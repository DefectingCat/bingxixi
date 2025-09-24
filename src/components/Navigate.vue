<script setup lang="ts">
// 顶部导航栏
// 显式 logo 以及常用按钮，右侧显式头像

import DarkMode from "./Navigate/DarkMode.vue";
</script>

<template>
  <div
    class="flex items-center h-15 w-full bg-white dark:bg-bg-2 px-4 justify-between"
  >
    <div class="">并夕夕</div>
    <div class="flex items-center">
      <DarkMode />
    </div>
  </div>
</template>

<style scoped></style>
