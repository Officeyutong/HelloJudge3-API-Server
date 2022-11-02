use std::str::FromStr;

use anyhow::anyhow;

use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "contest")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub owner_id: i32,
    #[sea_orm(column_type = "String(Some(128))")]
    pub name: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())")]
    pub description: String,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    #[sea_orm(default_value = false)]
    pub ranklist_visible: bool,
    #[sea_orm(default_value = false)]
    pub judge_result_visible: bool,
    // #[sea_orm(default_expr = "RankCriterion::MaxScore")]
    pub rank_criterion: RankCriterion,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())")]
    pub invite_code: String,
    #[sea_orm(default_value = false)]
    pub rated: bool,
    #[sea_orm(nullable)]
    pub rated_time: Option<chrono::NaiveDateTime>,
    #[sea_orm(default_value = true)]
    pub private_contest: bool,
    #[sea_orm(default_value = false)]
    pub closed: bool,
}

#[derive(EnumIter, DeriveActiveEnum, Debug, PartialEq, Clone)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum RankCriterion {
    #[sea_orm(string_value = "max_score")]
    MaxScore,
    #[sea_orm(string_value = "last_submit")]
    LastSubmit,
    #[sea_orm(string_value = "penalty")]
    Penalty,
}

impl Default for RankCriterion {
    fn default() -> Self {
        Self::MaxScore
    }
}

impl FromStr for RankCriterion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "max_score" => Ok(Self::MaxScore),
            "last_submit" => Ok(Self::LastSubmit),
            "penalty" => Ok(Self::Penalty),
            _ => Err(anyhow!("Invalid rank criterion: {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_rating_history::Entity")]
    UserRatingHistory,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::OwnerId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}
impl ActiveModelBehavior for ActiveModel {}

impl Related<super::contest_problem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRatingHistory.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        super::team_contest::Relation::Team.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::team_contest::Relation::Contest.def().rev())
    }
}
