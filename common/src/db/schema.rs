use crate::db::connection::DbPool; // Make sure this path is correct
use rusqlite::Result;

pub fn init_db(pool: DbPool) -> Result<()> {
    let conn = pool.get().map_err(|e| {
        rusqlite::Error::ToSqlConversionFailure(Box::new(e))
    })?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS file (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL UNIQUE,
            file_path TEXT NOT NULL,
            file_name TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            FOREIGN KEY (user_id)
                REFERENCES user(id)
                ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY,
            display_name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at INTEGER NOT NULL, 
            updated_at INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS refresh (
            token_hash TEXT PRIMARY KEY,
            user_id INTEGER NOT NULL UNIQUE,
            created_at INTEGER NOT NULL, 
            expires_at INTEGER NOT NULL,
            FOREIGN KEY (user_id)
                REFERENCES user(id)
                ON DELETE CASCADE
        )",
        [],
    )?;

    Ok(())
}