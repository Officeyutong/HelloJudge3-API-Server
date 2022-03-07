use std::str::FromStr;

use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{MySqlPool, Row};
use uuid::Uuid;

use crate::{
    core::ResultType,
    entity::{
        cached_accepted_problem, file_storage,
        model::{
            problem_extra_parameter::ExtraParameter, problem_file::ProblemFileEntry, Example,
            ProblemSubtask,
        },
        problem::{self, ProblemType},
        problem_file, problemtodo,
    },
};

pub async fn import_problem(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: problem");
    {
        let mut files_to_copy = Vec::<crate::cli::model::ProblemFileEntry>::new();

        let output1 = {
            let mut conn = hj2.acquire().await?;
            let mut rows = sqlx::query("SELECT * FROM problem").fetch(&mut conn);
            let mut curr_vec = vec![];
            while let Some(row) = rows.try_next().await? {
                let pid = row.try_get("id")?;
                let name: String = row.try_get("title")?;
                info!("Problem started: {}, {}", pid, name);
                let model = problem::ActiveModel {
                    id: Set(pid),
                    uploader_id: Set(row.try_get("uploader_id")?),
                    title: Set(name.clone()),
                    background: Set(row.try_get("background")?),
                    content: Set(row.try_get("content")?),
                    input_format: Set(row.try_get("input_format")?),
                    output_format: Set(row.try_get("output_format")?),
                    hint: Set(row.try_get("hint")?),
                    examples: Set(Example(serde_json::from_str(row.try_get("example")?)?)),
                    subtasks: Set(ProblemSubtask(serde_json::from_str(
                        row.try_get("subtasks")?,
                    )?)),
                    public: Set(row.try_get("public")?),
                    submission_visible: Set(row.try_get("submission_visible")?),
                    invite_code: Set(row.try_get("invite_code")?),
                    spj_filename: Set(row.try_get("spj_filename")?),
                    using_file_io: Set(row.try_get("using_file_io")?),
                    input_file_name: Set(row.try_get("input_file_name")?),
                    output_file_name: Set(row.try_get("output_file_name")?),
                    problem_type: Set(ProblemType::from_str(row.try_get("problem_type")?)?),
                    extra_parameter: Set(ExtraParameter(serde_json::from_str(
                        row.try_get("extra_parameter")?,
                    )?)),
                    can_see_results: Set(row.try_get("can_see_results")?),
                    create_time: Set(row.try_get("create_time")?),
                    remote_judge_oj: Set(row.try_get("remote_judge_oj")?),
                    remote_problem_id: Set(row.try_get("remote_problem_id")?),
                    cached_submit_count: Set(row.try_get("cached_submit_count")?),
                    cached_accepted_count: Set(row.try_get("cached_accepted_count")?),
                };
                // .insert(db)
                // .await?;/
                let downloads: Vec<String> = serde_json::from_str(row.try_get("downloads")?)?;
                let provides: Vec<String> = serde_json::from_str(row.try_get("downloads")?)?;
                let files: Vec<ProblemFileEntry> = serde_json::from_str(row.try_get("files")?)?;
                curr_vec.push((model, downloads, provides, files, pid, name));
            }
            curr_vec
        };
        for (model, downloads, provides, files, pid, name) in output1.into_iter() {
            model.insert(db).await?;
            for file in files.iter() {
                let this_download = downloads.contains(&file.name);
                let this_provide = provides.contains(&file.name);
                let file_id = Uuid::new_v4().to_string();
                files_to_copy.push(crate::cli::model::ProblemFileEntry {
                    file_id: file_id.clone(),
                    file_name: file.name.clone(),
                    problem_id: pid,
                });
                file_storage::ActiveModel {
                    id: Set(file_id.clone()),
                    name: Set(file.name.clone()),
                    size: Set(file.size as i64),
                    upload_time: Set({
                        let timestamp = file
                            .last_modified_time
                            .unwrap_or(chrono::Local::now().timestamp() as f64);
                        let integral = timestamp as i64;
                        let nsec = ((timestamp - (integral as f64)) * 1e9) as u32;
                        chrono::NaiveDateTime::from_timestamp(integral, nsec)
                    }),
                }
                .insert(db)
                .await?;
                problem_file::ActiveModel {
                    problem_id: Set(pid),
                    file_id: Set(file_id),
                    public: Set(this_download),
                    provide: Set(this_provide),
                }
                .insert(db)
                .await?;
            }
            info!("Problem done: {}, {}", pid, name);
        }

        tokio::fs::write(
            "problem-files-import.json",
            serde_json::to_string(&files_to_copy)?,
        )
        .await?;
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM cached_accepted_problems").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            cached_accepted_problem::ActiveModel {
                uid: Set(row.try_get("uid")?),
                problem_id: Set(row.try_get("problem_id")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM problem_todo").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            problemtodo::ActiveModel {
                uid: Set(row.try_get("uid")?),
                problem_id: Set(row.try_get("problem_id")?),
                join_time: Set(row.try_get("join_time")?),
            }
            .insert(db)
            .await?;
        }
    }
    return Ok(());
}
