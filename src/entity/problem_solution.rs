
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problem_solution")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub uid: i32,
    #[sea_orm(indexed)]
    pub problem_id: i32,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub content: String,
    pub upload_time: chrono::NaiveDateTime,
    #[sea_orm(indexed)]
    pub top: bool,
    #[sea_orm(indexed)]
    pub verified: bool,
    #[sea_orm(nullable)]
    pub verifier: Option<i32>,
    #[sea_orm(nullable)]
    pub verify_time: Option<chrono::NaiveDateTime>,
    #[sea_orm(nullable, column_type = "Custom(\"LONGTEXT\".into())")]
    pub verify_comment: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::problem::Entity",
        from = "Column::ProblemId",
        to = "super::problem::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Problem,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Uid",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}
