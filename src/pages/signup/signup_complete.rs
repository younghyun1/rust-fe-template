use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::pages::signup::signup::SIGNUP_STYLE;

#[component]
pub fn SignupComplete() -> impl IntoView {
    let query = use_query_map();
    let query_map = query.get();

    view! {
        <style>{SIGNUP_STYLE}</style>
        <div class="container">
            <div class="signup-form">
                <h2>"Signup Complete"</h2>
                <p>
                    {
                        let user_name = query_map.get("user_name").unwrap_or("Unknown".to_owned());
                        let user_email = query_map
                            .get("user_email")
                            .unwrap_or("Not provided".to_owned());
                        format!("Thank you, {} ({}).", user_name, user_email)
                    }
                </p>
                <p>{"Please verify your email by"}</p>
                <p>
                    {
                        let expiry_time = query_map.get("expiry_time").unwrap_or("N/A".to_owned());
                        use chrono::{DateTime, Local};
                        let formatted_time = DateTime::parse_from_rfc3339(&expiry_time)
                            .map(|dt| {
                                dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
                            })
                            .unwrap_or(expiry_time);
                        format!("{}", formatted_time)
                    }
                </p>
            </div>
        </div>
    }
}
