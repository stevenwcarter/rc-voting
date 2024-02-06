#![allow(non_snake_case)]

use crate::models::Session;
use crate::session::SessionSvc;
use crate::{context::GraphQLContext, models::User, schema::users};

use anyhow::{Context, Result};
use diesel::prelude::*;
use rand_core::{OsRng, RngCore};
use ring::pbkdf2;
use std::num::NonZeroU32;
use std::num::ParseIntError;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA1;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

const ITERATION_INDEX: usize = 0;
const SALT_INDEX: usize = 1;
const PBKDF2_INDEX: usize = 2;

pub struct UserData;

impl UserData {
    pub fn get_user_by_email(
        context: &GraphQLContext,
        email: &str,
    ) -> Result<User> {
        let mut conn = context
            .pool
            .get()
            .context("Could not get database connection")?;

        let user: User = users::table
            .filter(users::email.like(email))
            .first::<User>(&mut conn)
            .context("Could not find user by email")?;

        Ok(user)
    }
    pub fn create_user(context: &GraphQLContext, user: &User) -> anyhow::Result<()> {
        let mut conn = context
            .pool
            .get()
            .context("Could not get database connection")?;

        diesel::insert_into(users::table)
            .values(user)
            .execute(&mut conn)
            .context("Could not insert user to database")?;

        Ok(())
    }
    pub fn generate_password_hash(password: &str) -> anyhow::Result<String> {
        let mut salt_hex = [0u8; 24];
        OsRng.fill_bytes(&mut salt_hex);
        let iterations: NonZeroU32 = NonZeroU32::new(1_000).unwrap();
        let mut key = [0u8; 20];
        pbkdf2::derive(
            PBKDF2_ALG,
            iterations,
            &salt_hex,
            password.as_bytes(),
            &mut key,
        );
        let key_string = hex::encode(key);

        println!("{iterations}:{}:{key_string}", hex::encode(salt_hex));

        Ok(format!(
            "{}:{}:{}",
            iterations,
            hex::encode(salt_hex),
            key_string
        ))
    }
    pub fn validate_password(password: &str, password_hash: &str) -> bool {
        let parts: Vec<&str> = password_hash.split(':').collect();
        let iterations = parts
            .get(ITERATION_INDEX)
            .unwrap()
            .parse::<NonZeroU32>()
            .unwrap();
        let salt_hex = decode_hex(parts.get(SALT_INDEX).unwrap()).unwrap();
        let hash_hex = decode_hex(parts.get(PBKDF2_INDEX).unwrap().trim()).unwrap();

        let result = pbkdf2::verify(
            PBKDF2_ALG,
            iterations,
            &salt_hex,
            password.as_bytes(),
            &hash_hex,
        );

        if result.is_err() {
            return false;
        }

        true
    }

    pub async fn login_user(
        context: &GraphQLContext,
        email: String,
        password: String,
    ) -> Option<Session> {
        SessionSvc::clean_expired(context);
        let user_result = UserData::get_user_by_email(context, &email);

        if user_result.is_err() {
            return None;
        }

        let user_result = user_result.unwrap();

        let password_hash = &user_result.password_hash;

        let valid_password = UserData::validate_password(&password, password_hash);

        if !valid_password {
            None
        } else {
            let session = Session::new(context, &user_result);
            Some(session)
        }
    }

    pub fn get_users(context: &GraphQLContext) -> Result<Vec<User>> {
        let mut conn = context.pool.get().context("Could not get connection")?;

        let users: Vec<User> = users::table
            .load::<User>(&mut conn)
            .context("Could not load users from database")?;

        Ok(users)
    }

    pub fn get_user(context: &GraphQLContext, user_uuid: &str) -> Result<User, anyhow::Error> {
        let mut conn = context.pool.get().context("Could not get connection")?;

        users::table
            .filter(users::uuid.eq(user_uuid))
            .first::<User>(&mut conn)
            .context("Could not get user by id")
    }
}

