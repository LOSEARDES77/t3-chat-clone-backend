use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use std::{env, error::Error, fmt};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub session_id: Uuid,
    pub username: String,
    pub password: String,
}

pub struct ApiUser {
    pub api_key: String,
    pub is_valid: Option<bool>,
}

#[derive(Debug)]
pub struct UnAuthorizedError {
    route: String,
}

impl UnAuthorizedError {
    fn new(route: &str) -> Self {
        UnAuthorizedError {
            route: route.to_string(),
        }
    }
}

impl Error for UnAuthorizedError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn description(&self) -> &str {
        "Unauthorized access attempted"
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for UnAuthorizedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unauthorized access to route: {}", self.route)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiUser {
    type Error = UnAuthorizedError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if env::var("DISABLE_AUTH").unwrap_or("0".to_string()) == "1" {
            return Outcome::Success(ApiUser {
                api_key: String::new(),
                is_valid: Some(true),
            });
        }

        if let Some(auth_header) = req.headers().get_one("Authorization") {
            let api_key = auth_header.trim_start_matches("Bearer ");

            if api_key.is_empty() || api_key == auth_header {
                // Either empty or doesn't start with "Bearer "
                return Outcome::Error((
                    Status::Unauthorized,
                    UnAuthorizedError::new(&req.uri().to_string()),
                ));
            }

            // Validate the API key
            if is_valid_api_key(api_key) {
                return Outcome::Success(ApiUser {
                    api_key: api_key.to_string(),
                    is_valid: Some(true),
                });
            }
        }

        Outcome::Error((
            Status::Unauthorized,
            UnAuthorizedError::new(&req.uri().to_string()),
        ))
    }
}

fn is_valid_api_key(api_key: &str) -> bool {
    // Get valid API keys from environment variable
    // Format: "key1,key2,key3" or single key
    let valid_keys = env::var("API_KEYS").unwrap_or_default();

    if valid_keys.is_empty() {
        // If no API keys configured, reject all requests
        return false;
    }

    // Check if the provided key matches any of the valid keys
    valid_keys
        .split(',')
        .map(|key| key.trim())
        .any(|valid_key| valid_key == api_key)
}
