use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() 
{
    // Routing
    let app = Router::new()
                        .route("/", get(welcome_page))
                        .route("/test", get(test));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/* --- Handlers --- */

async fn welcome_page() -> String
{
    format!("Welcome on our ft_transcendence !")
}

async fn test() -> String
{
    format!("new page test")
}
