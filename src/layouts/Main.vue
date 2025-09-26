<script setup lang="ts"></script>

<template>
  <div class="flex flex-col h-full">
    <Navigate />

    <div class="flex justify-between flex-1">
      <Menu />
      <main class="flex-1">
        <router-view />
      </main>
    </div>
  </div>
</template>

<style scoped></style>
