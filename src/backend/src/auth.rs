use argon2::{password_hash::PasswordHasher, Argon2};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::appstate::{AppState, User};

// Données attendues dans le corps JSON de la requête POST
#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20, message = "Le pseudo doit faire entre 3 et 20 caractères"), 
                custom(function = "validate_username"))]
    pub username: String,
    #[validate(email(message = "Format d'email invalide"))]
    pub email: String,
    #[validate(length(min = 8, message = "Le mot de passe doit faire au moins 8 caractères"))]
    pub password: String,
}

//Custom check for username
fn validate_username(username: &str) -> Result<(), ValidationError>
{
    for i in username.chars()
    {
        if i == ' '
        {
            return Err(ValidationError::new("Le pseudo ne doit pas contenir d'espace."));
        }
    }
    Ok(())
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<ErrorResponse>)> {
    // 1. Validation des champs (exigence obligatoire du sujet)
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: validation_errors.to_string(),
            }),
        ));
    }

    // 2. Hachage et salage du mot de passe avec Argon2
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes())
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: String::from("Erreur lors du hachage du mot de passe"),
                }),
            )
        })?
        .to_string();

    // 3. Verrouillage en écriture de la mémoire partagée
    let mut users = state.users.write().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: String::from("Erreur interne du serveur"),
            }),
        )
    })?;

    // Vérifier si l'email est déjà utilisé
    if users.iter().any(|u| u.email == payload.email) {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: String::from("Cet email est déjà utilisé"),
            }),
        ));
    }

    let new_user = User {
        id: users.len() + 1,
        username: payload.username,
        email: payload.email,
        password_hash,
    };

    users.push(new_user.clone());

    // Renvoie le code HTTP 201 Created et le profil JSON (sans password_hash)
    Ok((StatusCode::CREATED, Json(new_user)))
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = state
        .users
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users.clone()))
}
