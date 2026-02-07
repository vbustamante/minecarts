import { ref } from "vue";
import { defineStore } from "pinia";
import { listProjects, type Project } from "../api/projects";

export const useProjectStore = defineStore("projects", {
  state: () => ({
    projects: ref<Project[]>([]),
    selectedProjectId: ref<string | null>(null),
  }),
  getters: {
    isSingleProject: (state) => state.projects.length === 1,
  },
  actions: {
    async fetchAll() {
      this.projects = await listProjects();
      if (this.projects.length === 1 && !this.selectedProjectId) {
        this.selectedProjectId = this.projects[0]!.id;
      }
    },
  },
});

