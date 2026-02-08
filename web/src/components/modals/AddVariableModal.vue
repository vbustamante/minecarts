<template>
  <Dialog :open="open" @close="handleClose" class="relative z-50">
    <div class="fixed inset-0 bg-black/30" aria-hidden="true" />

    <div class="fixed inset-0 flex items-center justify-center p-4">
      <DialogPanel class="w-full max-w-md rounded-lg bg-neutral-50 p-6 shadow-xl dark:bg-neutral-800">
        <DialogTitle class="text-lg font-semibold mb-4">Add Variable</DialogTitle>

        <form @submit.prevent="onSubmit" class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <label for="var-name" class="text-sm font-medium">Name</label>
            <input
              id="var-name"
              v-model="name"
              type="text"
              required
              :disabled="loading"
              placeholder="MY_VARIABLE"
              class="rounded border border-neutral-300 px-3 py-2 font-mono text-sm dark:border-neutral-600 dark:bg-neutral-700 disabled:opacity-50"
            />
          </div>

          <div class="flex flex-col gap-1">
            <label for="var-value" class="text-sm font-medium">Value</label>
            <textarea
              id="var-value"
              v-model="value"
              :disabled="loading"
              rows="3"
              class="rounded border border-neutral-300 px-3 py-2 font-mono text-sm dark:border-neutral-600 dark:bg-neutral-700 disabled:opacity-50"
            />
          </div>

          <div v-if="error" class="text-sm text-red-500">{{ error }}</div>

          <div class="flex justify-end gap-2 mt-2">
            <button
              type="button"
              @click="handleClose"
              :disabled="loading"
              class="rounded px-4 py-2 text-sm font-medium text-neutral-600 hover:bg-neutral-100 dark:text-neutral-300 dark:hover:bg-neutral-700 disabled:opacity-50"
            >
              Cancel
            </button>
            <button
              type="submit"
              :disabled="loading"
              class="flex items-center gap-2 rounded bg-neutral-700 px-4 py-2 text-sm font-medium text-neutral-100 hover:bg-neutral-600 disabled:opacity-50"
            >
              <Icon v-if="loading" icon="gg:spinner" width="16" height="16" class="animate-spin" />
              {{ loading ? "Adding..." : "Add" }}
            </button>
          </div>
        </form>
      </DialogPanel>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Dialog, DialogPanel, DialogTitle } from "@headlessui/vue";
import { Icon } from "@iconify/vue";
import { useProjectStore, useVariableStore } from "../../stores";

const props = defineProps<{ open: boolean; serviceId: string }>();
const emit = defineEmits<{ close: [] }>();

const projectStore = useProjectStore();
const variableStore = useVariableStore();
const name = ref("");
const value = ref("");
const loading = ref(false);
const error = ref<string | null>(null);

function handleClose() {
  if (!loading.value) {
    emit("close");
  }
}

async function onSubmit() {
  loading.value = true;
  error.value = null;

  if (!projectStore.selectedProjectId) {
    return;
  }

  try {
    await variableStore.upsert(projectStore.selectedProjectId, props.serviceId, name.value, value.value);
    name.value = "";
    value.value = "";
    emit("close");
  } catch {
    error.value = "Failed to add variable";
  } finally {
    loading.value = false;
  }
}
</script>
