use const_format::concatcp;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{tls, Client, ClientBuilder, StatusCode};

use crate::request::RegisterRequest;
use crate::response::RegisterResponse;
use crate::util::Json;

const BASE_URL: &str = "https://api.cloudflareclient.com/v0a2483";
const URL_REG: &str = concatcp!(BASE_URL, "/reg");

pub struct App {
    client: Client,
    token: String,
}

impl App {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static("okhttp/3.12.1"));
        headers.insert("CF-Client-Version", HeaderValue::from_static("a-6.81-4065"));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .max_tls_version(tls::Version::TLS_1_2)
            .build()
            .unwrap();

        Self {
            client,
            token: token.to_owned(),
        }
    }

    pub async fn register(&self, request: &RegisterRequest) -> Option<RegisterResponse> {
        let request_str = request.to_string();
        tracing::debug!(target: "App::register", request = %request_str);

        let output = self
            .client
            .post(URL_REG)
            .header("CF-Access-Jwt-Assertion", &self.token)
            .json(request)
            .send()
            .await;

        let output = match output {
            Ok(o) => o,
            Err(error) => {
                tracing::error!(target: "App::register", ?error);
                return None;
            }
        };

        match output.status() {
            StatusCode::OK => match output.json().await {
                Ok(o) => Some(o),
                Err(error) => {
                    tracing::error!(target: "App::register", ?error);
                    None
                }
            },
            _ => {
                let status = output.status().to_owned();
                let error = output.text().await.unwrap_or(status.to_string());

                tracing::error!(target: "App::register", %error);
                None
            }
        }
    }
}
