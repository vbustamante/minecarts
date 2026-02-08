import { ref } from "vue";
import { defineStore } from "pinia";
import * as api from "../api/variables";

export const useVariableStore = defineStore("variables", {
  state: () => ({
    /** variables keyed by service ID → Record<varName, varValue> */
    byService: ref<Record<string, Record<string, string>>>({}),
    loading: ref(false),
    error: ref<string | null>(null),
  }),
  actions: {
    async fetch(projectId: string, serviceId: string) {
      this.loading = true;
      this.error = null;
      try {
        this.byService[serviceId] = await api.listVariables(projectId, serviceId);
      } catch {
        this.error = "Failed to load variables";
      } finally {
        this.loading = false;
      }
    },
    async upsert(projectId: string, serviceId: string, name: string, value: string) {
      await api.upsertVariable(projectId, serviceId, name, value);
      if (!this.byService[serviceId]) {
        this.byService[serviceId] = {};
      }
      this.byService[serviceId][name] = value;
    },
    async remove(projectId: string, serviceId: string, name: string) {
      await api.deleteVariable(projectId, serviceId, name);
      if (this.byService[serviceId]) {
        delete this.byService[serviceId][name];
      }
    },
  },
});
