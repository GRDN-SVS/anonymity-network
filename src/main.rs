use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod config;
mod handlers;
mod errors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        // Read configuration file into a struct
        let node_config = match config::Config::new() {
            Ok(conf_instance) => conf_instance,
            Err(e) => {
                eprintln!("ERROR!: {}", e);
                std::process::exit(1);
            }
        };

        App::new().data(node_config).service(handlers::vote::forward)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}