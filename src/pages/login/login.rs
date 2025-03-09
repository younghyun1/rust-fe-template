use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use uuid::Uuid;
use web_sys::wasm_bindgen::JsCast;

use crate::{dto::api_response::ResponseFormat, GlobalAppState};

/// Request sent to the backend for login.
#[derive(serde_derive::Deserialize, serde_derive::Serialize, Clone, Debug)]
pub struct LoginRequest {
    pub user_email: String,
    pub user_password: String,
}

/// Expected login response from the backend.
#[derive(Clone, serde_derive::Deserialize, Debug)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: Uuid,
}

#[component]
pub fn Login() -> impl IntoView {
    // Build up the login form’s state.
    let (login_state, set_login_state) = signal(LoginRequest {
        user_email: String::new(),
        user_password: String::new(),
    });

    // Retrieve a readable and a writeable global state.
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");
    let global_state_set =
        use_context::<WriteSignal<GlobalAppState>>().expect("global_state not provided");

    // Get backend connection details from global state.
    let state_value = global_state.get_untracked();
    let backend_url = std::rc::Rc::new(state_value.backend_url.clone());
    let api_key = std::rc::Rc::new(state_value.api_key.clone());

    let navigate = use_navigate();

    // Update email in state.
    let on_email_input = {
        let set_login_state = set_login_state.clone();
        move |ev: web_sys::Event| {
            if let Some(input) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                let email = input.value();
                set_login_state.update(|state| state.user_email = email);
            }
        }
    };

    // Update password in state.
    let on_password_input = {
        let set_login_state = set_login_state.clone();
        move |ev: web_sys::Event| {
            if let Some(input) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                let pwd = input.value();
                set_login_state.update(|state| state.user_password = pwd);
            }
        }
    };

    // Handle form submission.
    let on_submit = {
        // Clone the necessary signals and values to allow the closure to be called multiple times.
        let login_state = login_state.clone();
        let backend_url = backend_url.clone();
        let api_key = api_key.clone();
        let global_state_set = global_state_set.clone();
        let navigate = navigate.clone();
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            let login_data = login_state.get();
            // Clone variables inside the closure body so they can be used on each submit without moving out.
            let backend_url = backend_url.clone();
            let api_key = api_key.clone();
            let global_state_set = global_state_set.clone();
            let navigate = navigate.clone();
            spawn_local(async move {
                let url = format!("{}/auth/login", backend_url.as_ref());
                let req = gloo_net::http::Request::post(&url)
                    .header("x-api-key", api_key.as_ref())
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&login_data).unwrap())
                    .unwrap();

                match req.send().await {
                    Ok(response) => match response.json::<ResponseFormat<LoginResponse>>().await {
                        Ok(resp) => {
                            if resp.success {
                                log!("Login successful: {:?}", resp.data);
                                // Update global state with the received user_id.
                                global_state_set
                                    .update(|state| state.user_id = Some(resp.data.user_id));
                                // Navigate to the home page (or dashboard) after login.
                                navigate("/", Default::default());
                            } else {
                                log!("Login failed at backend: {:?}", resp);
                            }
                        }
                        Err(err) => log!("Error parsing login response JSON: {:?}", err),
                    },
                    Err(err) => log!("Error sending login request: {:?}", err),
                }
            });
        }
    };

    view! {
        <>
            <style>{include_str!("./login.css")}</style>
            <div class="container">
                <div class="login-form">
                    <h2>"Log In"</h2>
                    <form on:submit=on_submit>
                        <div>
                            <label for="user_email">"Email*:"</label>
                            <input
                                id="user_email"
                                type="email"
                                placeholder="Your Email"
                                on:input=on_email_input
                            />
                        </div>
                        <div>
                            <label for="user_password">"Password*:"</label>
                            <input
                                id="user_password"
                                type="password"
                                placeholder="Your Password"
                                on:input=on_password_input
                            />
                        </div>
                        <button type="submit">"Log In"</button>
                    </form>
                </div>
            </div>
        </>
    }
}
