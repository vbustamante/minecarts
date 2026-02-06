use serde::Deserialize;

pub mod auth;
pub mod projects;
pub mod services;

#[derive(Deserialize)]
struct GraphQLResponse<T> {
    data: T,
}
