use slack::sur::SurServer;

#[tokio::main]
async fn main() {
    let token = "YOUR_SLACK_BOT_TOKEN";
    let slack_channel = "YOUR_SLACK_CHANNEL".to_string();
    let port = 15000;
    let server = SurServer::new(token.into());

    tokio::select! {
        _ = server.run_sender(slack_channel) => {}
        _ = server.run_receiver(port) => {}
    }
}
