import { ref } from "vue";
import { defineStore } from "pinia";
import axios from "axios";
import * as api from "../api/services";
import type { CreateServiceRequest, Service, UpdateServiceRequest } from "../api/types";

export const useServiceStore = defineStore("services", {
  state: () => ({
    services: ref<Service[]>([]),
    projectId: ref<string | null>(null),
    error: ref<string | null>(null),
  }),
  actions: {
    async fetchAll(projectId: string) {
      this.projectId = projectId;
      try {
        this.services = await api.listServices(projectId);
        this.error = null;
      } catch (e) {
        this.services = [];
        if (axios.isAxiosError(e) && e.response?.data?.error) {
          this.error = e.response.data.error;
        } else {
          this.error = "Failed to load services";
        }
      }
    },
    async create(req: CreateServiceRequest): Promise<Service> {
      const service = await api.createService(this.projectId!, req);
      this.services.push(service);
      return service;
    },
    async update(id: string, req: UpdateServiceRequest): Promise<Service> {
      const service = await api.updateService(this.projectId!, id, req);
      const idx = this.services.findIndex((s) => s.id === id);
      if (idx !== -1) this.services[idx] = service;
      return service;
    },
    async remove(id: string) {
      await api.deleteService(this.projectId!, id);
      this.services = this.services.filter((s) => s.id !== id);
    },
  },
});
