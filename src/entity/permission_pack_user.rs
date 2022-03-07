use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "permission_pack_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pack_id: i32,
    #[sea_orm(primary_key, column_type = "String(Some(20))")]
    pub phone: String,
    pub claimed: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::permission_pack::Entity",
        from = "Column::PackId",
        to = "super::permission_pack::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    PermissionPack,
}
impl ActiveModelBehavior for ActiveModel {}
