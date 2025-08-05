use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a simple route that responds with "Hello, World!"
    let hello = warp::path("hello")
        .map(|| warp::reply::html("Hello, World!"));

    println!("Start the server on port 8080...");
    warp::serve(hello)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
