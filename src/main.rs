use chrono::{DateTime, Utc};
use leptos::{prelude::*, task::spawn_local};
use std::time::Instant;

fn main() {
    leptos::mount::mount_to_body(AppShell)
}

#[derive(Clone, serde_derive::Deserialize)]
pub struct RootHandlerResponse {
    timestamp: DateTime<Utc>,
    server_uptime: String, // TODO: ISO-compliance
    responses_handled: u64,
    db_version: String,
    db_latency: String,
}

#[derive(Clone, serde_derive::Deserialize)]
pub struct Meta {
    time_to_process: String,
    timestamp: DateTime<Utc>,
    metadata: Option<String>,
}

#[derive(Clone, serde_derive::Deserialize)]
pub struct Response {
    success: bool,
    data: RootHandlerResponse,
    meta: Meta,
}

#[derive(Clone)]
struct AppState {
    pub logged_in: bool,
    pub request_client: reqwest::Client,
}

impl AppState {
    pub fn get_client(&self) -> &reqwest::Client {
        &self.request_client
    }
}

#[component]
fn AppShell() -> impl IntoView {
    let (state, _set_state) = signal(AppState {
        logged_in: false,
        request_client: reqwest::Client::new(),
    });
    let (response, set_response) = signal::<Option<Response>>(None);
    let on_click = move |_| {
        let start_time = web_time::Instant::now();
        spawn_local(async move {
            let state = state.get();
            let client = state.get_client();
            match client.get("http://localhost:3000").send().await {
                Ok(resp) => match resp.json::<Response>().await {
                    Ok(data) => set_response.set(Some(data)),
                    Err(_err) => set_response.set(None),
                },
                Err(_) => set_response.set(None),
            }
            let elapsed = start_time.elapsed();
            leptos::logging::log!("Fetch took: {:?}", elapsed);
        });
    };

    view! {
        <div style="background:black; color:white;">
            <TitleBar />
            <div>
                <button on:click=on_click>"Fetch Data"</button>
            </div>
            <div>{
                move || {
                    if let Some(ref response) = response.get() {
                        let data = &response.data;
                        format!(
                            "Timestamp: {}\nServer Uptime: {}\nResponses Handled: {}\nDB Version: {}\nDB Latency: {}",
                            data.timestamp, data.server_uptime, data.responses_handled, data.db_version, data.db_latency
                        )
                    } else {
                        "Error fetching data or parsing response.".to_string()
                    }
                }
            }</div>
        </div>
    }
}

#[component]
fn TitleBar() -> impl IntoView {
    view! { <div style="font-size: 24px; font-weight: bold;">"Rust Full-Stack Test"</div> }
}
