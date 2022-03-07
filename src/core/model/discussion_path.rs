use anyhow::anyhow;
use std::str::FromStr;

use crate::core::ResultType;
#[derive(Debug, Clone, PartialEq)]
pub enum DiscussionRoot {
    Custom(String),
    Broadcast,
    Discussion(Discussion),
    Blog(BlogDiscussion),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Discussion {
    Global,
    Problem(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlogDiscussion {
    User(i32),
}

impl ToString for BlogDiscussion {
    fn to_string(&self) -> String {
        match self {
            BlogDiscussion::User(v) => format!("user.{}", v),
        }
    }
}

impl ToString for Discussion {
    fn to_string(&self) -> String {
        match self {
            Discussion::Global => "global".into(),
            Discussion::Problem(v) => format!("problem.{}", v.to_string()),
        }
    }
}

impl ToString for DiscussionRoot {
    fn to_string(&self) -> String {
        match self {
            DiscussionRoot::Custom(v) => v.into(),
            DiscussionRoot::Broadcast => "broadcast".into(),
            DiscussionRoot::Discussion(v) => format!("discussion.{}", v.to_string()),
            DiscussionRoot::Blog(v) => format!("blog.{}", v.to_string()),
        }
    }
}

impl FromStr for BlogDiscussion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.split(".").collect::<Vec<&str>>();
        if parsed.len() != 2 {
            return Err(anyhow!("Expect one ."));
        }
        if parsed[0] != "blog" {
            return Err(anyhow!("Expect: blog.xxx"));
        }
        let uid = i32::from_str_radix(parsed[1], 10).map_err(|e| anyhow!("Invalid uid: {}", e))?;
        return Ok(Self::User(uid));
    }
}

impl FromStr for Discussion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "global" => Ok(Self::Global),
            s if s.starts_with("problem") => {
                let parsed = s.split(".").collect::<Vec<&str>>();
                if parsed.len() != 2 {
                    return Err(anyhow!("Expect one ."));
                }
                let uid = i32::from_str_radix(parsed[1], 10)
                    .map_err(|e| anyhow!("Invalid number: {}", e))?;
                Ok(Self::Problem(uid))
            }
            _ => Err(anyhow!("Invalid: {}", s)),
        }
    }
}

impl FromStr for DiscussionRoot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = || match s {
            "broadcast" => Ok(Self::Broadcast),
            s1 => match s1.split_once(".").ok_or(anyhow!("Expect ."))? {
                ("blog", blog) => Ok(Self::Blog(BlogDiscussion::from_str(blog)?)),
                ("discussion", discussion) => {
                    Ok(Self::Discussion(Discussion::from_str(discussion)?))
                }
                (prefix, _) => Err(anyhow!("Unknown prefix: {}", prefix)),
            },
        };
        let v1: ResultType<Self> = f();
        return match v1 {
            Ok(v) => Ok(v),
            Err(_) => Ok(Self::Custom(s.into())),
        };
    }
}
