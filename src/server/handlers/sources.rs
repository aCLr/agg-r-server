use crate::db::{models::User, queries::sources as sources_queries, Pool};
use crate::errors::ApiError;
use actix_web::web::{Data, Json, Path};
use agg_r::aggregator::Aggregator;
use agg_r::db::models::Source;
use serde::Deserialize;
use std::sync::Arc;

pub async fn get_list(db_pool: Data<Pool>) -> Result<Json<Vec<Source>>, ApiError> {
    Ok(Json(Source::get_list(&db_pool).await?))
}

#[derive(Deserialize)]
pub struct SearchSource {
    origin: String,
}

pub async fn search(
    aggregator: Data<Arc<Aggregator>>,
    query: Json<SearchSource>,
) -> Result<Json<Vec<Source>>, ApiError> {
    let sources = aggregator.search_source(query.origin.as_str()).await?;
    Ok(Json(sources))
}

pub async fn unsubscribe(
    db_pool: Data<Pool>,
    user: User,
    source_id: Path<i32>,
) -> Result<Json<()>, ApiError> {
    info!("delete");
    Ok(Json(
        sources_queries::unsubscribe(&db_pool, source_id.0, user.id).await?,
    ))
}

pub async fn subscribe(
    db_pool: Data<Pool>,
    user: User,
    source_id: Path<i32>,
) -> Result<Json<()>, ApiError> {
    info!("delete");
    Ok(Json(
        sources_queries::subscribe(&db_pool, source_id.0, user.id).await?,
    ))
}
