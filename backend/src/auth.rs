use serde::Serialize;
use std::sync::{Arc, RwLock};

// User struct (for each user registered on the application)
#[derive(Clone, Serialize)]
pub struct User
{
    pub id: usize,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] //security: doesn't send the password
    pub hash_pws: String,
}

//State of the application
//allow us to access everywhere (any routes) in the application
#[derive(Clone)]
pub struct AppState
{
    pub users: Arc<RwLock<Vec<User>>>, //vector of users(struct User)
}

impl AppState
{
    pub fn new() -> Self
    {
        Self 
        {
            users: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
