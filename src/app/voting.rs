use leptos::*;
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
pub fn VotingInterface(election_uuid: String) -> impl IntoView {
    let get_ballot_action = create_server_action::<GetBallot>();

    let ballot = create_resource(move || get_ballot_action.version().get(), move |_| get_ballot(election_uuid.clone()));

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
                                            ballot
                                                .into_iter()
                                                .map(move |(item, position)| {
                                                    view! { <ItemView item=item position=position/> }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                        }
                    };
                    view! {
                        <div class="h-full w-full flex items-center justify-center">
                            <div class="flex flex-col md:w-1/2 lg:w-1/3 center place-content-center align-content-center">
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
pub fn ItemView(item: Item, position: Option<i32>) -> impl IntoView {
    view! {
        <div class="text-left border border-blue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white">
            {item.title} - {position.unwrap_or(-1)}
            <div class="text-xl text-blue-700 hover:text-blue-900 font-semibold"></div>
        </div>
    }
}
#[component]
pub fn BallotView(ballot: Vec<Vote>) -> impl IntoView {
    view! {
        <div class="text-left border border-blue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white">
            // {ballot.election_uuid}
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

    let SessionContext(context): SessionContext = extract().await?;

    Vote::save_ballot(&context, &ballot).map_err(|_| ServerFnError::<String>::ServerError("could not save ballot".to_string()));

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
