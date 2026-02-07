<template>
  <Header v-if="isAuthenticated" />
  <router-view />
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import { useAuthStore, useProjectStore } from "./stores";
import {computed, watch} from "vue";

const auth = useAuthStore();
const projectStore = useProjectStore();

const isAuthenticated = computed(() => auth.isAuthenticated);

watch(isAuthenticated, (authenticated) => {
  if (authenticated) {
    projectStore.fetchAll();
  }
}, {immediate: true})

</script>
