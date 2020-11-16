use crate::db::models::RecordWithMeta;
use crate::db::queries;
use crate::db::Pool;
use crate::server::errors::ApiError;
use actix_web::web::{Data, Json, Path, Query};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordsQuery {
    Unread,
    All,
    Starred,
}

#[derive(Debug, Deserialize)]
pub struct GetFilteredRecordsRequest {
    pub source_id: Option<i32>,
    pub query: RecordsQuery,
    pub limit: i64,
    pub offset: i64,
}

pub async fn get_records(
    db_pool: Data<Pool>,
    params: Query<GetFilteredRecordsRequest>,
) -> Result<Json<Vec<RecordWithMeta>>, ApiError> {
    let records = match params.query {
        RecordsQuery::Unread => {
            queries::get_unread_records(&db_pool, params.source_id, params.limit, params.offset)
                .await
        }
        RecordsQuery::All => {
            queries::get_all_records(
                &db_pool,
                params.source_id,
                None,
                params.limit,
                params.offset,
            )
            .await
        }
        RecordsQuery::Starred => {
            queries::get_starred_records(&db_pool, params.source_id, params.limit, params.offset)
                .await
        }
    };
    Ok(Json(records))
}

#[derive(Debug, Deserialize)]
pub struct MarkRecord {
    starred: bool,
}

pub async fn mark_record(
    db_pool: Data<Pool>,
    record_id: Path<i32>,
    params: Json<MarkRecord>,
) -> Result<Json<RecordWithMeta>, ApiError> {
    Ok(Json(
        queries::mark_record(&db_pool, record_id.0, params.starred).await,
    ))
}
