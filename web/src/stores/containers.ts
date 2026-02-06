import { defineStore } from "pinia";
import { ref } from "vue";
import type {
  Container,
  CreateContainerRequest,
  UpdateContainerRequest,
} from "../api/types";
import * as api from "../api/containers";
import { connectWebSocket } from "../api/ws";

export const useContainerStore = defineStore("containers", () => {
  const containers = ref<Map<string, Container>>(new Map());
  let ws: WebSocket | null = null;

  function handleEvent(event: { action: string; container: Container }) {
    switch (event.action) {
      case "created":
      case "updated":
        containers.value.set(event.container.id, event.container);
        break;
      case "deleted":
        containers.value.delete(event.container.id);
        break;
    }
  }

  function connect() {
    if (ws) return;
    ws = connectWebSocket(handleEvent);
  }

  async function fetchAll() {
    const list = await api.listContainers();
    containers.value.clear();
    for (const c of list) {
      containers.value.set(c.id, c);
    }
  }

  async function create(req: CreateContainerRequest): Promise<Container> {
    return api.createContainer(req);
  }

  async function update(
    id: string,
    req: UpdateContainerRequest,
  ): Promise<Container> {
    return api.updateContainer(id, req);
  }

  return { containers, connect, fetchAll, create, update };
});
