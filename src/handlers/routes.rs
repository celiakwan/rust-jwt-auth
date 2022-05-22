use super::buyer;
use super::seller;
use super::user;
use rocket::Route;

pub fn api_routes() -> Vec<Route> {
    routes![
        user::create,
        user::login,
        user::logout,
        buyer::page1,
        buyer::page2,
        seller::page1,
        seller::page2
    ]
}
