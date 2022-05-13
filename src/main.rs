use actix_web::{
    web, 
    App, 
    HttpServer
};

use config::{
    configure_app, 
    configure_db, 
};

mod user;
mod config;
mod telemetry;
mod health_check;

#[actix_web::main]
async fn actix_run() -> std::io::Result<()> {
    let pool = match configure_db() {
        Ok(p) => web::Data::new(p),
        Err(e) => panic!("{}", e),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(configure_app)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn main() {
    dotenv::dotenv().ok();

    let subscriber = telemetry::get_subscriber("blog".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    match actix_run() {
        Err(e) => panic!("{}", e),
        Ok(_) => (),
    };
}
