<template>
  <div v-if="!projectsStore.selectedProjectId" class="flex flex-col min-h-[60vh] items-center justify-center gap-4">
    <h2 class="mb-4 text-xl font-semibold">Select a Railway project to get started:</h2>
    <ProjectPicker />
  </div>

  <div v-else>
    <div class="mt-4 flex items-center gap-2 justify-between">
      <button
        @click="showCreate = true"
        class="flex items-center gap-1 rounded-full bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
      >
        Create Container <Icon icon="carbon:add" width="20" height="20" />
      </button>
      <button
        @click="refresh"
        :disabled="loading"
        class="flex items-center justify-center rounded-full p-2 transition-colors"
        :class="loading ? 'text-gray-600 cursor-not-allowed' : 'text-gray-400 hover:bg-gray-100 '"
      >
        <Icon icon="carbon:renew" width="22" height="22" :class="{ 'animate-spin': loading }" />
      </button>
    </div>

    <CreateServiceModal :open="showCreate" @close="showCreate = false" />

    <div class="mt-4 grid gap-3">
      <ServiceCard v-for="service in serviceStore.services" :key="service.id" :service="service" @delete="confirmDelete" @select="selectService" />
    </div>

    <ConfirmDeleteModal
      :open="!!serviceToDelete"
      :service-name="serviceToDelete?.name ?? ''"
      :loading="deleting"
      @close="serviceToDelete = null"
      @confirm="onDelete"
    />

    <Teleport to="body">
      <Transition
        enter-active-class="transition-transform duration-300 ease-out"
        enter-from-class="translate-x-full"
        enter-to-class="translate-x-0"
        leave-active-class="transition-transform duration-300 ease-in"
        leave-from-class="translate-x-0"
        leave-to-class="translate-x-full"
      >
        <div v-if="selectedService" class="fixed top-12 right-0 bottom-0 z-40 w-full max-w-md rounded-tl-xl border-l border-t border-neutral-700 bg-neutral-50 shadow-xl dark:bg-neutral-800">
          <div class="flex h-full flex-col p-6">
            <div class="flex items-center justify-between mb-6">
              <div class="flex items-center gap-2">
                <img v-if="selectedService.icon" :src="selectedService.icon" width="24" height="24" alt="" />
                <Icon v-else icon="carbon:web-services-container" class="shrink-0 text-neutral-500" width="24" height="24" />
                <h2 class="text-lg font-semibold">{{ selectedService.name }}</h2>
              </div>
              <button @click="selectedService = null" class="rounded p-1 text-neutral-400 hover:bg-neutral-200 dark:hover:bg-neutral-700">
                <Icon icon="carbon:close" width="20" height="20" />
              </button>
            </div>

            <dl class="grid grid-cols-[auto_1fr] gap-x-4 gap-y-3 text-sm">
              <dt class="text-neutral-500">ID</dt>
              <dd class="font-mono truncate">{{ selectedService.id }}</dd>

              <dt class="text-neutral-500">Name</dt>
              <dd>{{ selectedService.name }}</dd>

              <dt class="text-neutral-500">Project ID</dt>
              <dd class="font-mono truncate">{{ selectedService.projectId }}</dd>

              <dt class="text-neutral-500">Icon</dt>
              <dd v-if="selectedService.icon" class="flex items-center gap-2">
                <img :src="selectedService.icon" width="20" height="20" alt="" />
              </dd>
              <dd v-else class="text-neutral-400">None</dd>

              <dt class="text-neutral-500">Created</dt>
              <dd>{{ formatDate(selectedService.createdAt) }}</dd>

              <dt class="text-neutral-500">Updated</dt>
              <dd>{{ formatDate(selectedService.updatedAt) }}</dd>
            </dl>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, Transition, Teleport, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Icon } from "@iconify/vue";
import ProjectPicker from "../components/ProjectPicker.vue";
import CreateServiceModal from "../components/CreateServiceModal.vue";
import ConfirmDeleteModal from "../components/ConfirmDeleteModal.vue";
import { useProjectStore, useServiceStore } from "../stores";
import type { Service } from "../api/types";
import ServiceCard from "../components/ServiceCard.vue";
import { formatDate } from "../helpers";

const route = useRoute();
const router = useRouter();
const projectsStore = useProjectStore();
const serviceStore = useServiceStore();
const showCreate = ref(false);
const loadingServices = ref(false);
const refreshing = ref(false);
const serviceToDelete = ref<Service | null>(null);
const selectedService = ref<Service | null>(null);
const deleting = ref(false);

const loading = computed(() => loadingServices.value || refreshing.value);

async function refresh() {
  if (refreshing.value || !projectsStore.selectedProjectId) return;
  refreshing.value = true;
  const minDelay = new Promise((r) => setTimeout(r, 2000));
  await Promise.all([serviceStore.fetchAll(projectsStore.selectedProjectId), minDelay]);
  refreshing.value = false;
}

function selectService(serviceId: string) {
  selectedService.value = serviceStore.services.find((s) => s.id === serviceId) ?? null;
}

function confirmDelete(serviceId: string) {
  serviceToDelete.value = serviceStore.services.find((s) => s.id === serviceId) ?? null;
}

async function onDelete() {
  if (!serviceToDelete.value) return;
  deleting.value = true;
  try {
    await serviceStore.remove(serviceToDelete.value.id);
    serviceToDelete.value = null;
  } finally {
    deleting.value = false;
  }
}

// Watch the project id on the route to make the store match
watch(
  () => route.params.projectId as string | undefined,
  (projectId) => {
    projectsStore.selectedProjectId = projectId ?? null;
  },
  { immediate: true },
);

// Watch the project id on the store to make the route match
watch(
  () => projectsStore.selectedProjectId,
  async (projectId) => {
    if (projectId) {
      if (route.params.projectId !== projectId) {
        router.replace({ name: "project", params: { projectId } });
      }
      loadingServices.value = true;
      await serviceStore.fetchAll(projectId);
      loadingServices.value = false;
    }
  },
  { immediate: true },
);
</script>
