use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(AppShell)
}

#[derive(Clone)]
struct AppState {
    logged_in: bool,
}

#[component]
fn AppShell() -> impl IntoView {
    let (response, set_response) = create_signal(String::new());
    let on_click = move |_| {
        spawn_local(async move {
            let client = reqwest::Client::new();
            match client.get("http://localhost:3000").send().await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => set_response(text),
                    Err(err) => set_response(format!("Error reading response: {}", err)),
                },
                Err(err) => set_response(format!("Request failed: {}", err)),
            }
        });
    };

    view! {
        <div style="background:black; color:white;">
            <TitleBar/>
            <div>
                <button on:click=on_click>"Fetch Data"</button>
            </div>
            <div>
                {move || response.get()}
            </div>
        </div>
    }
}

#[component]
fn TitleBar() -> impl IntoView {
    view! {
        <div style="font-size: 24px; font-weight: bold;">
            "Modern Leptos Page"
        </div>
    }
}
