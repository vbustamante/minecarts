import axios from "axios";
import { useAuthStore } from "../stores";
import router from "../router";

const client = axios.create({
  baseURL: import.meta.env.VITE_API_URL || "/api",
});

client.interceptors.request.use((config) => {
  const token = localStorage.getItem("session_token");
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

const skipInterceptor = new Set(["/auth/me", "/auth/logout"]);

client.interceptors.response.use(
  (response) => response,
  async (error) => {
    if (
      axios.isAxiosError(error) &&
      error.response?.status === 401 &&
      !skipInterceptor.has(error.config?.url ?? "")
    ) {
      const auth = useAuthStore();
      await auth.logout();
      router.push({ name: "login" });
    }
    return Promise.reject(error);
  },
);

export default client;
