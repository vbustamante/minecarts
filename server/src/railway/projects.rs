use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::railway::GraphQLResponse;

const GRAPHQL_URL: &str = "https://backboard.railway.com/graphql/v2";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub projects: Vec<Project>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExternalWorkspacesData {
    external_workspaces: Vec<Workspace>,
}


impl Project {
    pub async fn list(access_token: String) -> Result<Vec<Self>, reqwest::Error> {

        let body = serde_json::json!({
            "query": "query { externalWorkspaces { id name projects { id name } } }"
        });

        let client = Client::new();

        let res: GraphQLResponse<ExternalWorkspacesData> = client
            .post(GRAPHQL_URL)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let mut projects = Vec::new();

        for workspace in res.data.external_workspaces {
            projects.extend(workspace.projects);
        }

        Ok(projects)
    }
}
