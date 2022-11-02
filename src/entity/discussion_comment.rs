use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "discussion_comment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub content: String,
    #[sea_orm(indexed)]
    pub uid: i32,
    #[sea_orm(indexed,)]
    pub time: chrono::NaiveDateTime,
    #[sea_orm(indexed)]
    pub discussion_id: i32,
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
    #[sea_orm(
        belongs_to = "super::discussion::Entity",
        from = "Column::DiscussionId",
        to = "super::discussion::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Discussion,
}
impl ActiveModelBehavior for ActiveModel {}
