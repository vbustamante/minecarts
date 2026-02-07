<template>
  <Dialog :open="open" @close="handleClose" class="relative z-50">
    <div class="fixed inset-0 bg-black/30" aria-hidden="true" />

    <div class="fixed inset-0 flex items-center justify-center p-4">
      <DialogPanel class="w-full max-w-md rounded-lg bg-neutral-50 p-6 shadow-xl dark:bg-neutral-800">
        <DialogTitle class="text-lg font-semibold mb-4">Create Container</DialogTitle>

        <form @submit.prevent="onSubmit" class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <label for="name" class="text-sm font-medium">Name</label>
            <input
              id="name"
              v-model="name"
              type="text"
              required
              :disabled="loading"
              class="rounded border border-neutral-300 px-3 py-2 dark:border-neutral-600 dark:bg-neutral-700 disabled:opacity-50"
            />
          </div>

          <div class="flex flex-col gap-1">
            <label for="icon" class="text-sm font-medium">Icon URL</label>
            <div class="flex items-center gap-2">
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded border border-neutral-300 dark:border-neutral-600 overflow-hidden bg-white dark:bg-neutral-700">
                <img
                  v-if="icon && !iconError"
                  :src="icon"
                  class="h-full w-full object-contain"
                  @error="iconError = true"
                  alt="icon for service {{name}}"
                />
                <Icon v-else icon="carbon:image" class="text-neutral-400" width="20" height="20" />
              </div>
              <input
                id="icon"
                v-model="icon"
                type="url"
                placeholder="https://..."
                :disabled="loading"
                @input="iconError = false"
                class="w-full rounded border border-neutral-300 px-3 py-2 dark:border-neutral-600 dark:bg-neutral-700 disabled:opacity-50"
              />
            </div>
          </div>

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
              {{ loading ? "Creating..." : "Create" }}
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
import { useServiceStore } from "../stores";

defineProps<{ open: boolean }>();
const emit = defineEmits<{ close: [] }>();

const serviceStore = useServiceStore();
const name = ref("");
const icon = ref("");
const iconError = ref(false);
const loading = ref(false);

function handleClose() {
  if (!loading.value) {
    emit("close");
  }
}

async function onSubmit() {
  loading.value = true;
  try {
    await serviceStore.create({
      name: name.value,
      ...(icon.value ? { icon: icon.value } : {}),
    });
    name.value = "";
    icon.value = "";
    iconError.value = false;
    emit("close");
  } finally {
    loading.value = false;
  }
}
</script>
