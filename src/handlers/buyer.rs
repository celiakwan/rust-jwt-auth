use crate::models::auth::ApiKey;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

#[get("/buyer/page1")]
pub fn page1(_api_key: ApiKey) -> Result<Json<JsonValue>, Status> {
    Ok(Json(json!("This page(1) is only visible to Buyer")))
}

#[get("/buyer/page2")]
pub fn page2(_api_key: ApiKey) -> Result<Json<JsonValue>, Status> {
    Ok(Json(json!("This page(2) is only visible to Buyer")))
}
