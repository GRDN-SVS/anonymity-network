use actix_web::{post, web, HttpResponse, Responder};
use reqwest;

use crate::{config::Config, models::Vote};

#[post("/forward")]
pub async fn forward(vote: web::Json<Vote>, config_data: web::Data<Config>) -> impl Responder {
    // send the request to the next node
    let client = reqwest::Client::new();
    let res = client.post(config_data.get_next_node())
        .json(&vote.into_inner())
        .send()
        .await;

    HttpResponse::Ok().body("Vote Sent Correctly!")
}