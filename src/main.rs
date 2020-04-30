use dotenv;
use std::env;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.example").expect(".env not red");
    env_logger::init();

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    image_api::run(host, port).await
}