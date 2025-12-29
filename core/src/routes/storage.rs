use axum::extract::DefaultBodyLimit;
use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Extension, Json, Router,
};
use axum_extra::extract::Multipart; 
use common::utils::tonic_to_http_response;
use serde::Serialize;
use crate::proto::{UploadRequest, UploadResponse};
use crate::routes::AppState;
use crate::middleware::jwt::UserId;

#[derive(Serialize)]
pub struct StorageResponse {
    pub message: String,
    pub file_size: usize,
    pub file_path: String,
    pub file_name: String,
}

async fn upload(
    State(state): State<AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    mut multipart: Multipart,
) -> Result<Json<StorageResponse>, (StatusCode, String)> {
    let mut data = None;
    let mut upload_client = state.upload_grpc_client.clone();
    let mut file_name = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("Multipart error: {}", err)))? 
    {
        let name = field.name().unwrap_or_default().to_string();

        match name.as_str() {
            "file" => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file".into()))?;
                data = Some(bytes.to_vec());
            }
            "file_name" => {
                let text = field
                    .text()
                    .await
                    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read name".into()))?;
                file_name = Some(text);
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }

    let data = data.ok_or((StatusCode::BAD_REQUEST, "Missing file data".into()))?;
    println!("{}", user_id.to_string());
    let request = UploadRequest {
        data: data.to_vec(),
        filename: file_name.ok_or((StatusCode::BAD_REQUEST, "Missing file_name".into()))?,
        user_id: user_id
    };  

    let response: tonic::Response<UploadResponse> = upload_client
        .upload_file(tokio_stream::iter(std::iter::once(request)))
        .await
        .map_err(|e| {
            eprintln!("Error in gRPC upload: {:?}", e);
            tonic_to_http_response(e)
        })?;
    
    let inner = response.into_inner();
    
    Ok(Json(StorageResponse {
        message: "Successfully saved file".to_string(),
        file_name: inner.filename,
        file_path: inner.filepath,
        file_size: data.len(),
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::max(1000 * 1024 * 1024))
}   