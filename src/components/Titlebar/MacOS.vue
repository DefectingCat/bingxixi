<script setup lang="ts">
const emit = defineEmits(["minimize", "maximize", "close"]);
const { isMaximized } = defineProps<{
  isMaximized: boolean;
}>();
</script>

<template>
  <div class="flex h-full items-center pl-2 pointer-events-auto">
    <button
      class="button-circle w-3 h-3 group bg-mac-red hover:bg-mac-red-hover hover:scale-105 active:scale-95"
      aria-label="Close window"
      @click="emit('close')"
    >
      <span
        class="icon-[fluent--add-12-regular] button-icon group-hover:opacity-100 active:opacity-0 w-2 h-2 rotate-45"
      ></span>
    </button>
    <button
      class="button-circle w-3 h-3 group bg-mac-yellow hover:bg-mac-yellow-hover hover:scale-105 active:scale-95"
      aria-label="Minimize window"
      @click="emit('minimize')"
    >
      <span
        class="icon-[fluent--minimize-24-filled] button-icon group-hover:opacity-100 active:opacity-0 w-2 h-2"
      ></span>
    </button>
    <button
      class="button-circle w-3 h-3 group bg-mac-green hover:bg-mac-green-hover hover:scale-105 active:scale-95"
      aria-label="Maximize window"
      @click="emit('maximize')"
    >
      <span
        class="icon-[fluent--arrow-minimize-20-filled] button-icon group-hover:opacity-100 active:opacity-0 w-2 h-2"
        v-if="isMaximized"
      ></span>
      <span
        class="icon-[fluent--arrow-maximize-16-filled] button-icon group-hover:opacity-100 active:opacity-0 w-2 h-2"
        v-else
      ></span>
    </button>
  </div>
</template>

<style scoped>
button {
  appearance: none;
  padding: 0;
  border: none;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  transition: background-color 0.2s;
  cursor: pointer;
  margin-right: 8px;
}
</style>
