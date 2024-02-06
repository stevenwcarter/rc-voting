use std::{env, fmt, str::FromStr};

#[cfg(feature = "ssr")]
pub mod api;
pub mod app;
#[cfg(feature = "ssr")]
pub mod context;
#[cfg(feature = "ssr")]
pub mod db;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
pub mod graphql;
pub mod models;
#[cfg(feature = "ssr")]
pub mod schema;
#[cfg(feature = "ssr")]
pub mod session;
#[cfg(feature = "ssr")]
pub mod user_data;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

/// Return an environment variable typed generically
///
/// ```
/// use rc_voting_leptos::get_env;
/// assert!(get_env("PATH", "test").len() > 4);
/// ````
pub fn get_env(search_key: &str, default: &str) -> String {
    if let Some(value) = env::vars()
        .filter(|(key, _)| key.eq(search_key))
        .map(|(_, value)| value)
        .next()
    {
        value
    } else {
        default.to_string()
    }
}

/// Return an environment variable typed generically
///
/// ```
/// use rc_voting_leptos::get_env_typed;
/// assert!(get_env_typed::<u16>("SHLVL", 9) > 0);
/// ````
pub fn get_env_typed<T>(search_key: &str, default: T) -> T
where
    T: FromStr + fmt::Debug,
{
    if let Some(value) = env::vars()
        .filter(|(key, _)| key.eq(search_key))
        .map(|(_, value)| value)
        .next()
    {
        let value = value.parse::<T>();
        match value {
            Ok(value) => value,
            Err(_) => default,
        }
    } else {
        default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_env_variabls() {
        assert!(get_env("PATH", "test").len() > 4);
    }

    #[test]
    fn it_gets_env_variabls_default() {
        assert!(get_env("FOOBAR", "test").eq("test"));
    }

    #[ignore]
    #[test]
    fn it_gets_typed_env_variables() {
        assert!(get_env_typed::<u16>("SHLVL", 9) > 0);
    }

    #[test]
    fn it_gets_typed_env_variables_default() {
        assert!(get_env_typed::<u16>("FOOBAR", 9) == 9);
    }
}
