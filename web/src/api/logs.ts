import client from "./client";

export interface LogEntry {
  message: string;
}

export async function fetchBuildLogs(
  projectId: string,
  serviceId: string,
  deploymentId: string,
): Promise<LogEntry[]> {
  const { data } = await client.get<LogEntry[]>(
    `/projects/${projectId}/services/${serviceId}/deployments/${deploymentId}/logs/build`,
  );
  return data;
}

export async function fetchDeploymentLogs(
  projectId: string,
  serviceId: string,
  deploymentId: string,
): Promise<LogEntry[]> {
  const { data } = await client.get<LogEntry[]>(
    `/projects/${projectId}/services/${serviceId}/deployments/${deploymentId}/logs/deploy`,
  );
  return data;
}
