use leptos::*;
use leptos_icons::*;
use icondata as i;
use crate::app::login::UnAuthRedirect;
use crate::error_template::ErrorTemplate;
use crate::models::Election;
use leptos_router::*;
use super::items::ItemList;

#[component]
pub fn Elections() -> impl IntoView {
    view! {
        <UnAuthRedirect/>
        <div class="flex md:min-h-screen flex-col md:flex-row bg-gradient-to-br from-slate-400 to-slate-200 dark:from-slate-900 dark:to-slate-700">
            <div class="flex flex-col w-full md:w-1/5 from-slate-400 dark:from-slate-900 to-slate-200 dark:to-slate-800 md:min-h-screen p-6 bg-gradient-to-tr">
                <h1 class="text-veniceblue-700 text-3xl dark:text-veniceblue-200">
                    "Your Elections"
                </h1>
                <ElectionList/>
            </div>
            <div class="flex flex-col md:w-4/5 w-full bg-gradient-to-br from-slate-400 to-slate-200 dark:from-slate-900 dark:to-slate-700">
                <Outlet/>
            </div>
        </div>
    }
}

#[derive(Params, PartialEq)]
pub struct ElectionItemParams {
    pub election_uuid: String,
}

#[component]
pub fn ElectionItem() -> impl IntoView {
    // let load_election_action = create_server_action::<LoadElection>();
    let params = use_params::<ElectionItemParams>();
    let election_uuid = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.election_uuid.clone())
                .unwrap_or_default()
        })
    };
    let election = create_resource(election_uuid, |election_uuid| async move {
        load_election(election_uuid).await
    });
    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    let current_election = {
                        move || {
                            election
                                .get()
                                .map(move |election| match election {
                                    Err(e) => {
                                        view! { <pre>"Server error: " {e.to_string()}</pre> }
                                            .into_view()
                                    }
                                    Ok(election) => {
                                        view! { <ElectionVotingView election=election/> }
                                    }
                                })
                        }
                    };
                    { current_election }
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn NoElectionItem() -> impl IntoView {
    view! { <p>"Create an election"</p> }
}


#[component]
fn AddElectionForm(add_election: Action<AddElection, Result<Election, ServerFnError>>) -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    let value = add_election.value();
    create_effect(move |_| {
        value.with(|v| {
            if v.is_some() {
                set_name("".to_string());
                // leptos_axum::Redir
            }
        });
    });
    let _has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <label>
            <input class="peer/showLabel absolute scale-0" type="checkbox"/>
            <span class="block max-h-14 max-w-xs overflow-hidden rounded-lg bg-slate-100 dark:bg-slate-800 px-4 py-0 text-veniceblue-400 hover:text-veniceblue-500 shadow-lg transition-all duration-300 peer-checked/showLabel:max-h-60">
                <h3 class="flex h-14 cursor-pointer items-center font-bold">
                    "Create new election"
                </h3>
                <span class="mb-2">
                    <ActionForm action=add_election>
                        <input type="hidden" name="election_uuid" value=""/>
                        <div class="mt-4 gap-y-2 flex flex-col">
                            <div class="col-span-full">
                                <label for="name">"Name of new election"</label>
                                <div class="mt-2">
                                    <input
                                        type="text"
                                        maxlength="50"
                                        name="name"
                                        on:input=move |ev| {
                                            set_name(event_target_value(&ev));
                                        }

                                        prop:value=name
                                        class="bg-white dark:bg-gray-300 block flex-1 border border-veniceblue-500 border-solid py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm w-full"
                                    />
                                </div>
                            </div>
                            <input
                                type="submit"
                                value="Create election"
                                class="bg-veniceblue-500 dark:bg-veniceblue-800 hover:bg-veniceblue-700 text-white font-bold py-2 px-4 rounded mt-2 mb-4"
                            />
                        </div>
                    </ActionForm>
                </span>
            </span>
        </label>
    }
}

#[component]
pub fn ElectionList() -> impl IntoView {
    let add_election = create_server_action::<AddElection>();

    let elections = create_resource(move || add_election.version().get(), move |_| get_elections());

    view! {
        <AddElectionForm add_election=add_election/>
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    let current_elections = {
                        move || {
                            elections
                                .get()
                                .map(move |elections| match elections {
                                    Err(e) => {
                                        view! { <pre>"Server error: " {e.to_string()}</pre> }
                                            .into_view()
                                    }
                                    Ok(elections) => {
                                        if elections.is_empty() {
                                            view! {
                                                <p>"You don't currently have any elections running"</p>
                                            }
                                                .into_view()
                                        } else {
                                            elections
                                                .into_iter()
                                                .map(move |election| {
                                                    view! { <ElectionView election=election/> }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                        }
                    };
                    { current_elections }
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn ElectionView(election: Election) -> impl IntoView {
    view! {
        <A href=format!("/elections/{}", election.uuid)>
            <div class="text-left border border-veniceblue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white dark:bg-gray-400">
                <div class="flex flex-row text-xl text-veniceblue-700 hover:text-veniceblue-900 font-semibold">
                    <div>{election.name}</div>
                </div>
            </div>
        </A>
    }
}

#[component]
pub fn ElectionVotingView(election: Election) -> impl IntoView {
    view! {
        <div class="p-6 m-3 rounded-lg bg-slate-100 dark:bg-slate-700">
            <div class="flex flex-row justify-between w-full">
                <h2 class="text-2xl text-veniceblue-500 dark:text-veniceblue-400">
                    {election.name}
                </h2>
                <div class="text-xl text-veniceblue-700 dark:text-veniceblue-300">
                    <A href=format!("/voting/{}", election.uuid)>
                        <div class="flex border border-solid border-veniceblue-500 dark:border-veniceblue-300 rounded-full p-3 gap-2 items-center">
                            "Vote " <Icon icon=i::FaRightLongSolid/>
                        </div>
                    </A>
                </div>
            </div>
            <ItemList election_uuid=election.uuid/>
        </div>
    }
}

// Server Functions

#[server]
pub async fn get_elections() -> Result<Vec<Election>, ServerFnError> {
    use crate::models::Election;
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    Election::list(&context).map_err(|e| ServerFnError::ServerError(format!("Could not get list of elections: {}", e)))
}

#[server]
pub async fn load_election(uuid: String) -> Result<Election, ServerFnError> {
    use crate::models::Election;
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    Election::get(&context, &uuid).map_err(|_err| {
        ServerFnError::ServerError("Could not get election".to_string())
    })
}

#[server]
pub async fn add_election(name: String) -> Result<Election, ServerFnError> {
    use crate::models::Election;
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    let election = Election::new(&context, &name).map_err(|_err| {
        ServerFnError::ServerError("Could not create election".to_string())
    });

    if let Ok(election) = election.as_ref() {
        leptos_axum::redirect(&format!("/elections/{}", election.uuid));
    }

    election
}
