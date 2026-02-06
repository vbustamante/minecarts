<template>
  <div class="flex w-full items-center justify-between py-2">
    <h1>Minecarts!</h1>
    <div class="flex items-center gap-3">
      <span class="text-sm text-gray-400">{{ displayName }}</span>
      <button @click="handleLogout">
        <Icon icon="carbon:logout" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { Icon } from "@iconify/vue";
import { useAuthStore } from "../stores";
import { useRouter } from "vue-router";

const auth = useAuthStore();
const router = useRouter();

const displayName = computed(() => {
  if (!auth.user) return "";
  return auth.user.name || auth.user.email || auth.user.id;
});

async function handleLogout() {
  await auth.logout();
  router.push({ name: "login" });
}
</script>
