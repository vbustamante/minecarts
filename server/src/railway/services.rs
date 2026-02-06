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
    #[serde(rename = "projectId")]
    pub project_id: String,
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

impl RailwayService {
    pub async fn list(access_token: String, project_id: &str) -> Result<Vec<Self>, reqwest::Error> {
        let body = serde_json::json!({
            "query": r#"query project($id: String!) {
                project(id: $id) {
                    services {
                        edges {
                            node {
                                id
                                name
                                icon
                                createdAt
                                projectId
                            }
                        }
                    }
                }
            }"#,
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
}
