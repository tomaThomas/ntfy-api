use ntfy_api::{NtfyApi, NtfyMsg};

#[tokio::main(flavor = "current_thread")]
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
