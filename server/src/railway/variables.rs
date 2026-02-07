use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;
use crate::railway::GraphQLResponse;

const GRAPHQL_URL: &str = "https://backboard.railway.com/graphql/v2";

pub enum VariableError {
    Request(#[allow(dead_code)] reqwest::Error),
    NoProductionEnvironment,
}

#[derive(Deserialize)]
struct EnvironmentsData {
    project: ProjectEnvironments,
}

#[derive(Deserialize)]
struct ProjectEnvironments {
    environments: EnvironmentsConnection,
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
    id: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VariablesData {
    variables_for_service_deployment: HashMap<String, String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VariableUpsertData {
    variable_upsert: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VariableDeleteData {
    variable_delete: bool,
}

async fn get_production_environment_id(
    client: &Client,
    access_token: &str,
    project_id: &str,
) -> Result<String, VariableError> {
    let body = serde_json::json!({
        "query": include_str!("graphql/project_environments.gql"),
        "variables": { "id": project_id }
    });

    let res: GraphQLResponse<EnvironmentsData> = client
        .post(GRAPHQL_URL)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await
        .map_err(VariableError::Request)?
        .error_for_status()
        .map_err(VariableError::Request)?
        .json()
        .await
        .map_err(VariableError::Request)?;

    res.data
        .project
        .environments
        .edges
        .into_iter()
        .map(|e| e.node)
        .find(|env| env.name == "production")
        .map(|env| env.id)
        .ok_or(VariableError::NoProductionEnvironment)
}

pub async fn list(
    access_token: &str,
    project_id: &str,
    service_id: &str,
) -> Result<HashMap<String, String>, VariableError> {
    let client = Client::new();
    let environment_id = get_production_environment_id(&client, access_token, project_id).await?;

    let body = serde_json::json!({
        "query": include_str!("graphql/variables_for_service.gql"),
        "variables": {
            "projectId": project_id,
            "environmentId": environment_id,
            "serviceId": service_id,
        }
    });

    let res: GraphQLResponse<VariablesData> = client
        .post(GRAPHQL_URL)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await
        .map_err(VariableError::Request)?
        .error_for_status()
        .map_err(VariableError::Request)?
        .json()
        .await
        .map_err(VariableError::Request)?;

    Ok(res.data.variables_for_service_deployment)
}

pub async fn upsert(
    access_token: &str,
    project_id: &str,
    service_id: &str,
    name: &str,
    value: &str,
) -> Result<bool, VariableError> {
    let client = Client::new();
    let environment_id = get_production_environment_id(&client, access_token, project_id).await?;

    let body = serde_json::json!({
        "query": include_str!("graphql/variable_upsert.gql"),
        "variables": {
            "input": {
                "projectId": project_id,
                "environmentId": environment_id,
                "serviceId": service_id,
                "name": name,
                "value": value,
            }
        }
    });

    let res: GraphQLResponse<VariableUpsertData> = client
        .post(GRAPHQL_URL)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await
        .map_err(VariableError::Request)?
        .error_for_status()
        .map_err(VariableError::Request)?
        .json()
        .await
        .map_err(VariableError::Request)?;

    Ok(res.data.variable_upsert)
}

pub async fn delete(
    access_token: &str,
    project_id: &str,
    service_id: &str,
    name: &str,
) -> Result<bool, VariableError> {
    let client = Client::new();
    let environment_id = get_production_environment_id(&client, access_token, project_id).await?;

    let body = serde_json::json!({
        "query": include_str!("graphql/variable_delete.gql"),
        "variables": {
            "input": {
                "projectId": project_id,
                "environmentId": environment_id,
                "serviceId": service_id,
                "name": name,
            }
        }
    });

    let res: GraphQLResponse<VariableDeleteData> = client
        .post(GRAPHQL_URL)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await
        .map_err(VariableError::Request)?
        .error_for_status()
        .map_err(VariableError::Request)?
        .json()
        .await
        .map_err(VariableError::Request)?;

    Ok(res.data.variable_delete)
}
