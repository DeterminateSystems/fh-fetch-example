use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<html><head><script src=\"https://unpkg.com/@tailwindcss/browser@4\"></script></head><body class=\"bg-black text-white\"><main class=\"p-8\"><h1 class=\"text-5xl font-mono\">Hello from FlakeHub Cache!</h1></main></body></html>")
}
