
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wiki_page_version")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub wikipage_id: i32,
    pub uid: i32,
    pub title: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub content: String,
    pub time: chrono::NaiveDateTime,
    pub verified: bool,
    pub base:Option<i32>,
    #[sea_orm(nullable, indexed)]
    pub navigation_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::wiki_navigation_item::Entity",
        from = "Column::NavigationId",
        to = "super::wiki_navigation_item::Column::Id",
        on_delete = "SetNull",
        on_update = "SetNull"
    )]
    Navigation,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::Base",
        to = "Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]  
    Base,
}
impl ActiveModelBehavior for ActiveModel {}
