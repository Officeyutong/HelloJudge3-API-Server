
use sea_orm::entity::prelude::*;

use super::model::{SubmissionResult, UsizeList};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "submission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uid: i32,
    #[sea_orm(column_type = "String(Some(20))")]
    pub language: String,
    #[sea_orm(indexed)]
    pub problem_id: i32,
    pub submit_time: chrono::NaiveDateTime,
    #[sea_orm(default_value = false)]
    pub public: bool,
    #[sea_orm(nullable, indexed, default_value = None)]
    pub contest_id: Option<i32>,
    #[sea_orm(nullable, indexed, default_value = None)]
    pub virtual_contest_id: Option<i32>,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub code: String,
    // #[sea_orm(default_value = "{}")]
    pub judge_result: SubmissionResult,
    #[sea_orm(indexed, default_value = 0)]
    pub score: i32,
    #[sea_orm(indexed, default_value = 0)]
    pub memory_cost: i64,
    #[sea_orm(indexed, default_value = 0)]
    pub time_cost: i64,
    #[sea_orm(column_type = "String(Some(128))")]
    pub extra_compile_parameter: String,
    // #[sea_orm(default_value = "[]")]
    pub selected_compile_parameters: UsizeList,
    #[sea_orm(indexed)]
    pub status: SubmissionStatus,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub message: String,
    #[sea_orm(column_type = "String(Some(20))")]
    pub judger: String,
}

#[derive(EnumIter, DeriveActiveEnum, Debug, PartialEq, Clone)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum SubmissionStatus {
    #[sea_orm(string_value = "waiting")]
    Waiting,
    #[sea_orm(string_value = "judging")]
    Judging,
    #[sea_orm(string_value = "accepted")]
    Accepted,
    #[sea_orm(string_value = "unaccepted")]
    Unaccepted,
    #[sea_orm(string_value = "unknown")]
    Unknown,
    #[sea_orm(string_value = "compile_error")]
    CompileError,
}

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
        belongs_to = "super::problem::Entity",
        from = "Column::ProblemId",
        to = "super::problem::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Problem,
    #[sea_orm(
        belongs_to = "super::contest::Entity",
        from = "Column::ContestId",
        to = "super::contest::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Contest,
}
impl ActiveModelBehavior for ActiveModel {}
