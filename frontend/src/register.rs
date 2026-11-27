use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::ev::SubmitEvent;
use leptos::task::spawn_local;
use serde::{Serialize, Deserialize};

// Structure envoyée au Backend
#[derive(Serialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

// Réponse si succès dans la création du compte
#[derive(Deserialize, Clone, Debug)]
struct UserResponse {
    id: usize,
    username: String,
    email: String,
}

// Réponse en cas d'erreur dans la création du compte
#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[component]
pub fn RegisterPage() -> impl IntoView
{
    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (feedback, set_feedback) = signal(String::new());
    let reset = move |_| {  
                    set_password.set(String::new());
                    set_username.set(String::new()); 
                    set_email.set(String::new()); 
    };

    let on_submit = move |ev: SubmitEvent| {
        // empêche la page d'être rechargée durant la registration
        ev.prevent_default();

        let registration = RegisterRequest {
            username: username.get(),
            email: email.get(),
            password: password.get(),
        };

        spawn_local(async move {
            let request = match Request::post("/api/register").json(&registration) {
                Ok(req) => req,
                Err(err) => {
                    set_feedback.set(format!("Erreur Json: {}", err));
                    return;
                }
            };
            match request.send().await {
                Ok(resp) if resp.status() == 201 => {
                    if let Ok(user) = resp.json::<UserResponse>().await {
                        set_feedback.set(format!(
                                "Compte créé ! : ID #{} - {} ({})",
                                user.id, user.username, user.email));
                    }
                }
                Ok(resp) => {
                    if let Ok(err_resp) = resp.json::<ErrorResponse>().await {
                        set_feedback.set(format!("Refusé ({}) : {}",
                                        resp.status(), err_resp.error));
                    }
                    else {
                        set_feedback.set(format!("Erreur HTTP : {}", resp.status()));
                    }
                }
                Err(err) => {
                    set_feedback.set(format!("Erreur : backend injoignable ({})", err));
                }
            };
        });
    };

    view! {
        <main>
            <h1> "Inscription" </h1>
            <form on:submit=on_submit>
                <div>
                    <label> "Pseudonyme :" </label>
                    <input
                    type="text"
                    prop:value=username
                    on:input=move |ev| set_username.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label> "Email :" </label>
                    <input
                    type="text"
                    prop:value=email
                    on:input=move |ev| set_email.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <label> "Mot de passe :" </label>
                    <input
                    type="password"
                    prop:value=password
                    on:input=move |ev| set_password.set(event_target_value(&ev))
                    />
                </div>
                <div>
                    <button type="reset"
                    on:click=reset>
                    "Réinitialiser"
                    </button>
                    <button type="submit">
                    "Envoyer"
                    </button>
                </div>
            </form>
            <p> {feedback} </p>
        </main>
    }
}
