import type { Service } from "./types";

export async function listServices(projectId: string): Promise<Service[]> {
  const res = await fetch(`/api/services/${projectId}`);
  if (!res.ok) throw new Error(`Failed to list services: ${res.status}`);
  return res.json();
}
