import { ref, computed } from "vue";
import { defineStore } from "pinia";
import { fetchMe, logout as apiLogout, type User } from "../api/auth";

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const checked = ref(false);

  const isAuthenticated = computed(() => user.value !== null);

  async function checkAuth() {
    if (checked.value) return;
    user.value = await fetchMe();
    checked.value = true;
  }

  async function logout() {
    await apiLogout();
    user.value = null;
    checked.value = false;
  }

  return { user, checked, isAuthenticated, checkAuth, logout };
});
