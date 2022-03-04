use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problemset_problem")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub problemset_id: i32,
    #[sea_orm(primary_key)]
    pub problem_id: i32,
    pub sequence: i32,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::problemset::Entity",
        from = "Column::ProblemsetId",
        to = "super::problemset::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Problemset,
    #[sea_orm(
        belongs_to = "super::problem::Entity",
        from = "Column::ProblemId",
        to = "super::problem::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Problem,
}
impl Related<super::problemset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Problemset.def()
    }
}
impl Related<super::problem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Problem.def()
    }
}
