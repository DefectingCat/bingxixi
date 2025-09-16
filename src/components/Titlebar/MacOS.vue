<template>
  <div class="flex h-full items-center pl-2">
    <button
      class="button-circle w-3 h-3 group bg-mac-red hover:bg-mac-red-hover hover:scale-105 active:scale-95 mr-2"
      aria-label="Close window"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="8"
        height="8"
        viewBox="0 0 24 24"
        class="button-icon group-hover:opacity-100 active:opacity-0"
      >
        <path
          fill="currentColor"
          d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z"
        />
      </svg>
    </button>
    <button
      class="button-circle w-3 h-3 group bg-mac-yellow hover:bg-mac-yellow-hover hover:scale-105 active:scale-95 mr-2"
      aria-label="Minimize window"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="8"
        height="8"
        viewBox="0 0 24 24"
        class="button-icon group-hover:opacity-100 active:opacity-0"
      >
        <path fill="currentColor" d="M19 13H5v-2h14z" />
      </svg>
    </button>
    <button
      class="button-circle w-3 h-3 group bg-mac-green hover:bg-mac-green-hover hover:scale-105 active:scale-95"
      aria-label="Maximize window"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="8"
        height="8"
        viewBox="0 0 24 24"
        class="button-icon group-hover:opacity-100 active:opacity-0"
      >
        <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />
      </svg>
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
}
</style>
