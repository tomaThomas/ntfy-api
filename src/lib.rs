pub use ntfy_types::{
    NtfyAction, NtfyActionType, NtfyAttachment, NtfyErrorResponse, NtfyMsg, NtfyPriority,
    NtfyResponse,
};
use reqwest::{Client, Method, RequestBuilder, Response};

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

    // Send a message using JSON
    pub async fn post(&self, body: &NtfyMsg) -> Result<NtfyResponse, NtfyError> {
        Self::respond(
            self.request_builder(Method::POST, &self.settings.host)
                .json(&body)
                .send()
                .await?,
        )
        .await
    }

    /// Send a message to a topic
    pub async fn post_to_topic(
        &self,
        topic: &str,
        message: &str,
    ) -> Result<NtfyResponse, NtfyError> {
        // TODO: options using headers
        Self::respond(
            self.request_builder(Method::POST, &format!("{}{}", &self.settings.host, topic))
                .body(message.to_owned())
                .send()
                .await?,
        )
        .await
    }

    /// Sends a 'triggered' message to a topic by sending a GET request to the webhook endpoint
    pub async fn trigger(&self, topic: &str) -> Result<NtfyResponse, NtfyError> {
        Self::respond(
            self.request_builder(
                Method::GET,
                &format!("{}{}/publish", &self.settings.host, topic),
            )
            .send()
            .await?,
        )
        .await
    }

    /// Send an attachment to a topic
    pub async fn attachment(
        &self,
        topic: &str,
        filename: &str,
        data: Vec<u8>,
    ) -> Result<NtfyResponse, NtfyError> {
        Self::respond(
            self.request_builder(Method::PUT, &format!("{}{}", &self.settings.host, topic))
                .header("Filename", filename)
                .body(data)
                .send()
                .await?,
        )
        .await
    }

    async fn respond(response: Response) -> Result<NtfyResponse, NtfyError> {
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
