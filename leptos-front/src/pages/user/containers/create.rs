use leptos::*;
use enum_iterator::all;
use leptos::ev::SubmitEvent;
use leptos_router::use_navigate;

use api_types::container::ContainerType;
use api_types::dto::CreateContainerFormDTO;
use crate::api::user::create_container;
use crate::app_contexts::get_user;

use crate::molecules::menu::Menu;
use crate::atoms::{labeled_input::LabeledInput, labeled_select::LabeledSelect};

#[component]
pub fn CreateContainerPage() -> impl IntoView {
    let (name, set_name) = create_signal("container name".to_string());
    let (content_type, set_content_type) = create_signal(ContainerType::Other);
    let (volume, set_volume) = create_signal("0".to_string());
    let (loading, set_loading) = create_signal( false);
    let (error, set_error) = create_signal::<Option<String>>(None);

    let button_is_disabled = create_memo(move |_| { loading.get()});

    let create_container_action =
        create_action(move |(name, container_type, volume): &(String, ContainerType, i64)| {
            set_loading.set(true);
            let navigate = use_navigate();
            let form = CreateContainerFormDTO {name: name.clone(), container_type: container_type.clone(), volume: volume.clone()};
            async move {
                let user = get_user();
                let potential_response= create_container(user.get().token, &form).await;

                match potential_response {
                    Ok(_response) => {
                        //TODO save the container to avoid a nex fetch of all containers ?
                        set_loading.set(false);
                        let _ = navigate("/containers", Default::default());
                    },
                    Err(e) => {
                        set_error.set(Some(e.to_string()));
                        set_loading.set(false);
                    }
                };
            }
        });

    let creation_handler = move |ev: SubmitEvent| {
        ev.stop_propagation();
        ev.prevent_default();
        if loading.get() || name.get().is_empty() {
            return;
        }
        create_container_action.dispatch(  (name.get(), content_type.get(), volume.get().parse::<i64>().unwrap()));
    };

    let types_opts = all::<ContainerType>()
        .map(|opt|{
            (opt.clone(), opt.to_string())}).collect::<Vec<(ContainerType,String)>>();

    let (types_options,_) = create_signal(types_opts);

    view! {
        <div class="page" >
            <Menu />
            <div class="card-container">
                <div class="card">
                    <h2 class="card-title">
                        "Create container"
                    </h2>
                    <form on:submit=creation_handler >
                        <LabeledInput name="name" input_value=name input_return=set_name />
                        <LabeledInput name="volume" input_value=volume input_return=set_volume kind="number" />
                        <LabeledSelect name="type" input_value=content_type input_return=set_content_type options={types_options}>
                        </LabeledSelect>
                        <Show
                            when=move || { error.get().is_some() }
                            fallback=|| view! { <div style:display="none"/> }
                        >
                            <p class="error">{error.get().unwrap_or("".to_string())}</p>
                        </Show>
                        <button
                            prop:disabled=move || button_is_disabled.get()
                            type="submit"
                        >
                            "Create"
                        </button>
                    </form>
                </div>
            </div>
        </div>
    }
}