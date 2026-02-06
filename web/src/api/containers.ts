import type {
  Container,
  CreateContainerRequest,
  UpdateContainerRequest,
} from "./types";

const BASE = "/api/containers";

export async function listContainers(): Promise<Container[]> {
  const res = await fetch(BASE);
  if (!res.ok) throw new Error(`Failed to list containers: ${res.status}`);
  return res.json();
}

export async function getContainer(id: string): Promise<Container> {
  const res = await fetch(`${BASE}/${id}`);
  if (!res.ok) throw new Error(`Failed to get container: ${res.status}`);
  return res.json();
}

export async function createContainer(
  req: CreateContainerRequest,
): Promise<Container> {
  const res = await fetch(BASE, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  if (!res.ok) throw new Error(`Failed to create container: ${res.status}`);
  return res.json();
}

export async function updateContainer(
  id: string,
  req: UpdateContainerRequest,
): Promise<Container> {
  const res = await fetch(`${BASE}/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  if (!res.ok) throw new Error(`Failed to update container: ${res.status}`);
  return res.json();
}

export async function deleteContainer(id: string): Promise<void> {
  const res = await fetch(`${BASE}/${id}`, { method: "DELETE" });
  if (!res.ok) throw new Error(`Failed to delete container: ${res.status}`);
}
