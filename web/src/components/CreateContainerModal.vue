<template>
  <Dialog :open="open" @close="$emit('close')" class="relative z-50">
    <div class="fixed inset-0 bg-black/30" aria-hidden="true" />

    <div class="fixed inset-0 flex items-center justify-center p-4">
      <DialogPanel class="w-full max-w-md rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
        <DialogTitle class="text-lg font-semibold mb-4">Create Container</DialogTitle>

        <form @submit.prevent="onSubmit" class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <label for="name" class="text-sm font-medium">Name</label>
            <input
              id="name"
              v-model="name"
              type="text"
              required
              class="rounded border border-gray-300 px-3 py-2 dark:border-gray-600 dark:bg-gray-700"
            />
          </div>

          <div class="flex flex-col gap-1">
            <label for="image" class="text-sm font-medium">Image</label>
            <input
              id="image"
              v-model="image"
              type="text"
              required
              class="rounded border border-gray-300 px-3 py-2 dark:border-gray-600 dark:bg-gray-700"
            />
          </div>

          <div class="flex justify-end gap-2 mt-2">
            <button
              type="button"
              @click="$emit('close')"
              class="rounded px-4 py-2 text-sm font-medium hover:bg-gray-100 dark:hover:bg-gray-700"
            >
              Cancel
            </button>
            <button
              type="submit"
              class="rounded bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
            >
              Create
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
import { useContainerStore } from "../stores/containers";

defineProps<{ open: boolean }>();
const emit = defineEmits<{ close: [] }>();

const store = useContainerStore();
const name = ref("");
const image = ref("");

async function onSubmit() {
  await store.create({ name: name.value, image: image.value });
  name.value = "";
  image.value = "";
  emit("close");
}
</script>
