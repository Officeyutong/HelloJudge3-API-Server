use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendLinkEntry {
    pub name: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayConfig {
    // 每页题目数量
    pub problems_per_page: i32,
    // 每页的提交数量
    pub submissions_per_page: i32,
    // 每页显示的讨论数量
    pub discussions_per_page: i32,
    // 每页显示的评论数量
    pub comments_per_page: i32,
    // 每页显示的比赛数量
    pub contests_per_page: i32,
    // 主页显示的公告数量
    pub homepage_broadcast: i32,
    // 主页排行榜显示的数量
    pub homepage_ranklist: i32,
    // 主页显示的题目数量
    pub homepage_problems: i32,
    // 主页显示的讨论数量
    pub homepage_discussions: i32,
    // 排行榜每页显示数量
    pub users_on_ranklist_per_page: i32,
    // 问题集每页显示数量
    pub problemsets_per_page: i32,
    // 关注的人\关注者每页显示的个数
    pub followers_per_page: i32,
    // 用户个人信息页面内，显示的动态的每页的条数
    pub userfeeds_per_page: i32,
    // 信息流最长条数
    pub feed_stream_count_limit: i32,
    // 权限包的用户列表每页的记录条数
    pub permissionpack_user_per_page: i32,
    // 虚拟比赛列表每页的项数
    pub virtual_contests_per_page: i32,
    // 每页显示的博客数
    pub blogs_per_page: i32,
    // 博客页面显示的摘要长度
    pub blog_summary_length: i32,
    // 每页的初赛比赛数
    pub preliminary_contests_per_page: i32,
    // 每页的clarification数
    pub clarification_per_page: i32,
    // 管理界面每页的全局推送数
    pub admin_global_notification_per_page: i32,
    // 每页题解数量
    pub solutions_per_page: i32,
    // 图床每页图片数
    pub images_per_page: u32,
    // 每页Wiki版本数
    pub wiki_versions_per_page: i32,
    pub friend_links: Vec<FriendLinkEntry>,
    pub swiper_switch_interval: u32,
    pub show_ranklist_on_homepage: bool,
    pub display_repo_in_footer: bool,
}
impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            problems_per_page: 50,
            submissions_per_page: 20,
            discussions_per_page: 30,
            comments_per_page: 30,
            contests_per_page: 50,
            homepage_broadcast: 8,
            homepage_ranklist: 15,
            homepage_problems: 5,
            homepage_discussions: 5,
            users_on_ranklist_per_page: 30,
            problemsets_per_page: 50,
            followers_per_page: 10,
            userfeeds_per_page: 10,
            feed_stream_count_limit: 50,
            permissionpack_user_per_page: 50,
            virtual_contests_per_page: 20,
            blogs_per_page: 20,
            blog_summary_length: 50,
            preliminary_contests_per_page: 20,
            clarification_per_page: 5,
            admin_global_notification_per_page: 10,
            solutions_per_page: 5,
            images_per_page: 20,
            wiki_versions_per_page: 20,
            friend_links: vec![FriendLinkEntry {
                name: "LibreOJ".to_string(),
                url: "https://loj.ac".to_string(),
            }],
            swiper_switch_interval: 10 * 1000,
            show_ranklist_on_homepage: false,
            display_repo_in_footer: true,
        }
    }
}
