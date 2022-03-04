use clap::ArgMatches;
use log::info;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, EntityTrait,
    Schema,
};

use crate::entity::{
    Contest, ContestProblem, Follower, HomepageSwiper, PermissionGroup, PermissionPack,
    PermissionPackUser, Problem, ProblemTag, Problemset, ProblemsetProblem, Problemtodo,
    Submission, Tag, Team, TeamContest, TeamMember, TeamProblem, TeamProblemset, User,
    UserRatingHistory,Feed
};
use crate::{config::Config, core::ResultType};
use anyhow::anyhow;
#[inline]
async fn create<E: EntityTrait>(
    db: &DatabaseConnection,
    builder: &DatabaseBackend,
    schema: &Schema,
    entity: E,
    name: &str,
) -> ResultType<()> {
    db.execute(builder.build(&schema.create_table_from_entity(entity)))
        .await
        .map_err(|e| anyhow!("Failed to build entity {}: {}", name, e))?;

    return Ok(());
}
pub async fn init_handle(config: &Config, _args: &ArgMatches) -> ResultType<()> {
    let mut opt = ConnectOptions::new(config.common.database_uri.clone());
    opt.sqlx_logging(config.common.debug);
    let db = Database::connect(opt)
        .await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    create(&db, &builder, &schema, PermissionGroup, "permission_group").await?;

    create(&db, &builder, &schema, User, "user").await?;
    create(&db, &builder, &schema, Problem, "problem").await?;
    create(&db, &builder, &schema, Contest, "contest").await?;
    create(&db, &builder, &schema, ContestProblem, "contest_problem").await?;
    create(
        &db,
        &builder,
        &schema,
        UserRatingHistory,
        "user_rating_history",
    )
    .await?;
    create(&db, &builder, &schema, Submission, "submission").await?;
    create(&db, &builder, &schema, Problemset, "problemset").await?;
    create(
        &db,
        &builder,
        &schema,
        ProblemsetProblem,
        "problemset_problem",
    )
    .await?;

    create(&db, &builder, &schema, Team, "team").await?;
    create(&db, &builder, &schema, TeamProblem, "team_problem").await?;
    create(&db, &builder, &schema, TeamContest, "team_contest").await?;
    create(&db, &builder, &schema, TeamMember, "team_member").await?;
    create(&db, &builder, &schema, TeamProblemset, "team_problemset").await?;
    create(&db, &builder, &schema, PermissionPack, "permission_pack").await?;
    create(
        &db,
        &builder,
        &schema,
        PermissionPackUser,
        "permission_pack_user",
    )
    .await?;
    create(&db, &builder, &schema, HomepageSwiper, "homepage_swiper").await?;
    create(&db, &builder, &schema, Follower, "follower").await?;
    create(&db, &builder, &schema, Tag, "tag").await?;
    create(&db, &builder, &schema, ProblemTag, "problem_tag").await?;
    create(&db, &builder, &schema, Problemtodo, "problem_todo").await?;
    create(&db, &builder, &schema, Feed, "feed").await?;
    
    info!("Done!");
    return Ok(());
}
