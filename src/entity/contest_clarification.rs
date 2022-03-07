use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "contest_clarification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub contest_id: i32,
    pub sender: i32,
    #[sea_orm(indexed)]
    pub send_time: chrono::NaiveDateTime,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub content: String,
    // pub replied:bool,
    #[sea_orm(nullable, indexed)]
    pub replier: Option<i32>,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub reply_content: String,
    #[sea_orm(nullable)]
    pub reply_time: Option<chrono::NaiveDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contest::Entity",
        from = "Column::ContestId",
        to = "super::contest::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Contest,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Sender",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    SendUser,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Replier",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ReplyUser
}
