<script setup lang="ts">
const emit = defineEmits(["minimize", "maximize", "close"]);
const { isMaximized } = defineProps<{
  isMaximized: boolean;
}>();
</script>

<template>
  <div class="flex item-center ml-auto pointer-events-auto">
    <button
      id="titlebar-minimize"
      title="minimize"
      class="hover:bg-gray-300"
      @click="emit('minimize')"
    >
      <span
        class="icon-[fluent--minimize-24-filled] button-icon opacity-100 active:opacity-0 w-5 h-5 text-gray-800"
      ></span>
    </button>
    <button
      id="titlebar-maximize"
      title="maximize"
      class="hover:bg-gray-300"
      @click="emit('maximize')"
    >
      <span
        class="icon-[fluent--arrow-minimize-20-filled] button-icon opacity-100 active:opacity-0 w-5 h-5 text-gray-800"
        v-if="isMaximized"
      ></span>
      <span
        class="icon-[fluent--arrow-maximize-16-filled] button-icon opacity-100 active:opacity-0 w-5 h-5 text-gray-800"
        v-else
      ></span>
    </button>
    <button
      id="titlebar-close"
      title="close"
      class="hover:bg-gray-300"
      @click="emit('close')"
    >
      <span
        class="icon-[fluent--add-12-regular] button-icon opacity-100 active:opacity-0 w-6 h-6 text-gray-800 rotate-45"
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
  width: 50px;
}
</style>
