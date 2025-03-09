use leptos::prelude::*;

use crate::GlobalAppState;

#[component]
pub fn TopBar() -> impl IntoView {
    // Grab global state and its setter from context.
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");
    let _set_global_state =
        use_context::<WriteSignal<GlobalAppState>>().expect("global_state setter not provided");

    // The header will be fixed to the top with full width and no margin.
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
                    {if global_state.read().is_logged_in {
                        view! { <LoggedInUserProfile /> }.into_any()
                    } else {
                        view! { <LoggedOutUserProfile /> }.into_any()
                    }}
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
    view! {
        <>
            <a href="/account/settings" style="text-decoration: none; color: white;">
                <span style="font-size: 24px;">"👤"</span>
            </a>
            <span style="display: inline-block; width: 12px; height: 12px;
             border-radius: 50%; background-color: green;
             margin-right: 8px; margin-left: 20px;"></span>
        </>
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
