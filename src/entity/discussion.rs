use sea_orm::entity::prelude::*;

use crate::core::model::discussion_path::DiscussionRoot;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "discussion")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "String(Some(128))", indexed)]
    pub path: DiscussionRoot,
    #[sea_orm(column_type = "String(Some(100))")]
    pub title: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub content: String,
    #[sea_orm(indexed)]
    pub uid: i32,
    #[sea_orm(indexed, )]
    pub time: chrono::NaiveDateTime,
    pub private: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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
