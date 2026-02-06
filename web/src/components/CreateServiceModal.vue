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
              <Icon v-if="loading" icon="carbon:loading" width="16" height="16" class="animate-spin" />
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
const loading = ref(false);

function handleClose() {
  if (!loading.value) {
    emit("close");
  }
}

async function onSubmit() {
  loading.value = true;
  try {
    await serviceStore.create({ name: name.value });
    name.value = "";
    emit("close");
  } finally {
    loading.value = false;
  }
}
</script>
