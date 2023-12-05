use leptos::*;

#[component]
pub fn LabeledSelect<T: 'static>(
    name: &'static str,
    input_value: ReadSignal<T>,
    input_return: WriteSignal<T>,
    options: ReadSignal<Vec<(T, String)>>
) -> impl IntoView where
    T: 'static + Clone + PartialEq
{

    fn map_options<T: 'static + Clone + PartialEq>( opts: ReadSignal<Vec<(T, String)>>, selected_value: &T) -> impl IntoView{
        opts
            .get()
            .iter()
            .map(|(value, text)|{
                let selected = selected_value == value;
                    (view! { <option selected={selected}>{text}</option>}).into_view()
            })
            .collect_view()
    }

    view! {
        <label>
            <span>{name}</span>
            <select
            on:input=move |ev| {
                let target_value = event_target_value(&ev);
                match options.get().iter().find(|(_,printed)| printed == &target_value) {
                    Some((value,_)) =>{input_return.set(value.clone());}
                    None =>{}
                }
            }
            >
                {map_options(options, &input_value.get())}
            </ select>
        </label>
    }
}