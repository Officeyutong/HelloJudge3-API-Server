
use sea_orm::entity::prelude::*;

use super::model::StringList;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(default_value = false)]
    pub banned: bool,
    #[sea_orm(column_type = "String(Some(20))", unique, indexed)]
    pub username: String,
    #[sea_orm(column_type = "String(Some(256))", unique)]
    pub password: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())")]
    pub description: String,
    #[sea_orm(column_type = "String(Some(128))", indexed)]
    pub email: String,
    pub register_time: chrono::NaiveDateTime,
    // #[sea_orm(default = "[]")]
    // pub rating_history: RatingHistory,
    #[sea_orm(indexed, default = 1500)]
    pub rating: i32,
    #[sea_orm(column_type = "String(Some(20))", default = "default")]
    pub permission_group: String,
    #[sea_orm(default = "[]")]
    pub permissions: StringList,
    #[sea_orm(default = 0)]
    pub force_logout_before: i64,
    #[sea_orm(column_type = "String(Some(20))", nullable, default = "", indexed)]
    pub phone_number: Option<String>,
    #[sea_orm(default = false)]
    pub phone_verified: bool,
    #[sea_orm(nullable)]
    pub last_refreshed_cached_accepted_problems: Option<chrono::NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to = "super::permission_group::Entity",
    //     from = "Column::PermissionGroup",
    //     to = "super::permission_group::Column::Id",
    //     on_delete = "Restrict",
    //     on_update = "Restrict"
    // )]
    // PermissionGroup,
    #[sea_orm(has_many = "super::user_rating_history::Entity")]
    UserRatingHistory,
}
impl ActiveModelBehavior for ActiveModel {}

// impl Related<super::problem::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Problem.def()
//     }
// }

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        super::team_member::Relation::Team.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::team_member::Relation::User.def().rev())
    }
}
