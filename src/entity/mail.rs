
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "mail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub from_id: i32,
    pub to_id: i32,
    pub time: chrono::NaiveDateTime,
    pub text: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FromId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    FromUser,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ToId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ToUser,
}
