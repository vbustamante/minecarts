import { defineStore } from "pinia";
import { ref } from "vue";
import type {
  Container,
  CreateContainerRequest,
  UpdateContainerRequest,
} from "../api/types";
import * as api from "../api/containers";

export const useContainerStore = defineStore("containers", {
  state: () => ({
    containers: ref<Map<string, Container>>(new Map()),
  }),
  actions: {
    async fetchAll() {
      const list = await api.listContainers();
      this.containers.clear();
      for (const c of list) {
        this.containers.set(c.id, c);
      }
    },
    async create(req: CreateContainerRequest): Promise<Container> {
      const c = await api.createContainer(req);
      this.containers.set(c.id, c);
      return c;
    },
    async update(
        id: string,
        req: UpdateContainerRequest,
    ): Promise<Container> {
      const c = await api.updateContainer(id, req);
      this.containers.set(c.id, c);
      return c;
    },
    handleEvent(event: { action: string; container: Container }) {
      switch (event.action) {
        case "created":
        case "updated":
          this.containers.set(event.container.id, event.container);
          break;
        case "deleted":
          this.containers.delete(event.container.id);
          break;
      }
    },
  },
});
