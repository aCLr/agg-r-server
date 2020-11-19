use crate::db::{queries, Pool};
use crate::server::errors::ApiError;
use actix_web::web::{Data, Json, Path};
use agg_r::aggregator::Aggregator;
use agg_r::db::models::Source;
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

pub async fn delete_source(
    db_pool: Data<Pool>,
    source_id: Path<i32>,
) -> Result<Json<()>, ApiError> {
    info!("delete");
    Ok(Json(queries::delete_source(&db_pool, source_id.0).await))
}
