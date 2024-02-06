use leptos::*;
use leptos_router::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_user = create_server_action::<LoginUser>();

    view! {
        <h1 class="p-6 text-6xl text-blue-700">"Welcome to Leptos!"</h1>
        <A href="/sign-up">{"Sign Up"}</A>
        <ActionForm action=login_user>
            <input type="email" name="email"/>
            <input type="password" name="password"/>
            <input
                type="submit"
                value="Add"
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            />
        </ActionForm>
    }
}
#[component]
pub fn SignupPage() -> impl IntoView {
    let signup_user = create_server_action::<SignupUser>();

    view! {
        <h1 class="p-6 text-6xl text-blue-700">"Welcome to Leptos!"</h1>
        <A href="/">{"Log in"}</A>
        <ActionForm action=signup_user>
            <input type="email" name="email"/>
            <input type="password" name="password"/>
            <input
                type="submit"
                value="Add"
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            />
        </ActionForm>
    }
}
#[server(SignupUser)]
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
        .secure(true)
        .max_age(Duration::days(14))
        .build();

    if let Ok(session_cookie) = HeaderValue::from_str(&session_cookie.to_string()) {
        response.insert_header(header::SET_COOKIE, session_cookie);
        leptos_axum::redirect("/election");
    }

    Ok(())
}
#[server(LoginUser)]
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
        .secure(true)
        .max_age(Duration::days(14))
        .build();

    if let Ok(session_cookie) = HeaderValue::from_str(&session_cookie.to_string()) {
        response.insert_header(header::SET_COOKIE, session_cookie);
        leptos_axum::redirect("/elections");
    }

    Ok(())
}
