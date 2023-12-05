use leptos::*;
use crate::app_contexts::get_user;
use crate::pages::logging::LoggingPage;

#[component]
pub fn ProtectedComponent( children: Children) -> impl IntoView {


    view! {
        <Show
            when=move || {
            let user = get_user();
            user.get().is_default()
        }
            fallback=|| view! { <LoggingPage/> }
        >
            children()
        </Show>
    }
}