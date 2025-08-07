use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Starting Hello World API...");
    eprintln!("Starting Hello World API...");
    
    // Define a simple route that responds with "Hello, World!"
    let hello = warp::path("hello")
        .map(|| {
            println!("Received request to /hello");
            warp::reply::html("Hello, World!")
        });

    println!("Server starting on 0.0.0.0:8080...");
    println!("Access the API at: http://localhost:8080/hello");
    eprintln!("Server starting on 0.0.0.0:8080...");
    eprintln!("Access the API at: http://localhost:8080/hello");
    
    warp::serve(hello)
        .run(([0, 0, 0, 0], 8080))
        .await;
        
    println!("Server stopped");
    eprintln!("Server stopped");
}
