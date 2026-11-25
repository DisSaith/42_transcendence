use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use serde::{Serialize, Deserialize};

#[component]
fn HomePage() -> impl IntoView
{
    let (count, set_count) = signal(0);
    
    view! {
        <div>
            <h1> "DessinRapide" </h1>
            <h4> "ft_transcendence de acohaut" </h4>
            <A href="/register">"Créer son compte"</A>
            <hr/>
        </div>
        <div>
            <button
                on:click=move |_| {*set_count.write() += 1;}
            >
                "click me: " {count}
            </button>
            <p>
                "Double count: "
                {move || count.get() * 2}
            </p>
        </div>
    }
}

#[component]
fn RegisterPage() -> impl IntoView
{
    let (name, set_name) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let reset = move |_| {  
                    set_password.set(String::new());
                    set_name.set(String::new()); 
    };

    view! {
        <div>
            <h1> "Inscription" </h1>
        </div>
        <div>
            <input
                type="text"
                placeholder="Nom"
                prop:value=name
                on:input=move |ev| set_name.set(event_target_value(&ev))
            />
            <input
                type="password"
                placeholder="Mot de Passe"
                prop:value=password
                on:input=move |ev| set_password.set(event_target_value(&ev))
            />
            <button
                on:click=reset>
                "Reset"
            </button>
            <br> </br>
            <p> "Votre nom: " {name}</p>
            <hr> </hr>
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

            // Zone qui change automatiquement selon l'URL
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
    // for better panic messages in the browser console
    console_error_panic_hook::set_once();
    
    mount_to_body(App);
}
