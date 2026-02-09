import client from "./client";

export interface User {
  id: string;
  name: string | null;
  email: string | null;
  picture?: string | null;
}

const baseUrl = import.meta.env.VITE_API_URL || "/api";
export const loginUrl = `${baseUrl}/auth/login`;

export async function fetchMe(): Promise<User | null> {
  try {
    const { data } = await client.get<User>("/auth/me");
    return data;
  } catch {
    return null;
  }
}

export async function logout(): Promise<void> {
  await client.post("/auth/logout");
}
