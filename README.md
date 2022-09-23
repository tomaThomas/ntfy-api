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
    let ntfy_msg = NtfyMsg::builder("topic").message("Hello world!").build();
    match api.post(&ntfy_msg).await {
        Ok(_) => println!("Message sent"),
        Err(_) => println!("Error sending message"),
    }
}
 ```

### Configuration
 ```rust
let api = NtfyApi::new(NtfySettings {
    host: String::from("https://ntfy.sh/"),
    authorization: Some(NtfyAuthorization::new(String::from("username"), String::from("password"))),
});
 ```
