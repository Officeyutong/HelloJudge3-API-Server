use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JudgeStatusEntry {
    pub icon: String,
    pub text: String,
    pub color: String,
}
impl JudgeStatusEntry {
    pub fn new(icon: &str, text: &str, color: &str) -> Self {
        Self {
            color: color.to_string(),
            icon: icon.to_string(),
            text: text.to_string(),
        }
    }
}
#[derive(Clone)]
pub struct JudgeStatusConfig(HashMap<String, JudgeStatusEntry>);
impl Serialize for JudgeStatusConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for JudgeStatusConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(HashMap::<String, JudgeStatusEntry>::deserialize(
            deserializer,
        )?))
    }
}
impl Debug for JudgeStatusConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        HashMap::<String, JudgeStatusEntry>::fmt(&self.0, f)
    }
}
impl Default for JudgeStatusConfig {
    fn default() -> Self {
        Self(HashMap::from([
            (
                "waiting".to_string(),
                JudgeStatusEntry::new("circle notched loading icon", "等待评测中", "blue"),
            ),
            (
                "judging".to_string(),
                JudgeStatusEntry::new("circle notched loading icon", "评测中", "blue"),
            ),
            (
                "accepted".to_string(),
                JudgeStatusEntry::new("check icon", "通过", "green"),
            ),
            (
                "unaccepted".to_string(),
                JudgeStatusEntry::new("times icon", "未通过", "red"),
            ),
            (
                "wrong_answer".to_string(),
                JudgeStatusEntry::new("x icon", "答案错误", "red"),
            ),
            (
                "time_limit_exceed".to_string(),
                JudgeStatusEntry::new("clock outline icon", "超出时限", "red"),
            ),
            (
                "memory_limit_exceed".to_string(),
                JudgeStatusEntry::new("microchip icon", "内存超限", "purple"),
            ),
            (
                "runtime_error".to_string(),
                JudgeStatusEntry::new("exclamation circle icon", "运行时错误", "red"),
            ),
            (
                "skipped".to_string(),
                JudgeStatusEntry::new("cog icon", "跳过", "blue"),
            ),
            (
                "unknown".to_string(),
                JudgeStatusEntry::new("question circle icon", "未知", "black"),
            ),
            (
                "invisible".to_string(),
                JudgeStatusEntry::new("times icon", "不可见", "black"),
            ),
            (
                "unsubmitted".to_string(),
                JudgeStatusEntry::new("code icon", "未提交", "yellow"),
            ),
            (
                "judge_failed".to_string(),
                JudgeStatusEntry::new("times icon", "评测失败", "red"),
            ),
            (
                "compile_error".to_string(),
                JudgeStatusEntry::new("cog icon", "编译错误", "blue"),
            ),
        ]))
    }
}
