use crate::domain::po::logger_message_type_po::LogType as LogType;
use crate::domain::logger_message_dom::LoggerMessage;
use crate::services::configuration_srv as conf_srv;
use crate::services::logger_svc as logger_svc;
use crate::domain::date_dom::Date as Date;

use deadpool_postgres::tokio_postgres;
use deadpool_postgres::Runtime;
use deadpool_postgres::Config;
use deadpool_postgres::Pool;
use tokio_postgres::NoTls;
use once_cell::sync::Lazy;


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
    let message_error: String = logger_svc::log_to_str(
        &logger_svc::str_to_log(&"Failed to initialize database client".to_string(), &LogType::ERROR)
    );
    let client = POSTGRES_POOL.get().await.expect(&message_error);

    match client.execute("
        CREATE TABLE IF NOT EXISTS namespaces (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL
        );", &[]).await {
        Ok(_) => {
            logger_svc::log(&LoggerMessage::new_simplified(
                LogType::INFO,
                "Database 'namespace' created".to_string()
            ))
        },
        Err(err) => {
            logger_svc::log(&LoggerMessage::new_simplified(
                LogType::ERROR,
                "Something goes wrong creating 'namespace' database".to_string()
            ))
        }
    }
}
