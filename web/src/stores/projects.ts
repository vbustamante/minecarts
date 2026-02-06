import { ref } from "vue";
import { defineStore } from "pinia";
import { listProjects, type Project } from "../api/projects";

export const useProjectStore = defineStore("projects", {
  state: () => ({
    projects: ref<Project[]>([]),
    selectedProjectId: ref<string | null>(null),
  }),
  actions: {
    async fetchAll() {
      this.projects = await listProjects();
    },
  },
});

