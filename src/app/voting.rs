use std::sync::Arc;

use leptos::*;
use leptos::IntoView;
use crate::error_template::ErrorTemplate;
use leptos_router::*;
use super::items::ItemList;
use super::elections::ElectionItemParams;
use crate::models::{Item, Election, Vote, Ballot};

#[component]
pub fn Voting() -> impl IntoView {
    let params = use_params::<ElectionItemParams>();
    let election_uuid = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.election_uuid.clone())
                .unwrap_or_default()
        })
    };
    view! {
        <div class="flex h-screen w-full">
            <VotingInterface election_uuid=election_uuid()/>
        </div>
    }
}

#[component]
pub fn ItemView(ballot_items: Result<Vec<(Item, Option<i32>)>, ServerFnError>, item: Item, voted: bool, position: Option<i32>, election_uuid: String) -> impl IntoView {
    let handler_election_uuid = election_uuid.clone();
    let handler_item = item.clone();
    let click_handler = move |_| {
        if !voted {
                if let Ok(ballot_items) = ballot_items.clone() {
                    let mut votes: Vec<String> = ballot_items.iter().filter(|(_, i)| i.is_some()).map(|(item, _)| item.uuid.clone()).collect();
                    votes.push(handler_item.uuid.clone());

                    let new_ballot = Ballot {
                        election_uuid: handler_election_uuid.clone(),
                        votes,
                    };

                spawn_local(async move {
                    logging::log!("{:?}", new_ballot);

                    save_ballot(new_ballot).await;
                });
                }
            }
    };

    view! {
        <div
            on:click=click_handler
            class="text-left border border-blue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white"
        >
            {item.title}
            -
            {position.unwrap_or(-1)}
            <div class="text-xl text-blue-700 hover:text-blue-900 font-semibold"></div>
        </div>
    }
}

#[component]
pub fn VotingInterface(election_uuid: String) -> impl IntoView {
    let (election_uuid, set_election_uuid) = create_signal(election_uuid.clone());
    let get_ballot_action = create_server_action::<GetBallot>();
    let save_ballot_action = create_server_action::<SaveBallot>();

    let ballot = create_resource(move || (get_ballot_action.version(),save_ballot_action.version()), move |_| get_ballot(election_uuid().clone()));
    let on_update = move || {
        spawn_local(async move{
                get_ballot(election_uuid().clone()).await;
        })
    };
    let closure_ballot = ballot.clone();
    let ballot_items = move || { closure_ballot.clone() };

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    let current_elections = {
                        move || {
                            ballot
                                .get()
                                .map(move |ballot| match ballot {
                                    Err(e) => {
                                        view! { <pre>"Server error: " {e.to_string()}</pre> }
                                            .into_view()
                                    }
                                    Ok(ballot) => {
                                        if ballot.is_empty() {
                                            view! {
                                                <p>"You don't currently have any elections running"</p>
                                            }
                                                .into_view()
                                        } else {
                                            let voted_for = ballot
                                                .clone()
                                                .into_iter()
                                                .filter(|i| i.1.is_some())
                                                .map(move |(item, position)| {
                                                    view! {
                                                        <ItemView
                                                            ballot_items=ballot_items().get().unwrap()
                                                            item=item
                                                            voted=true
                                                            position=position
                                                            election_uuid=election_uuid().clone()
                                                        />
                                                    }
                                                })
                                                .collect_view();
                                            let unvoted = ballot
                                                .into_iter()
                                                .filter(|i| i.1.is_none())
                                                .map(move |(item, position)| {
                                                    view! {
                                                        <ItemView
                                                            ballot_items=ballot_items().get().unwrap()
                                                            item=item
                                                            voted=false
                                                            position=position
                                                            election_uuid=election_uuid().clone()
                                                        />
                                                    }
                                                })
                                                .collect_view();
                                            view! {
                                                <div class="">
                                                    <div>{voted_for}</div>
                                                    <h3>"Click one to include in the vote"</h3>
                                                    <div class="text-slate-400">{unvoted}</div>
                                                </div>
                                            }
                                                .into_view()
                                        }
                                    }
                                })
                        }
                    };
                    view! {
                        <div class="h-full w-full flex items-center justify-center">
                            <div class="flex flex-col w-full md:w-1/2 xl:w-1/3 center place-content-center align-content-center">
                                <div class="w-full">{current_elections}</div>
                            </div>
                        </div>
                    }
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn BallotView(ballot: Vec<Vote>) -> impl IntoView {
    view! {
        <div class="text-left border border-blue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white">
            {ballot.len()}
            <div class="text-xl text-blue-700 hover:text-blue-900 font-semibold"></div>
        </div>
    }
}

#[component]
pub fn ElectionVotingView(election: Election) -> impl IntoView {
    view! {
        <div class="p-6 m-3 rounded-lg bg-white">
            <h2 class="text-2xl text-blue-500">{election.name}</h2>
            <ItemList election_uuid=election.uuid/>
        </div>
    }
}

// Server Functions

#[server(GetBallot)]
pub async fn get_ballot(election_uuid: String) -> Result<Vec<(Item, Option<i32>)>, ServerFnError> {
    use leptos_axum::extract;
    use crate::api::SessionContext;

    logging::log!("Getting session context");
    let SessionContext(context): SessionContext = extract().await?;
    let session = context.session.as_ref().unwrap();

    logging::log!("Getting elections");
    Ok(Item::for_user(&context, &session.user_uuid, &election_uuid))
}

#[server(SaveBallot)]
pub async fn save_ballot(ballot: Ballot) -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use crate::api::SessionContext;
    use log::info;

    info!("{:#?}", ballot);

    let SessionContext(context): SessionContext = extract().await?;

    let _ = Vote::save_ballot(&context, &ballot).map_err(|_| ServerFnError::<String>::ServerError("could not save ballot".to_string()));

    Ok(())
}

#[server(RunElection)]
pub async fn run_election(election_uuid: String) -> Result<Option<Item>, ServerFnError> {
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    Ok(Vote::run_election(&context, &election_uuid))
}

#[server(RunSecondElection)]
pub async fn run_second_election(election_uuid: String, winner: Item) -> Result<Option<Item>, ServerFnError> {
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    Ok(Vote::run_second_election(&context, &election_uuid, &Some(winner)))
}
