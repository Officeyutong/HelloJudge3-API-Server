use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "image_store")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "String(Some(40))", auto_increment = false)]
    pub file_id: String,
    #[sea_orm(column_type = "String(Some(40))", indexed)]
    pub thumbnail_id: String,
    #[sea_orm(indexed)]
    pub uid: i32,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Uid",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::file_storage::Entity",
        from = "Column::ThumbnailId",
        to = "super::file_storage::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Thumbnail,
    #[sea_orm(
        belongs_to = "super::file_storage::Entity",
        from = "Column::FileId",
        to = "super::file_storage::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    File,
}
