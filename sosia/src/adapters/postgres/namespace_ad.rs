use crate::adapters::postgres_ad as postgres_ad;
use crate::domain::namespace_dom::Namespace;

use deadpool_postgres::tokio_postgres::GenericClient;
use deadpool_postgres::tokio_postgres::types::ToSql;
use deadpool_postgres::tokio_postgres::Error;
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Object;


pub async fn get(namespace: &String) -> Result<Option<Namespace>, Error> {
    let client: Object = postgres_ad::get_client().await;
    let row: Option<Row> = client.query_opt(
        "SELECT name FROM namespace WHERE name = $1;",
        &[namespace as &(dyn ToSql + Sync)],
    ).await?;

    Ok(row.map(|row| Namespace {
       name: row.get("name"),
    }))
}
