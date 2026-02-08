<template>
  <div
      :key="service.id"
      @click="$emit('select', service.id)"
      class="flex cursor-pointer items-center justify-between rounded-lg border bg-neutral-50 px-4 py-4 transition-colors hover:border-indigo-500 dark:bg-neutral-800 dark:hover:border-indigo-400"
      :class="isSelected ? 'border-indigo-500 dark:border-indigo-400' : 'border-neutral-200 dark:border-neutral-700'"
  >
    <div class="flex items-center gap-3 min-w-0">
      <span class="relative shrink-0">
        <img v-if="service.icon" :src="service.icon" alt="icon for service {{service.name}}" width="24" height="24">
        <Icon v-else icon="carbon:web-services-container" class="text-neutral-500" width="24" height="24"/>
        <span
          class="absolute -right-0.5 -bottom-0.5 block h-2.5 w-2.5 rounded-full border-2 border-neutral-50 dark:border-neutral-800"
          :class="statusDotColor"
        />
      </span>
      <div class="min-w-0 text-left" >
        <p class="font-medium truncate">
          {{ service.name }}
          <span class="text-xs text-neutral-500 ml-2">{{ service.deployment?.image ?? "Loading..." }}</span>
        </p>
        <div class="flex items-center gap-3 text-xs text-neutral-400 dark:text-neutral-500 mt-1">
          <span>Created {{ formatDate(service.createdAt) }}</span>
          <span v-if="service.createdAt !== service.updatedAt">Updated {{ formatDate(service.updatedAt) }}</span>
        </div>
      </div>
    </div>
    <button
        @click.stop="$emit('delete', service.id)"
        class="shrink-0 rounded p-1.5 text-neutral-400 hover:bg-neutral-200 hover:text-red-500 dark:hover:bg-neutral-700 dark:hover:text-red-400 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      <Icon icon="carbon:trash-can" width="18" height="18" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import type { Service } from "../api/types";
import { Icon } from "@iconify/vue";
import { formatDate } from "../helpers";

const route = useRoute();
const props = defineProps<{ service: Service }>();

const isSelected = computed(() => route.params.componentId === props.service.id);

defineEmits<{
  (e: 'delete', id: string): void
  (e: 'select', id: string): void
}>();

const statusDotColor = computed(() => {
  const status = props.service.deployment?.status;
  if (!status) return "bg-neutral-400";
  switch (status) {
    case "SUCCESS": return "bg-green-500";
    case "SLEEPING": return "bg-yellow-500";
    case "CRASHED":
    case "FAILED":
    case "REMOVING": return "bg-red-500";
    default: return "bg-neutral-400";
  }
});
</script>
