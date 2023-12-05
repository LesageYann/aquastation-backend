use leptos::*;

use crate::molecules::logging_form::LoggingForm;

#[component]
pub fn LoggingPage() -> impl IntoView {
    view! {
        <div class="log-page">
            <LoggingForm />
        </div>
    }
}


