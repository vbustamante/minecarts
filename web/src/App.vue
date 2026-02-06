<template>
  <Header v-if="auth.isAuthenticated" />
  <router-view />
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import { useAuthStore, useProjectStore } from "./stores";
import { onMounted } from "vue";

const auth = useAuthStore();
const projectStore = useProjectStore();

onMounted(() => {
  projectStore.fetchAll();
});
</script>
