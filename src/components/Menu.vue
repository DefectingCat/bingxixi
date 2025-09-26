<script setup lang="ts">
// 左侧菜单栏
import { ref, computed } from "vue";
import { ButtonProps } from "tdesign-vue-next";
import { HomeIcon, GitRepositoryIcon } from "tdesign-icons-vue-next";
import { useRoute } from "vue-router";

const route = useRoute();
const path = computed(() => route.path);

const collapsed = ref(false);
const iconName = computed(() =>
  collapsed.value ? "chevron-right" : "chevron-left",
);
const changeCollapsed: ButtonProps["onClick"] = () => {
  collapsed.value = !collapsed.value;
};
</script>

<template>
  <t-menu :collapsed="collapsed" :value="path">
    <t-menu-item value="/app" routerLink :to="{ path: '/app' }">
      <template #icon>
        <home-icon
          :fill-color="'transparent'"
          :stroke-color="'currentColor'"
          :stroke-width="2"
        />
      </template>
      首页
    </t-menu-item>
    <t-menu-item value="/app/about" routerLink :to="{ path: '/app/about' }">
      <template #icon>
        <git-repository-icon
          :fill-color="'transparent'"
          :stroke-color="'currentColor'"
          :stroke-width="2"
        />
      </template>
      关于
    </t-menu-item>
    <t-submenu value="2" mode="popup">
      <template #icon>
        <t-icon name="mail" />
      </template>
      <template #title>
        <span>信息区</span>
      </template>
      <t-menu-item value="2-1"> 菜单内容一 </t-menu-item>
      <t-menu-item value="2-2"> 菜单内容二 </t-menu-item>
      <t-menu-item value="2-3"> 菜单内容三 </t-menu-item>
    </t-submenu>
    <t-menu-item value="item3">
      <template #icon>
        <t-icon name="play-circle" />
      </template>
      视频区
    </t-menu-item>
    <t-menu-item value="item4" :disabled="true">
      <template #icon>
        <t-icon name="edit-1" />
      </template>
      资源编辑
    </t-menu-item>
    <template #operations>
      <t-button
        class="t-demo-collapse-btn"
        variant="text"
        shape="square"
        @click="changeCollapsed"
      >
        <template #icon><t-icon :name="iconName" /></template>
      </t-button>
    </template>
  </t-menu>
</template>

<style scoped></style>
