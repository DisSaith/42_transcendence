use gloo_net::http::Request;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

mod register;

use crate::register::RegisterPage;

async fn fetch_backend_status() -> String
{
    match Request::get("/api/health").send().await 
    {
        Ok(resp) if resp.ok() => format!("Connecté [{}]", resp.status()),
        Ok(resp) => format!("Déconnecté [{}]", resp.status()),
        Err(err) => format!("Erreur [{}]", err)
    }
}

#[component]
fn BackendStatus() -> impl IntoView 
{
    let backend_status = LocalResource::new(move || fetch_backend_status());

    view!{
        <p> "Backend : "
        { move || match backend_status.get() 
            {
                None => "en attente..".to_string(),
                Some(status) => status,
            }
        }
        </p>
    }
}

#[component]
fn HomePage() -> impl IntoView 
{
    view! {
        <div class="qd-card">
            <h1 class="qd-title">"DessinRapide"</h1>
            <p class="qd-subtitle">"ft_transcendence de nofelten & acohaut"</p>

            <A href="/register" attr:class="qd-btn">"Créer son compte"</A>

            <hr class="qd-divider" />

            <div class="qd-status-badge">
            <BackendStatus />  // component dyw
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            // Bare de navigation affichée sur toutes les pages
            <nav class="qd-nav">
                <A href="/">"Accueil"</A>
                <A href="/register">"Inscription"</A>
            </nav>
            
            // Différentes pages & URL
            <main class="qd-container">
                <Routes fallback=|| view! { <div class="qd-card"><h2 class="qd-title">"404 — Page introuvable"</h2></div> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/register") view=RegisterPage />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    // affiche les messages d'erreur dans la console du navigateur
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
