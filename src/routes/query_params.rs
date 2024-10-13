use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct QueryData{
    message : Option<String>,
    title:String,
    id:i32
}

pub async fn query_params(Query(query):Query<QueryData>) -> Json<QueryData> {
    Json(query)
}
