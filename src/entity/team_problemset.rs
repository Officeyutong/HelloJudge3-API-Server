use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "team_problemset")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub team_id: i32,
    #[sea_orm(primary_key)]
    pub problemset_id: i32,
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
        on_delete = "Cascade",
    )]
    Problemset,
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade",
    )]
    Team,
}
impl Related<super::problemset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Problemset.def()
    }
}
impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}
