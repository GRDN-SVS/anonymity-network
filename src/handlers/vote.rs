use actix_web::{get, web, HttpResponse, Responder, Result};
use reqwest;

use crate::{config::Config, errors::ConnectionError};

#[get("/forward")]
pub async fn forward(config_data: web::Data<Config>) -> Result<impl Responder, ConnectionError> {
    // send the request to the next node   
    let client = reqwest::Client::new();
    let res = match client.get(config_data.get_next_node())
        .send()
        .await {
            Ok(r) => r,
            Err(_) => return Err(ConnectionError { conn_name: "forward" })
        };

    Ok(HttpResponse::Ok().body(res.text().await.expect("Failed to transform request body into text")))
}