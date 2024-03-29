use sea_orm::entity::prelude::*;

use super::model::wiki_navigation_menu::WikiNavigationMenuList;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wiki_navigation_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(default_value = "新建导航", column_type = "String(Some(30))")]
    pub title: String,
    #[sea_orm(indexed, default_value = 1)]
    pub priority: i32,
    // #[sea_orm(default_value = "[]")]
    pub menu: WikiNavigationMenuList,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
