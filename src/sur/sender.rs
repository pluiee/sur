use std::{collections::HashMap, time::Duration};

use serde_json::Value;

use super::SurServer;

impl SurServer {
    fn create_slack_block(&self) -> String {
        let slack_block = r#"
            [
                {
                    "type": "actions",
                    "elements": [
                        {
                            "type": "button",
                            "text": {
                                "type": "plain_text",
                                "text": "Click Me",
                                "emoji": true
                            },
                            "value": "value",
                            "action_id": "id"
                        }
                    ]
                }
            ]
        "#;

        slack_block.into()
    }

    async fn send_message(
        &self,
        auth_header: String,
        body: HashMap<&str, String>,
    ) -> eyre::Result<Value> {
        let res = self
            .client
            .post("https://slack.com/api/chat.postMessage")
            .header("Authorization", auth_header.clone())
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if let Some(Value::Bool(true)) = res.get("ok") {
            Ok(res)
        } else {
            Err(eyre::Error::msg(format!(
                "error sending slack message: {}",
                res
            )))
        }
    }

    /// The sender repeatedly sends the message until the button is clicked or after maximum retries
    pub async fn run_sender(&self, channel: String) -> eyre::Result<()> {
        let slack_block = self.create_slack_block();
        let auth_header = format!("Bearer {}", self.token);
        let mut body = HashMap::new();
        body.insert("blocks", slack_block);
        body.insert("channel", channel);

        let res = self.send_message(auth_header.clone(), body.clone()).await?;

        let channel_id = res
            .get("channel")
            .expect("unable to identify key: channel")
            .to_string();
        let ts = res
            .get("message")
            .expect("unable to identify key: message")
            .get("ts")
            .expect("unable to identify key: ts")
            .to_string();

        let mut ack = self.ack.write().await;

        ack.insert((channel_id.clone(), ts.clone()), 0);

        drop(ack);

        let mut reminder_count = 0;
        let max_reminders = 20;
        let interval_sleep_secs = 2;

        body.insert("thread_ts", ts.clone());
        body.insert("text", "reminder".into());
        body.remove("blocks");

        loop {
            if reminder_count > max_reminders {
                break Ok(());
            }

            let ack = self.ack.read().await;

            if !ack.contains_key(&(channel_id.clone(), ts.clone())) {
                println!("acknowledgement verified, aborting sender...");
                break Ok(());
            }

            let _res = self.send_message(auth_header.clone(), body.clone()).await?;

            reminder_count += 1;

            tokio::time::sleep(Duration::from_secs(interval_sleep_secs)).await;
        }
    }
}
