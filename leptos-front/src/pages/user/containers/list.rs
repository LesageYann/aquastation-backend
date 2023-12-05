use leptos::*;
use leptos_router::*;
use leptos::logging::log;

use api_types::container::{Container, ContainerWithStatus};

use crate::api::user::fetch_user_container;
use crate::app_contexts::get_user;
use crate::molecules::menu::Menu;
use crate::molecules::containers_table::ContainersTable;
use crate::molecules::resource_loader::ResourceLoader;



#[component]
pub fn ContainersPage() -> impl IntoView {
    let user = get_user();

    let navigate = use_navigate();

    let resource = create_resource(move || user.get().token, |bearer: String| async { fetch_user_container(bearer).await});

    view! {
        <div class="page" >
            <Menu />
            <div class="card-container">
                <div class="card">
                    <h2 class="card-title">
                        "Containers status"
                    </h2>
                    <div class="card-content">
                        <ResourceLoader
                            resource={resource}
                            display={|signal: RwSignal<Vec<ContainerWithStatus>>| {
            log!("display rendering");
            view! { <ContainersTable containers={signal}/> }
        }}
                        />
                    </div>
                    <div class="actions">
                        <button
                            on:click=move |_ev| {
                                let _ = navigate("/containers/new", Default::default());
                            }
                        >
                            "New container"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}


