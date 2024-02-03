
use crate::error_template::{AppError, ErrorTemplate};
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
    let extractor = move |_| {
    spawn_local(async {
                let _result = axum_extract().await;
            });
    };


    view! {
        <h1 class="p-6 text-6xl text-blue-700">"Welcome to Leptos!"</h1>
        <button
            class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
            on:click=on_click
        >
            "Click Me: "
            {count}
        </button>
        <button on:click=extractor>EXTRACT</button>
    }
}
