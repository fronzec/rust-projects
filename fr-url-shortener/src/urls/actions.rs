use crate::models;
use crate::models::Url;
use diesel::r2d2::ConnectionManager;
use diesel::{insert_into, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_url(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    new_url: Url,
) -> Result<Url, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::urls::dsl::*;
    let inserted_rows = insert_into(urls)
        .values(new_url)
        .execute(&conn)
        .expect("insert must be executed correctly");
    println!("inserted rows {inserted_rows}");

    let mut results = urls
        .find(10)
        .load::<models::Url>(&conn)
        .expect("Error loading Url for the provided PK");
    Ok(results.pop().unwrap())
}
