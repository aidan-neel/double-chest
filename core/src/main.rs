mod routes;
mod middleware;

pub mod proto {
    tonic::include_proto!("upload"); 
    tonic::include_proto!("auth");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = routes::create_router().await?; 
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();     

    Ok(())
}
