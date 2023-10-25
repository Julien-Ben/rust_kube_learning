mod api;
mod models;
mod repository;

//modify imports below
use actix_web::{web::Data, App, HttpServer, web, middleware::Logger};
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use api::processing_api::process_image;
use repository::mongodb_repo::MongoRepo;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(db_data.clone())
            .configure(init)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(get_user);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(get_all_users);
    cfg.service(process_image);
}

