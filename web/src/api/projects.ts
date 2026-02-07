import client from "./client";

export interface Project {
  id: string;
  name: string;
}

export async function listProjects(): Promise<Project[]> {
  const { data } = await client.get<Project[]>("/projects");
  return data;
}
