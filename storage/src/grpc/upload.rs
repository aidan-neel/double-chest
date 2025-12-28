use common::models::file::File;
use crate::upload::{UploadRequest, UploadResponse};
use crate::upload::upload_service_server::UploadService;
use common::db::queries::insert_file_async;
use rusqlite::Connection;
use tokio_stream::StreamExt;    
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::io::AsyncWriteExt;
use std::sync::{Arc, Mutex};
use chrono::Utc;

#[derive(Clone)]
pub struct UploadServiceImpl {
    pub conn: Arc<Mutex<Connection>>
}

#[tonic::async_trait]
impl UploadService for UploadServiceImpl {
    async fn upload_file(
        &self,
        request: tonic::Request<tonic::Streaming<UploadRequest>>,
    ) -> Result<tonic::Response<UploadResponse>, tonic::Status> {
        let mut stream = request.into_inner();
        let mut final_data = Vec::new();
        let conn = self.conn.clone();
        
        while let Some(result) = stream.next().await {
            match result {
                Ok(chunk) => {
                    let folder_path = "./files/user1/";  
                    let file_path = format!("{}{}", folder_path, chunk.filename);
                    create_dir_all(folder_path)
                        .await
                        .map_err(|e| tonic::Status::internal(format!("Failed to create directory: {}", e)))?;
                    
                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(false)
                        .open(&file_path)
                        .await
                        .map_err(|e| tonic::Status::internal(format!("File error: {}", e)))?;
                    
                    file.write_all(&chunk.data)
                        .await
                        .map_err(|e| tonic::Status::internal(format!("Failed to write chunk: {}", e)))?;
                    
                    final_data.extend(chunk.data);
                    let now = Utc::now().timestamp();
                    insert_file_async(conn.clone(), File {
                        id: None, // auto generated
                        file_name: chunk.filename,
                        file_path: file_path,
                        file_size: final_data.len() as i64,
                        created_at: now,
                        updated_at: now,
                    })
                        .await
                        .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;
                }
                Err(err) => {
                    return Err(tonic::Status::internal(format!("Stream error {:?}", err)))
                }
            }
        }

        Ok(tonic::Response::new(UploadResponse {
            id: format!("Saved {} bytes", final_data.len())
        }))
    }
}