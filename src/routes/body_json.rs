use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct UserDetails{
    name:String,
    emp_id:i32
}
pub async fn json_body(Json(body):Json<UserDetails>) -> Json<UserDetails>{
    Json(body)
}
#[derive(Deserialize)]
pub struct EmpDetailsIn{
    name:String,
    emp_id:i32
}

#[derive(Serialize)]
pub struct EmpDetailsOut{
    name:String,
    emp_id:i32,
    username:String
}

pub async fn json_body2(Json(body):Json<EmpDetailsIn>) -> Json<EmpDetailsOut>{
    Json(EmpDetailsOut { name: body.name.clone(), emp_id: body.emp_id, username: (body.emp_id.to_string() + &body.name) })
}

