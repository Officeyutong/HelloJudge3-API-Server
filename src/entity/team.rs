
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "String(Some(30))", default = "新建团队")]
    pub name: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub description: String,
    #[sea_orm(indexed)]
    pub owner_id: i32,
    pub create_time: chrono::NaiveDateTime,
    #[sea_orm(default = true)]
    pub private: bool,
    pub invite_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl Related<super::problem::Entity> for Entity {
    fn to() -> RelationDef {
        super::team_problem::Relation::Problem.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::team_problem::Relation::Team.def().rev())
    }
}

impl Related<super::contest::Entity> for Entity {
    fn to() -> RelationDef {
        super::team_contest::Relation::Contest.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::team_contest::Relation::Team.def().rev())
    }
}
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::team_member::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::team_member::Relation::Team.def().rev())
    }
}

impl Related<super::problemset::Entity> for Entity {
    fn to() -> RelationDef {
        super::team_problemset::Relation::Problemset.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::team_problemset::Relation::Team.def().rev())
    }
}
