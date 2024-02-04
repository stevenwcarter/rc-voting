use crate::{
    error_template::ErrorTemplate,
    models::Item,
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn ItemList() -> impl IntoView {
    let add_item = create_server_action::<AddItem>();

    let items = create_resource(move || add_item.version().get(), move |_| get_items());

    view! {
        <ItemForm add_item=add_item/>
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
                                                    view! { <li>{item.title} <br/> {item.body}</li> }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                        }
                    };
                    view! { <ul>{current_items}</ul> }
                }}

            </ErrorBoundary>
        </Transition>
    }
}

#[component]
pub fn Item(item: Item) -> impl IntoView {
    view! {
        <div>
            <div>{item.id}</div>
            <div>{item.title}</div>
            <div>{item.body}</div>
            <div>{item.done}</div>
        </div>
    }
}

#[server(GetItems)]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    use crate::context::GraphQLContext;
    use crate::models::Item;
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;

    let Extension(context): Extension<Arc<GraphQLContext>> = extract().await?;

    Ok(Item::list(&context))
}

#[server(AddItem, "/api/v2")]
pub async fn add_todo(title: String, body: String) -> Result<Item, ServerFnError> {
    use crate::context::GraphQLContext;
    use crate::models::Item;
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;

    let Extension(context): Extension<Arc<GraphQLContext>> = extract().await?;

    Item::add_new(&context, &title, &body).map_err(|_err| {
        ServerFnError::ServerError("Could not extract method and query...".to_string())
    })
}

#[component]
fn ItemForm(add_item: Action<AddItem, Result<Item, ServerFnError>>) -> impl IntoView {
    // let add_item = create_server_action::<AddItem>();
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
                            class="block flex-1 border border-blue-500 border-solid bg-transparent py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm w-full"
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
