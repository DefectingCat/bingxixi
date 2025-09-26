<script setup lang="tsx">
import { UserIcon } from "tdesign-icons-vue-next";
import { useRouter } from "vue-router";

const { logged } = defineProps<{
  logged: boolean;
}>();

const router = useRouter();
function login() {
  router.push("/app/login");
}
</script>

<template>
  <!-- 未登录 -->
  <div class="py-4 px-8" v-if="!logged">
    <div
      class="flex justify-center items-center rounded-full w-16 h-16 bg-gray-100 dark:bg-bg-2 p-4 text-gray-500 dark:text-text-2"
    >
      <user-icon
        :fill-color="'transparent'"
        :stroke-color="'currentColor'"
        :stroke-width="2"
        class="w-full! h-full!"
      />
    </div>

    <div class="mt-4 flex justify-center">
      <t-button size="small" variant="base" @click="login">未登录</t-button>
    </div>
  </div>
</template>

<style scoped></style>
