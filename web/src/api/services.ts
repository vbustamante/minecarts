import client from "./client";
import type { CreateServiceRequest, Service, UpdateServiceRequest } from "./types";

function base(projectId: string) {
  return `/projects/${projectId}/services`;
}

export async function listServices(projectId: string): Promise<Service[]> {
  const { data } = await client.get<Service[]>(base(projectId));
  return data;
}

export async function createService(projectId: string, req: CreateServiceRequest): Promise<Service> {
  const { data } = await client.post<Service>(base(projectId), req);
  return data;
}

export async function updateService(projectId: string, id: string, req: UpdateServiceRequest): Promise<Service> {
  const { data } = await client.put<Service>(`${base(projectId)}/${id}`, req);
  return data;
}

export async function deleteService(projectId: string, id: string): Promise<void> {
  await client.delete(`${base(projectId)}/${id}`);
}
