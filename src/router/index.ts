import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import Main from "../layouts/Main.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "",
      redirect: "/app",
    },
    {
      path: "/app",
      name: "main",
      component: Main,
      children: [
        {
          path: "",
          name: "home",
          component: HomeView,
        },
        {
          path: "about",
          name: "about",
          component: () => import("../views/AboutView.vue"),
        },
        {
          path: "login",
          name: "login",
          component: () => import("../views/LoginView.vue"),
        },
      ],
    },
  ],
});

export default router;
