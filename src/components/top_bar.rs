use gloo_net::http::Request;
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

// This component is rendered when the user is logged in.
// It now includes a logout button to the left of the emoji and
// a clickable emoji that toggles a dropdown menu.
#[component]
pub fn LoggedInUserProfile() -> impl IntoView {
    // Get both the read and write signals from context.
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");
    let set_global_state =
        use_context::<WriteSignal<GlobalAppState>>().expect("global state setter not provided");
    // A navigator hook to programmatically change routes.
    let navigate = use_navigate();
    // Signal to track if the dropdown should be visible.
    let (show_dropdown, set_show_dropdown) = create_signal(false);

    // When logging out:
    // 1) Spawn an async task to call /auth/logout;
    // 2) If successful, reset global state and navigate to the home page.
    let logout = move || {
        spawn_local({
            let set_global_state = set_global_state.clone();
            let navigate = navigate.clone();
            async move {
                let backend_url = global_state.get().backend_url;
                let api_key = global_state.get().api_key;

                let url = format!("{}/auth/logout", backend_url);
                let response = Request::post(&url)
                    .header("x-api-key", api_key.as_ref())
                    .send()
                    .await;
                
                match response {
                    Ok(res) => {
                        if res.ok() {
                            set_global_state.set(GlobalAppState::default());
                            navigate("/", Default::default());
                        } else {
                            log!("Logout failed with status: {:?}", res.status());
                        }
                    }
                    Err(err) => {
                        log!("Logout error: {:?}", err);
                    }
                }
            }
        });
    };

    view! {
        // Wrap the components in a div for positioning.
        <div style="display: flex; align-items: center; position: relative;">
            // The logout button is placed to the left.
            <button
                on:click=move |_| logout()
                style="margin-right: 10px; background: none; border: none; color: white; cursor: pointer;"
            >
                "Logout"
            </button>
            // The profile icon toggles the dropdown when clicked.
            <div on:click=move |_| set_show_dropdown.update(|v| *v = !*v) style="cursor: pointer;">
                <span style="font-size: 24px;">"👤"</span>
            </div>
            // Conditionally render the dropdown menu.
            {move || {
                if show_dropdown.get() {
                    view! {
                        <div style="
                        position: absolute;
                        top: 30px;
                        right: 0;
                        background-color: #444;
                        border: 1px solid #333;
                        border-radius: 4px;
                        padding: 10px;
                        z-index: 1000;
                        ">
                            <ul style="list-style: none; margin: 0; padding: 0;">
                                <li style="padding: 5px 0;">
                                    <a
                                        href="/account/settings"
                                        style="text-decoration: none; color: white;"
                                    >
                                        "Account Settings"
                                    </a>
                                </li>
                                {}
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

// For the logged-out case we keep the original simple buttons.
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

// Conditionally renders one of the above user profile components based on
// whether global_state.is_logged_in is true.
#[component]
pub fn UserProfile() -> impl IntoView {
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");

    // Wrap the rendering in a closure so that it re-renders on state changes.
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
