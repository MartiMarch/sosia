use deadpool_postgres::{
    tokio_postgres,
    Runtime,
    Config,
    Pool
};
use crate::services::{
    configuration_srv as conf_srv
};
use tokio_postgres::NoTls;
use once_cell::sync::Lazy;
use crate::adapters::{
    postgres_ad,
};


static POSTGRES_POOL: Lazy<Pool> = Lazy::new(|| {
    let configuration = Config {
        user: Some(conf_srv::get(None).postgres_user.clone()),
        password:Some(conf_srv::get(None).postgres_password.clone()),
        host: Some(conf_srv::get(None).postgres_host.clone()),
        port: Some(conf_srv::get(None).postgres_port.clone()),
        dbname: Some(conf_srv::get(None).postgres_database.clone()),
        ..Default::default()
    };
    configuration.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create postgres pool")
});


pub fn get_pool() -> &'static Pool {
    &POSTGRES_POOL
}

pub async fn initialize_database() {
    let pool= postgres_ad::get_pool();
    let client = pool.get().await.unwrap();

    let exists_database = client.query(
        "SELECT 1 FROM pg_database WHERE datname = 'sosia';",
        &[]
    ).await.unwrap();
    if exists_database.is_empty() {
        client.execute("CREATE DATABASE sosia;", &[]).await.unwrap();
    }
}
