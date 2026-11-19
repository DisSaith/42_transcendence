#![allow(unused)]

use axum::{
    Json,
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
    routing::{get, post},
    Router,
    extract::State,
    response::{Html, IntoResponse}
};
use std::sync::{Arc, Mutex};

struct User
{
    nbr: u32,
}

#[tokio::main]
async fn main() 
{
    let users = Arc::new(Mutex::new(User { nbr: 0 }));
    // Routing
    let app = Router::new()
                        .route("/", get(welcome_page))
                        .route("/test", get(test))
                        .route("/hello", get(hello))
                        .with_state(users);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
                                .await
                                .expect("failed to bind on port 3000"); //remplace .unwrap()
    println!("\nServer running on http://localhost:3000");
    
    axum::serve(listener, app)
            .await
            .expect("server error");
}

/* --- Handlers --- */

async fn welcome_page() -> &'static str
{
    "Welcome on our ft_transcendence !"
}

async fn test() -> &'static str
{
    "new page test"
}

async fn hello(State(users): State<Arc<Mutex<User>>>) -> impl IntoResponse
{
    let mut data = users.lock().unwrap(); //mutex (unlock at the end of the handler)
    data.nbr += 1;
    println!("current nbr users -> {}", data.nbr);
    Html("<Strong>Hello Buddy</strong>")
}
