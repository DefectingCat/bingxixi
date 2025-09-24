<script setup lang="ts">
import Navigate from "../components/Navigate.vue";
</script>

<template>
  <Navigate />

  <div>
    <Menu />
    <main class="flex-1">
      <router-view />
    </main>
  </div>
</template>

<style scoped></style>
