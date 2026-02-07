import client from "./client";

export interface User {
  id: string;
  name: string | null;
  email: string | null;
  picture?: string | null;
}

export const loginUrl = "/api/auth/login";

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
