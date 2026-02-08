<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition-transform duration-300 ease-out"
      enter-from-class="translate-x-full"
      enter-to-class="translate-x-0"
      leave-active-class="transition-transform duration-300 ease-in"
      leave-from-class="translate-x-0"
      leave-to-class="translate-x-full"
    >
      <div v-if="service" class="fixed top-12 right-0 bottom-0 z-40 w-full max-w-md rounded-tl-xl border-l border-t border-neutral-700 bg-neutral-50 shadow-xl dark:bg-neutral-800">
        <div class="flex h-full flex-col p-6">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-2">
              <img v-if="service.icon" :src="service.icon" width="24" height="24" alt="" />
              <Icon v-else icon="carbon:web-services-container" class="shrink-0 text-neutral-500" width="24" height="24" />
              <h2 class="text-lg font-semibold">{{ service.name }}</h2>
            </div>
            <button @click="close" class="rounded p-1 text-neutral-400 hover:bg-neutral-200 dark:hover:bg-neutral-700">
              <Icon icon="carbon:close" width="20" height="20" />
            </button>
          </div>

          <!-- Tabs -->
          <div class="mb-4 flex border-b border-neutral-300 dark:border-neutral-600">
            <div
              @click="panelTab = 'details'"
              class="px-4 py-2 text-sm font-medium -mb-px border-b-2 transition-colors hover:cursor-pointer"
              :class="panelTab === 'details' ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400' : 'border-transparent text-neutral-500 hover:text-neutral-700 dark:hover:text-neutral-300'"
            >Details</div>
            <div
              @click="panelTab = 'variables'"
              class="px-4 py-2 text-sm font-medium -mb-px border-b-2 transition-colors hover:cursor-pointer"
              :class="panelTab === 'variables' ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400' : 'border-transparent text-neutral-500 hover:text-neutral-700 dark:hover:text-neutral-300'"
            >Variables <span v-if="variableCount !== null" class="ml-1 inline-flex h-4 min-w-4 items-center justify-center rounded-full bg-indigo-100 px-1 text-[10px] font-semibold text-indigo-700 dark:bg-indigo-900 dark:text-indigo-300">{{ variableCount > 9 ? '9+' : variableCount }}</span><Icon v-else icon="carbon:renew" width="12" height="12" class="ml-1 inline animate-spin text-neutral-400" /></div>
          </div>

          <!-- Details tab -->
          <dl v-if="panelTab === 'details'" class="grid grid-cols-[auto_1fr] gap-x-4 gap-y-3 text-sm">
            <dt class="text-neutral-500">ID</dt>
            <dd class="font-mono truncate">{{ service.id }}</dd>

            <dt class="text-neutral-500">Name</dt>
            <dd>{{ service.name }}</dd>

            <dt class="text-neutral-500">Project ID</dt>
            <dd class="font-mono truncate">{{ service.projectId }}</dd>

            <dt class="text-neutral-500">Icon</dt>
            <dd v-if="service.icon" class="font-mono">
              <a :href="service.icon" target="_blank">{{ service.icon }} <Icon class="inline" icon="carbon:link"/> </a>
            </dd>
            <dd v-else class="text-neutral-400">None</dd>

            <dt class="text-neutral-500">Status</dt>
            <dd v-if="service.deployment?.status">{{ service.deployment.status }}</dd>
            <dd v-else class="text-neutral-400">No deployment</dd>

            <template v-if="service.deployment">
              <dt class="text-neutral-500">Image</dt>
              <dd v-if="service.deployment.image" class="font-mono truncate">{{ service.deployment.image }}</dd>
              <dd v-else class="text-neutral-400">None</dd>

              <dt class="text-neutral-500">Instances</dt>
              <dd>{{ service.deployment.instances.length }}</dd>
            </template>

            <dt class="text-neutral-500">Created</dt>
            <dd>{{ formatDate(service.createdAt) }}</dd>

            <dt class="text-neutral-500">Updated</dt>
            <dd>{{ formatDate(service.updatedAt) }}</dd>
          </dl>

          <!-- Variables tab -->
          <div v-else-if="panelTab === 'variables'" class="flex-1 flex flex-col overflow-y-auto">
            <div class="mb-3 flex items-start justify-between gap-2">
              <div class="flex-1 rounded-lg bg-indigo-50 px-3 py-2 text-xs text-indigo-800 dark:bg-indigo-950 dark:text-indigo-300">
              Variables can reference other variables using <code v-pre class="rounded bg-indigo-100 px-1 font-mono dark:bg-indigo-900">${{NAMESPACE.VAR}}</code> syntax.
              <a href="https://docs.railway.com/variables/reference#template-syntax" target="_blank" class="underline hover:text-indigo-600 dark:hover:text-indigo-200">Learn more</a>
              </div>
              <button
                @click="showAddVariable = true"
                class="shrink-0 flex items-center gap-1 rounded-lg bg-neutral-700 px-3 py-1.5 text-xs font-medium text-neutral-100 hover:bg-neutral-600"
              >
                <Icon icon="carbon:add" width="14" height="14" /> Add
              </button>
            </div>
            <AddVariableModal :open="showAddVariable" :service-id="service!.id" @close="showAddVariable = false" />
            <div v-if="variableStore.loading && !serviceVariables" class="flex items-center gap-2 text-sm text-neutral-500">
              <Icon icon="carbon:renew" width="16" height="16" class="animate-spin" /> Loading variables...
            </div>
            <div v-else-if="variableStore.error" class="text-sm text-red-500">{{ variableStore.error }}</div>
            <div v-else-if="serviceVariables && Object.keys(serviceVariables).length === 0" class="text-sm text-neutral-500">No variables set.</div>
            <div v-else-if="serviceVariables" class="flex flex-col gap-2">
              <div v-for="[name, value] in sortedVariables" :key="name" class="group flex items-start justify-between gap-2 rounded border border-neutral-200 px-3 py-2 dark:border-neutral-600">
                <div class="min-w-0 flex-1">
                  <div class="text-xs font-medium text-neutral-500">{{ name }}</div>
                  <div class="mt-0.5 break-all font-mono text-sm">{{ value }}</div>
                </div>
                <div class="flex shrink-0 gap-0.5">
                  <button @click="copyValue(value)" title="Copy value" class="rounded p-1 text-neutral-400 hover:bg-neutral-200 hover:text-neutral-600 dark:hover:bg-neutral-700 dark:hover:text-neutral-300">
                    <Icon icon="carbon:copy" width="14" height="14" />
                  </button>
                  <button @click="openEdit(name, value)" title="Edit variable" class="rounded p-1 text-neutral-400 hover:bg-neutral-200 hover:text-neutral-600 dark:hover:bg-neutral-700 dark:hover:text-neutral-300">
                    <Icon icon="carbon:edit" width="14" height="14" />
                  </button>
                  <button @click="confirmDeleteVariable(name)" title="Delete variable" class="rounded p-1 text-neutral-400 hover:bg-neutral-200 hover:text-red-500 dark:hover:bg-neutral-700 dark:hover:text-red-400">
                    <Icon icon="carbon:trash-can" width="14" height="14" />
                  </button>
                </div>
              </div>
            </div>
            <EditVariableModal
              :open="!!editingVariable"
              :service-id="service!.id"
              :var-name="editingVariable?.name ?? ''"
              :var-value="editingVariable?.value ?? ''"
              @close="editingVariable = null"
            />
            <ConfirmDeleteModal
              :open="!!deletingVariableName"
              title="Delete Variable"
              :name="deletingVariableName ?? ''"
              :loading="deletingVariable"
              @close="deletingVariableName = null"
              @confirm="removeVariable"
            />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref, Transition, Teleport, watch, onUnmounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Icon } from "@iconify/vue";
import AddVariableModal from "./modals/AddVariableModal.vue";
import ConfirmDeleteModal from "./modals/ConfirmDeleteModal.vue";
import EditVariableModal from "./modals/EditVariableModal.vue";
import { useProjectStore, useServiceStore, useVariableStore } from "../stores";
import { formatDate } from "../helpers";

const route = useRoute();
const router = useRouter();
const projectsStore = useProjectStore();
const serviceStore = useServiceStore();
const variableStore = useVariableStore();

const service = computed(() => {
  const id = route.params.componentId as string | undefined;
  return id ? serviceStore.servicesById[id] ?? null : null;
});

const panelTab = ref<"details" | "variables">("details");
const showAddVariable = ref(false);
const editingVariable = ref<{ name: string; value: string } | null>(null);

function copyValue(value: string) {
  navigator.clipboard.writeText(value);
}

function openEdit(name: string, value: string) {
  editingVariable.value = { name, value };
}

const deletingVariableName = ref<string | null>(null);
const deletingVariable = ref(false);

function confirmDeleteVariable(name: string) {
  deletingVariableName.value = name;
}

async function removeVariable() {
  if (!service.value || !projectsStore.selectedProjectId || !deletingVariableName.value) return;
  deletingVariable.value = true;
  try {
    await variableStore.remove(projectsStore.selectedProjectId, service.value.id, deletingVariableName.value);
    deletingVariableName.value = null;
  } finally {
    deletingVariable.value = false;
  }
}

const serviceVariables = computed(() =>
  service.value ? variableStore.byService[service.value.id] : undefined,
);

const variableCount = computed(() =>
  serviceVariables.value ? Object.keys(serviceVariables.value).length : null,
);

const sortedVariables = computed(() => {
  if (!serviceVariables.value) return [];
  return Object.entries(serviceVariables.value).sort(([a], [b]) => a.localeCompare(b));
});

let variablePollTimer: ReturnType<typeof setInterval> | null = null;

function startVariablePolling() {
  stopVariablePolling();
  if (service.value && projectsStore.selectedProjectId) {
    variableStore.fetch(projectsStore.selectedProjectId, service.value.id);
    variablePollTimer = setInterval(() => {
      if (service.value && projectsStore.selectedProjectId) {
        variableStore.fetch(projectsStore.selectedProjectId, service.value.id);
      }
    }, 10_000);
  }
}

function stopVariablePolling() {
  if (variablePollTimer !== null) {
    clearInterval(variablePollTimer);
    variablePollTimer = null;
  }
}

watch(() => service.value?.id, (newId, oldId) => {
  if (newId !== oldId) {
    panelTab.value = "details";
    if (newId) {
      startVariablePolling();
    } else {
      stopVariablePolling();
    }
  }
}, { immediate: true });

onUnmounted(stopVariablePolling);

function close() {
  router.push({ name: "project", params: { projectId: projectsStore.selectedProjectId! } });
}
</script>
