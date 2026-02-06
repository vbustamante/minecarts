export type ContainerStatus = "running" | "stopped" | "created";

export interface Container {
  id: string;
  name: string;
  image: string;
  status: ContainerStatus;
}

export interface CreateContainerRequest {
  name: string;
  image: string;
}

export interface UpdateContainerRequest {
  name?: string;
  image?: string;
  status?: ContainerStatus;
}

export type EventAction = "created" | "updated" | "deleted";

export interface ContainerEvent {
  action: EventAction;
  container: Container;
}
