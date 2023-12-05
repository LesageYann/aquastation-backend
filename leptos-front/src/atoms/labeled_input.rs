use leptos::*;

#[component]
pub fn LabeledInput(
    name: &'static str,
    input_value: ReadSignal<String>,
    input_return: WriteSignal<String>,
    #[prop(default = "text")]
    kind: &'static str,
) -> impl IntoView {
    let class_vec = move || if kind == "select" {
        "selector"
    } else if input_value.get().is_empty() {
        ""
    } else {
        "filled"
    };

    view! {
        <label class=class_vec>
            <span>{name}</span>
            <input
            type=kind
            on:input=move |ev| {input_return.set(event_target_value(&ev));}
            prop:value=input_value.get() />
        </label>
    }
}