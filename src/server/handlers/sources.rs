use crate::db::Pool;
use crate::server::errors::ApiError;
use actix_web::web::{Data, Json};
use feedr::aggregator::Aggregator;
use feedr::db::models::Source;
use serde::Deserialize;
use std::sync::Arc;

pub async fn get_sources(db_pool: Data<Pool>) -> Result<Json<Vec<Source>>, ApiError> {
    Ok(Json(Source::get_list(&db_pool).await?))
}

#[derive(Deserialize)]
pub struct CreateSource {
    origin: String,
}

pub async fn create_source(
    aggregator: Data<Arc<Aggregator>>,
    query: Json<CreateSource>,
) -> Result<Json<Vec<Source>>, ApiError> {
    let sources = aggregator.search_source(query.origin.as_str()).await?;
    Ok(Json(sources))
}
