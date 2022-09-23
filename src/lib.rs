use ntfy_types;
pub use ntfy_types::{
    NtfyAction, NtfyActionType, NtfyAttachment, NtfyErrorResponse, NtfyMsg, NtfyPriority,
    NtfyResponse,
};
use reqwest::{Client, Method, RequestBuilder};

#[derive(Clone, Debug)]
pub struct NtfySettings {
    pub host: String,
    pub authorization: Option<NtfyAuthorization>,
}

impl NtfySettings {
    pub fn new(host: String) -> NtfySettings {
        NtfySettings {
            host,
            authorization: None,
        }
    }

    pub fn auth(mut self, auth: NtfyAuthorization) -> NtfySettings {
        self.authorization = Some(auth);
        self
    }
}

#[derive(Clone, Debug)]
pub struct NtfyAuthorization {
    pub username: String,
    pub password: String,
}

impl NtfyAuthorization {
    pub fn new(username: String, password: String) -> NtfyAuthorization {
        NtfyAuthorization { username, password }
    }
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

    pub async fn post_json(&self, body: &NtfyMsg) -> Result<NtfyResponse, NtfyError> {
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

    pub async fn post(&self, msg: &NtfyMsg) -> Result<NtfyResponse, NtfyError> {
        let mut req = self.request_builder(
            Method::POST,
            &format!("{}{}", self.settings.host, msg.topic),
        );
        if let Some(title) = &msg.title {
            req = req.header("X-Title", title);
        }
        if let Some(priority) = msg.priority {
            req = req.header("X-Priority", format!("{}", u8::from(priority)));
        }
        if let Some(tags) = &msg.tags {
            req = req.header("X-Tags", tags.join(","));
        }
        if let Some(delay) = &msg.delay {
            req = req.header("X-Delay", delay);
        }
        if let Some(actions) = &msg.actions {
            let mut action_header = String::new();
            for action in actions {
                action_header.push_str("action=");
                action_header.push_str(action.action.value());
                action_header.push_str(",label=");
                action_header.push_str(&action.label);
                action_header.push_str(",url=");
                action_header.push_str(&action.url);
                if let Some(clear) = &action.clear {
                    action_header.push_str(",clear=");
                    action_header.push_str(&format!("{}", clear));
                }
                action_header.push(';');
            }
            req = req.header("X-Actions", action_header);
        }
        if let Some(attach) = &msg.attach {
            req = req.header("X-Attach", attach);
        }
        if let Some(email) = &msg.email {
            req = req.header("X-Email", email);
        }
        let response = req
            .body(msg.message.clone().unwrap_or_default())
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
