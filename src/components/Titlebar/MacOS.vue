<script setup lang="ts">
const emit = defineEmits(["minimize", "maximize", "close"]);
</script>

<template>
  <div class="flex h-full items-center pl-2 pointer-events-auto">
    <button
      class="button-circle w-[15px] h-[15px] group bg-mac-red hover:bg-mac-red-hover hover:scale-105 active:scale-95"
      aria-label="Close window"
      @click="emit('close')"
    >
      <span
        class="icon-[fluent--add-12-regular] button-icon group-hover:opacity-100 active:opacity-0 w-[9px] h-[10px] rotate-45"
      ></span>
    </button>
    <button
      class="button-circle w-[15px] h-[15px] group bg-mac-yellow hover:bg-mac-yellow-hover hover:scale-105 active:scale-95"
      aria-label="Minimize window"
      @click="emit('minimize')"
    >
      <span
        class="icon-[fluent--minimize-24-filled] button-icon group-hover:opacity-100 active:opacity-0 w-[9px] h-[10px]"
      ></span>
    </button>
    <button
      class="button-circle w-[15px] h-[15px] group bg-mac-green hover:bg-mac-green-hover hover:scale-105 active:scale-95"
      aria-label="Maximize window"
      @click="emit('maximize')"
    >
      <!-- <span -->
      <!--   class="icon-[fluent--arrow-minimize-20-filled] button-icon group-hover:opacity-100 active:opacity-0 w-[10px] h-[10px]" -->
      <!--   v-if="isMaximized" -->
      <!-- ></span> -->
      <span
        class="icon-[fluent--arrow-maximize-16-filled] button-icon group-hover:opacity-100 active:opacity-0 w-[9px] h-[10px]"
      ></span>
    </button>
  </div>
</template>

<style scoped>
button {
  appearance: none;
  padding: 0;
  border: none;
  transition: background-color 0.2s;
  cursor: pointer;
  margin-right: 8px;
}
</style>
