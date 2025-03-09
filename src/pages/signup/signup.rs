use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use urlencoding::encode;
use web_sys::wasm_bindgen::JsCast;

use crate::{dto::api_response::ResponseFormat, GlobalAppState};

/// The signup form style extracted as a constant for improved readability.
pub const SIGNUP_STYLE: &str = include_str!("./signup.css");

/// Country and subdivision types
#[derive(Clone, Debug, serde::Deserialize)]
struct CountryData {
    countries: Vec<IsoCountry>,
}

#[derive(Clone, serde_derive::Deserialize, Debug)]
pub struct IsoCountry {
    pub country_code: i32,
    pub country_alpha2: String,
    pub country_alpha3: String,
    pub country_eng_name: String,
    pub country_currency: i32,
    pub phone_prefix: String,
    pub country_flag: String,
    pub is_country: bool,
    pub country_primary_language: i32,
}

#[derive(Clone, serde_derive::Deserialize, Debug)]
pub struct Subdivision {
    pub subdivision_id: i32,
    pub country_code: i32,
    pub subdivision_code: String,
    pub subdivision_name: String,
    pub subdivision_type: Option<String>,
}

/// ISO Languages: This is the reply by the backend.
#[derive(Clone, serde_derive::Deserialize, Debug)]
pub struct IsoLanguage {
    pub language_code: i32,
    pub language_alpha2: String,
    pub language_alpha3: String,
    pub language_eng_name: String,
}

/// The request struct that you will send off to the backend.
#[derive(serde_derive::Deserialize, serde_derive::Serialize, Clone, Debug, Default)]
pub struct SignupRequest {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_country: i32,
    pub user_language: i32,
    pub user_subdivision: Option<i32>,
}

/// Struct representing the response from the signup endpoint.
#[derive(Clone, serde_derive::Deserialize, Debug)]
pub struct SignupResponse {
    pub user_name: String,
    pub user_email: String,
    pub verify_by: String,
}

#[component]
pub fn Signup() -> impl IntoView {
    // Signals for dropdown data.
    let (countries, set_countries) = signal(Vec::<IsoCountry>::new());
    let (subdivisions, set_subdivisions) = signal(Vec::<Subdivision>::new());
    let (languages, set_languages) = signal(Vec::<IsoLanguage>::new());

    // Track the currently selected country (unused, so prefixed with an underscore to silence warnings).
    let (_selected_country, set_selected_country) = signal(String::new());
    // The main object that builds up the signup request.
    let (request_state, set_request_state) = signal(SignupRequest::default());

    // Grab global state and its setter from context.
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");

    let navigate = use_navigate();

    // Wrap backend_url and api_key in Rc so they can be cloned into multiple closures.
    let backend_url = std::rc::Rc::new(global_state.get().backend_url.clone());
    let api_key = std::rc::Rc::new(global_state.get().api_key.clone());

    // Initially fetch the list of countries.
    spawn_local({
        let set_countries = set_countries.clone();
        let backend_url = backend_url.clone();
        let api_key = api_key.clone();
        async move {
            let url = format!("{}/dropdown/country/get-all", backend_url.as_ref());
            match gloo_net::http::Request::get(&url)
                .header("x-api-key", api_key.as_ref())
                .send()
                .await
            {
                Ok(response) => match response.json::<ResponseFormat<CountryData>>().await {
                    Ok(countries_resp) => {
                        if countries_resp.success {
                            log!(
                                "Fetched {} countries successfully. Meta: {:?}",
                                countries_resp.data.countries.len(),
                                countries_resp.meta
                            );
                            set_countries.set(countries_resp.data.countries);
                        } else {
                            log!("Server failure fetching countries: {:?}", countries_resp);
                        }
                    }
                    Err(err) => log!("Error parsing countries JSON response: {:?}", err),
                },
                Err(err) => log!("Error sending request: {:?}", err),
            }
        }
    });

    // Initially fetch the list of languages.
    spawn_local({
        let set_languages = set_languages.clone();
        let backend_url = backend_url.clone();
        let api_key = api_key.clone();
        async move {
            let url = format!("{}/dropdown/language/get-all", backend_url.as_ref());
            match gloo_net::http::Request::get(&url)
                .header("x-api-key", api_key.as_ref())
                .send()
                .await
            {
                Ok(response) => match response.json::<ResponseFormat<Vec<IsoLanguage>>>().await {
                    Ok(languages_resp) => {
                        if languages_resp.success {
                            log!(
                                "Fetched {} languages successfully. Meta: {:?}",
                                languages_resp.data.len(),
                                languages_resp.meta
                            );
                            set_languages.set(languages_resp.data);
                        } else {
                            log!("Server failure fetching languages: {:?}", languages_resp);
                        }
                    }
                    Err(err) => log!("Error parsing languages JSON response: {:?}", err),
                },
                Err(err) => log!("Error sending languages request: {:?}", err),
            }
        }
    });

    let on_subdivision_change = {
        let set_request_state = set_request_state.clone();
        move |ev: web_sys::Event| {
            if let Some(target) = ev.target() {
                let input: web_sys::HtmlSelectElement = target.unchecked_into();
                let subdivision_val = input.value();
                let subdivision_code = subdivision_val.parse::<i32>().ok();
                set_request_state.update(|state| state.user_subdivision = subdivision_code);
            }
        }
    };

    let on_language_change = {
        let set_request_state = set_request_state.clone();
        move |ev: web_sys::Event| {
            if let Some(target) = ev.target() {
                let input: web_sys::HtmlSelectElement = target.unchecked_into();
                let lang_val = input.value();
                let language_code = lang_val.parse::<i32>().unwrap_or(0);
                set_request_state.update(|state| state.user_language = language_code);
            }
        }
    };

    let on_name_input = {
        let set_request_state = set_request_state.clone();
        move |ev: web_sys::Event| {
            if let Some(input) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                let name = input.value();
                set_request_state.update(|state| state.user_name = name);
            }
        }
    };

    let on_email_input = {
        let set_request_state = set_request_state.clone();
        move |ev: web_sys::Event| {
            if let Some(input) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                let email = input.value();
                set_request_state.update(|state| state.user_email = email);
            }
        }
    };

    let on_password_input = {
        let set_request_state = set_request_state.clone();
        move |ev: web_sys::Event| {
            if let Some(input) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            {
                let pwd = input.value();
                set_request_state.update(|state| state.user_password = pwd);
            }
        }
    };

    let backend_url_for_country_change = backend_url.clone();
    let api_key_for_country_change = api_key.clone();
    // Define the on_country_change handler
    let on_country_change = {
        let set_selected_country = set_selected_country.clone();
        let set_request_state = set_request_state.clone();
        let set_subdivisions = set_subdivisions.clone();
        move |ev: web_sys::Event| {
            let backend_url_for_country_change = backend_url_for_country_change.clone();
            let api_key_for_country_change = api_key_for_country_change.clone();

            if let Some(target) = ev.target() {
                let input: web_sys::HtmlSelectElement = target.unchecked_into();
                let country_val = input.value();

                set_selected_country.set(country_val.clone());
                let country_code = country_val.parse::<i32>().unwrap_or(0);
                set_request_state.update(|state| {
                    state.user_country = country_code;
                });
                set_request_state.update(|state| {
                    state.user_subdivision = None;
                });

                spawn_local(async move {
                    let url = format!(
                        "{}/dropdown/country/subdivision?country_id={}",
                        backend_url_for_country_change, country_val
                    );
                    match gloo_net::http::Request::get(&url)
                        .header("x-api-key", &api_key_for_country_change)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            match response.json::<ResponseFormat<Vec<Subdivision>>>().await {
                                Ok(sub_div_resp) => {
                                    if sub_div_resp.success {
                                        log!(
                                            "Fetched {} subdivisions successfully. Meta: {:?}",
                                            sub_div_resp.data.len(),
                                            sub_div_resp.meta
                                        );
                                        set_subdivisions.set(sub_div_resp.data);
                                    } else {
                                        log!(
                                            "Server failure fetching subdivisions: {:?}",
                                            sub_div_resp
                                        );
                                    }
                                }
                                Err(err) => {
                                    log!("Error parsing subdivisions JSON response: {:?}", err)
                                }
                            }
                        }
                        Err(err) => log!("Error sending subdivisions request: {:?}", err),
                    }
                });
            }
        }
    };

    // Define the on_submit handler
    let backend_url_for_on_submit = backend_url.clone();
    let api_key_for_on_submit = api_key.clone();
    let navigate_for_on_submit = navigate.clone();

    let on_submit = {
        let request_state = request_state.clone();
        move |ev: leptos::ev::SubmitEvent| {
            let backend_url_for_on_submit = backend_url_for_on_submit.clone();
            let api_key_for_on_submit = api_key_for_on_submit.clone();
            let navigate_for_on_submit = navigate_for_on_submit.clone();

            ev.prevent_default();
            let request_state = request_state.get();
            spawn_local(async move {
                let url = format!("{}/auth/signup", backend_url_for_on_submit);
                let req = gloo_net::http::Request::post(&url)
                    .header("x-api-key", &api_key_for_on_submit)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&request_state).unwrap())
                    .unwrap();
                match req.send().await {
                    Ok(response) => match response.json::<ResponseFormat<SignupResponse>>().await {
                        Ok(resp) => {
                            if resp.success {
                                log!("Signup Response: {:?}", resp.data);

                                // Extract the values to pass as query params:
                                let user_name = &resp.data.user_name;
                                let user_email = &resp.data.user_email;
                                let expiry_time = &resp.data.verify_by;

                                // Build the query string, encoding values in case they contain characters
                                let query_params = format!(
                                    "?user_name={}&user_email={}&expiry_time={}",
                                    encode(user_name),
                                    encode(user_email),
                                    encode(expiry_time)
                                );

                                // Navigate to the signup-complete route with query parameters.
                                navigate_for_on_submit(
                                    &format!("/account/signup-complete{}", query_params),
                                    Default::default(),
                                );
                            } else {
                                log!("Signup failed at backend: {:?}", resp);
                            }
                        }
                        Err(err) => log!("Error parsing signup response JSON: {:?}", err),
                    },
                    Err(err) => log!("Error sending signup request: {:?}", err),
                }
            });
        }
    };

    view! {
        <>
            <style>{SIGNUP_STYLE}</style>
            <div class="container">
                <div class="signup-form">
                    <h2>"Sign Up"</h2>
                    <form on:submit=on_submit>
                        <div>
                            <label for="user_name">"Username*:"</label>
                            <input
                                id="user_name"
                                type="text"
                                placeholder="Your Name"
                                on:input=on_name_input
                            />
                        </div>
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
                        <div>
                            <label for="user_country">"Country*:"</label>
                            <select id="user_country" on:change=on_country_change>
                                <option value="">"Select Country"</option>
                                {move || {
                                    countries
                                        .get()
                                        .into_iter()
                                        .map(|country| {
                                            view! {
                                                <option value=country
                                                    .country_code
                                                    .to_string()>
                                                    {format!(
                                                        "{} {}",
                                                        country.country_flag,
                                                        country.country_eng_name,
                                                    )}
                                                </option>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </select>
                        </div>
                        <div>
                            <label for="user_subdivision">"Subdivision:"</label>
                            <select id="user_subdivision" on:change=on_subdivision_change>
                                <option value="">"Select Subdivision"</option>
                                {move || {
                                    subdivisions
                                        .get()
                                        .into_iter()
                                        .map(|subdivision| {
                                            view! {
                                                <option value=subdivision
                                                    .subdivision_id
                                                    .to_string()>{subdivision.subdivision_name.clone()}</option>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </select>
                        </div>
                        <div>
                            <label for="user_language">"Language*:"</label>
                            <select id="user_language" on:change=on_language_change>
                                <option value="">"Select Language"</option>
                                {move || {
                                    languages
                                        .get()
                                        .into_iter()
                                        .map(|lang| {
                                            view! {
                                                <option value=lang
                                                    .language_code
                                                    .to_string()>{lang.language_eng_name.clone()}</option>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </select>
                        </div>
                        <button type="submit">"Sign Up"</button>
                    </form>
                </div>
            </div>
        </>
    }
}
