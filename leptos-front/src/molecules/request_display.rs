use leptos::*;
use crate::api::AQSRequest;
use crate::api::error::AQSError;

#[component]
pub fn RequestDisplayer<T: 'static, F, IV>(content: RwSignal<AQSRequest<T>>, display: F)  -> impl IntoView
    where
        T: Clone,
        F: Fn(RwSignal<T>) -> IV,
        IV: IntoView,
{
    match content.get() {
        Err(AQSError::InvalidData) => {
            (view! { <div> "Invalid data"</div>}).into_view()
        }
        Err(AQSError::NetworkError) => {
            (view! { <div> "Network error"</div>}).into_view()
        }
        Ok(data) => {
            let signal = create_rw_signal(data);
            display(signal).into_view()
        }
    }
}