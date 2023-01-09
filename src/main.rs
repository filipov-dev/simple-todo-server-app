mod methods;
mod models;

use std::sync::Arc;
use std::{env, sync::Mutex, vec};
use actix_cors::Cors;
use actix_web::{App, HttpServer, web, middleware::Logger};
use env_logger::Env;
use crate::models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = match env::var("HOST") {
        Ok(v) => v,
        Err(_e) => "0.0.0.0".to_string(),
    };

    let port = match env::var("PORT") {
        Ok(v) => v,
        Err(_e) => "8080".to_string(),
    };

    let state = web::Data::new(
        AppState {
            tasks: Arc::new(Mutex::new(vec![])),
        }
    );

    let bind = format!("{}:{}", host, port);

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| {
                true
            })
            .max_age(3600);

        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .service(methods::get_tasks)
            .service(methods::create_task)
            .service(methods::change_task_name)
            .service(methods::toggle_task_status)
            .service(methods::delete_task)

    })
    .bind(bind)?
    .run()
    .await
}