import { defineStore } from "pinia";
import axios from "axios";
import * as api from "../api/services";
import type { CreateServiceRequest, Service, UpdateServiceRequest } from "../api/types";

export const useServiceStore = defineStore("services", {
  state: () => ({
    servicesById: {} as Record<string, Service>,
    projectId: null as string | null,
    error: null as string | null,
  }),
  getters: {
    services: (state): Service[] => Object.values(state.servicesById),
  },
  actions: {
    async fetchAll(projectId: string) {
      this.projectId = projectId;
      try {
        const fetched = await api.listServices(projectId);
        const fetchedIds = new Set(fetched.map((s) => s.id));
        for (const id of Object.keys(this.servicesById)) {
          if (!fetchedIds.has(id)) {
            delete this.servicesById[id];
          }
        }
        for (const service of fetched) {
          this.servicesById[service.id] = service;
        }
        this.error = null;
      } catch (e) {
        this.servicesById = {};
        if (axios.isAxiosError(e) && e.response?.data?.error) {
          this.error = e.response.data.error;
        } else {
          this.error = "Failed to load services";
        }
      }
    },
    async create(req: CreateServiceRequest): Promise<Service> {
      const service = await api.createService(this.projectId!, req);
      this.servicesById[service.id] = service;
      return service;
    },
    async update(id: string, req: UpdateServiceRequest): Promise<Service> {
      const service = await api.updateService(this.projectId!, id, req);
      this.servicesById[id] = service;
      return service;
    },
    async remove(id: string) {
      await api.deleteService(this.projectId!, id);
      delete this.servicesById[id];
    },
  },
});
