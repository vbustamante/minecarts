<template>
  <div v-if="!projectsStore.selectedProjectId" class="flex flex-col min-h-[60vh] items-center justify-center gap-4">
    <template v-if="projectsStore.fetched && projectsStore.projects.length === 0">
      <div class="rounded-lg bg-red-500/10 border border-red-500/30 px-6 py-4 text-red-400 text-center max-w-md">
        <p class="font-semibold mb-2">No Railway projects found</p>
        <p class="text-sm">
          You need to create a project on Railway first. Go to
          <a href="https://railway.com/new" target="_blank" rel="noopener noreferrer" class="underline text-red-300 hover:text-red-200">railway.com/new</a>
          and select "Empty Project", then come back to this page.
        </p>
      </div>
    </template>
    <template v-else-if="projectsStore.fetched">
      <h2 class="mb-4 text-xl font-semibold">Select a Railway project to get started:</h2>
      <ProjectPicker />
    </template>
  </div>

  <div v-else>
    <div class="mt-4 flex items-center gap-2 justify-between">
      <button
        @click="showCreate = true"
        class="flex items-center gap-1 rounded-full bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
      >
        Create Container <Icon icon="carbon:add" width="20" height="20" />
      </button>
      <div class="flex items-center gap-1">
        <button
          @click="refresh"
          :disabled="loading"
          title="Refresh"
          class="transition-colors"
          :class="loading ? 'text-gray-600 cursor-not-allowed' : 'text-gray-400 hover:bg-gray-100 '"
        >
          <Icon icon="carbon:renew" width="22" height="22" :class="{ 'animate-spin': loading }" />
        </button>
        <select
          v-model="autoRefreshInterval"
          title="Auto refresh interval"
          class="text-sm text-gray-400"
        >
          <option v-for="opt in intervalOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
        </select>
      </div>
    </div>

    <div class="mt-2 flex items-center gap-2">
      <div class="relative flex-1">
        <Icon icon="carbon:search" width="18" height="18" class="absolute left-3 top-1/2 -translate-y-1/2 text-neutral-400" />
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search services..."
          class="w-full rounded-lg border border-neutral-300 bg-transparent py-2 pl-9 pr-3 text-sm dark:border-neutral-600"
        />
      </div>
      <select
        v-model="sortBy"
        title="Sort order"
        class="rounded-lg border border-neutral-300 bg-transparent px-3 py-2 text-sm dark:border-neutral-600"
      >
        <option v-for="opt in sortOptions" :key="opt.value" :value="opt.value">{{ opt.label }}</option>
      </select>
    </div>

    <CreateServiceModal :open="showCreate" @close="showCreate = false" />

    <div v-if="serviceStore.error" class="mt-4 rounded-lg border border-red-300 bg-red-50 p-4 text-red-800 dark:border-red-700 dark:bg-red-900/30 dark:text-red-300">
      {{ serviceStore.error }}
    </div>

    <div v-else-if="serviceStore.services.length === 0 && !loading" class="mt-12 flex flex-col items-center justify-center text-neutral-500">
      <p>No services yet. Click "Create Container" above to get started.</p>
    </div>

    <div v-else-if="filteredServices.length === 0" class="mt-12 flex flex-col items-center justify-center text-neutral-500">
      <p>No services match your search.</p>
    </div>

    <div v-else class="mt-4 grid gap-3">
      <ServiceCard v-for="service in filteredServices" :key="service.id" :service="service" @delete="confirmDelete" @select="selectService" />
    </div>

    <ConfirmDeleteModal
      :open="!!serviceToDelete"
      title="Delete Service"
      :name="serviceToDelete?.name ?? ''"
      :loading="deleting"
      @close="serviceToDelete = null"
      @confirm="onDelete"
    />

    <ServiceDetailsPanel />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, onUnmounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Icon } from "@iconify/vue";
import ProjectPicker from "../components/ProjectPicker.vue";
import CreateServiceModal from "../components/modals/CreateServiceModal.vue";
import ConfirmDeleteModal from "../components/modals/ConfirmDeleteModal.vue";
import ServiceDetailsPanel from "../components/ServiceDetailsPanel.vue";
import { useProjectStore, useServiceStore } from "../stores";
import type { Service } from "../api/types";
import ServiceCard from "../components/ServiceCard.vue";

const route = useRoute();
const router = useRouter();
const projectsStore = useProjectStore();
const serviceStore = useServiceStore();
const showCreate = ref(false);
const loadingServices = ref(false);
const refreshing = ref(false);
const serviceToDelete = ref<Service | null>(null);

const loading = computed(() => loadingServices.value || refreshing.value);

const searchQuery = ref("");
type SortKey = "newest" | "oldest" | "alpha" | "updated-desc" | "updated-asc";
const sortOptions: { label: string; value: SortKey }[] = [
  { label: "Newest", value: "newest" },
  { label: "Oldest", value: "oldest" },
  { label: "Alphabetical", value: "alpha" },
  { label: "Latest Updated", value: "updated-desc" },
  { label: "Earliest Updated", value: "updated-asc" },
];
const sortBy = ref<SortKey>("newest");

const filteredServices = computed(() => {
  let result = serviceStore.services;

  const q = searchQuery.value.toLowerCase().trim();
  if (q) {
    result = result.filter((s) => s.name.toLowerCase().includes(q));
  }

  return [...result].sort((a, b) => {
    switch (sortBy.value) {
      case "newest":
        return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
      case "oldest":
        return new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime();
      case "alpha":
        return a.name.localeCompare(b.name);
      case "updated-desc":
        return new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime();
      case "updated-asc":
        return new Date(a.updatedAt).getTime() - new Date(b.updatedAt).getTime();
    }
  });
});

const intervalOptions = [
  { label: "Off", value: 0 },
  { label: "10s", value: 10_000 },
  { label: "30s", value: 30_000 },
  { label: "1m", value: 60_000 },
  { label: "5m", value: 300_000 },
];
const autoRefreshInterval = ref(30_000);
let autoRefreshTimer: ReturnType<typeof setInterval> | null = null;

function startAutoRefresh() {
  stopAutoRefresh();
  if (autoRefreshInterval.value > 0) {
    autoRefreshTimer = setInterval(() => refresh(), autoRefreshInterval.value);
  }
}

function stopAutoRefresh() {
  if (autoRefreshTimer !== null) {
    clearInterval(autoRefreshTimer);
    autoRefreshTimer = null;
  }
}

watch(autoRefreshInterval, startAutoRefresh, { immediate: true });
onUnmounted(stopAutoRefresh);

async function refresh() {
  if (refreshing.value || !projectsStore.selectedProjectId) return;
  refreshing.value = true;
  const minDelay = new Promise((r) => setTimeout(r, 2000));
  await Promise.all([serviceStore.fetchAll(projectsStore.selectedProjectId), minDelay]);
  refreshing.value = false;
}

function selectService(serviceId: string) {
  router.push({ name: "component", params: { projectId: projectsStore.selectedProjectId!, componentId: serviceId } });
}

function confirmDelete(serviceId: string) {
  serviceToDelete.value = serviceStore.servicesById[serviceId] ?? null;
}

const deleting = ref(false);
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
