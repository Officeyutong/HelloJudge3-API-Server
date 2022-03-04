pub mod problem_example;
pub mod problem_extra_parameter;
pub mod problem_file;
pub mod problem_subtask;
pub mod submission_result;
pub mod problemset_foreign_problem;
pub use self::problem_example::Example;
pub use self::problem_file::ProblemFile;
pub use self::problem_subtask::ProblemSubtask;
pub use self::submission_result::SubmissionResult;
pub use self::problemset_foreign_problem::ProblemsetForeignProblem;

// pub use self::rating_history::RatingHistory;
#[macro_export]
macro_rules! declare_simple_json_type {
    ($name:ident,$inside:ty,$type_name:ident) => {
        #[derive(Clone, PartialEq)]
        pub struct $name(pub $inside);
        impl Into<$name> for $inside {
            fn into(self) -> $name {
                return $name(self);
            }
        }
        impl Into<sea_orm::Value> for $name {
            fn into(self) -> sea_orm::Value {
                return sea_orm::Value::Json(Some(Box::new(
                    serde_json::to_value(&self.0).unwrap(),
                )));
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <$inside>::fmt(&self.0, f)
            }
        }

        impl sea_orm::TryGetable for $name {
            fn try_get(
                res: &sea_orm::QueryResult,
                pre: &str,
                col: &str,
            ) -> Result<Self, sea_orm::TryGetError> {
                use sea_orm::DbErr;
                use sea_orm::TryGetError;
                let json_val = serde_json::Value::try_get(res, pre, col)?;
                let my_val = serde_json::from_value::<$inside>(json_val)
                    .map_err(|e| TryGetError::DbErr(DbErr::Type(e.to_string())))?;
                return Ok(Self(my_val));
            }
        }
        impl sea_orm::sea_query::ValueType for $name {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                use sea_orm::sea_query::ValueTypeErr;
                match v {
                    sea_orm::Value::Json(p) => {
                        return Ok(Self(
                            serde_json::from_value(*p.ok_or(ValueTypeErr)?).map_err(|e| {
                                use log::error;
                                error!("Failed to deserialize: {}", e);
                                ValueTypeErr
                            })?,
                        ))
                    }
                    _ => return Err(ValueTypeErr),
                };
            }

            fn type_name() -> String {
                stringify!($type_name).into()
            }
            fn column_type() -> sea_orm::sea_query::ColumnType {
                use sea_orm::sea_query::ColumnType;
                return ColumnType::Json;
            }
        }
    };
}

declare_simple_json_type!(StringList, Vec::<String>, string_list);
declare_simple_json_type!(UsizeList, Vec::<usize>, usize_list);
