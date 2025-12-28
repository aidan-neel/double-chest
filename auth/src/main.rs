mod grpc;
mod jwt;

use tonic::transport::Server;
use proto::token_service_server::TokenServiceServer;
use grpc::token::TokenServiceImpl;

use common::db::connection::establish_pool;
use common::db::schema::init_db;

pub mod proto {
    tonic::include_proto!("auth");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let pool = establish_pool();
    init_db(pool.clone())?; 

    let token_service = TokenServiceImpl { 
        pool: pool.clone() 
    };

    println!("TokenService listening on {}", addr);

    Server::builder()
        .add_service(TokenServiceServer::new(token_service))
        .serve(addr)
        .await?;

    Ok(())  
}