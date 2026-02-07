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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServiceRequest {
    pub name: String,
    pub project_id: String,
    pub image: String,
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
    pub async fn list(access_token: String, project_id: &str) -> Result<Vec<Self>, reqwest::Error> {
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

        let services = res
            .data
            .project
            .services
            .edges
            .into_iter()
            .map(|e| e.node)
            .collect();

        Ok(services)
    }

    pub async fn create(access_token: String, req: CreateServiceRequest) -> Result<Self, reqwest::Error> {
        let body = serde_json::json!({
            "query": include_str!("graphql/service_create.gql"),
            "variables": {
                "input": {
                    "projectId": req.project_id,
                    "name": req.name,
                    "source": { "image": req.image },
                }
            }
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
