use crate::domain::po::logger_message_type_po::LogType as LogType;
use crate::domain::logger_message_dom::LoggerMessage;
use crate::services::configuration_srv as conf_srv;
use crate::services::logger_srv as logger_srv;

use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::Runtime;
use deadpool_postgres::Config;
use deadpool_postgres::Object;
use deadpool_postgres::Pool;
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

pub async fn get_client() -> Object {
    let message_error: String = logger_srv::log_to_str(
        &logger_srv::str_to_log(&"Failed to initialize database client".to_string(), &LogType::ERROR)
    );
    POSTGRES_POOL.get().await.expect(&message_error)
}

async fn exec(sql_command: &String) {
    match get_client().await.execute(sql_command.as_str(), &[]).await {
        Ok(_) => {
            logger_srv::log(&LoggerMessage::new_simplified(
                LogType::INFO,
                "Table 'namespace' created".to_string()
            ))
        },
        Err(err) => {
            logger_srv::log(&LoggerMessage::new_simplified(
                LogType::ERROR,
                format!("Something goes wrong creating 'namespace' database: {err}")
            ))
        }
    }
}

pub async fn initialize_database() {
    exec(
        &"CREATE TABLE IF NOT EXISTS namespace (
            name VARCHAR(255) NOT NULL PRIMARY KEY
        );".to_string()
    ).await;
}
