use leptos::*;
use leptos_router::*;
use leptos_use::{use_cookie, utils::FromToStringCodec};

use super::voting::SetSignedIn;

#[component]
pub fn UnAuthRedirect() -> impl IntoView {
    let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("X-Login-Session-ID");
    let redirect_view = move || auth_cookie.get().is_none().then(|| view! { <Redirect path="/login"/> });

    view! { <div>{redirect_view}</div> }
}
#[component]
pub fn AuthRedirect(path: &'static str) -> impl IntoView {
    let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("X-Login-Session-ID");
    let path = move || path.to_string();
    let redirect_view = move || auth_cookie.get().is_some().then(|| view! { <Redirect path=path()/> });

    view! { <div>{redirect_view}</div> }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <AuthRedirect path="/"/>
        <div class="bg-gray-100 flex justify-center items-center h-screen">
            // Left: image
            <div class="w-1/2 h-screen hidden lg:block">
                <img
                    src="/ballot-box.jpg"
                    alt="voting image"
                    class="object-cover w-full h-full dark:bg-gray-900 dark:bg-blend-screen"
                />
            </div>
            // Right: login form
            <div class="lg:p-36 md:p-52 sm:20 p-8 w-full lg:w-1/2">
                <h1 class="text-2xl font-semibold mb-4 text-blue-700">"Log in to RC Voting"</h1>
                <A
                    class="text-white bg-blue-700 hover:bg-blue-600 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
                    href="/sign-up"
                >
                    {"Sign up instead.."}
                </A>
                <LoginForm redirect=true/>
            </div>
        </div>
    }
}

#[component]
pub fn LoginForm(redirect: bool) -> impl IntoView {
    let login_action = create_server_action::<LoginUser>();
    let (redirectView, setRedirectView) = create_signal::<Option<View>>(None);

    create_effect(move |_| {
        if login_action.value().get().is_some() {
            if !redirect {
                let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("X-Login-Session-ID");
                if auth_cookie.get().is_some() {
                    let set_is_signed_in = use_context::<SetSignedIn>();
                    if let Some(set_signed_in) = set_is_signed_in{
                        set_signed_in.0.set(true);
                    }
                }
                return;
            }
            setRedirectView(Some(view! { <Redirect path="/elections"/> }));
        }
    });

    view! {
        {redirectView}
        <ActionForm action=login_action>
            <div class="mb-4 mt-4">
                <label for="email" class="block text-gray-600">
                    Email
                </label>
                <input
                    type="email"
                    name="email"
                    class="w-full border border-gray-300 rounded-md py-2 px-3 focus:outline-none focus:border-blue-500"
                    autocomplete="off"
                />
            </div>
            <div class="mb-4">
                <label for="password" class="block text-gray-600">
                    Password
                </label>
                <input
                    type="password"
                    name="password"
                    class="w-full border border-gray-300 rounded-md py-2 px-3 focus:outline-none focus:border-blue-500"
                    autocomplete="off"
                />
            </div>
            <input
                type="submit"
                value="Log in"
                class="bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded-md py-2 px-4 w-full"
            />
        </ActionForm>
    }
}

#[component]
pub fn SignupForm(redirect: bool) -> impl IntoView {
    let signup_user = create_server_action::<SignupUser>();

    let (redirectView, setRedirectView) = create_signal::<Option<View>>(None);

    create_effect(move |_| {
        if signup_user.value().get().is_some() {
            if !redirect {
                let (auth_cookie, _) = use_cookie::<String, FromToStringCodec>("X-Login-Session-ID");
                if auth_cookie.get().is_some() {
                    let set_is_signed_in = use_context::<SetSignedIn>();
                    if let Some(set_signed_in) = set_is_signed_in{
                        set_signed_in.0.set(true);
                    }
                }
                return;
            }
            setRedirectView(Some(view! { <Redirect path="/elections"/> }));
        }
    });

    view! {
        {redirectView}
        <ActionForm action=signup_user>
            <div class="mb-4 mt-4">
                <label for="email" class="block text-gray-600">
                    Email
                </label>
                <input
                    type="email"
                    name="email"
                    class="w-full border border-gray-300 rounded-md py-2 px-3 focus:outline-none focus:border-blue-500"
                    autocomplete="off"
                />
            </div>
            <div class="mb-4">
                <label for="password" class="block text-gray-600">
                    Password
                </label>
                <input
                    type="password"
                    name="password"
                    class="w-full border border-gray-300 rounded-md py-2 px-3 focus:outline-none focus:border-blue-500"
                    autocomplete="off"
                />
            </div>
            <input
                type="submit"
                value="Sign up"
                class="bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded-md py-2 px-4 w-full"
            />
        </ActionForm>
    }
}

#[component]
pub fn SignupPage() -> impl IntoView {
    view! {
        <AuthRedirect path="/"/>
        <div class="bg-gray-100 flex justify-center items-center h-screen">
            // Left: image
            <div class="w-1/2 h-screen hidden lg:block">
                <img src="/ballot-box.jpg" alt="voting image" class="object-cover w-full h-full"/>
            </div>
            // Right: login form
            <div class="lg:p-36 md:p-52 sm:20 p-8 w-full lg:w-1/2">
                <h2 class="text-2xl font-semibold mb-4 text-blue-700">"Create an account"</h2>
                <A
                    class="text-white bg-blue-700 hover:bg-blue-600 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
                    href="/"
                >
                    {"Log in instead"}
                </A>
                <SignupForm redirect=true/>
            </div>
        </div>
    }
}

// Server Functions

#[server]
async fn signup_user(email: String, password: String) -> Result<(), ServerFnError> {
    use crate::context::GraphQLContext;
    use axum::Extension;
    use leptos_axum::extract;
    use std::sync::Arc;
    use leptos_axum::ResponseOptions;
    use uuid::Uuid;
    use cookie::{time::Duration, Cookie};
    use crate::user_data::UserData;
    use crate::models::{User, Session};
    use http::{header, HeaderValue};

    let Extension(context): Extension<Arc<GraphQLContext>> = extract().await?;
    let response = expect_context::<ResponseOptions>();

    let password_hash = UserData::generate_password_hash(&password).unwrap();

    let user = User {
        password_hash,
        email,
        uuid: Uuid::new_v4().to_string(),
    };

    let create_user_result = UserData::create_user(&context, &user);
    if create_user_result.is_err() {
        return Err(ServerFnError::ServerError("Could not create user".to_string()));
    }

    let session = Session::new(&context, &user);
    let session_cookie = Cookie::build(("X-Login-Session-ID", session.uuid.clone()))
        .path("/")
        .secure(false)
        .max_age(Duration::days(14))
        .build();

    if let Ok(session_cookie) = HeaderValue::from_str(&session_cookie.to_string()) {
        response.insert_header(header::SET_COOKIE, session_cookie);
    }

    Ok(())
}

#[server]
async fn login_user(
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    use crate::context::GraphQLContext;
    use axum::Extension;
    use leptos_axum::ResponseOptions;
    use leptos_axum::extract;
    use std::sync::Arc;
    use cookie::{time::Duration, Cookie};
    use crate::user_data::UserData;
    use crate::models::Session;
    use http::{header, HeaderValue};

    let Extension(context): Extension<Arc<GraphQLContext>> = extract().await?;
    let response = expect_context::<ResponseOptions>();

    let user_result = UserData::get_user_by_email(&context, &email);
    if user_result.is_err() {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    }
    let user_result = user_result.unwrap();
    let password_hash = &user_result.password_hash;

    let valid_password = UserData::validate_password(&password, password_hash);

    if !valid_password {
        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
    }

    let session = Session::new(&context, &user_result);

    let session_cookie = Cookie::build(("X-Login-Session-ID", session.uuid.clone()))
        .path("/")
        .secure(false)
        .max_age(Duration::days(14))
        .build();

    if let Ok(session_cookie) = HeaderValue::from_str(&session_cookie.to_string()) {
        response.insert_header(header::SET_COOKIE, session_cookie);
    }

    Ok(())
}
