<script setup lang="ts">
// 导航栏中的用户按钮
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
