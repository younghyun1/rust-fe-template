use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;

use crate::GlobalAppState;

#[component]
pub fn TopBar() -> impl IntoView {
    // Grab global state from context.
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");

    // Styling for a fixed top header.
    let header_style = "background-color: #222; width: 100%; position: fixed; top: 0; left: 0; padding: 0; margin: 0; font-family: sans-serif;";

    view! {
        <header class="top-bar" style=header_style>
            <div
                class="top-section"
                style="display: flex; align-items: center; justify-content: space-between; padding: 10px 15px;"
            >
                <div class="logo">
                    <a
                        href="/"
                        style="display: flex; align-items: center; text-decoration: none; color: white;"
                    >
                        <span style="font-size: 24px; margin-right: 8px;">"🏠"</span>
                        <span style="font-size: 20px;">"Younghyun's Blog"</span>
                    </a>
                </div>
                <div class="user-profile" style="display: flex; align-items: center;">
                    <UserProfile />
                </div>
            </div>
            <nav class="bottom-nav" style="background-color: #333; padding: 8px 15px;">
                <ul style="list-style: none; display: flex; gap: 15px; margin: 0; padding: 0;">
                    <li>
                        <a href="/" style="text-decoration: none; color: white;">
                            "Home"
                        </a>
                    </li>
                    <li>
                        <a href="/about" style="text-decoration: none; color: white;">
                            "About"
                        </a>
                    </li>
                    <li>
                        <a href="/works" style="text-decoration: none; color: white;">
                            "Works"
                        </a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}

#[component]
pub fn LoggedInUserProfile() -> impl IntoView {
    // Signal to control dropdown visibility.
    let (dropdown_open, set_dropdown_open) = signal(false);
    let toggle_dropdown = move |_| set_dropdown_open.update(|open| *open = !*open);

    view! {
        <div style="position: relative; display: flex; align-items: center;">
            <LogoutButton />
            <a
                on:click=toggle_dropdown
                href="#"
                style="text-decoration: none; color: white; margin-left: 10px;"
            >
                <span style="font-size: 24px; margin-left: 20px;">"👤"</span>
            </a>
            <span style="display: inline-block; width: 12px; height: 12px;
            border-radius: 50%; background-color: green; margin-left: 10px;"></span>
            {move || {
                if dropdown_open.get() {
                    view! {
                        <div style="position: absolute; top: 100%; right: 0;
                         background-color: #333; padding: 10px;
                         border: 1px solid white; border-radius: 4px; min-width: 150px;">
                            <ul style="list-style: none; padding: 0; margin: 0;">
                                <li style="padding: 5px 10px; cursor: pointer;">"Placeholder A"</li>
                                <li style="padding: 5px 10px; cursor: pointer;">"Placeholder B"</li>
                                <li style="padding: 5px 10px; cursor: pointer;">"Placeholder C"</li>
                            </ul>
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn LoggedOutUserProfile() -> impl IntoView {
    view! {
        <>
            <a
                href="/account/login"
                style="text-decoration: none; color: white; border: 1px solid white;
                padding: 4px 8px; border-radius: 4px;"
            >
                <div>"Login"</div>
            </a>
            <a
                href="/account/signup"
                style="text-decoration: none; color: white; border: 1px solid white;
                padding: 4px 8px; border-radius: 4px; margin-left: 20px;"
            >
                <div>"Sign Up"</div>
            </a>
            <span style="display: inline-block; width: 12px; height: 12px;
            border-radius: 50%; background-color: red;
            margin-right: 8px; margin-left: 20px;"></span>
        </>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");

    // To ensure reactive updates we wrap the conditional in a closure.
    view! {
        {move || {
            let state = global_state.get();
            log!("UserProfile re-render: global_state changed to {:?}", state);
            if state.is_logged_in {
                view! { <LoggedInUserProfile /> }.into_any()
            } else {
                view! { <LoggedOutUserProfile /> }.into_any()
            }
        }}
    }
}

/// New component to perform logout.
#[component]
pub fn LogoutButton() -> impl IntoView {
    let global_state_set =
        use_context::<WriteSignal<GlobalAppState>>().expect("global_state setter not provided");
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");
    let navigate = use_navigate();

    // Cache backend_url and api_key from global state.
    let state = global_state.get_untracked();
    let backend_url = state.backend_url.clone();
    let api_key = state.api_key.clone();

    let on_logout = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        let backend_url = backend_url.clone();
        let api_key = api_key.clone();
        let global_state_set = global_state_set.clone();
        let navigate = navigate.clone();

        spawn_local(async move {
            let url = format!("{}/auth/logout", backend_url);
            let req = gloo_net::http::Request::post(&url)
                .header("x-api-key", &api_key)
                .body("")
                .unwrap();

            match req.send().await {
                Ok(_) => {
                    if let Some(win) = web_sys::window() {
                        if let Ok(Some(storage)) = win.local_storage() {
                            let _ = storage.clear();
                        }
                    }
                    global_state_set.update(|state| {
                        state.user_id = None;
                        state.email = None;
                        state.is_logged_in = false;
                    });
                    navigate("/account/login", Default::default());
                }
                Err(err) => {
                    log!("Logout failed: {:?}", err);
                }
            }
        });
    };

    view! {
        <a
            href="/account/login"
            on:click=on_logout
            style="text-decoration: none; color: white; border: 1px solid white;
             padding: 4px 8px; border-radius: 4px;"
        >
            <div>"Logout"</div>
        </a>
    }
}
