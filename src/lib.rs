mod path;
pub use path::*;
use reqwest::Client;
use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error: {0}")]
    RequestError(#[from] ReqwestError),

    #[error("Unsupported HTTP method: {0}")]
    UnsupportedMethod(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

/// Builder for constructing API bindings.
pub struct Builder {
    pub endpoint: String,
    pub paths: Vec<Path>,
    pub api_place: APIPlace,
}

/// Enum representing where the API place can be.
pub enum APIPlace {
    Header,
    QueryParam,
    Body,
    None,
}

impl Builder {
    pub fn new(endpoint: &str, api_place: APIPlace) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            paths: Vec::new(),
            api_place,
        }
    }

    fn add_path(&mut self, path: Path) -> &mut Self {
        self.paths.push(path);
        self
    }

    pub fn build(self) -> QuickAPIBinding {
        QuickAPIBinding {
            client: Client::new(),
            endpoint: self.endpoint,
            paths: self.paths,
        }
    }
}

#[macro_export]
macro_rules! create_api_request {
    ($self:ident, $method:ident, $path:expr, $body:expr, $headers:expr) => {{
        let mut request_builder = match $method {
            "get" => $self.client.get($path),
            "post" => $self.client.post($path),
            "put" => $self.client.put($path),
            "delete" => $self.client.delete($path),
            _ => return Err(ApiError::UnsupportedMethod($method.to_string())),
        };

        for (key, value) in $headers {
            request_builder = request_builder.header(key, value);
        }

        if let Some(body) = $body {
            request_builder = request_builder.json(&body.get());
        }

        request_builder.send().await
    }};
}

/// Struct representing the final API binding.
pub struct QuickAPIBinding {
    client: Client,
    endpoint: String,
    paths: Vec<Path>,
}
impl QuickAPIBinding {
    pub async fn send(&self, path: &Path) -> Result<reqwest::Response, ApiError> {
        let method = path.method.as_str();

        let headers: Vec<(&str, &str)> = path
            .headers
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        create_api_request!(
            self,
            method,
            format!("{}{}", self.endpoint, path.path),
            &path.body,
            headers
        )
        .map_err(ApiError::RequestError)
    }
}

mod tests {
    use std::collections::HashMap;

    use super::*;
    const API: &str = "";
    pub struct TestAPI {
        quick_api: QuickAPIBinding,
    }
}
