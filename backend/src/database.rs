use std::sync::OnceLock;
use diesel::{connection::SimpleConnection, Connection};

pub mod schema;
pub mod nodes;


fn get_conn() -> diesel::prelude::SqliteConnection {
    static DB_URL: OnceLock<String> = OnceLock::new();
    let db_url = DB_URL.get_or_init(||{
        dotenvy::dotenv().ok();
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env")
    });

    let mut conn = diesel::sqlite::SqliteConnection::establish(db_url).expect("Error connecting to the database");
    conn.batch_execute("PRAGMA foreign_keys = ON").expect("Error enabling foreign key constraints");
    return conn;
}
