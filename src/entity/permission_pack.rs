use sea_orm::entity::prelude::*;

use super::model::StringList;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "permission_pack")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub description: String,
    // #[sea_orm(default_value = "[]")]
    pub permissions: StringList,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
