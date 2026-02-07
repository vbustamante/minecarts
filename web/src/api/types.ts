export interface Service {
  id: string;
  name: string;
  icon: string | null;
  createdAt: string;
  updatedAt: string;
  projectId: string;
}

export interface CreateServiceRequest {
  name: string;
}

export interface UpdateServiceRequest {
  name?: string;
  icon?: string;
}
