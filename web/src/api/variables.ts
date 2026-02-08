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
