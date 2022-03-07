use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "challenge_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: i32,
    #[sea_orm(primary_key)]
    pub challenge_id: i32,
    #[sea_orm(primary_key)]
    pub problemset_id: i32,
    #[sea_orm(indexed)]
    pub finished: bool,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::challenge::Entity",
        from = "Column::ChallengeId",
        to = "super::challenge::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Challenge,

    #[sea_orm(
        belongs_to = "super::problemset::Entity",
        from = "Column::ProblemsetId",
        to = "super::problemset::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Problemset,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Uid",
        to = "super::user::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    User,
}
