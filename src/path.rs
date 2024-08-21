use super::APIPlace;
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct Path {
    pub path: String,
    pub body: Option<Body>,
    pub method: reqwest::Method,
    pub headers: HashMap<String, String>,
    pub query_slice: QuerySlice,
}
impl Path {
    pub fn insert_api_key(
        &mut self,
        k: &str,
        v: &str,
        where_to_insert: APIPlace,
    ) -> Option<String> {
        match where_to_insert {
            APIPlace::Header => {
                self.headers.insert(k.to_string(), v.to_string());
                Some(v.to_string())
            }
            APIPlace::QueryParam => {
                self.query_slice.insert(k, v);
                None
            }
            APIPlace::Body => {
                if let Some(body) = self.body.as_mut() {
                    body.push_value(k, v);
                    Some(v.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    pub fn insert_query_param(&mut self, k: &str, v: &str) {
        self.query_slice.insert(k, v);
    }
    pub fn insert_header(&mut self, k: &str, v: &str) {
        self.headers.insert(k.to_string(), v.to_string());
    }

    pub fn get_body(&self) -> Option<&Body> {
        match self.body {
            Some(ref body) => Some(body),
            None => None,
        }
    }
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    pub fn get_query_slice(&self) -> &QuerySlice {
        &self.query_slice
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

pub struct PathBuilder {
    pub path: String,
    pub body: Option<Body>,
    pub headers: Vec<Header>,
    pub query_params: Vec<QueryParam>,
    pub method: Method,
}

impl PathBuilder {
    pub fn new(path: &str, method: Method) -> Self {
        Self {
            path: path.to_string(),
            body: None,
            headers: Vec::new(),
            query_params: Vec::new(),
            method,
        }
    }

    pub fn with_body(mut self, body: Body) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_header(mut self, header: Header) -> Self {
        self.headers.push(header);
        self
    }

    pub fn with_query_param(mut self, query_param: QueryParam) -> Self {
        self.query_params.push(query_param);
        self
    }

    pub fn build(self) -> Path {
        let query_slice = QuerySlice::from(self.query_params);
        let headers: HashMap<String, String> =
            self.headers.into_iter().map(|h| (h.key, h.value)).collect();
        let method = match self.method {
            Method::GET => reqwest::Method::GET,
            Method::POST => reqwest::Method::POST,
            Method::PUT => reqwest::Method::PUT,
            Method::DELETE => reqwest::Method::DELETE,
            Method::PATCH => reqwest::Method::PATCH,
        };
        Path {
            path: self.path,
            body: self.body,
            query_slice,
            headers,
            method,
        }
    }

    /*fn build_query_string(&self) -> String {
        if self.query_params.is_empty() {
            return String::new();
        }

        let query_string: Vec<String> = self
            .query_params
            .iter()
            .map(|param| format!("{}={}", param.key, param.value))
            .collect();

        format!("?{}", query_string.join("&"))
    }*/
}
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}
pub struct Body {
    pub body: Value,
}

impl Body {
    pub fn new(body: HashMap<String, String>) -> Self {
        Self {
            body: json!(body.into_iter().collect::<Vec<_>>()),
        }
    }
    pub fn push_value(&mut self, key: &str, value: &str) {
        self.body
            .as_object_mut()
            .unwrap()
            .insert(key.to_string(), json!(value));
    }
    pub fn get(&self) -> &Value {
        &self.body
    }
}

#[derive(Clone)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Header {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct QueryParam {
    pub key: String,
    pub value: String,
}

impl QueryParam {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}
#[derive(Clone)]
pub struct QuerySlice {
    pub slice: HashMap<String, String>,
}
impl QuerySlice {
    pub fn from(query_params: Vec<QueryParam>) -> Self {
        let mut slice = HashMap::new();
        for param in query_params {
            slice.insert(param.key, param.value);
        }
        Self { slice }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.slice.get(key)
    }
    pub fn insert(&mut self, key: &str, value: &str) {
        self.slice.insert(key.to_string(), value.to_string());
    }
    pub fn remove(&mut self, key: &str) {
        self.slice.remove(key);
    }

    pub(crate) fn build_query_string(&self) -> String {
        if self.slice.is_empty() {
            return String::new();
        }

        let query_string: Vec<String> = self
            .slice
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        format!("?{}", query_string.join("&"))
    }
}
