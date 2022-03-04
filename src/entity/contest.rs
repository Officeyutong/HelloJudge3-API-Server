use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "contest")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub owner_id: i32,
    #[sea_orm(column_type = "String(Some(128))", default = "新建比赛")]
    pub name: String,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())", default = "")]
    pub description: String,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    #[sea_orm(default = false)]
    pub ranklist_visible: bool,
    #[sea_orm(default = false)]
    pub judge_result_visible: bool,
    #[sea_orm(default = "max_score")]
    pub rank_criterion: RankCriterion,
    #[sea_orm(column_type = "Custom(\"LONGTEXT\".to_string())")]
    pub invite_code: String,
    #[sea_orm(default = false)]
    pub rated: bool,
    #[sea_orm(nullable)]
    pub rated_time: Option<chrono::NaiveDateTime>,
    #[sea_orm(default = true)]
    pub private_contest: bool,
    #[sea_orm(default = false)]
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
