use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use serde_json::{from_str, Value};
use urlencoding::decode;

use super::SurServer;

impl SurServer {
    pub async fn handle_get() -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().finish())
    }
    async fn handle_post(data: web::Data<Self>, _req: HttpRequest, body: String) -> Result<String> {
        let body = decode(&body).expect("UTF-8").into_owned();
        let sur = data.get_ref();
        if let Some(pos) = body.find('{') {
            let json_str = &body[pos..];
            match from_str::<Value>(json_str) {
                Ok(json) => {
                    let channel_id = json
                        .get("channel")
                        .expect("unable to identify key: channel")
                        .get("id")
                        .expect("unable to identify key: id")
                        .to_string();
                    let ts = json
                        .get("container")
                        .expect("unable to identify key: container")
                        .get("message_ts")
                        .expect("unable to identify key: message_ts")
                        .to_string();
                    println!("({channel_id}, {ts})");

                    let mut ack = sur.ack.write().await;
                    ack.remove(&(channel_id, ts));
                }
                _ => println!("Error parsing"),
            }
        }
        Ok("interaction successful".into())
    }
    pub async fn run_receiver(&self, port: u64) -> eyre::Result<()> {
        let addr = format!("0.0.0.0:{port}");
        let data = web::Data::new(self.clone());
        HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .route("/", web::post().to(Self::handle_post))
                .route("/", web::get().to(Self::handle_get))
        })
        .bind(addr)?
        .run()
        .await
        .map_err(Into::into)
    }
}
