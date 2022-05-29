use crate::models::auth::{create_token, ApiKey};
use crate::models::user::{NewUser, User};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[post("/create", format = "application/json", data = "<new_user>")]
pub fn create(new_user: Json<NewUser>) -> Result<Json<usize>, Status> {
    User::create(new_user.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>) -> Result<Json<JsonValue>, Status> {
    let header = Default::default();
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();
    match User::get_by_username_and_password(&username, &password) {
        Some(user) => match create_token(&user, header) {
            Ok(token) => User::update_logged_in(&user.username, true)
                .map(|_| Json(json!(token.as_str())))
                .map_err(|_| Status::InternalServerError),
            Err(_) => Err(Status::InternalServerError),
        },
        None => Err(Status::Unauthorized),
    }
}

#[get("/logout")]
pub fn logout(api_key: ApiKey) -> Result<Json<JsonValue>, Status> {
    User::update_logged_in(&api_key.sub, false)
        .map(|_| Json(json!("You have logged out")))
        .map_err(|_| Status::InternalServerError)
}
