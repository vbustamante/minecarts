<template>
  <Dialog :open="open" @close="handleClose" class="relative z-50">
    <div class="fixed inset-0 bg-black/30" aria-hidden="true" />

    <div class="fixed inset-0 flex items-center justify-center p-4">
      <DialogPanel class="w-full max-w-sm rounded-lg bg-neutral-50 p-6 shadow-xl dark:bg-neutral-800">
        <DialogTitle class="text-lg font-semibold mb-2">Delete Service</DialogTitle>
        <p class="text-sm text-neutral-500 dark:text-neutral-400 mb-4">
          Are you sure you want to delete <span class="font-medium text-neutral-700 dark:text-neutral-200">{{ serviceName }}</span>? This action cannot be undone.
        </p>

        <div class="flex justify-end gap-2">
          <button
            type="button"
            @click="handleClose"
            :disabled="loading"
            class="rounded px-4 py-2 text-sm font-medium text-neutral-600 hover:bg-neutral-100 dark:text-neutral-300 dark:hover:bg-neutral-700 disabled:opacity-50"
          >
            Cancel
          </button>
          <button
            type="button"
            @click="emit('confirm')"
            :disabled="loading"
            class="flex items-center gap-2 rounded bg-red-600 px-4 py-2 text-sm font-medium text-white hover:text-red-700 disabled:opacity-50"
          >
            <Icon v-if="loading" icon="gg:spinner" width="16" height="16" class="animate-spin" />
            <Icon v-else icon="carbon:trash-can" width="16" height="16" />
            {{ loading ? "Deleting..." : "Delete" }}
          </button>
        </div>
      </DialogPanel>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { Dialog, DialogPanel, DialogTitle } from "@headlessui/vue";
import { Icon } from "@iconify/vue";

const props = defineProps<{ open: boolean; serviceName: string; loading: boolean }>();
const emit = defineEmits<{ close: []; confirm: [] }>();

function handleClose() {
  if (!props.loading) {
    emit("close");
  }
}
</script>
