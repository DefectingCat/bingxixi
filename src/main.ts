import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia";
import router from "./router";
import "tdesign-vue-next/es/style/index.css";

createApp(App).use(createPinia()).use(router).mount("#app");
