use leptos::*;


#[component]
pub fn Menu() -> impl IntoView {
    let pages = vec!["dashboard", "containers"];
    view! {
        <nav>
            <ul>
            {
                pages
                    .into_iter()
                    .map(|n| {
                        let link = "/".to_owned()+n;
                        view! {
                            <li> <a href=link  >{n}</a></li>
                        }
                    })
                .collect_view()
            }
            </ul>
        </nav>
    }
}