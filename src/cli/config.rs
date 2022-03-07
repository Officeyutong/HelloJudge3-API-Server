use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CLIConfig {
    pub default_hj2_url: String,
    pub default_reset_db_name: String,
    pub import_config: ImportConfig,
    pub create_tables: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImportConfig {
    pub user: bool,
    pub problem: bool,
    pub contest: bool,
    pub submission: bool,
    pub problemset: bool,
    pub team: bool,
    pub wiki: bool,
    pub challenge: bool,
    pub file_storage: bool,
    pub other: bool,
    pub discussion: bool,
    pub preliminary: bool,
    pub permission_pack: bool,
    pub tag: bool,
}
impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            user: true,
            problem: true,
            contest: true,
            submission: true,
            problemset: true,
            team: true,
            wiki: true,
            challenge: true,
            other: true,
            file_storage: true,
            discussion: true,
            preliminary: true,
            permission_pack: true,
            tag: true,
        }
    }
}
impl Default for CLIConfig {
    fn default() -> Self {
        Self {
            default_hj2_url: "mysql://127.0.0.1/hj2".into(),
            default_reset_db_name: "hj3".into(),
            import_config: ImportConfig::default(),
            create_tables: true,
        }
    }
}
