use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

mod register;

use crate::register::RegisterPage;

fn maj_backend_status(backend_status: WriteSignal<String>)
{
    Effect::new(move |_| {
        spawn_local(async move {
            match Request::get("/api/health").send().await
        {
            Ok(resp) if resp.ok() => backend_status.set(format!("Opérationnel [{}]", resp.status())),
            Ok(resp) => backend_status.set(format!("Déconnecté [{}]", resp.status())),
            Err(er) => backend_status.set(format!("Error {}", er)),
        }})
        });
}

#[component]
fn HomePage() -> impl IntoView
{
    let (backend_status, set_backend_status) = signal("chargement..".to_string());

    maj_backend_status(set_backend_status);

    view! {
        <div>
            <h1> "DessinRapide" </h1>
            <h4> "ft_transcendence de nofelten & acohaut" </h4>
            <hr/>
        </div>
        <div>
            <A href="/register">"Créer son compte"</A>
            <hr/>
        </div>
        <div>
            <h6> "Backend status : " {backend_status} </h6>
        </div>
    }
}

#[component]
fn App() -> impl IntoView 
{
    view! {
        <Router>
            // Barre de navigation affichée sur toutes les pages
            <nav>
                <A href="/">"Accueil"</A>
                " | "
                <A href="/register">"Register"</A>
            </nav>
            <hr/>

            // Différentes pages & URL
            <main>
                <Routes fallback=|| "Page introuvable (404)">
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
