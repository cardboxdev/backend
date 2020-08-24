use super::routes::{self, AnswerFailure, FailureCode};
use crate::services::Database;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};

pub struct Config {
    pub bind_address: String,
    pub database_url: String,
    pub accesso_url: String,
}

pub async fn create_server(config: Config) -> std::io::Result<()> {
    let app = cardbox_logic::App {
        db: Database::new(config.database_url).expect("Failed to create database"),
    };

    let app_lock = std::sync::RwLock::new(app);
    let app_data = web::Data::new(app_lock);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let error_message = format!("{}", err);
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AnswerFailure {
                        error: FailureCode::InvalidPayload,
                        message: Some(error_message),
                    }),
                )
                .into()
            }))
            .app_data(web::QueryConfig::default().error_handler(|err, _| {
                let error_message = format!("{}", err);
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AnswerFailure {
                        error: FailureCode::InvalidQueryParams,
                        message: Some(error_message),
                    }),
                )
                .into()
            }))
            .wrap(
                middleware::DefaultHeaders::new()
                    // .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .service(routes::health::service())
            .default_service(web::route().to(routes::not_found::route))
    })
    .bind(config.bind_address)?
    .run()
    .await
}
