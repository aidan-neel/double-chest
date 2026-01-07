use rusqlite::{Connection, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn establish_connection() -> Result<Connection> {
    let path = "../data/db.sqlite3";  
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            eprintln!("Failed to create database directory: {}", e);
        }).ok(); 
    }
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub fn establish_pool() -> DbPool {
    let path = "./data/db.sqlite3";
    
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let manager = SqliteConnectionManager::file(path);
    
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}