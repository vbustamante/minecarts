use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::railway::GraphQLResponse;

const GRAPHQL_URL: &str = "https://backboard.railway.com/graphql/v2";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RailwayService {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
}

#[derive(Serialize)]
pub struct ServiceWithDeployment {
    #[serde(flatten)]
    pub service: RailwayService,
    pub deployment: DeploymentInfo,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentInfo {
    pub created_at: String,
    pub status: Option<String>,
    pub instances: Vec<Instance>,
    pub image: Option<String>,
    pub repo: Option<String>,
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeploymentMeta {
    pub image: Option<String>,
    pub repo: Option<String>,
    pub branch: Option<String>,
}

#[derive(Deserialize)]
struct EnvironmentsConnection {
    edges: Vec<EnvironmentEdge>,
}

#[derive(Deserialize)]
struct EnvironmentEdge {
    node: Environment,
}

#[derive(Deserialize)]
struct Environment {
    name: String,
    deployments: DeploymentsConnection,
}

#[derive(Deserialize)]
struct DeploymentsConnection {
    edges: Vec<DeploymentEdge>,
}

#[derive(Deserialize)]
struct DeploymentEdge {
    node: Deployment,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Deployment {
    service_id: String,
    created_at: String,
    status: Option<String>,
    instances: Vec<Instance>,
    meta: Option<DeploymentMeta>,
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("{0}")]
    Request(#[from] reqwest::Error),
    #[error("No production environment")]
    NoProductionEnvironment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServiceRequest {
    pub name: String,
    pub project_id: String,
    pub image: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateServiceRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize)]
struct ProjectData {
    project: ProjectNode,
}

#[derive(Deserialize)]
struct ProjectNode {
    services: ServicesConnection,
    environments: EnvironmentsConnection,
}

#[derive(Deserialize)]
struct ServicesConnection {
    edges: Vec<ServiceEdge>,
}

#[derive(Deserialize)]
struct ServiceEdge {
    node: RailwayService,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceCreateData {
    service_create: RailwayService,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceUpdateData {
    service_update: RailwayService,
}

impl RailwayService {
    pub async fn list(access_token: String, project_id: &str) -> Result<Vec<ServiceWithDeployment>, ServiceError> {
        let body = serde_json::json!({
            "query": include_str!("graphql/project_services.gql"),
            "variables": { "id": project_id }
        });

        let client = Client::new();

        let res: GraphQLResponse<ProjectData> = client
            .post(GRAPHQL_URL)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let project = res.data.project;

        let production_env = project
            .environments
            .edges
            .into_iter()
            .map(|e| e.node)
            .find(|env| env.name == "production")
            .ok_or(ServiceError::NoProductionEnvironment)?;

        let mut deployments_by_service: HashMap<String, Deployment> = HashMap::new();
        for edge in production_env.deployments.edges {
            let deployment = edge.node;
            let service_id = deployment.service_id.clone();
            deployments_by_service
                .entry(service_id)
                .and_modify(|existing| {
                    if deployment.created_at > existing.created_at {
                        *existing = deployment.clone();
                    }
                })
                .or_insert(deployment);
        }

        let services = project
            .services
            .edges
            .into_iter()
            .map(|e| e.node)
            .filter_map(|service| {
                let deployment = deployments_by_service.remove(&service.id)?;

                let (image, repo, branch) = deployment.meta
                    .map(|meta| (meta.image, meta.repo, meta.branch))
                    .unwrap_or_default();

                Some(ServiceWithDeployment {
                    service,
                    deployment: DeploymentInfo {
                        created_at: deployment.created_at,
                        status: deployment.status,
                        instances: deployment.instances,
                        image,
                        repo,
                        branch,
                    },
                })
            })
            .collect();

        Ok(services)
    }

    pub async fn create(access_token: String, req: CreateServiceRequest) -> Result<Self, reqwest::Error> {
        let mut input = serde_json::json!({
            "projectId": req.project_id,
            "name": req.name,
            "source": { "image": req.image },
        });
        if let Some(icon) = &req.icon {
            input["icon"] = serde_json::Value::String(icon.clone());
        }
        let body = serde_json::json!({
            "query": include_str!("graphql/service_create.gql"),
            "variables": { "input": input }
        });

        let client = Client::new();

        let res: GraphQLResponse<ServiceCreateData> = client
            .post(GRAPHQL_URL)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(res.data.service_create)
    }

    pub async fn update(access_token: String, id: &str, req: UpdateServiceRequest) -> Result<Self, reqwest::Error> {
        let mut input = serde_json::Map::new();
        if let Some(name) = &req.name {
            input.insert("name".into(), serde_json::Value::String(name.clone()));
        }
        if let Some(icon) = &req.icon {
            input.insert("icon".into(), serde_json::Value::String(icon.clone()));
        }

        let body = serde_json::json!({
            "query": include_str!("graphql/service_update.gql"),
            "variables": {
                "id": id,
                "input": input,
            }
        });

        let client = Client::new();

        let res: GraphQLResponse<ServiceUpdateData> = client
            .post(GRAPHQL_URL)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(res.data.service_update)
    }

    pub async fn delete(access_token: String, id: &str) -> Result<(), reqwest::Error> {
        let body = serde_json::json!({
            "query": include_str!("graphql/service_delete.gql"),
            "variables": { "id": id }
        });

        let client = Client::new();
        client
            .post(GRAPHQL_URL)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        // TODO: error management when status is 200 but the graphql server returns an error

        Ok(())
    }
}
