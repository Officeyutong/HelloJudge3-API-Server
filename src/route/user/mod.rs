mod admin_mode;
mod auth_email;
mod follow;
mod login;
mod logout;
mod model;
mod profile;
pub mod query_login_state;
mod register;
mod require_reset_password;
mod reset_password;
mod change_email;
pub fn make_scope() -> Scope {
    return web::scope("/user")
        .service(query_login_state::query_login_state)
        .service(query_login_state::this_should_be_the_first_request)
        .service(login::login)
        .service(auth_email::auth_email)
        .service(register::register)
        .service(logout::logout)
        .service(require_reset_password::require_reset_password)
        .service(reset_password::reset_password)
        .service(toggle_follow_state)
        .service(get_followee_list)
        .service(get_follower_list)
        .service(toggle_follow_state)
        .service(get_user_profile)
        .service(toggle_admin_mode)
        .service(update_profile)
        .service(change_email::change_email);
}

use actix_web::{web, Scope};

use self::{
    admin_mode::toggle_admin_mode,
    follow::{get_followee_list, get_follower_list, toggle_follow_state},
    profile::{get_user_profile, update_profile},
};
