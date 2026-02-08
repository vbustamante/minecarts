import client from "./client";

function base(projectId: string, serviceId: string) {
  return `/projects/${projectId}/services/${serviceId}/variables`;
}

export async function listVariables(
  projectId: string,
  serviceId: string,
): Promise<Record<string, string>> {
  const { data } = await client.get<Record<string, string>>(
    base(projectId, serviceId),
  );
  return data;
}

export async function upsertVariable(
  projectId: string,
  serviceId: string,
  name: string,
  value: string,
): Promise<void> {
  await client.put(base(projectId, serviceId), { name, value });
}

export async function deleteVariable(
  projectId: string,
  serviceId: string,
  name: string,
): Promise<void> {
  await client.delete(base(projectId, serviceId), { data: { name } });
}
