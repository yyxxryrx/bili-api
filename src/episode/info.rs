use crate::{APIResponse, error::APIResult, make_headers};
use make_serde::{MakeSerde, SummonFrom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, MakeSerde, SummonFrom)]
#[make_serde(try)]
pub enum EpisodeType {
    /// 番剧
    Series = 1,
    /// 电影
    Movie = 2,
    /// 纪录片
    Documentary = 3,
    /// 国创
    DomesticallyOriginal = 4,
    /// 电视剧
    TV = 5,
    /// 综艺
    VarietyShow = 7,
}

/// Episode information
///
/// 剧集信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeInfo {
    /// aid
    pub aid: u64,
    /// bvid
    pub bvid: String,
    /// cid
    pub cid: u64,
    /// cover
    ///
    /// 封面
    pub cover: String,
    /// episode id
    ///
    /// ep_id
    pub id: u64,
    /// is new
    ///
    /// 是否是新的
    #[serde(default)]
    pub is_new: bool,
    /// link
    ///
    /// 链接
    pub link: String,
    /// long title
    ///
    /// 长标题
    pub long_title: String,
    /// pub time
    ///
    /// 发布时间
    #[serde(default)]
    pub pub_time: i64,
    /// pv
    ///
    /// pv
    #[serde(default)]
    pub pv: i64,
    /// release date
    ///
    /// 发布日期
    #[serde(default)]
    pub release_date: String,
    /// rights
    ///
    /// 权益
    pub rights: EpisodeRights,
    /// share copy
    ///
    /// 分享文案
    #[serde(default)]
    pub share_copy: String,
    /// share url
    ///
    /// 分享链接
    #[serde(default)]
    pub share_url: String,
    /// short link
    ///
    /// 短链接
    #[serde(default)]
    pub short_link: String,
    /// status
    ///
    /// 状态
    #[serde(default)]
    pub status: i32,
    /// subtitle
    ///
    /// 副标题
    #[serde(default)]
    pub subtitle: String,
    /// title
    ///
    /// 标题
    pub title: String,
    /// vid
    ///
    /// vid
    #[serde(default)]
    pub vid: String,
}

/// Episode rights
///
/// 剧集权益
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeRights {
    pub allow_demand: Option<i32>,
    pub allow_dm: i32,
    pub allow_download: i32,
    pub area_limit: i32,
}

/// Season information data
///
/// 剧集信息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfoData {
    /// activity
    ///
    /// 活动
    #[serde(default)]
    pub activity: Option<SeasonActivity>,
    /// alias
    ///
    /// 别名
    #[serde(default)]
    pub alias: String,
    /// areas
    ///
    /// 地区
    #[serde(default)]
    pub areas: Vec<SeasonArea>,
    /// background cover
    ///
    /// 背景封面
    #[serde(default)]
    pub bkg_cover: String,
    /// cover
    ///
    /// 封面
    pub cover: String,
    /// episodes
    ///
    /// 剧集列表
    #[serde(default)]
    pub episodes: Vec<EpisodeInfo>,
    /// evaluate
    ///
    /// 简介
    #[serde(default)]
    pub evaluate: String,
    /// link
    ///
    /// 链接
    pub link: String,
    /// media id
    ///
    /// 媒体ID
    pub media_id: u64,
    /// mode
    ///
    /// 模式
    pub mode: i32,
    /// new episode
    ///
    /// 最新剧集
    #[serde(default)]
    pub new_ep: Option<NewEpisode>,
    /// publish info
    ///
    /// 发布信息
    #[serde(default)]
    pub publish: Option<SeasonPublish>,
    /// rating
    ///
    /// 评分
    #[serde(default)]
    pub rating: Option<SeasonRating>,
    /// record
    ///
    /// 记录
    #[serde(default)]
    pub record: String,
    /// rights
    ///
    /// 权益
    #[serde(default)]
    pub rights: Option<SeasonRights>,
    /// season id
    ///
    /// season_id
    pub season_id: u64,
    /// season title
    ///
    /// 剧集标题
    #[serde(default)]
    pub season_title: String,
    /// seasons
    ///
    /// 其他季
    #[serde(default)]
    pub seasons: Vec<Season>,
    /// series
    ///
    /// 系列
    #[serde(default)]
    pub series: Option<SeasonSeries>,
    /// share copy
    ///
    /// 分享文案
    #[serde(default)]
    pub share_copy: String,
    /// share url
    ///
    /// 分享链接
    #[serde(default)]
    pub share_url: String,
    /// square cover
    ///
    /// 方形封面
    #[serde(default)]
    pub square_cover: String,
    /// stat
    ///
    /// 统计
    #[serde(default)]
    pub stat: Option<SeasonStat>,
    /// status
    ///
    /// 状态
    #[serde(default)]
    pub status: i32,
    /// subtitle
    ///
    /// 副标题
    #[serde(default)]
    pub subtitle: String,
    /// title
    ///
    /// 标题
    pub title: String,
    /// total
    ///
    /// 总集数
    #[serde(default)]
    pub total: i32,
    /// type
    ///
    /// 类型
    pub r#type: EpisodeType,
    /// up info
    ///
    /// UP主信息
    #[serde(default)]
    pub up_info: Option<UpInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonActivity {
    pub head_bg_url: String,
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonArea {
    pub id: super::SeriesRegion,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEpisode {
    pub desc: String,
    pub id: u64,
    pub is_new: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonNewEpisode {
    pub cover: String,
    pub id: u32,
    pub index_show: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonPublish {
    pub is_finish: i32,
    pub is_started: i32,
    pub pub_time: String,
    pub pub_time_show: String,
    pub weekday: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonRating {
    pub count: u64,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonRights {
    pub allow_bp: i32,
    pub allow_bp_rank: i32,
    pub allow_download: i32,
    pub allow_review: i32,
    pub area_limit: i32,
    pub ban_area_show: i32,
    pub can_watch: i32,
    pub copyright: String,
    pub is_cover_show: i32,
    pub is_preview: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub badge: String,
    pub badge_type: i32,
    pub cover: String,
    pub media_id: u64,
    pub new_ep: Option<SeasonNewEpisode>,
    pub season_id: u64,
    pub season_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonSeries {
    pub display_type: i32,
    pub series_id: u64,
    pub series_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonStat {
    pub coins: u64,
    // pub danmaku: u64,
    pub favorites: u64,
    pub likes: u64,
    pub reply: u64,
    pub share: u64,
    pub views: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpInfo {
    pub avatar: String,
    pub follower: u64,
    pub is_follow: i32,
    pub mid: u64,
    pub nickname: Option<String>,
    pub uname: String,
}

/// Get season info
///
/// 获取剧集信息
pub async fn get_season_info(
    client: &reqwest::Client,
    eid: super::EpisodeID,
) -> APIResult<SeasonInfoData> {
    let resp = client
        .get("https://api.bilibili.com/pgc/view/web/season")
        .headers(make_headers!())
        .query(&[eid.to_tuple()])
        .send()
        .await?
        .text()
        .await?;
    println!("Json: {resp}");
    let resp: APIResponse<SeasonInfoData> = serde_json::from_str(&resp)?;
    resp.into_result()
}
