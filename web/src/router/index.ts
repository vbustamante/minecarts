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
      path: "/login",
      name: "login",
      component: LoginView,
      meta: { skipAuth: true },
    },
  ],
});

router.beforeEach(async (to) => {
  if (to.meta.skipAuth) return;

  const auth = useAuthStore();
  await auth.checkAuth();

  if (!auth.isAuthenticated) {
    return { name: "login" };
  }
});

export default router;
