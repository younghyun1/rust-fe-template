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
    view! { <div style="background:black; color:white;">example text</div> }
}

#[component]
fn TitleBar() -> impl IntoView {
    let (state, set_state) = signal(AppState { logged_in: false });

    view! {}
}
