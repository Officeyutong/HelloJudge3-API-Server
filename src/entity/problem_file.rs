use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problem_file")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub problem_id: i32,
    #[sea_orm(primary_key, column_type = "String(Some(40))")]
    pub file_id: String,
    #[sea_orm(indexed, default_value = false)]
    pub public: bool,
    #[sea_orm(indexed, default_value = false)]
    pub provide: bool,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::problem::Entity",
        from = "Column::ProblemId",
        to = "super::problem::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Problem,
    #[sea_orm(
        belongs_to = "super::file_storage::Entity",
        from = "Column::FileId",
        to = "super::file_storage::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    File,
}
