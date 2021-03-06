use crate::db::Pool;
use crate::server::auth::Authorization;
use crate::server::handlers::routes::routes;
use crate::settings::SETTINGS;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use agg_r::aggregator::Aggregator;
use std::sync::Arc;

pub async fn server(aggregator: Arc<Aggregator>, db_pool: Pool) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .wrap(Logger::default())
            .wrap(Authorization::default())
            .configure(routes)
            .app_data(Data::new(aggregator.clone()))
            .app_data(Data::new(db_pool.clone()))
    });

    server
        .bind(format!("{}:{}", SETTINGS.server.host, SETTINGS.server.port))?
        .run()
        .await
}
