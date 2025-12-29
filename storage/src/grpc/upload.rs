use common::models::file::File;
use crate::upload::{UploadRequest, UploadResponse};
use crate::upload::upload_service_server::UploadService;
use common::db::queries::insert_file_async;
use tokio_stream::StreamExt;    
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::io::AsyncWriteExt;
use chrono::Utc;
use common::db::connection::DbPool;

#[derive(Clone)]
pub struct UploadServiceImpl {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl UploadService for UploadServiceImpl {
    async fn upload_file(
        &self,
        request: tonic::Request<tonic::Streaming<UploadRequest>>,
    ) -> Result<tonic::Response<UploadResponse>, tonic::Status> {
        let mut stream = request.into_inner();
        let mut final_size: i64 = 0;
        let mut filename = String::new();
        let mut user_id_int: i32 = 0;
        let mut file_path = String::new();
        let mut file_handle = None;

        while let Some(result) = stream.next().await {
            let chunk = result.map_err(|e| tonic::Status::internal(e.to_string()))?;

            if file_handle.is_none() {
                // CAPTURE METADATA
                user_id_int = chunk.user_id;
                filename = chunk.filename.clone();

                // Validation: Ensure we aren't getting 0/Empty
                if user_id_int == 0 {
                    return Err(tonic::Status::invalid_argument("user_id cannot be 0"));
                }

                let folder_path = format!("./files/{}/", user_id_int);
                file_path = format!("{}{}", folder_path, filename);

                create_dir_all(&folder_path).await
                    .map_err(|e| tonic::Status::internal(format!("Dir error: {}", e)))?;

                let h = OpenOptions::new()
                    .create(true).write(true).truncate(true).open(&file_path).await
                    .map_err(|e| tonic::Status::internal(format!("File error: {}", e)))?;
                file_handle = Some(h);
            }

            if let Some(ref mut h) = file_handle {
                h.write_all(&chunk.data).await
                    .map_err(|e| tonic::Status::internal(format!("Write error: {}", e)))?;
                final_size += chunk.data.len() as i64;
            }
        }

        // IMPORTANT: Drop the file handle so it's flushed to disk before DB entry
        drop(file_handle);

        // FINAL CHECK before DB
        println!("{}", user_id_int);
        if user_id_int == 0 {
            return Err(tonic::Status::internal("Failed to receive valid user_id"));
        }

        let now = Utc::now().timestamp();
        insert_file_async(self.pool.clone(), File {
            id: None,
            user_id: user_id_int,
            file_name: filename.clone(),
            file_path: file_path.clone(),
            file_size: final_size,
            created_at: now,
            updated_at: now,
        })
        .await
        .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;

        Ok(tonic::Response::new(UploadResponse {
            id: format!("Saved {} bytes", final_size),
            filename,
            filepath: file_path,
        }))
    }
}