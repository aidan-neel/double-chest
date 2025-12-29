mod grpc;

use tonic::transport::Server;
use upload::upload_service_server::UploadServiceServer;
use grpc::upload::UploadServiceImpl;

use common::db::connection::{establish_pool};

use common::db::schema::init_db;

pub mod upload {
    tonic::include_proto!("upload");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let pool = establish_pool();
    init_db(pool.clone())?; 
    let upload_service = UploadServiceImpl { pool: pool.clone() };

    println!("UploadService listening on {}", addr);

    Server::builder()
        .add_service(UploadServiceServer::new(upload_service))
        .serve(addr)
        .await?;
    
    Ok(())
}
