use log::info;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait, Database, DatabaseBackend,
    DatabaseConnection, EntityTrait, Schema, Set,
};

use crate::core::SYSTEM_NOTIFICATION_USERID;
use crate::entity::model::StringList;
use crate::entity::{
    permission_group, user, CachedAcceptedProblem, Challenge, ChallengeProblemset, ChallengeRecord,
    Contest, ContestClarification, ContestProblem, Discussion, DiscussionComment, Feed,
    FileStorage, Follower, HomepageSwiper, ImageStore, Mail, PermissionGroup, PermissionPack,
    PermissionPackUser, PreliminaryContest, PreliminaryProblem, Problem, ProblemFile,
    ProblemSolution, ProblemTag, Problemset, ProblemsetProblem, Problemtodo, Submission, Tag, Team,
    TeamContest, TeamFile, TeamMember, TeamProblem, TeamProblemset, User, UserRatingHistory,
    VirtualContest, WikiConfig, WikiNavigationItem, WikiPage, WikiPageVersion,
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
pub async fn init_handle(config: &Config, insert_records: bool) -> ResultType<()> {
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

    create(&db, &builder, &schema, FileStorage, "file_storage").await?;
    create(&db, &builder, &schema, ImageStore, "image_store").await?;
    create(&db, &builder, &schema, Mail, "mail").await?;
    create(
        &db,
        &builder,
        &schema,
        PreliminaryContest,
        "preliminary_contest",
    )
    .await?;
    create(
        &db,
        &builder,
        &schema,
        PreliminaryProblem,
        "preliminary_problem",
    )
    .await?;
    create(&db, &builder, &schema, ProblemFile, "problem_file").await?;
    create(&db, &builder, &schema, TeamFile, "team_file").await?;
    create(&db, &builder, &schema, Feed, "feed").await?;
    create(&db, &builder, &schema, ProblemSolution, "problem_solution").await?;

    create(&db, &builder, &schema, Challenge, "challenge").await?;
    create(
        &db,
        &builder,
        &schema,
        ChallengeProblemset,
        "challenge_problemset",
    )
    .await?;
    create(&db, &builder, &schema, ChallengeRecord, "challenge_record").await?;

    create(&db, &builder, &schema, VirtualContest, "virtual_contest").await?;

    create(&db, &builder, &schema, WikiConfig, "wiki_config").await?;
    create(
        &db,
        &builder,
        &schema,
        WikiNavigationItem,
        "wiki_navigation_item",
    )
    .await?;
    create(&db, &builder, &schema, WikiPage, "wiki_page").await?;
    create(&db, &builder, &schema, WikiPageVersion, "wiki_page_version").await?;

    create(&db, &builder, &schema, Discussion, "discussion").await?;
    create(
        &db,
        &builder,
        &schema,
        DiscussionComment,
        "discussion_comment",
    )
    .await?;

    create(
        &db,
        &builder,
        &schema,
        CachedAcceptedProblem,
        "cached_accepted_problem",
    )
    .await?;

    create(
        &db,
        &builder,
        &schema,
        ContestClarification,
        "contest_clarification",
    )
    .await?;
    if insert_records {
        create_records(&db).await?;
    }
    info!("Tables created.");
    return Ok(());
}
#[inline]
async fn create_records(db: &DatabaseConnection) -> ResultType<()> {
    permission_group::ActiveModel {
        id: Set("default".to_string()),
        inherit: Set(None),
        name: Set("普通用户".to_string()),
        permissions: Set(StringList(vec![
            "challenge.use".into(),
            "problemset.use.public".into(),
            "remote_judge.use".into(),
            "blog.use".into(),
            "virtualcontest.use".into(),
            "[provider:allteams]".into(),
            "[provider:all-challenge]".into(),
        ])),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!("Failed to insert: permission group default: {}", e))?;

    permission_group::ActiveModel {
        id: Set("admin".to_string()),
        inherit: Set(Some("default".to_string())),
        name: Set("管理员".to_string()),
        permissions: Set(StringList(vec!["*".to_string()])),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!("Failed to insert: permission group admin: {}", e))?;
    user::ActiveModel {
        banned: Set(false),
        description: Set("".into()),
        email: Set("!".into()),
        force_logout_before: Set(0),
        id: Set(SYSTEM_NOTIFICATION_USERID),
        last_refreshed_cached_accepted_problems: Set(None),
        password: Set("NO-LOGIN".into()),
        permission_group: Set("admin".into()),
        permissions: Set(StringList(vec![])),
        phone_number: Set(None),
        phone_verified: Set(true),
        rating: Set(1500),
        register_time: Set(chrono::Local::now().naive_local()),
        username: Set("系统通知".into()),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!("Failed to insert: user admin: {}", e))?;
    return Ok(());
}
