use diesel::prelude::*;
use diesel::{delete, insert_into};

use crate::db::Pool;
use crate::errors::ApiError;
use crate::schema::{records, records_user_settings, sources_user_settings};
use diesel::sql_types::{Bool, Nullable};
use tokio_diesel::*;

sql_function!(fn coalesce(x: Nullable<Bool>, y: Bool) -> Bool);

pub async fn unsubscribe(db_pool: &Pool, source_id: i32, user_id: i32) -> Result<(), ApiError> {
    let records = records::table.filter(records::source_id.eq(source_id));
    delete(
        records_user_settings::table.filter(
            records_user_settings::record_id
                .eq_any(records.select(records::id))
                .and(records_user_settings::user_id.eq(user_id)),
        ),
    )
    .execute_async(db_pool)
    .await?;
    delete(
        sources_user_settings::table.filter(
            sources_user_settings::source_id
                .eq(source_id)
                .and(sources_user_settings::user_id.eq(user_id)),
        ),
    )
    .execute_async(db_pool)
    .await?;
    Ok(())
}

pub async fn subscribe(db_pool: &Pool, source_id: i32, user_id: i32) -> Result<(), ApiError> {
    insert_into(sources_user_settings::table)
        .values((
            (sources_user_settings::source_id.eq(source_id)),
            (sources_user_settings::user_id.eq(user_id)),
        ))
        .execute_async(db_pool)
        .await?;
    Ok(())
}
