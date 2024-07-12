use {
    api::config::config,
    api::router,
    core::result::Result::Ok,
    dotenv::dotenv,
    sqlx::{migrate, postgres::PgPoolOptions, Pool, Postgres},
    tracing::info,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config().await.to_owned();

    info!("All neccessarry prequesits have been met\n\n");
    info!("Connecting to database pool and running local migrations\n\n");

    let db: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url())
        .await
        .unwrap();

    migrate!("./migrations")
        .run(&db)
        .await
        .expect("Unable to migrate dabase");

    info!("Starting HTTP server\n\n");

    router::serve(config, db).await?;

    Ok(())
}
