use leptos::{logging::log, prelude::*, task::spawn_local};

use crate::{dto::api_response::ResponseFormat, GlobalAppState};

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

#[component]
pub fn Signup() -> impl IntoView {
    let (countries, set_countries) = signal(Vec::<IsoCountry>::new());

    // Grab global state and its setter from context.
    let global_state =
        use_context::<ReadSignal<GlobalAppState>>().expect("global_state not provided");

    let backend_url: String = global_state.read().backend_url.clone();
    let api_key: String = global_state.read().api_key.clone();

    spawn_local({
        let set_countries = set_countries.clone();
        async move {
            let url = format!("{}/dropdown/country/get-all", backend_url);
            match gloo_net::http::Request::get(&url)
                .header("x-api-key", &api_key)
                .send()
                .await
            {
                Ok(response) => {
                    match response.json::<ResponseFormat<CountryData>>().await {
                        Ok(countries_resp) => {
                            if countries_resp.success {
                                log!(
                                    "Fetched {} countries successfully. Meta: {:?}",
                                    countries_resp.data.countries.len(),
                                    countries_resp.meta
                                );
                                // Now update the reactive state!
                                set_countries.set(countries_resp.data.countries);
                            } else {
                                log!(
                                    "Server indicated failure fetching countries: {:?}",
                                    countries_resp
                                );
                            }
                        }
                        Err(err) => {
                            log!("Error parsing countries JSON response: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    log!("Error sending request: {:?}", err);
                }
            }
        }
    });

    view! {
        <>
            <style>
                {r#"
                /* The outer container fills the available viewport space minus the top bar */
                .container {
                  display: flex;
                  align-items: center;
                  justify-content: center;
                  height: calc(100vh - 90px); /* subtract the top bar height */
                  margin: 0;
                  padding: 0;
                  overflow: hidden;
                  background: #000; /* dark background for neon effect */
                }
                
                /* The signup box: transparent background with a thick neon green border */
                .signup-form {
                  border: 5px solid #39FF14; /* neon green border */
                  background: transparent;
                  border-radius: 8px;
                  padding: 24px;
                  max-width: 400px;
                  width: 100%;
                  box-sizing: border-box;
                  text-align: center;
                }
                
                /* Consistent neon green styling for text elements */
                .signup-form h2,
                .signup-form label,
                .signup-form input,
                .signup-form select,
                .signup-form button {
                  font-family: Arial, sans-serif;
                  color: #39FF14; /* neon green text */
                }
                
                .signup-form h2 {
                  font-size: 2rem;
                  margin-bottom: 16px;
                }
                
                .signup-form form {
                  display: flex;
                  flex-direction: column;
                  gap: 16px;
                }
                
                .signup-form form div {
                  display: flex;
                  flex-direction: column;
                  align-items: flex-start;
                }
                
                .signup-form input,
                .signup-form select {
                  padding: 8px;
                  border: 2px solid #39FF14;
                  border-radius: 4px;
                  background: transparent;
                  width: 100%;
                  box-sizing: border-box;
                  color: #39FF14;
                }
                
                .signup-form input::placeholder,
                .signup-form select::placeholder {
                  color: #39FF14;
                }
                
                .signup-form button {
                  padding: 10px;
                  border: 2px solid #39FF14;
                  background: transparent;
                  border-radius: 4px;
                  cursor: pointer;
                  font-weight: bold;
                  transition: background 0.3s ease, color 0.3s ease;
                }
                
                .signup-form button:hover {
                  background: #39FF14;
                  color: #000;
                }
                "#}
            </style>
            <div class="container">
                <div class="signup-form">
                    <h2>"Sign Up"</h2>
                    <form>
                        <div>
                            <label for="user_name">"Name:"</label>
                            <input id="user_name" type="text" placeholder="Your Name" />
                        </div>
                        <div>
                            <label for="user_email">"Email:"</label>
                            <input id="user_email" type="email" placeholder="Your Email" />
                        </div>
                        <div>
                            <label for="user_password">"Password:"</label>
                            <input id="user_password" type="password" placeholder="Your Password" />
                        </div>
                        <div>
                            <label for="user_country">"Country:"</label>
                            <select id="user_country">
                                <option value="">"Select Country"</option>
                                {move || {
                                    countries
                                        .get()
                                        .into_iter()
                                        .map(|country| {
                                            // Use the current list of countries to generate options.
                                            // Note: Make sure that IsoCountry implements a trait to get a proper display value.
                                            view! {
                                                <option value=country
                                                    .country_alpha2
                                                    .clone()>
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
                            <label for="user_language">"Language:"</label>
                            <select id="user_language">
                                <option value="">"Select Language (placeholder)"</option>
                            </select>
                        </div>
                        <div>
                            <label for="user_subdivision">"Subdivision:"</label>
                            <select id="user_subdivision">
                                <option value="">
                                    "Select Subdivision (optional placeholder)"
                                </option>
                            </select>
                        </div>
                        <button type="submit">"Sign Up"</button>
                    </form>
                </div>
            </div>
        </>
    }
}
