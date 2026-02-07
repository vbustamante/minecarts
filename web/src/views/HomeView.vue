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

    <CreateServiceModal :open="showCreate" @close="showCreate = false" />

    <div v-if="serviceStore.error" class="mt-4 rounded-lg border border-red-300 bg-red-50 p-4 text-red-800 dark:border-red-700 dark:bg-red-900/30 dark:text-red-300">
      {{ serviceStore.error }}
    </div>

    <div v-else-if="serviceStore.services.length === 0 && !loading" class="mt-12 flex flex-col items-center justify-center text-neutral-500">
      <p>No services yet. Click "Create Container" above to get started.</p>
    </div>

    <div v-else class="mt-4 grid gap-3">
      <ServiceCard v-for="service in serviceStore.services" :key="service.id" :service="service" @delete="confirmDelete" @select="selectService" />
    </div>

    <ConfirmDeleteModal
      :open="!!serviceToDelete"
      :service-name="serviceToDelete?.name ?? ''"
      :loading="deleting"
      @close="serviceToDelete = null"
      @confirm="onDelete"
    />

    <!-- Service Details Panel -->
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
              <button @click="closeServicePanel" class="rounded p-1 text-neutral-400 hover:bg-neutral-200 dark:hover:bg-neutral-700">
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

              <template v-if="selectedService.deployment">
                <dt class="text-neutral-500">Image</dt>
                <dd v-if="selectedService.deployment.image" class="font-mono truncate">{{ selectedService.deployment.image }}</dd>
                <dd v-else class="text-neutral-400">None</dd>

                <dt class="text-neutral-500">Instances</dt>
                <dd>{{ selectedService.deployment.instances.length }}</dd>
              </template>

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
import { computed, ref, Transition, Teleport, watch, onUnmounted } from "vue";
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

const loading = computed(() => loadingServices.value || refreshing.value);

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

const selectedService = ref<Service | null>(null);
function selectService(serviceId: string) {
  router.push({ name: "component", params: { projectId: projectsStore.selectedProjectId!, componentId: serviceId } });
}

function closeServicePanel() {
  router.push({ name: "project", params: { projectId: projectsStore.selectedProjectId! } });
}

function confirmDelete(serviceId: string) {
  serviceToDelete.value = serviceStore.services.find((s) => s.id === serviceId) ?? null;
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

// Sync componentId route param → selectedService
watch(
  [() => route.params.componentId as string | undefined, () => serviceStore.services],
  ([componentId]) => {
    if (componentId) {
      selectedService.value = serviceStore.services.find((s) => s.id === componentId) ?? null;
    } else {
      selectedService.value = null;
    }
  },
  { immediate: true },
);
</script>
