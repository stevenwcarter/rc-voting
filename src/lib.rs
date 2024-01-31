use std::{env, fmt, str::FromStr};

pub mod api;
pub mod context;
pub mod db;
pub mod graphql;
pub mod models;
pub mod routes;
pub mod schema;

/// Return an environment variable typed generically
///
/// ```
/// use dnd_react::get_env;
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
/// use dnd_react::get_env_typed;
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
