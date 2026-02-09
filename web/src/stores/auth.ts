import { ref, computed } from "vue";
import { defineStore } from "pinia";
import { fetchMe, logout as apiLogout, type User } from "../api/auth";

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const checked = ref(false);
  const token = ref<string | null>(localStorage.getItem("session_token"));

  const isAuthenticated = computed(() => user.value !== null);

  function setToken(t: string) {
    token.value = t;
    localStorage.setItem("session_token", t);
  }

  function clearToken() {
    token.value = null;
    localStorage.removeItem("session_token");
  }

  async function checkAuth() {
    if (checked.value) return;
    if (!token.value) {
      checked.value = true;
      return;
    }
    user.value = await fetchMe();
    checked.value = true;
  }

  async function logout() {
    await apiLogout();
    clearToken();
    user.value = null;
    checked.value = false;
  }

  return { user, checked, isAuthenticated, token, setToken, clearToken, checkAuth, logout };
});
