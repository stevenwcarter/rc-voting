use axum::{Extension, Router};
use std::sync::Arc;

use super::middleware;
use crate::context::GraphQLContext;

pub fn voting_routes(context: Arc<GraphQLContext>) -> Router {
    Router::new()
        // .route("/", get(global_die_stats))
        // .route("/playerStats/:player_id", get(player_die_stats))
        .layer(Extension(context.clone()))
        .layer(middleware())
}

// async fn global_die_stats(SessionContext(context): SessionContext) -> impl IntoResponse {
//     err_wrapper(::get_global_die_stats(&context))
// }
// async fn player_die_stats(
//     SessionContext(context): SessionContext,
//     Path(_player_id): Path<String>,
// ) -> impl IntoResponse {
//     err_wrapper(DieStats::get_player_die_stats(&context))
// }
// #[derive(Debug, Serialize)]
// struct Context {
//     winner: Option<Item>,
//     second: Option<Item>,
//     items: Vec<(Item, Option<i32>)>,
// }

// impl Context {
//     pub fn new(conn: &DbConn) -> Context {
//         Context {
//             winner: Vote::run_election(conn),
//             second: None,
//             items: Vec::new(), // not used if not logged in
//         }
//     }

//     pub fn for_user(user: Auth, conn: &DbConn) -> Context {
//         let winner = Vote::run_election(conn);
//         let second = Vote::run_second_election(conn, &winner);
//         Context {
//             winner,
//             second,
//             items: Item::for_user(user.0, conn),
//         }
//     }
// }

// #[derive(Debug)]
// struct Auth(i32);

// impl<'a, 'r> FromRequest<'a, 'r> for Auth {
//     type Error = !;

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, !> {
//         request
//             .cookies()
//             .get_private("user_id")
//             .and_then(|cookie| cookie.value().parse().ok())
//             .map(|id| Auth(id))
//             .or_forward(())
//     }
// }

// #[post("/login", data = "<input>")]
// fn login(mut cookies: Cookies, input: Form<NewUser>, conn: DbConn) -> Template {
//     let user = input.into_inner();
//     if user.username.is_empty() {
//         index(conn)
//     } else {
//         let u = user.login(&conn);
//         cookies.add_private(Cookie::new("user_id", u.id.to_string()));
//         votes(Auth(u.id), conn)
//     }
// }

// #[post("/vote", data = "<ballot>")]
// fn vote(ballot: Json<Ballot>, user: Auth, conn: DbConn) -> &'static str {
//     Vote::save_ballot(user.0, ballot.into_inner(), &conn);
//     "voted"
// }

// #[get("/")]
// fn votes(user: Auth, conn: DbConn) -> Template {
//     Template::render("vote", Context::for_user(user, &conn))
// }

// #[get("/", rank = 2)]
// fn index(conn: DbConn) -> Template {
//     Template::render("index", Context::new(&conn))
// }

// #[head("/")]
// fn index_head(conn: DbConn) -> Template {
//     index(conn)
// }

// fn rocket() -> (Rocket, Option<DbConn>) {
//     let rocket = rocket::ignite()
//         .attach(DbConn::fairing())
//         .mount("/", routes![index, index_head, login, votes, vote])
//         .attach(Template::fairing());

//     let conn = match cfg!(test) {
//         true => DbConn::get_one(&rocket),
//         false => None,
//     };

//     (rocket, conn)
// }

// fn main() {
//     rocket().0.launch();
// }
