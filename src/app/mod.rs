use crate::{
    app::{
        elections::{ElectionItem, Elections, NoElectionItem},
        login::{LoginPage, SignupPage},
        voting::Voting,
    },
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_darkmode::Darkmode;
use leptos_meta::*;
use leptos_router::*;

pub mod elections;
pub mod items;
pub mod login;
pub mod voting;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let darkmode = Darkmode::init();
    view! {
        <Stylesheet id="leptos" href="/pkg/rc-voting-leptos.css"/>

        <Title text="Ranked Choice Voting"/>

        <Html lang="en" class=move || if darkmode.is_dark() { "dark" } else { "" }/>

        <Body class="bg-white dark:bg-gray-700"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="">
                <Routes>
                    <Route path="/" view=RedirectElections/>
                    <Route path="/elections" view=Elections>
                        <Route path=":election_uuid" view=ElectionItem/>
                        <Route path="" view=NoElectionItem/>
                    </Route>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/sign-up" view=SignupPage/>
                    <Route path="/voting/:election_uuid" view=Voting/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn RedirectElections() -> impl IntoView {
    view! { <Redirect path="/elections"/> }
}
