use std::str::FromStr;

use log::error;
use sea_orm::{
    sea_query::{ColumnType, ValueType, ValueTypeErr},
    DbErr, TryGetError, TryGetable, Value,
};

use crate::core::model::discussion_path::DiscussionRoot;

impl Into<Value> for DiscussionRoot {
    fn into(self) -> Value {
        return Value::String(Some(Box::new(self.to_string())));
    }
}

impl TryGetable for DiscussionRoot {
    fn try_get(
        res: &sea_orm::QueryResult,
        pre: &str,
        col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        let str_val = String::try_get(res, pre, col)?;
        return DiscussionRoot::from_str(&str_val)
            .map_err(|e| TryGetError::DbErr(DbErr::Type(e.to_string())));
    }
}

impl ValueType for DiscussionRoot {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            Value::String(s) => {
                let t = s.ok_or(ValueTypeErr)?;
                let parsed = DiscussionRoot::from_str(&t).map_err(|e| {
                    error!("Failed to parse discussion root: {}", e);
                    ValueTypeErr
                })?;
                Ok(parsed)
            }
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "discussion_path".into()
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        ColumnType::String(None)
    }
}
