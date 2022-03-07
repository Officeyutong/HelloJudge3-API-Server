use anyhow::anyhow;

use sea_orm::entity::prelude::*;
use std::str::FromStr;

use super::{
    model::{problem_extra_parameter::ExtraParameter, Example, ProblemSubtask},
    problemset_problem,
};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "problem")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
    pub uploader_id: i32,
    #[sea_orm(column_type = "String(Some(100))", default = "新建题目")]
    pub title: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())", default = "")]
    pub background: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())", default = "")]
    pub content: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())", default = "")]
    pub input_format: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())", default = "")]
    pub output_format: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())", default = "")]
    pub hint: String,
    #[sea_orm(default = "[]")]
    pub examples: Example,
    // #[sea_orm(default = "[]")]
    // pub files: ProblemFile,
    // #[sea_orm(default = "[]")]
    // pub downloads: StringList,
    // #[sea_orm(default = "[]")]
    // pub provides: StringList,
    #[sea_orm(default = "[]")]
    pub subtasks: ProblemSubtask,
    #[sea_orm(default = false, indexed)]
    pub public: bool,
    #[sea_orm(default = true, indexed)]
    pub submission_visible: bool,
    #[sea_orm(nullable)]
    pub invite_code: Option<String>,
    #[sea_orm(column_type = "String(Some(20))", default = "")]
    pub spj_filename: String,
    #[sea_orm(default = false)]
    pub using_file_io: bool,
    #[sea_orm(column_type = "String(Some(30))", default = "")]
    pub input_file_name: String,
    #[sea_orm(column_type = "String(Some(30))", default = "")]
    pub output_file_name: String,
    #[sea_orm(default = "traditional")]
    pub problem_type: ProblemType,
    #[sea_orm(default = serde_json::json!([
        {"lang": "cpp", "parameter": "-std=c++98", "name": "C++98", "force": false},
        {"lang": "cpp", "parameter": "-std=c++11", "name": "C++11", "force": false},
        {"lang": "cpp", "parameter": "-std=c++14", "name": "C++14", "force": false},
        {"lang": "cpp", "parameter": "-std=c++17", "name": "C++17", "force": false},
        {"lang": ".*", "parameter": "-O2", "name": "O2优化", "force": false},
    ]))]
    pub extra_parameter: ExtraParameter,
    #[sea_orm(default = false)]
    pub can_see_results: bool,
    pub create_time: chrono::NaiveDateTime,
    #[sea_orm(column_type = "String(Some(10))", nullable)]
    pub remote_judge_oj: Option<String>,
    #[sea_orm(column_type = "String(Some(20))", nullable)]
    pub remote_problem_id: Option<String>,
    #[sea_orm(default = 0)]
    pub cached_submit_count: i64,
    #[sea_orm(default = 0)]
    pub cached_accepted_count: i64,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(EnumIter, DeriveActiveEnum, Debug, PartialEq, Clone)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum ProblemType {
    #[sea_orm(string_value = "traditional")]
    Traditional,
    #[sea_orm(string_value = "remote_judge")]
    RemoteJudge,
    #[sea_orm(string_value = "submit_answer")]
    SubmitAnswer,
}

impl FromStr for ProblemType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "traditional" => Ok(Self::Traditional),
            "remote_judge" => Ok(Self::RemoteJudge),
            "submit_answer" => Ok(Self::SubmitAnswer),
            _ => Err(anyhow!("Invalid problem type: {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UploaderId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::contest::Entity> for Entity {
    fn to() -> RelationDef {
        super::contest_problem::Relation::Contest.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::contest_problem::Relation::Problem.def().rev())
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        super::team_problem::Relation::Team.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::team_problem::Relation::Problem.def().rev())
    }
}

impl Related<super::problemset::Entity> for Entity {
    fn to() -> RelationDef {
        problemset_problem::Relation::Problemset.def()
    }
    fn via() -> Option<RelationDef> {
        Some(problemset_problem::Relation::Problem.def().rev())
    }
}
