use std::sync::OnceLock;
use diesel::Connection;

pub mod schema;
pub mod nodes;


fn get_conn() -> diesel::prelude::SqliteConnection {
    static DB_URL: OnceLock<String> = OnceLock::new();
    let db_url = DB_URL.get_or_init(||{
        dotenvy::dotenv().ok();
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env")
    });

    diesel::sqlite::SqliteConnection::establish(db_url).expect("Error connecting to the database")
}
