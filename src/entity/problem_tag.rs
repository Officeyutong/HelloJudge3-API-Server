use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problem_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub problem_id: i32,
    #[sea_orm(primary_key)]
    pub tag_id: String,
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
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Tag,
}
impl ActiveModelBehavior for ActiveModel {}
