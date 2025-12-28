use rusqlite::params;
use crate::{db::connection::DbPool, models::{file::File, refresh::Refresh, user::User}};
use tokio::task;

pub async fn insert_file_async(pool: DbPool, file: File) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {  
    task::spawn_blocking(move || {
        let conn = pool.get().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        conn.execute(
            "INSERT INTO file (file_path, file_name, file_size) VALUES (?1, ?2, ?3)",
            params![file.file_path, file.file_name, file.file_size as i64],
        )?;
        Ok(())
    })
    .await? // Handles JoinError
    .map_err(|e: rusqlite::Error| e.into()) // Handles rusqlite error
}

pub async fn insert_refresh_async(pool: DbPool, refresh: Refresh) -> Result<(), Box<dyn std::error::Error + Send + Sync>> { 
    task::spawn_blocking(move || {
        let conn = pool.get().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        
        conn.execute(
            "INSERT INTO refresh (token_hash, user_id, expires_at, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![refresh.token_hash, refresh.user_id, refresh.expires_at, refresh.created_at],
        )?;
        Ok(())
    })
    .await?
    .map_err(|e: rusqlite::Error| e.into())
} 

pub async fn insert_user_async(pool: DbPool, user: User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_result = task::spawn_blocking(move || -> rusqlite::Result<()> {
        let conn = pool.get().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        conn.execute(
            r#"INSERT INTO user (email, password_hash, display_name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)"#,
            params![user.email, user.password_hash, user.display_name, user.created_at, user.updated_at],
        )?;

        Ok(())
    })
    .await?;    

    Ok(user_result?)
}

pub async fn get_user(pool: DbPool, email: String) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
    let user_result = task::spawn_blocking(move || -> rusqlite::Result<Option<User>> {
        let conn = pool.get().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        let res = conn.query_row(
            r#"SELECT id, display_name, email, password_hash, created_at, updated_at FROM "user" WHERE email = ?1"#,
            params![email],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    display_name: row.get(1)?,
                    email: row.get(2)?,
                    password_hash: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        );

        match res {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    })
    .await?;    

    Ok(user_result?)
}