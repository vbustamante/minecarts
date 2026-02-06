<template>
  <div v-if="!projectsStore.selectedProjectId" class="flex flex-col min-h-[60vh] items-center justify-center gap-4">
    <h2 class="mb-4 text-xl font-semibold">Select a Railway project to get started:</h2>
    <ProjectPicker />
  </div>

  <div v-else>
    <button
      @click="showCreate = true"
      class="mt-4 flex items-center gap-1 rounded-full bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
    >
      Create Container <Icon icon="carbon:add" width="20" height="20" />
    </button>

    <CreateServiceModal :open="showCreate" @close="showCreate = false" />

    <ul>
      <li v-for="service in serviceStore.services" :key="service.id"> - {{ service.name }}</li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { Icon } from "@iconify/vue";
import ProjectPicker from "../components/ProjectPicker.vue";
import CreateServiceModal from "../components/CreateServiceModal.vue";
import { useProjectStore, useServiceStore } from "../stores";

const projectsStore = useProjectStore();
const serviceStore = useServiceStore();
const showCreate = ref(false);
const loadingServices = ref(false);


watch(
  () => projectsStore.selectedProjectId,
  async (projectId) => {
    if (projectId) {
      loadingServices.value = true;
      await serviceStore.fetchAll(projectId);
      loadingServices.value = false;
    }
  },
  { immediate: true },
);
</script>
