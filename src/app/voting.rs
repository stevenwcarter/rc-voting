use super::elections::ElectionItemParams;
use super::items::ItemList;
use crate::app::login::{LoginForm, SignupForm};
use crate::models::{Ballot, Election, Item, Vote};
use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::{use_cookie, utils::FromToStringCodec};

#[derive(Copy, Clone)]
pub struct SetSignedIn(pub WriteSignal<bool>);

#[derive(Copy, Clone)]
pub struct ElectionUuidContext(Signal<String>);

#[component]
pub fn Voting() -> impl IntoView {
    let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("X-Login-Session-ID");
    let is_initially_signed_in = move || auth_cookie.with_untracked(|c| c.is_some());
    let (is_signed_in, set_is_signed_in) = create_signal(is_initially_signed_in());
    provide_context(SetSignedIn(set_is_signed_in));

    let params = use_params::<ElectionItemParams>();
    let election_uuid = Signal::derive(move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.election_uuid.clone())
                .unwrap_or_default()
        })
    });
    provide_context(ElectionUuidContext(election_uuid));

    view! {
        <div class="flex min-h-screen min-w-full bg-gradient-to-b from-slate-800 to-slate-600 p-16">
            <Show when=is_signed_in fallback=|| view! { <InlineLogin/> }>
                <div class="min-w-full">
                    <A href="/elections">
                        <div class="bg-white dark:bg-gray-800 flex gap-2 border border-solid border-blue-500 rounded-full text-blue-800 dark:text-blue-200 text-xl flex-nowrap absolute left-2 top-2 py-3 px-4 items-center">
                            <Icon icon=i::FaLeftLongSolid/>
                            "Your elections"
                        </div>
                    </A>
                    <VotingInterface/>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn InlineLogin() -> impl IntoView {
    let (sign_in, set_sign_in) = create_signal::<bool>(false);
    let toggle_signed_in = move |_| set_sign_in.update(|sign_in| *sign_in = !*sign_in);
    view! {
        <div class="h-screen min-w-full flex flex-col items-center align-center justify-center">
            <div class="text-2xl text-blue-600">"Sign in or sign up to participate"</div>
            <button class="bg-blue-400 rounded-lg text-white p-3 m-2" on:click=toggle_signed_in>
                <Show when=move || { sign_in() } fallback=|| { "Sign up instead" }>
                    "Log in instead"
                </Show>
            </button>
            <Show
                when=move || { sign_in() }
                fallback=|| {
                    view! { <LoginForm redirect=false/> }
                }
            >

                <SignupForm redirect=false/>
            </Show>
        </div>
    }
}

#[component]
pub fn ItemView(
    ballot_items: Vec<(Item, Option<i32>)>,
    item: Item,
    voted: bool,
    position: Option<i32>,
) -> impl IntoView {
    let ballot_items = Signal::derive(move || ballot_items.clone());
    let election_uuid = use_context::<ElectionUuidContext>().unwrap().0;
    let handler_item = item.clone();
    let save_ballot_action = use_context::<SaveBallotAction>().unwrap().0;

    let up_item = item.clone();
    let down_item = item.clone();
    let remove_item = item.clone();

    let get_voted_items = move || {
        ballot_items()
            .iter()
            .filter(|(_, i)| i.is_some())
            .cloned()
            .collect::<Vec<(Item, Option<i32>)>>()
    };

    let move_up_click_handler = move |_| {
        if !voted {
            return;
        }

        let mut votes: Vec<String> = get_voted_items()
            .iter()
            .map(|(item, _)| item.uuid.clone())
            .collect();
        let index_to_move = votes.iter().position(|v| *v == up_item.uuid.clone());
        if let Some(index_to_move) = index_to_move {
            if index_to_move == 0 {
                return;
            }
            votes.swap(index_to_move, index_to_move - 1);

            let new_ballot = Ballot {
                election_uuid: election_uuid().clone(),
                votes,
            };

            save_ballot_action.dispatch(new_ballot.into());
        }
    };
    let move_down_click_handler = move |_| {
        if !voted {
            return;
        }

        let mut votes: Vec<String> = get_voted_items()
            .iter()
            .map(|(item, _)| item.uuid.clone())
            .collect();
        let index_to_move = votes.iter().position(|v| *v == down_item.uuid.clone());
        if let Some(index_to_move) = index_to_move {
            if index_to_move == votes.len() - 1 {
                return;
            }
            votes.swap(index_to_move, index_to_move + 1);

            let new_ballot = Ballot {
                election_uuid: election_uuid().clone(),
                votes,
            };

            save_ballot_action.dispatch(new_ballot.into());
        }
    };
    let remove_from_voting_click_handler = move |_| {
        if !voted {
            return;
        }
        let votes: Vec<String> = get_voted_items()
            .iter()
            .map(|(item, _)| item.uuid.clone())
            .filter(|v| *v != remove_item.uuid.clone())
            .collect();

        let new_ballot = Ballot {
            election_uuid: election_uuid().clone(),
            votes,
        };

        save_ballot_action.dispatch(new_ballot.into());
    };
    let add_to_voting_click_handler = move |_| {
        if voted {
            return;
        }
        let mut votes: Vec<String> = get_voted_items()
            .iter()
            .map(|(item, _)| item.uuid.clone())
            .collect();
        votes.push(handler_item.uuid.clone());

        let new_ballot = Ballot {
            election_uuid: election_uuid().clone(),
            votes,
        };

        save_ballot_action.dispatch(new_ballot.into());
    };

    view! {
        <div
            on:click=add_to_voting_click_handler.clone()
            class="flex text-left dark:text-gray-100 border border-blue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white dark:bg-gray-800"
            class=("cursor-pointer", move || !voted)
        >
            <div class="flex flex-col justify-around gap-5" class=("invisible", move || !voted)>
                <button
                    class="text-4xl"
                    class=("invisible", move || position.unwrap_or(-1) == 0)
                    on:click=move_up_click_handler
                    title="move up in vote order"
                >
                    <Icon icon=i::BsArrowUpSquare/>
                </button>
                <button
                    class="text-4xl"
                    class=(
                        "invisible",
                        move || {
                            position.unwrap_or_default() == (get_voted_items().len() as i32 - 1)
                        },
                    )

                    on:click=move_down_click_handler
                    title="move down in vote order"
                >
                    <Icon icon=i::BsArrowDownSquare/>
                </button>
            </div>
            <div class="w-full flex flex-col items-center justify-center p-4">
                <div class="text-2xl mb-3">{item.title}</div>
                <div>{item.body}</div>
            </div>
            <div class="flex flex-col items-center justify-center">
                <Show
                    when=move || voted
                    fallback=move || {
                        let inner_add_to_voting_click_handler = add_to_voting_click_handler.clone();
                        view! {
                            <button
                                class="text-5xl text-green-800"
                                on:click=inner_add_to_voting_click_handler
                                title="vote for this option"
                            >
                                <Icon icon=i::BiCircleRegular/>
                            </button>
                        }
                    }
                >

                    <button
                        class="text-4xl text-red-800 text-opacity-70 hover:text-opacity-100"
                        on:click=remove_from_voting_click_handler.clone()
                        title="remove vote"
                    >
                        <Icon icon=i::CgRemoveR/>
                    </button>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn Winners() -> impl IntoView {
    let winners = use_context::<WinnersContext>().unwrap().0;

    let winner = move || {
        if let Some(Ok((winner, _))) = winners.get() {
            winner
        } else {
            None
        }
    };
    let runner_up = move || {
        if let Some(Ok((_, runner_up))) = winners.get() {
            runner_up
        } else {
            None
        }
    };

    view! {
        <div class="text-blue-800 dark:text-blue-200 flex flex-col items-center justify-center w-full">
            <div class="text-xl">"Current Winners:"</div>
            <div class="flex flex-row gap-4 w-full justify-center items-center">
                <Show when=move || winner().is_some() fallback=|| "No winners yet">
                    <div class="text-xl">"Winner"</div>
                    <div class="text-2xl">{move || winner().unwrap().title}</div>
                </Show>
            </div>
            <div class="flex flex-row gap-4 w-full justify-center items-center">
                <Show when=move || runner_up().is_some() fallback=|| "">
                    <div class="text-xl">"Runner up"</div>
                    <div class="text-2xl">{move || runner_up().unwrap().title}</div>
                </Show>
            </div>
        </div>
    }
}

#[derive(Copy, Clone)]
struct SaveBallotAction(Action<SaveBallot, Result<(), ServerFnError>>);

type WinnersResult = Resource<usize, Result<(Option<Item>, Option<Item>), ServerFnError>>;
#[derive(Copy, Clone)]
struct WinnersContext(WinnersResult);

#[component]
pub fn VotingInterface() -> impl IntoView {
    let election_uuid = use_context::<ElectionUuidContext>().unwrap().0;
    let save_ballot_action = create_server_action::<SaveBallot>();
    provide_context(SaveBallotAction(save_ballot_action));

    let ballot = create_resource(save_ballot_action.version(), move |_| {
        get_ballot(election_uuid())
    });

    let winners = create_resource(save_ballot_action.version(), move |_| {
        run_elections(election_uuid())
    });
    provide_context(WinnersContext(winners));

    view! {
        <div class="h-full w-full flex items-center justify-center">
            <div class="flex flex-col w-full md:w-2/3 xl:w-1/2 center place-content-center align-content-center">
                <div class="w-full">
                    <Transition fallback=move || {
                        view! { <p>"Loading winners..."</p> }
                    }>
                        {move || {
                            winners
                                .get()
                                .map(|data| match data {
                                    Err(e) => {
                                        view! { <pre>"Error: " {e.to_string()}</pre> }.into_view()
                                    }
                                    Ok(_winners) => view! { <Winners/> },
                                })
                        }}

                    </Transition>
                    <Transition fallback=move || {
                        view! { <p>"Loading ballot..."</p> }
                    }>
                        {move || {
                            ballot
                                .get()
                                .map(|data| match data {
                                    Err(e) => {
                                        view! { <pre>"Error: " {e.to_string()}</pre> }.into_view()
                                    }
                                    Ok(ballot) => view! { <BallotInterface ballot=ballot/> },
                                })
                        }}

                    </Transition>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BallotInterface(ballot: Vec<(Item, Option<i32>)>) -> impl IntoView {
    let ballot = Signal::derive(move || ballot.clone());
    let voted_for = move || {
        ballot()
            .into_iter()
            .filter(|i| i.1.is_some())
            .map(move |(item, position)| {
                view! { <ItemView ballot_items=ballot() item=item voted=true position=position/> }
            })
            .collect_view()
    };
    let unvoted = move || {
        ballot()
            .into_iter()
            .filter(|i| i.1.is_none())
            .map(move |(item, position)| {
                view! { <ItemView ballot_items=ballot() item=item voted=false position=position/> }
            })
            .collect_view()
    };

    view! {
        <div class="">
            <div>{voted_for}</div>
            <div class="mt-8 bg-slate-200 dark:bg-slate-800 p-4 sm:rounded-2xl">
                <h3 class="text-xl text-blue-900 dark:text-blue-100">
                    "Click the circle to vote for an option"
                </h3>
                <h4 class="text-lg text-blue-900 dark:text-blue-100">
                    "Then use the arrows above to rank"
                </h4>
                <div>{unvoted}</div>
            </div>
        </div>
    }
    .into_any()
}

#[component]
pub fn BallotView(ballot: Vec<Vote>) -> impl IntoView {
    view! {
        <div class="text-left border border-blue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white dark:bg-gray-800">
            {ballot.len()}
            <div class="text-xl text-blue-700 hover:text-blue-900 font-semibold"></div>
        </div>
    }
}

#[component]
pub fn ElectionVotingView(election: Election) -> impl IntoView {
    view! {
        <div class="p-6 m-3 rounded-lg bg-white dark:bg-gray-800">
            <h2 class="text-2xl text-blue-500">{election.name}</h2>
            <ItemList election_uuid=election.uuid/>
        </div>
    }
}

// Server Functions

#[server]
pub async fn get_winners(
    election_uuid: String,
) -> Result<(Option<Item>, Option<Item>), ServerFnError> {
    use crate::api::SessionContext;
    use leptos_axum::extract;

    let SessionContext(context): SessionContext = extract().await?;

    Ok(Vote::run_elections(&context, &election_uuid))
}

#[server]
pub async fn get_ballot(election_uuid: String) -> Result<Vec<(Item, Option<i32>)>, ServerFnError> {
    use crate::api::SessionContext;
    use leptos_axum::extract;

    let SessionContext(context): SessionContext = extract().await?;
    let session = context.session.as_ref().unwrap();

    Ok(Item::for_user(&context, &session.user_uuid, &election_uuid))
}

#[server]
pub async fn save_ballot(ballot: Ballot) -> Result<(), ServerFnError> {
    use crate::api::SessionContext;
    use leptos_axum::extract;

    let SessionContext(context): SessionContext = extract().await?;

    let _ = Vote::save_ballot(&context, &ballot)
        .map_err(|_| ServerFnError::<String>::ServerError("could not save ballot".to_string()));

    Ok(())
}

#[server]
pub async fn run_elections(
    election_uuid: String,
) -> Result<(Option<Item>, Option<Item>), ServerFnError> {
    use crate::api::SessionContext;
    use leptos_axum::extract;

    let SessionContext(context): SessionContext = extract().await?;

    Ok(Vote::run_elections(&context, &election_uuid))
}
