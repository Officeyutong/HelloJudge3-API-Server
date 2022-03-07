use sea_orm::entity::prelude::*;

use super::model::StringList;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "permission_group")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "String(Some(20))", auto_increment = false)]
    pub id: String,
    #[sea_orm(column_type = "String(Some(50))", default = "新建权限组")]
    pub name: String,
    #[sea_orm(default = "[]")]
    pub permissions: StringList,
    #[sea_orm(column_type = "String(Some(20))", nullable)]
    pub inherit: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
