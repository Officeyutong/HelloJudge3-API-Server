pub mod model;
// pub mod contest_problem;
macro_rules! declare_model_exports {
    ($name:ident) => {
        paste::paste! {
            pub mod [< $name:lower >];
            pub use [< $name:lower >] ::ActiveModel as [< $name:camel ActivaModel>];
            pub(crate) use [< $name:lower >] ::Entity as [<$name:camel >];
            pub use [< $name:lower >] ::Model as [<$name:camel Model>];
        }
    };
}
declare_model_exports!(user);
declare_model_exports!(user_rating_history);

declare_model_exports!(problem);
declare_model_exports!(problem_tag);
declare_model_exports!(problem_file);

declare_model_exports!(contest);
declare_model_exports!(contest_problem);
declare_model_exports!(contest_clarification);
declare_model_exports!(submission);

declare_model_exports!(team);
declare_model_exports!(team_problem);
declare_model_exports!(team_contest);
declare_model_exports!(team_member);
declare_model_exports!(team_problemset);
declare_model_exports!(team_file);

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

declare_model_exports!(file_storage);

declare_model_exports!(image_store);

declare_model_exports!(preliminary_contest);
declare_model_exports!(preliminary_problem);

declare_model_exports!(mail);
declare_model_exports!(virtual_contest);

declare_model_exports!(problem_solution);

declare_model_exports!(challenge);
declare_model_exports!(challenge_problemset);
declare_model_exports!(challenge_record);

declare_model_exports!(wiki_config);
declare_model_exports!(wiki_page);
declare_model_exports!(wiki_page_version);
declare_model_exports!(wiki_navigation_item);

declare_model_exports!(discussion);
declare_model_exports!(discussion_comment);

declare_model_exports!(cached_accepted_problem);
