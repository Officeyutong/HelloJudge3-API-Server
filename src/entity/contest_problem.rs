use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "contest_problem")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub problem_id: i32,
    #[sea_orm(primary_key)]
    pub contest_id: i32,
    pub sequence: i32,
    #[sea_orm(default_value = 1.0)]
    pub score_weight: f64,
}


impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::problem::Entity",
        from = "Column::ProblemId",
        to = "super::problem::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade",
    )]
    Problem,
    #[sea_orm(
        belongs_to = "super::contest::Entity",
        from = "Column::ContestId",
        to = "super::contest::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade",
    )]
    Contest,
}
impl Related<super::problem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Problem.def()
    }
}
impl Related<super::contest::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contest.def()
    }
}
