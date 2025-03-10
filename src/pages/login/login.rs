use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use uuid::Uuid;
use web_sys::{wasm_bindgen::JsCast, RequestCredentials};

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

    // Retrieve a readable and writeable global state from context.
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");
    let global_state_set =
        use_context::<WriteSignal<GlobalAppState>>().expect("global_state setter not provided");

    // Get backend connection details from global state.
    let state_value = global_state.get_untracked();
    let backend_url = std::rc::Rc::new(state_value.backend_url.clone());
    let api_key = std::rc::Rc::new(state_value.api_key.clone());

    let navigate = use_navigate();

    // Update email in the login state.
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

    let (pw_error, set_pw_error) = signal(false);

    // Clear error state on input.
    let on_password_input = {
        let set_login_state = set_login_state.clone();
        let set_pw_error = set_pw_error.clone();
        move |ev: web_sys::Event| {
            if let Some(input) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                let pwd = input.value();
                set_login_state.update(|state| state.user_password = pwd);
                set_pw_error.set(false);
            }
        }
    };

    let on_submit = {
        let login_state = login_state.clone();
        let backend_url = backend_url.clone();
        let api_key = api_key.clone();
        let global_state_set = global_state_set.clone();
        let navigate = navigate.clone();
        let set_pw_error = set_pw_error.clone();
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            let login_data = login_state.get();
            let backend_url = backend_url.clone();
            let api_key = api_key.clone();
            let global_state_set = global_state_set.clone();
            let navigate = navigate.clone();
            let set_pw_error = set_pw_error.clone();
            spawn_local(async move {
                let url = format!("{}/auth/login", backend_url.as_ref());
                let req = gloo_net::http::Request::post(&url)
                    .credentials(RequestCredentials::Include)
                    .header("x-api-key", api_key.as_ref())
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&login_data).unwrap())
                    .unwrap();

                match req.send().await {
                    Ok(response) => match response.json::<ResponseFormat<LoginResponse>>().await {
                        Ok(resp) => {
                            if resp.success {
                                global_state_set.update(|state| {
                                    state.user_id = Some(resp.data.unwrap().user_id);
                                    state.email = Some(login_data.user_email.clone());
                                    state.is_logged_in = true;
                                });
                                log!("Login successful: {:?}", global_state.get());
                                navigate("/", Default::default());
                            } else {
                                log!("Backend returned error: {:?}", resp);
                                if let Some(code) = resp.error_code {
                                    log!("Received error code: {}", code);
                                    if code == 15 {
                                        set_pw_error.set(true);
                                        log!("Set pw_error = true");
                                    }
                                }
                                log!("Login failed: {:?}", resp);
                            }
                        }
                        Err(err) => log!("Error parsing JSON: {:?}", err),
                    },
                    Err(err) => log!("Error sending request: {:?}", err),
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
                                class:input-error=move || pw_error.get()
                            />
                        </div>
                        <button type="submit">"Log In"</button>
                    </form>
                </div>
            </div>
        </>
    }
}
