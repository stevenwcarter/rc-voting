use leptos::*;
use leptos_router::*;

#[component]
pub fn Elections() -> impl IntoView {
    view! {
        <h1>"Elections"</h1>
        <Outlet/>
    }
}

#[derive(Params, PartialEq)]
struct ElectionItemParams {
    election_uuid: String,
}

#[component]
pub fn ElectionItem() -> impl IntoView {
    let params = use_params::<ElectionItemParams>();
    let election_uuid = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.election_uuid.clone())
                .unwrap_or_default()
        })
    };
    view! { <h1>"Election item " {election_uuid}</h1> }
}

#[component]
pub fn NoElectionItem() -> impl IntoView {
    view! { <p>"Create an election"</p> }
}
