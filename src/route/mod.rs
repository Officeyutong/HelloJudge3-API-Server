use actix_web::{web, HttpResponse, Responder, Scope};
pub mod user;
pub fn api_root_make_scope() -> Scope {
    return web::scope("")
        .service(redirect_login)
        .service(redirect_query_login_state)
        .service(redirect_get_user_profile)
        .service(redirect_auth_email)
        .service(redirect_register)
        .service(redirect_logout)
        .service(redirect_require_reset_password)
        .service(redirect_reset_password)
        .service(redirect_update_profile)
        .service(redirect_change_email);
}

// const $varname: &str = $src;
// #[actix_web::post($varname)]
// async fn $funcname () -> impl actix_web::Responder {
//     actix_web::HttpResponse::PermanentRedirect()
//         .append_header(("Location", $target))
//         .finish()
// }

#[actix_web::get("/change_email/{token}")]
async fn redirect_change_email(info: web::Path<(String,)>) -> impl Responder {
    HttpResponse::PermanentRedirect()
        .append_header(("Location", format!("/api/user/change_email/{}", info.0)))
        .finish()
}
macro_rules! declare_redirect_inner {
    ($src:expr, $target:expr, $funcname: ident) => {
        #[allow(non_camel_case_types, missing_docs)]
        pub struct $funcname;
        impl ::actix_web::dev::HttpServiceFactory for $funcname {
            fn register(self, __config: &mut actix_web::dev::AppService) {
                pub async fn $funcname() -> impl actix_web::Responder {
                    actix_web::HttpResponse::PermanentRedirect()
                        .append_header(("Location", $target))
                        .finish()
                }
                let __resource = ::actix_web::Resource::new($src)
                    .name(stringify!($funcname))
                    .guard(::actix_web::guard::Post())
                    .to($funcname);
                ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
            }
        }
    };
}

macro_rules! declare_redirect {
    ($name:ident, $src:expr, $target:expr) => {
        paste::paste! {
            declare_redirect_inner!($src, $target, [< redirect_ $name >]);
        }
    };
}

macro_rules! declare_user_redirect {
    ($name:ident) => {
        declare_redirect!(
            $name,
            concat!("/", stringify!($name)),
            concat!("/api/user/", stringify!($name))
        );
    };
}
declare_user_redirect!(login);
declare_user_redirect!(query_login_state);
declare_user_redirect!(get_user_profile);
declare_user_redirect!(auth_email);
declare_user_redirect!(register);
declare_user_redirect!(logout);
declare_user_redirect!(require_reset_password);
declare_user_redirect!(reset_password);
declare_user_redirect!(update_profile);
