<template>
  <Listbox
    :model-value="projectStore.selectedProjectId"
    @update:model-value="projectStore.selectedProjectId = $event"
  >
    <div class="relative">
      <ListboxButton
        class="flex items-center gap-1 rounded-md bg-gray-800 px-3 py-1.5 text-sm hover:bg-gray-700"
        :class="selectedProject ? 'text-white' : 'text-neutral-500'"
      >
        {{ selectedProject?.name ?? "Select" }}
        <Icon icon="carbon:chevron-down" width="16" height="16" />
      </ListboxButton>
      <ListboxOptions
        class="absolute z-10 mt-1 max-h-60 w-56 overflow-auto rounded-md bg-neutral-900 py-1 shadow-lg ring-1 ring-white/10"
      >
        <ListboxOption
          v-for="project in projectStore.projects"
          :key="project.id"
          :value="project.id"
          v-slot="{ active, selected }"
        >
          <li
            class="cursor-pointer px-3 py-2 text-sm"
            :class="{
              'bg-indigo-600 text-white': active,
              'text-gray-300': !active,
            }"
          >
            <span :class="{ 'font-semibold': selected }">{{ project.name }}</span>
          </li>
        </ListboxOption>
      </ListboxOptions>
    </div>
  </Listbox>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { Icon } from "@iconify/vue";
import {
  Listbox,
  ListboxButton,
  ListboxOptions,
  ListboxOption,
} from "@headlessui/vue";
import { useProjectStore } from "../stores";

const projectStore = useProjectStore();

const selectedProject = computed(() =>
  projectStore.projects.find((p) => p.id === projectStore.selectedProjectId),
);
</script>
