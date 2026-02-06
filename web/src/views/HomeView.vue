<template>
  <div v-if="!projectsStore.selectedProjectId" class="flex flex-col min-h-[60vh] items-center justify-center gap-4">
    <h2 class="mb-4 text-xl font-semibold">Select a Railway project to get started:</h2>
    <ProjectPicker />
  </div>

  <div v-else>
    <ul>
      <li v-for="service in serviceStore.services" :key="service.id"> - {{ service.name }}</li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { watch } from "vue";
import ProjectPicker from "../components/ProjectPicker.vue";
import { useProjectStore, useServiceStore } from "../stores";

const projectsStore = useProjectStore();
const serviceStore = useServiceStore();

watch(
  () => projectsStore.selectedProjectId,
  (projectId) => {
    if (projectId) {
      serviceStore.fetchAll(projectId);
    }
  },
  { immediate: true },
);
</script>
