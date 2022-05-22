use super::user::{Role, User};
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{Error, Header, SignWithKey, Token, VerifyWithKey};
use jwt::token::Signed;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Outcome::{Failure, Success};
use rocket::Request;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::str::FromStr;

pub struct ApiKey {
    pub sub: String,
    pub role: Role,
    pub exp: i64
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let token_strs: Vec<_> = request.headers().get("Authentication").collect();
        if token_strs.len() != 1 {
            return Failure((Status::Forbidden, ()));
        }
        match read_token(token_strs[0]) {
            Ok(api_key) => {
                let path = request.raw_segment_str(0).expect("Failed to extract the path segment");
                match path.as_str() {
                    "buyer" => authorize_with_role(Role::Buyer, api_key),
                    "seller" => authorize_with_role(Role::Seller, api_key),
                    "logout" => authorize(api_key),
                    _ => Failure((Status::InternalServerError, ()))
                }
            },
            Err(_) => Failure((Status::Forbidden, ()))
        }
    }
}

pub fn create_token(
    user: &User,
    header: Header
) -> Result<Token<Header, BTreeMap<&'static str, String>, Signed>, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(5))
        .expect("Failed to create timestamp")
        .timestamp();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user.username.to_string());
    claims.insert("role", user.role.to_string());
    claims.insert("exp", expiration.to_string());
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();
    let token = Token::new(header, claims);
    token.sign_with_key(&key)
}

fn read_token(token_str: &str) -> Result<ApiKey, Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret")?;
    let verified_token: Result<Token<Header, BTreeMap<String, String>, _>, Error> =
        VerifyWithKey::verify_with_key(token_str, &key);
    match verified_token {
        Ok(token) => {
            Ok(ApiKey {
                sub: token.claims()["sub"].to_string(),
                role: Role::from_str(&token.claims()["role"]).expect("Failed to convert string to Role"),
                exp: token.claims()["exp"].parse::<i64>().expect("Failed to convert string to i64"),
            })
        },
        Err(e) => Err(e)
    }
}

fn authorize_with_role(role: Role, api_key: ApiKey) -> Outcome<ApiKey, ()> {
    let is_authorized = api_key.role == role && api_key.exp > Utc::now().timestamp() && is_logged_in(&api_key);
    get_auth_result(is_authorized, api_key)
}

fn authorize(api_key: ApiKey) -> Outcome<ApiKey, ()> {
    let is_authorized = api_key.exp > Utc::now().timestamp() && is_logged_in(&api_key);
    get_auth_result(is_authorized, api_key)
}

fn get_auth_result(is_authorized: bool, api_key: ApiKey) -> Outcome<ApiKey, ()> {
    match is_authorized {
        true => Success(api_key),
        false => Failure((Status::Forbidden, ()))
    }
}

fn is_logged_in(api_key: &ApiKey) -> bool {
    User::get_by_username(&api_key.sub)
    .expect(&format!("Failed to find the user by username: {}", api_key.sub))
    .logged_in
}
