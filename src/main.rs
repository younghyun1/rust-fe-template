use components::top_bar::TopBar;
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use pages::about::about::About;
use pages::home::home::Home;
use pages::login::login::Login;
use pages::signup::signup::Signup;
use pages::signup::signup_complete::SignupComplete;
use pages::works::works::Works;
use uuid::Uuid;

pub mod components {
    pub mod top_bar;
}
pub mod pages {
    pub mod about {
        pub mod about;
    }
    pub mod home {
        pub mod home;
    }
    pub mod login {
        pub mod login;
    }
    pub mod signup {
        pub mod signup;
        pub mod signup_complete;
    }
    pub mod works {
        pub mod works;
    }
}
pub mod dto {
    pub mod api_response;
}

// Derive Serialize/Deserialize so we can persist this state.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GlobalAppState {
    pub is_logged_in: bool,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub email: Option<String>,
    pub backend_url: String,
    pub api_key: String,
}

impl Default for GlobalAppState {
    fn default() -> Self {
        Self {
            is_logged_in: false,
            user_id: None,
            session_id: None,
            email: None,
            backend_url: String::from("http://cyhdev.com/api"),
            api_key: String::from("5f706c3c-5651-4d76-94a7-b999067b66aa"),
        }
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    // Try to read persisted state from localStorage. If unavailable, use the default.
    let stored_state =
        LocalStorage::get("global_state").unwrap_or_else(|_| GlobalAppState::default());
    let (global_state, set_global_state) = signal(stored_state);

    // Provide the global state and its setter to the rest of your app.
    provide_context(global_state);
    provide_context(set_global_state);

    // Whenever the state changes, persist it automatically.
    create_effect({
        let global_state = global_state.clone();
        move |_| {
            let _ = LocalStorage::set("global_state", &global_state.get());
        }
    });

    view! {
        <style>
            {r#"
              /* Ensure the body has no margin and add padding-top so that content isn’t hidden behind the fixed top bar */
              body {
                  background-color: black;
                  color: white;
                  margin: 0;
                  padding-top: 90px;
                  font-family: sans-serif;
              }
              "#}
        </style>

        <Router>
            <TopBar />

            <Routes fallback=|| {
                view! {
                    <div>
                        <h1>"404 - Page Not Found"</h1>
                        <p>"We're sorry, but the page you were looking for doesn't exist."</p>
                        <a href=path!("")>"Return Home"</a>
                    </div>
                }
            }>
                <Route path=path!("") view=Home />
                <Route path=path!("about") view=About />
                <Route path=path!("works") view=Works />
                <Route path=path!("/account/signup") view=Signup />
                <Route path=path!("/account/signup-complete") view=SignupComplete />
                <Route path=path!("/account/login") view=Login />
            </Routes>
        </Router>
    }
}
