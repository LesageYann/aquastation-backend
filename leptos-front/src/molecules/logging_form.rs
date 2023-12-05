use gloo_storage::{Storage, SessionStorage};
use leptos::{ev::SubmitEvent, *};
use leptos_router::*;
use leptos::logging::log;

use api_types::dto::{AuthResponse, LogFormDTO};

use crate::app_contexts::get_user;
use crate::atoms::labeled_input::LabeledInput;
use crate::models::user::User;
use crate::api::auth::{logging};

#[component]
pub fn LoggingForm() -> impl IntoView {
    let (email_address, set_email_address) = create_signal("yannlesag@gmail.com".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (loading, set_loading) = create_signal(false);
    let (bad_credential, set_bad_credential) = create_signal(false);

    let set_user = get_user();

    let login_action =
        create_action( move |(email, password): &(String, String)| {
            set_loading.set(true);
            let navigate = use_navigate();
            let form = LogFormDTO {password: password.clone(), email: email.clone()};
            async move {
                let potential_response= logging(&form).await;

                match potential_response {
                    Ok(response) => match response.status() {
                        reqwest::StatusCode::OK => {
                            match response.json::<AuthResponse>().await {
                                Ok(body) => {
                                    log!("auth {}", body.auth.access_token);
                                    let user = User::new(body.user.username, body.user.id, [body.auth.token_type, body.auth.access_token].join(" "));
                                    let _ = SessionStorage::set("user", user.clone());
                                    set_user.set(user);
                                    set_loading.set(false);
                                    let _ = navigate("/dashboard", Default::default());
                                }
                                Err(_e) => {
                                    set_loading.set(false);
                                }
                            }
                        }
                        _ => {
                            set_bad_credential.set(true);
                            set_loading.set(false);
                        }
                    },
                    Err(_e) => {
                        set_loading.set(false);
                    }
                };
            }
        });

    let login_handler = move |ev: SubmitEvent| {
        ev.stop_propagation();
        ev.prevent_default();
        if loading.get() || email_address.get().is_empty() || password.get().is_empty() {
            return;
        }
        login_action.dispatch(  (email_address.get(), password.get()));
    };

    view! {
        <div class="ponics-logging card">
            <h2 class="card-title">"Log in"</h2>
            <form on:submit=login_handler >
                <LabeledInput name="email" input_value=email_address input_return=set_email_address kind="email" />
                <LabeledInput name="Password" input_value=password input_return=set_password kind="password" />
                <Show
                    when=move || { bad_credential.get() }
                    fallback=|| view! { <div style:display="none"/> }
                >
                    <p class="error">"Bad email or password"</p>
                </Show>
                <button type="submit">
                    "Log in"
                </button>
            </form>
        </div>
    }
}