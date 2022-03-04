use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "follower")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub source: i32,
    #[sea_orm(primary_key)]
    pub target: i32,
    #[sea_orm(indexed)]
    pub time: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Source",
        to = "super::user::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Source,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Target",
        to = "super::user::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Target,
}
impl ActiveModelBehavior for ActiveModel {}
