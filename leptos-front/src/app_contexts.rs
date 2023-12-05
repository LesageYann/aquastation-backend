use gloo_storage::{Storage, SessionStorage};
use leptos::{SignalGet, RwSignal, use_context, create_rw_signal, provide_context};
use leptos::logging::log;
use crate::models::user::User;

pub fn get_user() -> RwSignal<User>{
    match use_context::<RwSignal<User>>() {
        Some(user) => {
            log!("user found  in context: {}", user.get().username);
            user
        },
        None => {
            let user = match SessionStorage::get::<User>("user") {
                Ok(stored_user) => {
                    log!("user found  in storage: {}", stored_user.username);
                    create_rw_signal(stored_user)
                }
                Err(_)=> {
                    log!("none user");
                    create_rw_signal(User::default())
                }
            };
            provide_context(user);
            user
        }
    }

}

