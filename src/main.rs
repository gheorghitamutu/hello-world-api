use warp::Filter;
use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== Hello World API Starting ===");
    eprintln!("=== Hello World API Starting ===");
    
    // Get countdown duration from environment or default to 10 seconds for faster startup
    let countdown_seconds: u64 = env::var("COUNTDOWN_SECONDS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(10);
    
    // Get port from environment or default to 8080
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);
    
    println!("Configuration:");
    println!("- COUNTDOWN_SECONDS: {}", countdown_seconds);
    println!("- PORT: {}", port);
    println!("- User: {:?}", env::var("USER"));
    println!("- Home: {:?}", env::var("HOME"));
    
    // Countdown loop - configurable duration
    if countdown_seconds > 0 {
        println!("Starting countdown before server initialization...");
        eprintln!("Starting countdown before server initialization...");
        
        for seconds_left in (1..=countdown_seconds).rev() {
            println!("⏰ Time left: {} seconds", seconds_left);
            eprintln!("⏰ Time left: {} seconds", seconds_left);
            sleep(Duration::from_secs(1)).await;
        }
        
        println!("⏰ Countdown complete! Initializing server...");
        eprintln!("⏰ Countdown complete! Initializing server...");
    } else {
        println!("⏰ Countdown disabled, starting server immediately...");
        eprintln!("⏰ Countdown disabled, starting server immediately...");
    }
    
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
    
    // Start server with error handling
    println!("✅ Starting server on port {}", port);
    println!("🚀 Access the API at:");
    println!("   - http://localhost:{}/", port);
    println!("   - http://localhost:{}/hello", port);
    println!("   - http://localhost:{}/health", port);
    eprintln!("✅ Server starting on port {}", port);
    
    // Simple server start - let Warp handle errors
    println!("🔗 Binding to 0.0.0.0:{}", port);
    eprintln!("🔗 Binding to 0.0.0.0:{}", port);
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
