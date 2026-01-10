use crate::{APIResponse, error::APIResult};
use make_serde::{MakeSerde, SummonFrom};
use serde::{Deserialize, Serialize};

/// The information of video
///
/// 视频的信息
///
/// docs: [视频基本信息](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/info.html)
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfoData {
    /// The bvid of video
    ///
    /// 视频的bvid
    pub bvid: String,
    /// The aid of video
    ///
    /// 视频的aid
    pub aid: u64,
    /// The number of the video list
    ///
    /// 视频分P的数量
    pub videos: u32,
    /// partition tid
    ///
    /// 分区tid
    pub tid: u16,
    /// partition tid v2
    ///
    /// 分区tid（v2）
    pub tid_v2: u16,
    /// sub partition name
    ///
    /// 子分区名称
    pub tname: String,
    /// sub partition name v2
    ///
    /// 子分区名称（v2）
    pub tname_v2: String,
    /// video type
    ///
    /// 视频类型
    #[serde(with = "copyright_serde")]
    pub copyright: Copyright,
    /// cover picture url
    ///
    /// 稿件封面图片url
    pub pic: String,
    /// video title
    ///
    /// 稿件标题
    pub title: String,
    /// publish time (second level timestamp)
    ///
    /// 稿件发布时间（秒级时间戳）
    #[serde(default)]
    pub pubdate: f64,
    /// contribution time (second level timestamp)
    ///
    /// 用户投稿时间（秒级时间戳）
    pub ctime: f64,
    /// video introduction
    ///
    /// 视盘简介
    #[serde(default)]
    pub desc: String,
    /// the new version video introduction
    ///
    /// 新版视频简介
    pub desc_v2: Vec<VideoDescV2>,
    /// video status
    ///
    /// 视频状态
    pub state: i8,
    /// # Deprecated
    ///
    /// video attribute
    ///
    /// # 已经弃用
    ///
    /// 稿件属性位配置
    #[serde(default)]
    pub attribute: Option<u8>,
    /// video length(all p) (unit: seconds)
    ///
    /// 稿件总时长（所有分P）（单位：秒）
    pub duration: f32,
    /// Crash video jumps to avid
    ///
    /// > This field only exists in collision videos
    ///
    /// 撞车视频跳转aid
    ///
    /// > 仅有撞车视频存在此字段
    #[serde(default)]
    pub forward: Option<u64>,
    /// Activity ID in which the manuscript participated
    ///
    /// 稿件参与的活动id
    pub mission_id: Option<i32>,
    /// Redirect URL
    ///
    /// > This field only exists for TV dramas or movies
    /// > AV/BV ->EP for TV dramas and movies
    ///
    /// 重定向url
    ///
    /// > 仅番剧或影视视频存在此字段
    /// > 用于番剧&影视的av/bv -> ep
    #[serde(default)]
    pub redirect_url: Option<String>,
    /// Video attribute flag
    ///
    /// 视频属性标志
    pub rights: VideoInfoRights,
    /// The up info
    ///
    /// 视频UP主信息
    pub owner: VideoOwner,
    /// Video status
    ///
    /// 视频状态数
    pub stat: VideoStatus,
    /// Dispute/Warning Information
    ///
    /// 争议/警告信息
    pub argue_info: VideoArgueInfo,
    /// Dynamic textual content synchronized with video releases
    ///
    /// 视频同步发布的动态的文字内容
    pub dynamic: String,
    /// Video 1P cid
    ///
    /// 视频1P的cid
    pub cid: u64,
    /// video 1p dimension
    ///
    /// 视频1P的分辨率
    pub dimension: VideoDimension,
    /// Used for teenage mode
    ///
    /// 用于青少年模式
    pub teenage_mode: i32,
    pub is_chargeable_season: bool,
    /// Can it be displayed in Story Mode?
    ///
    /// 是否可以在 Story Mode 展示?
    pub is_story: bool,
    /// Is it a charging exclusive video
    ///
    /// 是否为充电专属视频
    pub is_upower_exclusive: bool,
    pub is_upower_play: bool,
    /// Does the charging exclusive video support trial viewing
    ///
    /// 充电专属视频是否支持试看
    pub is_upower_preview: bool,
    /// Is caching not allowed?
    ///
    /// 是否不允许缓存?
    pub no_cache: bool,
    /// Video Split P List
    ///
    /// 视频分P列表
    pub pages: Vec<VideoPageInfo>,
    /// Video CC subtitle information
    ///
    /// 视频CC字幕信息
    pub subtitle: VideoSubtitleInfo,
    /// Video Collection Information
    ///
    /// > Videos not included in the collection do not have this option
    ///
    /// 视频合集信息
    ///
    /// > 不在合集中的视频无此项
    #[serde(default)]
    pub ugc_season: Option<VideoUgcSeasonInfo>,
    /// List of cooperative members
    ///
    /// > Non cooperative videos do not have this option
    pub staff: Option<Vec<VideoStaffInfo>>,
    pub is_season_display: bool,
    /// User Dress up Information
    ///
    /// 用户装扮信息
    pub user_garb: VideoUserGarb,
    pub honor_reply: VideoHonorReply,
    /// Empty
    ///
    /// 空串
    pub like_icon: String,
    /// Do you need to redirect to BV number?
    ///
    /// 需要跳转到BV号?
    pub need_jump_bv: bool,
    /// Prohibit displaying UP main information?
    ///
    /// 禁止展示UP主信息?
    pub disable_show_up_info: bool,
    /// Is it a Story Mode video?
    ///
    /// 是否为 Story Mode 视频?
    pub is_story_play: u8,
    /// Are you submitting a video for yourself?
    ///
    /// 是否为自己投稿的视频?
    pub is_view_self: bool,
}

/// New video introduction content
///
/// 新版视频简介内容
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoDescV2 {
    /// 简介内容
    ///
    /// > raw_type=Normal 时显示原文
    /// > raw_type=At 时显示'@'+raw_text+' '并链接至biz_id的主页
    pub raw_text: String,
    /// 类型（因为type在rust里是关键子所以原字段名 **type** 用 **raw_type** 代替
    #[serde(rename = "type", with = "desc_v2_type")]
    pub raw_type: DescV2Type,
    /// 被@的用户的mid
    #[serde(with = "biz_id_serde")]
    pub biz_id: BizID,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoInfoRights {
    /// Is contracting allowed
    ///
    /// 是否允许承包
    pub bp: i32,
    /// Does it support charging
    ///
    /// 是否支持充电
    pub elec: i32,
    /// Allown download?
    ///
    /// 是否允许下载
    pub download: i32,
    /// Is moive?
    ///
    /// 是否是电影
    pub movie: i32,
    /// Is PGC pay?
    ///
    /// 是否PGC付费
    pub pay: i32,
    /// Has high bitrate?
    ///
    /// 是否有高码率
    pub hd5: i32,
    /// No reprint?
    ///
    /// 是否显示“禁止转载”标志
    pub no_reprint: i32,
    /// Auto play?
    ///
    /// 是否自动播放
    pub autoplay: i32,
    /// Is ugc pay?
    ///
    /// 是否UGC付费
    pub ugc_pay: i32,
    /// is cooperation?
    ///
    /// 是否为联合投稿
    pub is_cooperation: i32,
    /// 作用尚不明确
    pub ugc_pay_preview: i32,
    /// 作用尚不明确
    pub no_background: i32,
    /// 作用尚不明确
    pub clean_mode: i32,
    /// Is it an interactive video
    ///
    /// 是否为互动视频
    pub is_stein_gate: i32,
    /// Is it a panoramic video
    ///
    /// 是否为全景视频
    pub is_360: i32,
    /// 作用尚不明确
    pub no_share: i32,
    /// 作用尚不明确
    pub arc_pay: i32,
    /// 作用尚不明确
    pub free_watch: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoOwner {
    /// UP主mid
    pub mid: u64,
    /// UP主昵称
    pub name: String,
    /// UP主头像
    pub face: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoStatus {
    /// 稿件aid
    pub aid: u64,
    /// 播放数
    pub view: u64,
    /// 弹幕数
    pub danmaku: u32,
    /// 评论数
    pub reply: u32,
    /// 收藏数
    pub favorite: u32,
    /// 投币数
    pub coin: u32,
    /// 分享数
    pub share: u32,
    /// 当前排名
    pub now_rank: u16,
    /// 历史最高排名
    pub his_rank: u16,
    /// 获赞数
    pub like: u32,
    /// 点踩数（恒为0）
    pub dislike: u8,
    /// 视频评分
    pub evaluation: String,
    /// 作用尚不明确（恒为0）
    pub vt: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoArgueInfo {
    /// 作用尚不明确
    pub argue_link: String,
    /// 警告/争议提示信息
    pub argue_msg: String,
    /// 作用尚不明确
    pub argue_type: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoDimension {
    /// 当前分P 宽度
    pub width: u32,
    /// 当前分P 高度
    pub height: u32,
    /// 是否将宽高对换
    pub rotate: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoPageInfo {
    /// 分P cid
    pub cid: u64,
    /// 分P 序号
    pub page: u16,
    /// 视频来源
    #[serde(with = "video_from_serde")]
    pub from: VideoFrom,
    /// 分P 标题
    pub part: String,
    /// 分P 持续时间
    pub duration: f64,
    /// 站外视频vid
    pub vid: String,
    /// 站外视频跳转链接
    pub weblink: String,
    /// 当前分P 分辨率 （部分较老视频可能无分辨率值）
    pub dimension: Option<VideoDimension>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoSubtitleInfo {
    pub allow_submit: bool,
    pub list: Vec<VideoSubtitleList>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoSubtitleList {
    /// 字幕id
    pub id: u64,
    /// 字幕语言
    pub lan: String,
    /// 字幕语言名称
    pub lan_doc: String,
    /// 是否锁定
    pub is_lock: bool,
    /// 字幕上传者mid
    // pub author_mid: u64,
    /// json 格式字幕文件链接
    pub subtitle_url: String,
    /// 字幕上传者信息
    pub author: VideoSubtitleAuthor,
}

/// 字幕上传者信息
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoSubtitleAuthor {
    /// 字幕上传者mid
    pub mid: u64,
    /// 字幕上传者名称
    pub name: String,
    /// 字幕上传者性别
    #[serde(with = "crate::user::user_sex_serde")]
    pub sex: crate::user::UserSex,
    /// 字幕上传者头像
    pub face: String,
    /// 字幕上传者签名
    pub sign: String,
    /// 作用尚不明确
    pub rank: i32,
    /// 作用尚不明确
    pub birthday: i32,
    /// 作用尚不明确
    pub is_fake_account: i32,
    /// 作用尚不明确
    pub is_deleted: i32,
}

/// 视频合集信息
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoUgcSeasonInfo {
    /// 视频合集id
    pub id: u32,
    /// 视频合集标题
    pub title: String,
    /// 视频合集作者id
    pub mid: u64,
    /// 视频合集介绍
    pub intro: String,
    /// 作用尚不明确
    pub sign_state: i32,
    /// 稿件属性位
    pub attribute: i32,
    /// 视频合集中分部列表，名称可由UP主自定义，默认为正片
    pub sections: Vec<VideoUgcSeasonSection>,
    /// 视频合集状态数
    pub stat: VideoUgcSeasonStat,
    /// 视频合集中视频的数量
    pub ep_count: u16,
    /// 作用尚不明确
    pub season_type: i32,
    /// 是否为付费合集
    pub is_pay_season: bool,
    /// 作用尚不明确
    pub enable_vt: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoUgcSeasonSection {
    /// 视频合集中分部所属视频合集id
    pub season_id: u32,
    /// 视频合集中分部id
    pub section_id: Option<u32>,
    /// 视频合集中分部标题
    pub title: String,
    /// 作用尚不明确
    #[serde(rename = "type")]
    pub section_type: i32,
    /// 视频合集中分部的视频列表
    pub episodes: Vec<VideoUgcSeasonSectionEpisode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoUgcSeasonSectionEpisode {
    /// 视频合集中分部中视频所属视频合集id
    pub season_id: u32,
    /// 视频合集中视频合集分部中视频所属合集分部id
    pub section_id: u32,
    /// 视频合集分部中视频id（以下简称视频）
    pub id: u64,
    /// 视频aid
    pub aid: u64,
    /// 视频cid
    pub cid: u64,
    /// 视频标题
    pub title: String,
    /// # 已弃用
    ///
    /// 稿件属性位配置
    pub attribute: u16,
    // TODO: arc 未实现，先用 serde_json::Value 占位
    /// 视频详细信息
    pub arc: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoUgcSeasonStat {
    /// 视频合集id
    pub season_id: u64,
    /// 视频合集总浏览量
    pub view: u64,
    /// 视频合集总评论量
    pub danmaku: u64,
    /// 视频合集总收藏数
    pub fav: u64,
    /// 视频合集总投币数
    pub coin: u64,
    /// 视频合集总分享数
    pub share: u64,
    /// 视频合集当前排名
    pub now_rank: u16,
    /// 视频合集历史排名
    pub his_rank: u16,
    /// 视频合集总获赞数
    pub like: u64,
    /// 作用尚不明确
    pub vt: i32,
    /// 作用尚不明确
    pub vv: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoStaffInfo {
    /// 成员mid
    pub mid: u64,
    /// 成员名称
    pub title: String,
    /// 成员昵称
    pub name: String,
    /// 成员头像url
    pub face: String,
    /// 成员大会员状态
    pub vip: VideoStaffVip,
    /// 成员认证信息
    pub official: VideoStaffOfficial,
    /// 成员粉丝数
    pub follower: u64,
    pub label_style: i32,
}

/// 成员大会员状态
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoStaffVip {
    /// 成员会员类型
    #[serde(rename = "type")]
    pub vip_type: crate::user::VipType,
    /// 会员状态
    ///
    /// `false`: 无
    ///
    /// `true`: 有
    #[serde(with = "crate::util::serde_u8_2_bool")]
    pub status: bool,
    /// 到期时间 (Unix 毫秒时间戳)
    pub due_date: f64,
    pub vip_pay_type: i32,
    pub theme_type: i32,
    /// (这里我没办法了，文档里没写，只能用Value了)
    pub label: serde_json::Value,
}

/// 成员认证信息
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoStaffOfficial {
    /// 成员认证级别
    pub role: crate::user::OfficialRole,
    /// 成员认证名称
    pub title: String,
    /// 成员认证备注
    pub desc: String,
    /// 成员认证类型
    ///
    /// `false`: 无
    ///
    /// `true`: 有
    #[serde(rename = "type", with = "crate::util::serde_i8_2_bool")]
    pub official_type: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoUserGarb {
    /// 某url?
    pub url_image_ani_cut: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoHonorReply {
    pub honor: Option<Vec<VideoHonor>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoHonor {
    /// 当前稿件aid
    pub aid: u64,
    /// 类型
    #[serde(rename = "type")]
    pub honor_type: VideoHonorType,
    /// 描述
    pub desc: String,
    pub weekly_recommend_num: i32,
}

/// 类型
///
/// 来自 [获取视频详细信息(web端)](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/info.html#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E8%AF%A6%E7%BB%86%E4%BF%A1%E6%81%AF-web%E7%AB%AF)
///
/// 位置：`data` > `honor_reply` > `honor` > `type`
#[derive(Debug, Clone, Copy, SummonFrom, MakeSerde)]
#[repr(u8)]
#[summon(type=u8)]
#[make_serde(type=u8)]
pub enum VideoHonorType {
    /// 入站必刷收录
    MandatoryRefreshOnEntry = 1,
    /// 第?期每周必看
    WeeklyMustWatch = 2,
    /// 全站排行榜最高第?名
    PeakGlobalRanking = 3,
    /// 热门
    Trending = 4,
    /// 未知
    #[other]
    Unknown(u8)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Copyright {
    Original,
    Reprint,
}

impl From<Copyright> for u8 {
    fn from(value: Copyright) -> Self {
        match value {
            Copyright::Original => 1,
            Copyright::Reprint => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DescV2Type {
    Normal,
    At,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BizID {
    Mid(u64),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VideoFrom {
    Bilibili,
    MangoTV,
    TencentVideo,
    Other(String),
}

impl From<&str> for VideoFrom {
    fn from(value: &str) -> Self {
        match value {
            "vupload" => Self::Bilibili,
            "hunan" => Self::MangoTV,
            "qq" => Self::TencentVideo,
            _ => Self::Other(value.to_string()),
        }
    }
}

impl From<&VideoFrom> for String {
    fn from(value: &VideoFrom) -> Self {
        match value {
            VideoFrom::Bilibili => "vupload".to_string(),
            VideoFrom::MangoTV => "hunan".to_string(),
            VideoFrom::TencentVideo => "qq".to_string(),
            VideoFrom::Other(from) => from.clone(),
        }
    }
}

mod desc_v2_type {
    use super::DescV2Type;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DescV2Type, D::Error>
    where
        D: Deserializer<'de>,
    {
        let num = u8::deserialize(deserializer)?;
        match num {
            1 => Ok(DescV2Type::Normal),
            2 => Ok(DescV2Type::At),
            _ => Err(serde::de::Error::custom(format!(
                "unknown copyright value: {}",
                num
            ))),
        }
    }

    pub fn serialize<S>(desc_v2_type: &DescV2Type, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let num = match desc_v2_type {
            DescV2Type::Normal => 1,
            DescV2Type::At => 2,
        };
        serializer.serialize_u8(num)
    }
}

mod copyright_serde {
    use super::Copyright;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Copyright, D::Error>
    where
        D: Deserializer<'de>,
    {
        let num = u8::deserialize(deserializer)?;
        match num {
            1 => Ok(Copyright::Original),
            2 => Ok(Copyright::Reprint),
            _ => Err(serde::de::Error::custom(format!(
                "unknown copyright value: {}",
                num
            ))),
        }
    }

    // 如果需要序列化回去也可以实现
    pub fn serialize<S>(copyright: &Copyright, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let num = match copyright {
            Copyright::Original => 1,
            Copyright::Reprint => 2,
        };
        serializer.serialize_u8(num)
    }
}

mod biz_id_serde {
    use super::BizID;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BizID, D::Error>
    where
        D: Deserializer<'de>,
    {
        let num = u64::deserialize(deserializer)?;
        match num {
            0 => Ok(BizID::None),
            _ => Ok(BizID::Mid(num)),
        }
    }

    pub fn serialize<S>(biz_id: &BizID, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let num = match biz_id {
            BizID::Mid(mid) => *mid,
            BizID::None => 0,
        };
        serializer.serialize_u64(num)
    }
}

mod video_from_serde {
    use super::VideoFrom;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<VideoFrom, D::Error>
    where
        D: Deserializer<'de>,
    {
        let from = String::deserialize(deserializer)?;
        Ok(from.as_str().into())
    }

    pub fn serialize<S>(from: &VideoFrom, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let from = String::from(from);
        serializer.serialize_str(&from)
    }
}

/// 根据视频ID获取信息
pub async fn get_video_info(
    client: &reqwest::Client,
    id: &super::VideoID,
) -> APIResult<VideoInfoData> {
    let response = client
        .get("https://api.bilibili.com/x/web-interface/view")
        .query(&id.to_query())
        .send()
        .await?;
    let json = response.text().await?;
    let data: APIResponse<VideoInfoData> = serde_json::from_str(&json)?;
    data.into_result()
}
