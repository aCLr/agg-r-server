use diesel::prelude::*;
use diesel::Queryable;
use diesel::{delete, insert_into};

use crate::db::models::{RecordWithMeta, User};
use crate::db::Pool;
use crate::schema::{records, records_meta, sources, users};
use chrono::NaiveDateTime;
use diesel::pg::upsert::excluded;
use diesel::sql_types::{Bool, Nullable};
use tokio_diesel::*;

sql_function!(fn coalesce(x: Nullable<Bool>, y: Bool) -> Bool);

#[derive(Queryable)]
struct Meta {
    pub starred: bool,
}

pub async fn get_unread_records(
    db_pool: &Pool,
    source_id: Option<i32>,
    limit: i64,
    offset: i64,
) -> Vec<RecordWithMeta> {
    let last_read_date = users::table
        .filter(users::id.eq(1))
        .select(users::last_read_date)
        .limit(1)
        .first_async::<NaiveDateTime>(db_pool)
        .await
        .unwrap();
    let query = records::table
        .left_join(records_meta::dsl::records_meta)
        .filter(records::date.gt(last_read_date))
        .order(records::date.desc())
        .limit(limit)
        .offset(offset)
        .select((
            records::id,
            records::title,
            records::guid,
            records::source_id,
            records::content,
            records::date,
            records::image,
            records_meta::starred.nullable(),
        ));

    match source_id {
        Some(source_id) => {
            query
                .filter(records::source_id.eq(source_id))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
        None => query.load_async::<RecordWithMeta>(db_pool).await,
    }
    .unwrap()
}

pub async fn get_starred_records(
    db_pool: &Pool,
    source_id: Option<i32>,
    limit: i64,
    offset: i64,
) -> Vec<RecordWithMeta> {
    let query = records::table
        .left_join(records_meta::dsl::records_meta)
        .filter(records_meta::starred)
        .order(records::date.desc())
        .limit(limit)
        .offset(offset)
        .select((
            records::id,
            records::title,
            records::guid,
            records::source_id,
            records::content,
            records::date,
            records::image,
            records_meta::starred.nullable(),
        ));

    match source_id {
        Some(source_id) => {
            query
                .filter(records::source_id.eq(source_id))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
        None => query.load_async::<RecordWithMeta>(db_pool).await,
    }
    .unwrap()
}

pub async fn get_all_records(
    db_pool: &Pool,
    source_id: Option<i32>,
    record_id: Option<i32>,
    limit: i64,
    offset: i64,
) -> Vec<RecordWithMeta> {
    let query = records::table
        .left_join(records_meta::dsl::records_meta)
        .order(records::date.desc())
        .limit(limit)
        .offset(offset)
        .select((
            records::id,
            records::title,
            records::guid,
            records::source_id,
            records::content,
            records::date,
            records::image,
            records_meta::starred.nullable(),
        ));
    match (source_id, record_id) {
        (None, None) => query.load_async::<RecordWithMeta>(db_pool).await,
        (Some(s), Some(r)) => {
            query
                .filter(records::source_id.eq(s).and(records::id.eq(r)))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
        (Some(s), None) => {
            query
                .filter(records::source_id.eq(s))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
        (None, Some(r)) => {
            query
                .filter(records::id.eq(r))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
    }
    .unwrap()
}

pub async fn mark_record(db_pool: &Pool, record_id: i32, starred: bool) -> RecordWithMeta {
    let starred = records_meta::starred.eq(coalesce(starred, false));

    insert_into(records_meta::table)
        .values((
            records_meta::record_id.eq(record_id),
            records_meta::user_id.eq(1),
            starred,
        ))
        .on_conflict((records_meta::user_id, records_meta::record_id))
        .do_update()
        .set((records_meta::starred.eq(excluded(records_meta::starred)),))
        .execute_async(db_pool)
        .await
        .unwrap();
    get_all_records(db_pool, None, Some(record_id), 1, 0)
        .await
        .first()
        .cloned()
        .unwrap()
}

pub async fn delete_source(db_pool: &Pool, source_id: i32) {
    let records = records::table.filter(records::source_id.eq(source_id));
    delete(records_meta::table.filter(records_meta::record_id.eq_any(records.select(records::id))))
        .execute_async(db_pool)
        .await
        .unwrap();
    delete(records).execute_async(db_pool).await.unwrap();
    delete(sources::table.filter(sources::id.eq(source_id)))
        .execute_async(db_pool)
        .await
        .unwrap();
}

pub async fn get_user_by_token(db_pool: &Pool, token: String) -> Option<User> {
    let user = users::table
        .filter(users::token.eq(token))
        .first_async::<User>(db_pool)
        .await;
    match user {}
}


pub async fn check_user_exists()