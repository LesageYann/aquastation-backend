use leptos::*;
use leptos_router::*;

use api_types::container::{ContainerWithStatus};

#[component]
pub fn ContainersTable(containers: RwSignal<Vec<ContainerWithStatus>>) -> impl IntoView {
    view! {
         <table>
             <thead>
                 <tr>
                     <th>
                         "name"
                     </th>
                     <th>
                         "volume"
                     </th>
                     <th>
                         "probe status"
                     </th>
                     <th>
                         "actions"
                     </th>
                 </tr>
             </thead>
             <tbody>
                 {containers.get()
                     .into_iter()
                     .map(|entity|{
                         view! {
                         <tr>
                             <td>
                                 {entity.props.name}
                             </td>
                             <td>
                                 {entity.props.volume.to_string()}
                             </td>
                             <td>
                                 {entity.status
                                     .into_iter()
                                     .map(|probe| {
                                         view!{<div>{probe.name}</div>}
                                     }).collect_view()
                                 }
                             </td>
                             <td>
                                <button
                                    on:click=move |_ev| {
                                        let navigate = use_navigate();
                                        let _ = navigate((("/containers/".to_string() + &entity.props.id.to_string()).to_string() + "/new").as_str(), Default::default());
                                    }
                                >
                                        +
                                </button>
                             </td>
                         </tr>
                         }
                     })
                     .collect_view()
                 }
             </tbody>
         </table>
    }
}