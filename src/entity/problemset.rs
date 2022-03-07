
use sea_orm::entity::prelude::*;

use super::{model::ProblemsetForeignProblem, problemset_problem};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problemset")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "String(Some(100))", default = "新建习题集")]
    pub name: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub description: String,
    #[sea_orm(indexed)]
    pub owner_id: i32,
    pub create_time: chrono::NaiveDateTime,
    #[sea_orm(default = true)]
    pub private: bool,
    pub invite_code: String,
    #[sea_orm(default = "[]")]
    pub foreign_problems: ProblemsetForeignProblem,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}
impl ActiveModelBehavior for ActiveModel {}

impl Related<super::problem::Entity> for Entity {
    fn to() -> RelationDef {
        problemset_problem::Relation::Problem.def()
    }
    fn via() -> Option<RelationDef> {
        Some(problemset_problem::Relation::Problemset.def().rev())
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        super::team_problemset::Relation::Team.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::team_problemset::Relation::Problemset.def().rev())
    }
}
