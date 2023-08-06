use std::env;
use std::io::{Error, ErrorKind};

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;

pub(crate) type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// Macro to embed the migrations into the compiled applications.
embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let database_url = env::var("APP_DATABASE_URL")
        .expect("APP_DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = Pool::builder().max_size(25).build(manager);
        println!("pool");
        match result {
            Ok(pool) => {
                println!("pool created successfully");
                pool
            },
            Err(err) => {
                println!("cannot create pool {}", err.to_string());
                panic!()
            }
        }
    // Basic Pool Manager implementation
    // Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, Error> {
    POOL.get().map_err(|e| {
        eprintln!("error getting connection");
        Error::new(
            ErrorKind::Other,
            format!("Failed getting DB connection: {}", e),
        )
    })
}

pub fn get_pool_clone() -> Pool {
    POOL.clone()
}
