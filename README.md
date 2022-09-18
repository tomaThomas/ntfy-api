# ntfy-api

This library contains async rust bindings for the [ntfy](https://github.com/binwiederhier/ntfy) API.
It uses [reqwest](https://github.com/seanmonstar/reqwest) as HTTP client.

Features implemented:
 - [X] Publish as JSON
 - [ ] Post directly to topic
 - [ ] File attachments
 - [ ] Websockets

 ## Usage
 `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
ntfy-api = "^0.1.1"
```

 ```rust
use ntfy_api::{NtfyApi, NtfyMsg};

#[tokio::main]
async fn main() {
    let api = NtfyApi::default();
    let ntfy_msg = NtfyMsg {
        topic: String::from("alerts"),
        message: Some(String::from("Message body")),
        title: Some(String::from("New Message")),
        tags: None,
        priority: None,
        attach: None,
        filename: None,
        click: None,
        action: None,
    };
    match api.post(&ntfy_msg).await {
        Ok(_) => println!("Message sent"),
        Err(_) => println!("Error sending message"),
    }
}
 ```

### Configuration
 ```rust
let api = NtfyApi::new(NtfySettings {
    host: String::from("https://ntfysh/"),
    authorization: Some(NtfyAuthorization {
        username: String::from("username"),
        password: String::from("password"),
    }),
});
 ```
