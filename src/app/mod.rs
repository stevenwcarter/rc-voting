use crate::{
    app::{
        elections::{ElectionItem, Elections, NoElectionItem},
        login::{LoginPage, SignupPage},
    },
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use self::items::ItemList;

pub mod elections;
pub mod items;
pub mod login;

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
                    <Route path="" view=LoginPage/>
                    <Route path="/sign-up" view=SignupPage/>
                    <Route path="/elections" view=Elections>
                        <Route path=":election_uuid" view=ElectionItem/>
                        <Route path="" view=NoElectionItem/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
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
        <ItemList/>
    }
}
