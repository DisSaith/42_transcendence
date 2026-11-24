use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;

#[component]
fn App() -> impl IntoView 
{
    let (count, set_count) = signal(0);
    let (name, set_name) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let reset = move |_| {  
                    set_password.set(String::new());
                    set_name.set(String::new()); 
    };

    view! {
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

fn main() {
    // for better panic messages in the browser console
    console_error_panic_hook::set_once();
    
    mount_to_body(App);
}
