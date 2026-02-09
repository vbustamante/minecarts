export interface DeploymentInfo {
  id: string;
  status: string | null;
  instances: { id: string }[];
  image?: string;
  repo?: string;
  branch?: string;
}

export interface Service {
  id: string;
  name: string;
  icon: string | null;
  createdAt: string;
  updatedAt: string;
  projectId: string;
  deployment?: DeploymentInfo;
}

export interface CreateServiceRequest {
  name: string;
  image: string;
  icon?: string;
}

export interface UpdateServiceRequest {
  name?: string;
  icon?: string;
}
