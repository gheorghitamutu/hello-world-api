use warp::Filter;
use std::env;
use tokio::time::{sleep, Duration};
use tracing::{info, debug, trace};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with TRACE as default level
    let result = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("hello_world_api=trace,warp=trace"))
        )
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .try_init();
    
    match result {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to initialize tracing: {}", e);
            // Fall back to println for debugging
            println!("=== Hello World API Starting (tracing failed) ===");
        }
    }

    info!("=== Hello World API Starting ===");
    
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
    
    info!("Configuration:");
    info!("- COUNTDOWN_SECONDS: {}", countdown_seconds);
    info!("- PORT: {}", port);
    debug!("- User: {:?}", env::var("USER"));
    debug!("- Home: {:?}", env::var("HOME"));
    trace!("Trace level logging enabled - very detailed debugging info will be shown");
    
    // Countdown loop - configurable duration
    if countdown_seconds > 0 {
        info!("Starting countdown before server initialization...");
        
        for seconds_left in (1..=countdown_seconds).rev() {
            info!("⏰ Time left: {} seconds", seconds_left);
            sleep(Duration::from_secs(1)).await;
        }
        
        info!("⏰ Countdown complete! Initializing server...");
    } else {
        info!("⏰ Countdown disabled, starting server immediately...");
    }
    
    info!("Server binding to 0.0.0.0:{}...", port);
    
    // Define all routes
    let hello = warp::path("hello")
        .map(|| {
            trace!("Processing /hello request");
            info!("Received request to /hello");
            warp::reply::html("Hello, World!")
        });

    let health = warp::path("health")
        .map(|| {
            trace!("Processing /health request");
            info!("Health check requested");
            warp::reply::html("OK")
        });

    let root = warp::path::end()
        .map(|| {
            trace!("Processing root / request");
            info!("Root endpoint accessed");
            warp::reply::html("Hello World API is running! Try /hello or /health")
        });

    let routes = hello.or(health).or(root)
        .with(warp::trace::request());
    
    // Start server with error handling
    info!("✅ Starting server on port {}", port);
    info!("🚀 Access the API at:");
    info!("   - http://localhost:{}/", port);
    info!("   - http://localhost:{}/hello", port);
    info!("   - http://localhost:{}/health", port);
    
    // Simple server start - let Warp handle errors
    info!("🔗 Binding to 0.0.0.0:{}", port);
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
