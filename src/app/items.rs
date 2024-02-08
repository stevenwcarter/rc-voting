use crate::{
    error_template::ErrorTemplate, models::Item
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn ItemList(election_uuid: String) -> impl IntoView {
    let add_item = create_server_action::<AddItem>();

    let resource_election_uuid = election_uuid.clone();
    let items = create_resource(move || add_item.version().get(), move |_| {
        // set_election_uuid_signal(election_uuid());
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
    view! {
        <div class="grid items-center grid-cols-8 text-left border border-blue-500 border-solid p-3 m-3 rounded-lg shadow-xl bg-white">
            // <div class="align-middle">{item.uuid}</div>
            <div class="col-span-2">{item.title}</div>
            <div class="col-span-6">{item.body}</div>
        // <div class="align-middle">{item.done}</div>
        </div>
    }
}


#[component]
fn ItemForm(add_item: Action<AddItem, Result<Item, ServerFnError>>, election_uuid: String) -> impl IntoView {
    // let add_item = create_server_action::<AddItem>();
    // TODO - grab from query parameters
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
    // if value.get().is_some() {
    //     set_title("".to_string());
    //     set_body("".to_string());
    // }
    let _has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <ActionForm action=add_item>
            <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                <div class="col-span-full">
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
                            class="block flex-1 border border-blue-500 border-solid bg-white py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm w-full"
                        />
                    </div>
                </div>
                <div class="col-span-full">
                    <label for="body">"Describe your vote option"</label>
                    <div class="mt-2">
                        <textarea
                            name="body"
                            class="block w-full rounded-md border border-blue-500 border-solid py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
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

#[server(GetItems)]
pub async fn get_items(election_uuid: String) -> Result<Vec<Item>, ServerFnError> {
    use crate::models::Item;
    use leptos_axum::extract;
    use crate::api::SessionContext;

    let SessionContext(context): SessionContext = extract().await?;

    Ok(Item::list(&context, &election_uuid))
}

#[server(AddItem)]
pub async fn add_item(election_uuid: String, title: String, body: String) -> Result<Item, ServerFnError> {
    use crate::models::Item;
    use leptos_axum::extract;
    use crate::api::SessionContext;
    use log::*;

    warn!("Adding... {election_uuid} {title} {body}");
    let SessionContext(context): SessionContext = extract().await?;

    Item::add_new(&context, &election_uuid, &title, &body).map_err(|err| {
        ServerFnError::ServerError(format!("Could not extract method and query... {:?}", err))
    })
}
