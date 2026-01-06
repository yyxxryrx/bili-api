use crate::{APIResponse, error::APIResult, make_serde, sign_and_auth::wbi::WbiSign};
use make_serde::{MakeSerde, SummonFrom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NavData {
    /// 是否已登录
    #[serde(rename = "isLogin")]
    pub is_login: bool,
    // TODO: 这里先用 u8 占位，到时候换 Enum
    /// 是否验证邮箱地址
    ///
    /// 0: 未验证
    ///
    /// 1: 已验证
    pub email_verified: u8,
    /// 用户头像 url
    pub face: String,
    // pub face_nft: i64,
    // pub face_nft_type: i64,
    /// 等级信息
    pub level_info: LevelInfo,
    /// 用户 mid
    pub mid: i64,
    /// 是否验证手机号
    pub mobile_verified: u64,
    /// 拥有硬币数量
    pub money: f64,
    /// 当前节操值
    pub moral: i64,
    /// 认证信息
    pub official: Official,
    /// 认证信息 2
    #[serde(rename = "officialVerify")]
    pub official_verify: OfficialVerify,
    /// 头像框信息
    pub pendant: Pendant,
    /// (?)
    pub scores: i64,
    /// 用户昵称
    pub uname: String,
    /// 会员到期时间
    #[serde(rename = "vipDueDate")]
    pub vip_due_date: u64,
    #[serde(rename = "vipStatus")]
    // TODO: 先用 u8 占位，之后换成 Enum
    /// 会员开通状态
    pub vip_status: u8,
    // TODO: 同上
    /// 会员类型
    #[serde(rename = "vipType")]
    pub vip_type: u8,
    // TODO: 同上
    /// 会员开通状态
    pub vip_pay_type: u8,
    /// (?)
    pub vip_theme_type: i64,
    /// 会员标签
    pub vip_label: Label,
    /// 是否显示会员图标
    pub vip_avatar_subscript: i64,
    /// 会员昵称颜色
    pub vip_nickname_color: String,
    /// 会员信息
    pub vip: Option<Vip>,
    /// B币钱包信息
    pub wallet: Wallet,
    /// 是否拥有推广
    pub has_shop: bool,
    /// 商品推广页面
    pub shop_url: String,
    /// (?)
    pub allowance_count: i64,
    /// (?)
    pub answer_status: i64,
    /// 是否硬核会员
    pub is_senior_member: i64,
    /// Wbi 签名实时口令
    pub wbi_img: WbiImg,
    /// 是否风纪委员
    pub is_jury: bool,
}

/// 等级信息
#[derive(Debug, Serialize, Deserialize)]
pub struct LevelInfo {
    /// 当前等级
    pub current_level: u8,
    /// 当前等级经验最低值
    pub current_min: u32,
    /// 当前经验
    pub current_exp: u32,
    /// 升到下一等级需达到的经验
    #[serde(with = "serde_next_exp_info")]
    pub next_exp: NextExpInfo,
}

/// 升到下一等级需达到的经验
#[derive(Debug, Clone)]
pub enum NextExpInfo {
    Inf,
    Value(u32),
    Other(String),
}

impl From<&str> for NextExpInfo {
    fn from(value: &str) -> Self {
        if value == "--" {
            Self::Inf
        } else {
            match value.parse::<u32>() {
                Ok(val) => Self::Value(val),
                Err(..) => Self::Other(value.to_string()),
            }
        }
    }
}

impl From<&NextExpInfo> for String {
    fn from(value: &NextExpInfo) -> Self {
        match value {
            NextExpInfo::Inf => "--".to_string(),
            NextExpInfo::Value(val) => val.to_string(),
            NextExpInfo::Other(val) => val.clone(),
        }
    }
}

make_serde! {
    pub mod serde_next_exp_info(NextExpInfo) {
        (dse) => {
            String::deserialize(dse).map(|s| NextExpInfo::from(s.as_str()))
        },
        (info, ser) => {
            ser.serialize_str(String::from(info).as_str())
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Official {
    /// 认证类型
    pub role: crate::user::OfficialRole,
    /// 认证信息（无为空）
    pub title: String,
    /// 认证备注（无为空）
    pub desc: String,
    /// 是否认证
    #[serde(rename = "type")]
    pub official_type: OfficialType,
}

/// 是否认证
#[derive(Debug, Clone, Copy, MakeSerde, SummonFrom)]
#[summon(type=i8)]
#[make_serde(type=i8,try)]
pub enum OfficialType {
    /// 已认证
    Verified = 0,
    /// 无
    None = -1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfficialVerify {
    /// 是否认证
    #[serde(rename = "type")]
    pub official_verify_type: OfficialType,
    /// 认证信息（无为空）
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pendant {
    /// 挂件id
    pub pid: i64,
    /// 挂件名称
    pub name: String,
    /// 挂件图片url
    pub image: String,
    /// (?)
    pub expire: i64,
    // pub image_enhance: String,
    // pub image_enhance_frame: String,
}

/// 会员信息
///
/// **注意：这个字段文档里没有，所以没写注释和文档了**
/// **详情请看 [导航栏用户信息](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/login_info.html#%E5%AF%BC%E8%88%AA%E6%A0%8F%E7%94%A8%E6%88%B7%E4%BF%A1%E6%81%AF)**
#[derive(Debug, Serialize, Deserialize)]
pub struct Vip {
    /// 会员类型
    #[serde(rename = "type")]
    pub vip_type: i64,
    /// 会员开通状态
    pub status: i64,
    pub due_date: i64,
    pub vip_pay_type: i64,
    pub theme_type: i64,
    pub label: Label,
    pub avatar_subscript: i64,
    pub nickname_color: String,
    pub role: i64,
    pub avatar_subscript_url: String,
    pub tv_vip_status: i64,
    pub tv_vip_pay_type: i64,
    pub tv_due_date: i64,
}

///
#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    /// (?)
    pub path: String,
    /// 会员名称
    pub text: String,
    /// 会员标签
    pub label_theme: String,
    // pub text_color: String,
    // pub bg_style: i64,
    // pub bg_color: String,
    // pub border_color: String,
    // pub use_img_label: bool,
    // pub img_label_uri_hans: String,
    // pub img_label_uri_hant: String,
    // pub img_label_uri_hans_static: String,
    // pub img_label_uri_hant_static: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    /// 登录用户mid
    pub mid: u64,
    /// 拥有B币数量
    pub bcoin_balance: u64,
    /// 每月奖励B币数
    pub coupon_balance: u32,
    /// (?)
    pub coupon_due_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WbiImg {
    /// Wbi 签名的参数 `imgKey` 的伪装 url
    pub img_url: String,
    /// Wbi 签名参数 `subKey` 的伪装 url
    pub sub_url: String,
}

impl NavData {
    pub fn create_wbi(&self) -> WbiSign {
        WbiSign::new(self.wbi_img.img_url.as_str(), self.wbi_img.sub_url.as_str())
    }
}

/// 导航栏用户信息
pub async fn nav(client: &reqwest::Client) -> APIResult<NavData> {
    client
        .get("https://api.bilibili.com/x/web-interface/nav")
        .send()
        .await?
        .json::<APIResponse<NavData>>()
        .await?
        .into_result()
}
