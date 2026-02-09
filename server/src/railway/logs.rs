use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::railway::GraphQLResponse;

const GRAPHQL_URL: &str = "https://backboard.railway.com/graphql/v2";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildLogsData {
    build_logs: Vec<LogEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeploymentLogsData {
    deployment_logs: Vec<LogEntry>,
}

pub async fn build_logs(
    access_token: &str,
    deployment_id: &str,
    limit: Option<i32>,
) -> Result<Vec<LogEntry>, reqwest::Error> {
    let body = serde_json::json!({
        "query": include_str!("graphql/build_logs.gql"),
        "variables": {
            "deploymentId": deployment_id,
            "limit": limit.unwrap_or(1000),
        }
    });

    let client = Client::new();
    let res: GraphQLResponse<BuildLogsData> = client
        .post(GRAPHQL_URL)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(res.data.build_logs)
}

pub async fn deployment_logs(
    access_token: &str,
    deployment_id: &str,
    limit: Option<i32>,
) -> Result<Vec<LogEntry>, reqwest::Error> {
    let body = serde_json::json!({
        "query": include_str!("graphql/deployment_logs.gql"),
        "variables": {
            "deploymentId": deployment_id,
            "limit": limit.unwrap_or(1000),
        }
    });

    let client = Client::new();
    let res: GraphQLResponse<DeploymentLogsData> = client
        .post(GRAPHQL_URL)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(res.data.deployment_logs)
}
