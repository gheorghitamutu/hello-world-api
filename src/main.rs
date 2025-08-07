use warp::Filter;
use std::env;

#[tokio::main]
async fn main() {
    println!("=== Hello World API Starting ===");
    eprintln!("=== Hello World API Starting ===");
    
    // Get port from environment or default to 8080
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);
    
    println!("Environment check:");
    println!("- PORT: {}", port);
    println!("- User: {:?}", env::var("USER"));
    println!("- Home: {:?}", env::var("HOME"));
    
    println!("Server binding to 0.0.0.0:{}...", port);
    eprintln!("Server binding to 0.0.0.0:{}...", port);
    
    // Define all routes
    let hello = warp::path("hello")
        .map(|| {
            println!("Received request to /hello");
            warp::reply::html("Hello, World!")
        });

    let health = warp::path("health")
        .map(|| {
            println!("Health check requested");
            warp::reply::html("OK")
        });

    let root = warp::path::end()
        .map(|| {
            println!("Root endpoint accessed");
            warp::reply::html("Hello World API is running! Try /hello or /health")
        });

    let routes = hello.or(health).or(root);
    
    // Try to bind to the primary port first
    println!("✅ Starting server on port {}", port);
    println!("🚀 Access the API at:");
    println!("   - http://localhost:{}/", port);
    println!("   - http://localhost:{}/hello", port);
    println!("   - http://localhost:{}/health", port);
    eprintln!("✅ Server starting on port {}", port);
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
