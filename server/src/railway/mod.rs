use serde::Deserialize;

pub mod auth;
pub mod logs;
pub mod projects;
pub mod services;
pub mod variables;

#[derive(Deserialize)]
struct GraphQLResponse<T> {
    data: T,
}
