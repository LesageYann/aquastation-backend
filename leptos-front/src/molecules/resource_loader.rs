use leptos::*;
use leptos::logging::log;
use leptos::Resource;
use crate::api::error::AQSError;

#[component]
pub fn ResourceLoader<T, S, F, IV>(

    resource: Resource<S, Result<T, AQSError>>,
    display: F,
) -> impl IntoView
    where
        S: PartialEq + Clone + 'static,
        T: Clone + Serializable + 'static,
        F: Fn(RwSignal<T>) -> IV + 'static,
        IV: IntoView,
{

    view! { {move || {
        match resource.get() {
            None =>  (view! { <div class="loader lds-dual-ring" /> }).into_view(),
            Some(Ok(containers)) => {
                let signal = create_rw_signal(containers);
                display(signal).into_view()
            },
            Some(Err(error)) => {
                match error {
                    AQSError::InvalidData => {
                        (view! { <div> "Invalid data"</div>}).into_view()
                    }
                    AQSError::NetworkError => {
                        (view! { <div> "Network error"</div>}).into_view()
                    }
                }
            }
        }
    }
    } }

}