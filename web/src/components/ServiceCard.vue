<template>
  <div
      :key="service.id"
      @click="$emit('select', service.id)"
      class="flex cursor-pointer items-center justify-between rounded-lg border border-neutral-200 bg-neutral-50 px-4 py-4 transition-colors hover:border-indigo-500 dark:border-neutral-700 dark:bg-neutral-800 dark:hover:border-indigo-400"
  >
    <div class="flex items-center gap-3 min-w-0">
      <img v-if="service.icon" :src="service.icon" alt="icon for service {{service.name}}" width="24" height="24">
      <Icon v-else icon="carbon:web-services-container" class="shrink-0 text-neutral-500" width="24" height="24"/>
      <div class="min-w-0 text-left" >
        <p class="font-medium truncate">
          {{ service.name }}
          <span class="text-xs text-neutral-500 ml-2">{{ service.id }}</span>
        </p>
        <div class="flex items-center gap-3 text-xs text-neutral-400 dark:text-neutral-500 mt-1">
          <span>Created {{ formatDate(service.createdAt) }}</span>
          <span v-if="service.createdAt !== service.updatedAt">Updated {{ formatDate(service.updatedAt) }}</span>
        </div>
      </div>
    </div>
    <button
        @click.stop="$emit('delete', service.id)"
        class="shrink-0 rounded p-1.5 text-neutral-400 hover:bg-neutral-200 hover:text-red-500 dark:hover:bg-neutral-700 dark:hover:text-red-400"
    >
      <Icon icon="carbon:trash-can" width="18" height="18" />
    </button>
  </div>
</template>

<script setup lang="ts">
import type { Service } from "../api/types";
import { Icon } from "@iconify/vue";
import { formatDate } from "../helpers";

defineProps<{ service: Service }>();

defineEmits<{
  (e: 'delete', id: string): void
  (e: 'select', id: string): void
}>();
</script>
