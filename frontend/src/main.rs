use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

mod register;

use crate::register::RegisterPage;

fn maj_backend_status(backend_status: WriteSignal<String>) {
    Effect::new(move |_| {
        spawn_local(async move {
            match Request::get("/api/health").send().await {
                Ok(resp) if resp.ok() => backend_status.set(format!("Opérationnel [{}]", resp.status())),
                Ok(resp) => backend_status.set(format!("Déconnecté [{}]", resp.status())),
                Err(er) => backend_status.set(format!("Erreur : {}", er)),
            }
        });
    });
}

#[component]
fn HomePage() -> impl IntoView {
    let (backend_status, set_backend_status) = signal("chargement...".to_string());

    maj_backend_status(set_backend_status);

    view! {
        <div class="qd-card">
            <h1 class="qd-title">"DessinRapide"</h1>
            <p class="qd-subtitle">"ft_transcendence de nofelten & acohaut"</p>

            <A href="/register" attr:class="qd-btn">"Créer son compte"</A>

            <hr class="qd-divider" />

            <div class="qd-status-badge">
                "Backend : " {backend_status}
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
