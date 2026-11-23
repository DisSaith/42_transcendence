use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct HealthResponse {
    status: String,
}

#[component]
fn App() -> impl IntoView {
    let (backend_status, set_backend_status) = signal("Connexion au backend...".to_string());

    // Exécuté au montage du composant
    Effect::new(move |_| {
        spawn_local(async move {
            match Request::get("/api/health").send().await {
                Ok(resp) if resp.ok() => {
                    if let Ok(data) = resp.json::<HealthResponse>().await {
                        set_backend_status.set(format!("Connecté : {}", data.status));
                    } else {
                        set_backend_status.set("Réponse reçue, mais format JSON inattendu".to_string());
                    }
                }
                Ok(resp) => set_backend_status.set(format!("Erreur HTTP : {}", resp.status())),
                Err(err) => set_backend_status.set(format!("Backend inaccessible : {err}")),
            }
        });
    });

    view! {
        <main>
            <h1>"ft_transcendence"</h1>
            <p>{backend_status}</p>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
