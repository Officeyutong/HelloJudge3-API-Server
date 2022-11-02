use sea_orm::entity::prelude::*;

use super::model::PreliminarySubquestionList;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "preliminary_problem")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub contest: i32,
    pub problem_type: PreliminaryProblemType,
    #[sea_orm(indexed,default_value = -1)]
    pub problem_id: i32,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".into())")]
    pub content: String,
    // #[sea_orm(default_value = "[]")]
    pub questions: PreliminarySubquestionList,
    #[sea_orm(default_value = 0)]
    pub score: f64,
}

#[derive(EnumIter, DeriveActiveEnum, Debug, PartialEq, Clone)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum PreliminaryProblemType {
    #[sea_orm(string_value = "selection")]
    Selection,
    #[sea_orm(string_value = "fill_blank")]
    FillBlank,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
