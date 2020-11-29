#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use agg_r::aggregator;
use agg_r::config;
mod settings;
use settings::SETTINGS;
use std::sync::Arc;
use tokio::time::Duration;

mod db;
mod result;
mod schema;
mod server;

#[actix_rt::main]
async fn main() {
    env_logger::init();
    settings::init();
    let db_pool = db::init_pool(settings::SETTINGS.database.url.as_str());
    agg_r::db::migrate(&db_pool).expect("migrations failed");
    let http_config = config::HttpConfigBuilder::default()
        .enabled(SETTINGS.collectors.http.enabled)
        .sleep_secs(SETTINGS.collectors.http.sleep_secs)
        .scrape_source_secs_interval(SETTINGS.collectors.http.scrape_source_secs_interval)
        .build()
        .expect("can't build http config");
    let tg_config = config::TelegramConfigBuilder::default()
        .enabled(SETTINGS.collectors.tg.enabled)
        .database_directory(SETTINGS.collectors.tg.database_directory.clone())
        .log_verbosity_level(SETTINGS.collectors.tg.log_verbosity_level)
        .phone(SETTINGS.collectors.tg.phone.clone())
        .api_hash(SETTINGS.collectors.tg.api_hash.clone())
        .api_id(SETTINGS.collectors.tg.api_id)
        .build()
        .expect("can't build telegram config");
    let agg_config = config::AggregatorConfigBuilder::default()
        .http(http_config)
        .telegram(tg_config)
        .build()
        .expect("can't build aggregator config");

    let aggregator =
        Arc::new(aggregator::AggregatorBuilder::new(&agg_config, db_pool.clone()).build());
    let agg_runner = aggregator.clone();
    tokio::spawn(async move { agg_runner.run().await });
    tokio::time::delay_for(Duration::from_secs(2)).await;

    if SETTINGS.collectors.sync.before_start {
        aggregator
            .synchronize(SETTINGS.collectors.sync.secs_depth, None)
            .await
            .expect("can't synchronize");
    }
    if SETTINGS.server.enabled {
        server::server::server(aggregator, db_pool)
            .await
            .expect("can't run server");
    }
}
