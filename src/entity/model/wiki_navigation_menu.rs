use serde::{Deserialize, Serialize};

use crate::declare_simple_json_type;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SubMenu {
    pub title: String,
    pub target: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_link: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WikiNavigationMenu {
    pub title: String,
    pub target: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_link: Option<String>,
    pub children: Vec<SubMenu>,
}

declare_simple_json_type!(
    WikiNavigationMenuList,
    Vec::<WikiNavigationMenu>,
    wiki_navigation_menu
);
