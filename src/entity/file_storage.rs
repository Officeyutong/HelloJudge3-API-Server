
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file_storage")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "String(Some(40))", auto_increment = false)]
    pub id: String,
    #[sea_orm(indexed, column_type = "String(Some(256))")]
    pub name: String,
    #[sea_orm(indexed)]
    pub size: i64,
    #[sea_orm(indexed)]
    pub upload_time: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
