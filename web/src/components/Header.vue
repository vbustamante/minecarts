<template>
  <div class="flex w-full items-center justify-between py-2">
    <div class="flex items-center gap-4">
      <h1>Minecarts!</h1>
      <div v-if="projectStore.selectedProjectId" class="flex items-center gap-2 mt-1">
        <span class="text-sm text-gray-400">Project:</span>
        <ProjectPicker />
      </div>
    </div>
    <div class="flex items-center gap-3">
      <img
        v-if="auth.user?.picture"
        :src="auth.user.picture"
        :alt="displayName"
        class="h-6 w-6 rounded-full"
      />
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
import ProjectPicker from "./ProjectPicker.vue";
import { useAuthStore, useProjectStore } from "../stores";
import { useRouter } from "vue-router";

const auth = useAuthStore();
const projectStore = useProjectStore();
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
