import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import LoginView from "../views/LoginView.vue";
import { useAuthStore } from "../stores";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/project/:projectId",
      name: "project",
      component: HomeView,
    },
    {
      path: "/project/:projectId/component/:componentId",
      name: "component",
      component: HomeView,
    },
    {
      path: "/login",
      name: "login",
      component: LoginView,
      meta: { skipAuth: true },
    },
  ],
});

router.beforeEach(async (to) => {
  const auth = useAuthStore();

  if (to.query.session_id) {
    auth.setToken(to.query.session_id as string);
    return { path: to.path, query: {}, replace: true };
  }

  if (to.meta.skipAuth) return;

  await auth.checkAuth();

  if (!auth.isAuthenticated) {
    return { name: "login" };
  }
});

export default router;
