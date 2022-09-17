use ntfy_types::{NtfyErrorResponse, NtfyMsg, NtfyResponse};
use reqwest::{Client, Method, RequestBuilder};

#[derive(Clone, Debug)]
pub struct NtfySettings {
    pub host: String,
    pub authorization: Option<NtfyAuthorization>,
}

#[derive(Clone, Debug)]
pub struct NtfyAuthorization {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct NtfyApi {
    settings: NtfySettings,
    client: Client,
}

impl NtfyApi {
    pub fn new(settings: NtfySettings) -> NtfyApi {
        let client = Client::new();
        NtfyApi { settings, client }
    }

    fn request_builder(&self, method: Method, url: &str) -> RequestBuilder {
        let req = self.client.request(method, url);
        if let Some(creds) = &self.settings.authorization {
            req.basic_auth(&creds.username, Some(&creds.password))
        } else {
            req
        }
    }

    pub async fn post(&self, body: &NtfyMsg) -> Result<NtfyResponse, NtfyError> {
        let response = self
            .request_builder(Method::POST, &self.settings.host)
            .json(&body)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json::<NtfyResponse>().await?)
        } else {
            Err(NtfyError::ApiError(
                response.json::<NtfyErrorResponse>().await?,
            ))
        }
    }
}

impl Default for NtfyApi {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            client: Default::default(),
        }
    }
}

impl Default for NtfySettings {
    fn default() -> Self {
        Self {
            host: String::from("https://ntfy.sh/"),
            authorization: None,
        }
    }
}

#[derive(Debug)]
pub enum NtfyError {
    ApiError(NtfyErrorResponse),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for NtfyError {
    fn from(e: reqwest::Error) -> Self {
        NtfyError::ReqwestError(e)
    }
}
