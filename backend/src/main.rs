//cargo run -F debug
#[cfg(feature = "debug")]
const DEBUG: bool = true;

//cargo run (par defaut)
#[cfg(not(feature = "debug"))]
const DEBUG: bool = false;

mod auth;
mod appstate;

use axum::{
    routing::{get, post},
    Router,
};
use appstate::AppState;

#[tokio::main]
async fn main() 
{
    let app_state = AppState::new();

    // Routing
    let app = Router::new()
                        .route("/api/health", get(health_check))
                        .route("/api/register", post(auth::register))
                        .route("/api/users", get(auth::list_users))
                        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
                                .await
                                .expect("failed to bind on port 3000"); //remplace .unwrap()
    if DEBUG {
        println!("\nServer running on http://localhost:3000");
    }
    
    axum::serve(listener, app)
            .await
            .expect("server error");
}

async fn health_check() -> &'static str
{
    "Backend opérationnel !"
}
