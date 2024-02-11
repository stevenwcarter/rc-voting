use crate::{
    error_template::ErrorTemplate, models::Item
};
use leptos_icons::*;
use icondata as i;
use leptos::*;
use leptos_router::*;

#[derive(Copy, Clone)]
pub struct DeleteItemContext(Action<DeleteItem, Result<(), ServerFnError>>);
#[derive(Copy, Clone)]
pub struct UpdateItemContext(Action<UpdateItem, Result<(), ServerFnError>>);

#[component]
pub fn ItemList(election_uuid: String) -> impl IntoView {
    let add_item = create_server_action::<AddItem>();
    let delete_item_action = create_server_action::<DeleteItem>();
    let update_item_action = create_server_action::<UpdateItem>();

    provide_context(DeleteItemContext(delete_item_action));
    provide_context(UpdateItemContext(update_item_action));

    let resource_election_uuid = election_uuid.clone();
    let items = create_resource(move || (delete_item_action.version().get(), update_item_action.version().get(), add_item.version().get()), move |_| {
        get_items(resource_election_uuid.clone())
    });

    view! {
        <ItemForm add_item=add_item election_uuid=election_uuid/>
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>
                {move || {
                    let current_items = {
                        move || {
                            items
                                .get()
                                .map(move |items| match items {
                                    Err(e) => {
                                        view! { <pre>"Server error: " {e.to_string()}</pre> }
                                            .into_view()
                                    }
                                    Ok(items) => {
                                        if items.is_empty() {
                                            view! { <p>"No votable items found"</p> }.into_view()
                                        } else {
                                            items
                                                .into_iter()
                                                .map(move |item| {
                                                    view! { <Item item=item/> }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                        }
                    };
                    { current_items }
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn Item(item: Item) -> impl IntoView {
    let item = Signal::derive(move || item.clone());

    let (is_editing, set_is_editing) = create_signal(false);
    let (title, set_title) = create_signal(item().title);
    let (body, set_body) = create_signal(item().body);

    let delete_action = use_context::<DeleteItemContext>().unwrap().0;
    let update_action = use_context::<UpdateItemContext>().unwrap().0;

    let delete_handler = move |_| {
        delete_action.dispatch(item().into());
    };

    let toggle_done_handler = move |_| {
        let mut new_item = item().clone();
        new_item.done = !new_item.done;

        update_action.dispatch(new_item.into());
    };

    let edit_submit_handler = move |_| {
        let new_item = Item {
            title: title(),
            body: body(),
            ..item()
        };
        update_action.dispatch(new_item.into());
        set_is_editing(false);
    };

    view! {
        <div class="flex text-left border border-blue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white dark:bg-gray-900 dark:text-white">

            <div class="flex flex-col space-around m-3">
                <button
                    class="text-2xl text-blue-600"
                    on:click=move |_| { set_is_editing.update(|editing| *editing = !*editing) }
                >
                    <Icon icon=i::BsPencilSquare/>
                </button>
            </div>
            <div class="flex flex-col w-full px-4 dark:bg-gray-900 dark:text-white">
                <Show
                    when=is_editing
                    fallback=move || {
                        view! {
                            <div class="text-2xl">{item().title}</div>
                            <div class="">{item().body}</div>
                        }
                    }
                >

                    <input
                        type="text"
                        name="title"
                        class="block w-full rounded-md border border-blue-500 border-solid py-1.5 text-gray-900 dark:text-gray-100 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 dark:bg-gray-900"
                        prop:value=title
                        on:input=move |e| set_title(event_target_value(&e))
                    />
                    <textarea
                        name="body"
                        class="block w-full rounded-md border border-blue-500 border-solid py-1.5 text-gray-900 dark:text-gray-100 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 dark:bg-gray-900"
                        prop:value=body
                        on:input=move |e| set_body(event_target_value(&e))
                    ></textarea>
                    <button
                        class="border border-solid bg-blue-500 text-white text-xl"
                        on:click=edit_submit_handler
                    >
                        "Update"
                    </button>
                    "Editing"
                </Show>
            </div>
            <div class="flex flex-col space-around gap-y-8 m-4">
                <button
                    class="align-middle text-3xl"
                    on:click=toggle_done_handler
                    title="toggle 'done' status"
                >
                    <Show
                        when=move || item().done
                        fallback=|| {
                            view! {
                                <div class="text-gray-700">
                                    <Icon icon=i::TbCircleCheck/>
                                </div>
                            }
                        }
                    >

                        <div class="text-green-500">
                            <Icon icon=i::TbCircleCheckFilled/>
                        </div>
                    </Show>
                </button>
                <button
                    class="align-middle text-3xl text-red-500"
                    on:click=delete_handler
                    title="delete this item"
                >
                    <Icon icon=i::BiTrashAltRegular/>
                </button>
            </div>
        </div>
    }
}


#[component]
fn ItemForm(add_item: Action<AddItem, Result<Item, ServerFnError>>, election_uuid: String) -> impl IntoView {
    let (title, set_title) = create_signal("".to_string());
    let (body, set_body) = create_signal("".to_string());

    let value = add_item.value();
    create_effect(move |_| {
        value.with(|v| {
            if v.is_some() {
                set_title("".to_string());
                set_body("".to_string());
            }
        });
    });

    let _has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <ActionForm action=add_item>
            <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                <div class="col-span-full dark:text-white">
                    <input type="hidden" name="election_uuid" value=election_uuid/>
                    <label for="title">"Add a short summary"</label>
                    <div class="mt-2">
                        <input
                            type="text"
                            maxlength="50"
                            name="title"
                            on:input=move |ev| {
                                set_title(event_target_value(&ev));
                            }

                            prop:value=title
                            class="block flex-1 border border-blue-500 border-solid bg-white dark:bg-gray-800 dark:text-white py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm w-full"
                        />
                    </div>
                </div>
                <div class="col-span-full dark:text-white">
                    <label for="body">"Describe your vote option"</label>
                    <div class="mt-2">
                        <textarea
                            name="body"
                            class="block w-full rounded-md border border-blue-500 border-solid py-1.5 text-gray-900 dark:text-gray-100 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 dark:bg-gray-900"
                            rows="3"
                            on:input=move |ev| {
                                set_body(event_target_value(&ev));
                            }

                            prop:value=body
                        ></textarea>
                    </div>
                </div>
                <input
                    type="submit"
                    value="Add"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                />
            </div>
        </ActionForm>
    }
}

// Server Functions

#[server]
pub async fn get_items(election_uuid: String) -> Result<Vec<Item>, ServerFnError> {
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    Ok(Item::list(&context, &election_uuid))
}

#[server]
pub async fn add_item(election_uuid: String, title: String, body: String) -> Result<Item, ServerFnError> {
    use leptos_axum::extract;
    use crate::api::SessionContext;
    use log::*;

    warn!("Adding... {election_uuid} {title} {body}");
    let SessionContext(context): SessionContext = extract().await?;

    Item::add_new(&context, &election_uuid, &title, &body).map_err(|err| {
        ServerFnError::ServerError(format!("Could not extract method and query... {:?}", err))
    })
}

#[server]
pub async fn update_item(item: Item) -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    item.update(&context);

    Ok(())
}
#[server]
pub async fn delete_item(item: Item) -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    Item::delete(&context, &item.election_uuid, &item.uuid);

    Ok(())
}
