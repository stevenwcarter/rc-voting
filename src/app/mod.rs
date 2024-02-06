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
        <Title text="Ranked Choice Voting"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="">
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
