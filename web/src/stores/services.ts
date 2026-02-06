import { ref } from "vue";
import { defineStore } from "pinia";
import { listServices } from "../api/services";
import type { Service } from "../api/types";

export const useServiceStore = defineStore("services", {
  state: () => ({
    services: ref<Service[]>([]),
  }),
  actions: {
    async fetchAll(projectId: string) {
      this.services = await listServices(projectId);
    },
  },
});
