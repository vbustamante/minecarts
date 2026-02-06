import type { CreateServiceRequest, Service, UpdateServiceRequest } from "./types";

function base(projectId: string) {
  return `/api/projects/${projectId}/services`;
}

export async function listServices(projectId: string): Promise<Service[]> {
  const res = await fetch(base(projectId));
  if (!res.ok) throw new Error(`Failed to list services: ${res.status}`);
  return res.json();
}

export async function createService(projectId: string, req: CreateServiceRequest): Promise<Service> {
  const res = await fetch(base(projectId), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  if (!res.ok) throw new Error(`Failed to create service: ${res.status}`);
  return res.json();
}

export async function updateService(projectId: string, id: string, req: UpdateServiceRequest): Promise<Service> {
  const res = await fetch(`${base(projectId)}/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  if (!res.ok) throw new Error(`Failed to update service: ${res.status}`);
  return res.json();
}

export async function deleteService(projectId: string, id: string): Promise<void> {
  const res = await fetch(`${base(projectId)}/${id}`, { method: "DELETE" });
  if (!res.ok) throw new Error(`Failed to delete service: ${res.status}`);
}
