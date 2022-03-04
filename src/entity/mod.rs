pub mod model;
// pub mod contest_problem;
macro_rules! declare_model_exports {
    ($name:ident) => {
        paste::paste! {
            pub mod [< $name:lower >];
            pub use [< $name:lower >] ::ActiveModel as [< $name:camel ActivaModel>];
            pub use [< $name:lower >] ::Entity as [<$name:camel >];
            pub use [< $name:lower >] ::Model as [<$name:camel Model>];
        }
    };
}
declare_model_exports!(user);
declare_model_exports!(user_rating_history);

declare_model_exports!(problem);
declare_model_exports!(problem_tag);

declare_model_exports!(contest);
declare_model_exports!(contest_problem);

declare_model_exports!(submission);

declare_model_exports!(team);
declare_model_exports!(team_problem);
declare_model_exports!(team_contest);
declare_model_exports!(team_member);
declare_model_exports!(team_problemset);

declare_model_exports!(problemset);
declare_model_exports!(problemset_problem);

declare_model_exports!(permission_pack);
declare_model_exports!(permission_pack_user);

declare_model_exports!(homepage_swiper);

declare_model_exports!(follower);

declare_model_exports!(tag);

declare_model_exports!(problemtodo);

declare_model_exports!(permission_group);

declare_model_exports!(feed);