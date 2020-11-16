use crate::server::handlers::records::{get_records, mark_record};
use crate::server::handlers::sources::{create_source, get_sources};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/records")
                    .route("/", web::get().to(get_records))
                    .route("/{record_id}", web::post().to(mark_record)),
            )
            .service(
                web::scope("/sources")
                    .route("/", web::get().to(get_sources))
                    .route("/", web::post().to(create_source)),
            ),
    );
}
