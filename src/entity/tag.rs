use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "String(Some(20))", auto_increment = false)]
    pub id: String,
    #[sea_orm(default = "新建标签")]
    pub display: String,
    #[sea_orm(column_type = "String(Some(30))", default = "")]
    pub color: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
