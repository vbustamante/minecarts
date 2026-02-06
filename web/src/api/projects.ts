export interface Project {
  id: string;
  name: string;
}


export async function listProjects(): Promise<Project[]> {
  const res = await fetch("/api/projects");
  if (!res.ok) throw new Error(`Failed to list projects: ${res.status}`);
  return res.json();
}
