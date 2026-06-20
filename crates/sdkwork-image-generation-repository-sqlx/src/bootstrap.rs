//! SDKWork Image database pool bootstrap via `sdkwork-database`.

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool, PoolError};

pub use sdkwork_image_database_host::{
    bootstrap_image_database, bootstrap_image_database_from_env, ImageDatabaseHost,
};

pub type ImageDatabasePool = DatabasePool;

pub async fn connect_image_database_pool_from_env() -> Result<ImageDatabasePool, PoolError> {
    let config = DatabaseConfig::from_env("IMAGE")?;
    create_pool_from_config(config).await
}

pub async fn connect_and_bootstrap_image_database_from_env() -> Result<ImageDatabaseHost, String> {
    let pool = connect_image_database_pool_from_env()
        .await
        .map_err(|error| error.to_string())?;
    bootstrap_image_database(pool).await
}
