use ntfy_api::{NtfyApi, NtfyMsg};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api = NtfyApi::default();
    let ntfy_msg = NtfyMsg::builder("alerts")
        .message("Hello world")
        .title("New message!")
        .build();
    match api.post(&ntfy_msg).await {
        Ok(_) => println!("Message sent"),
        Err(_) => println!("Error sending message"),
    }
}
