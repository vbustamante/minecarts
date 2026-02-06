export interface Service {
  id: string;
  name: string;
  icon: string | null;
  created_at: string;
  project_id: string;
}

export interface CreateServiceRequest {
  name: string;
}

export interface UpdateServiceRequest {
  name?: string;
  icon?: string;
}
