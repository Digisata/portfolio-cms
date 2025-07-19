use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::env;

pub struct Cors {
    allowed_origins: Vec<String>,
    is_dev: bool,
}

impl Cors {
    pub fn new() -> Self {
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string());
        let is_dev = matches!(environment.as_str(), "dev" | "development");

        let allowed_origins = if is_dev {
            vec![] // Empty vec for dev, will allow all
        } else {
            // Get allowed URLs from environment variable
            let allowed_urls =
                env::var("ALLOWED_URLS").unwrap_or_else(|_| "http://localhost:3000".to_string());

            // Parse multiple URLs (comma-separated)
            allowed_urls
                .split(',')
                .map(|url| url.trim().to_string())
                .filter(|url| !url.is_empty())
                .collect()
        };

        Self {
            allowed_origins,
            is_dev,
        }
    }

    fn is_origin_allowed(&self, origin: &str) -> bool {
        if self.is_dev {
            return true;
        }
        self.allowed_origins.contains(&origin.to_string())
    }
}

impl Default for Cors {
    fn default() -> Self {
        Self::new()
    }
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Handle Access-Control-Allow-Origin
        if self.is_dev {
            // Allow all origins in development
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        } else {
            // Check if the request origin is allowed
            if let Some(origin) = request.headers().get_one("Origin") {
                if self.is_origin_allowed(origin) {
                    response.set_header(Header::new("Access-Control-Allow-Origin", origin));
                } else {
                    // Log blocked origin for debugging
                    eprintln!("CORS: Blocked origin: {}", origin);
                    // Don't set the Access-Control-Allow-Origin header
                    // This will cause CORS to fail for unauthorized origins
                    return;
                }
            }
        }

        // Set other CORS headers
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, DELETE, OPTIONS, PUT",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization, X-Requested-With, X-Api-Key",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Access-Control-Max-Age", "3600"));
        response.set_header(Header::new("Vary", "Origin"));
    }
}
