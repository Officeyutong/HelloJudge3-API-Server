
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problem_todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub problem_id: i32,
    #[sea_orm(primary_key)]
    pub uid: i32,
    #[sea_orm(indexed)]
    pub join_time: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::problem::Entity",
        from = "Column::ProblemId",
        to = "super::problem::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Problem,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Uid",
        to = "super::user::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    User,
}
impl ActiveModelBehavior for ActiveModel {}
