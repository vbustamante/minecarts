export interface User {
  id: string;
  name: string | null;
  email: string | null;
}

export const loginUrl = "/api/auth/login";

export async function fetchMe(): Promise<User | null> {
  const res = await fetch("/api/auth/me");
  if (res.status === 401) return null;
  if (!res.ok) throw new Error(`Failed to fetch user: ${res.status}`);
  return res.json();
}

export async function logout(): Promise<void> {
  await fetch("/api/auth/logout", { method: "POST" });
}
