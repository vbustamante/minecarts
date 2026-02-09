use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::railway::GraphQLResponse;

const GRAPHQL_URL: &str = "https://backboard.railway.com/graphql/v2";

// --- Public types sent to frontend ---

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<DeploymentInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentInfo {
    pub id: String,
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

// --- GQL response types ---

#[derive(Debug, Clone, Deserialize)]
struct GqlDeploymentMeta {
    image: Option<String>,
    repo: Option<String>,
    branch: Option<String>,
}

#[derive(Deserialize)]
struct GqlDeployment {
    id: String,
    status: Option<String>,
    instances: Vec<Instance>,
    meta: Option<GqlDeploymentMeta>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GqlServiceRef {
    icon: Option<String>,
    project_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GqlServiceInstance {
    service_id: String,
    service_name: String,
    service: GqlServiceRef,
    created_at: String,
    updated_at: String,
    latest_deployment: Option<GqlDeployment>,
}

#[derive(Deserialize)]
struct ServiceInstanceEdge {
    node: GqlServiceInstance,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GqlEnvironment {
    name: String,
    service_instances: Connection<ServiceInstanceEdge>,
}

#[derive(Deserialize)]
struct EnvironmentEdge {
    node: GqlEnvironment,
}

#[derive(Deserialize)]
struct Connection<E> {
    edges: Vec<E>,
}

#[derive(Deserialize)]
struct ProjectData {
    project: ProjectNode,
}

#[derive(Deserialize)]
struct ProjectNode {
    environments: Connection<EnvironmentEdge>,
}

// --- Error / request types ---

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

        let production_env = res
            .data
            .project
            .environments
            .edges
            .into_iter()
            .map(|e| e.node)
            .find(|env| env.name == "production")
            .ok_or(ServiceError::NoProductionEnvironment)?;

        let services = production_env
            .service_instances
            .edges
            .into_iter()
            .map(|e| e.node)
            .map(|si| {
                let deployment = si.latest_deployment.map(|d| {
                    let (image, repo, branch) = d.meta
                        .map(|meta| (meta.image, meta.repo, meta.branch))
                        .unwrap_or_default();

                    DeploymentInfo {
                        id: d.id,
                        status: d.status,
                        instances: d.instances,
                        image,
                        repo,
                        branch,
                    }
                });

                ServiceWithDeployment {
                    service: RailwayService {
                        id: si.service_id,
                        name: si.service_name,
                        icon: si.service.icon,
                        created_at: si.created_at,
                        updated_at: si.updated_at,
                        project_id: si.service.project_id,
                    },
                    deployment,
                }
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
