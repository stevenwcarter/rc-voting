
use crate::{error_template::{AppError, ErrorTemplate}, models::Item};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rc-voting-leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="my-0 mx-auto max-w-3xl text-center">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn ItemList() -> impl IntoView {
    let add_item = create_server_multi_action::<AddItem>();
    // let submissions = add_todo.submissions();
    // let item_list = create_resource(|| (), |_| async move { get_items().await });

    view! {
        // {move || match item_list.get() {
        //     None => view! { <p>"Loading items"</p> }.into_view(),
        //     Some(data) => {
        //         view! {
        //             <For each=data key=|item| item.id.clone() let:child>
        //                 <Item item=child/>
        //             </For>
        //         }
        //             .into_view()
        //     }
        // }}
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
    use leptos_axum::extract;
    use std::sync::Arc;
    use axum::Extension;
    use crate::context::GraphQLContext;
    use crate::models::Item;

    let Extension(context): Extension<Arc<GraphQLContext>> = extract().await?;

    Item::list(&context)
}

#[server(AxumExtract, "/test-api")]
pub async fn axum_extract() -> Result<String, ServerFnError> {
    use leptos_axum::extract;
    use std::sync::Arc;
use axum::Extension;
    use crate::context::GraphQLContext;
    use diesel::prelude::*;
    use crate::models::Item;
    use crate::schema::items;

    let Extension(context): Extension<Arc<GraphQLContext>> = extract().await?;
    let mut conn = context.pool.get().expect("Could not get connection");

    let item = items::table.first::<Item>(&mut conn).expect("Could not query database");

    log::info!("Vote is {:#?}", item);

    Ok("test".to_string())
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1 class="p-6 text-6xl text-blue-700">"Welcome to Leptos!"</h1>
        <button
            class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
            on:click=on_click
        >
            "Click Me: "
            {count}
        </button>
        <ItemForm/>
    }
}

#[server(AddItem, "/api/v2")]
pub async fn add_todo(title: String, body: String) -> Result<Item, ServerFnError> {
    use leptos_axum::extract;
    use std::sync::Arc;
use axum::Extension;
    use crate::context::GraphQLContext;
    use crate::models::Item;

    let Extension(context): Extension<Arc<GraphQLContext>> = extract().await?;

    Item::add_new(&context, &title, &body).map_err(|_err| ServerFnError::ServerError("Could not extract method and query...".to_string()))
}

#[component]
fn ItemForm() -> impl IntoView {
    let add_item = create_server_action::<AddItem>();
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
