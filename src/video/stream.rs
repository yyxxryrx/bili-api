use crate::video::VideoID;
use crate::{APIResponse, error::APIResult, make_headers, make_serde, sign_and_auth::wbi::WbiSign};
use make_serde::{MakeSerde, SummonFrom};
use serde::{Deserialize, Serialize};

/// Video clarity
///
/// 视频清晰度
#[derive(Debug, Clone, Copy, PartialEq, Eq, MakeSerde)]
#[make_serde(try)]
pub enum VideoQuality {
    /// 240P 极速
    P240 = 6,
    /// 360P 流畅
    P360 = 16,
    /// 480P 清晰
    P480 = 32,
    /// 720P 高清
    P720 = 64,
    /// 720P60 高帧率
    P720_60 = 74,
    /// 1080P 高清
    P1080 = 80,
    /// 1080P+ 高码率
    P1080Plus = 112,
    /// 1080P60 高帧率
    P1080_60 = 116,
    /// 4K 超清
    P4K = 120,
    /// HDR 真彩
    HDR = 125,
    /// 杜比视界
    Dolby = 126,
    /// 8K 超高清
    P8K = 127,
}

impl TryFrom<u32> for VideoQuality {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(Self::P240),
            16 => Ok(Self::P360),
            32 => Ok(Self::P480),
            64 => Ok(Self::P720),
            74 => Ok(Self::P720_60),
            80 => Ok(Self::P1080),
            112 => Ok(Self::P1080Plus),
            116 => Ok(Self::P1080_60),
            120 => Ok(Self::P4K),
            125 => Ok(Self::HDR),
            126 => Ok(Self::Dolby),
            127 => Ok(Self::P8K),
            _ => Err(format!("Unknown video quality: {}", value)),
        }
    }
}

impl From<&VideoQuality> for u32 {
    fn from(val: &VideoQuality) -> Self {
        *val as u32
    }
}

/// Video stream format
///
/// 视频流格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, MakeSerde, SummonFrom)]
#[make_serde(try,type=u32)]
pub enum VideoFnval {
    /// FLV / MP4
    Flv = 0,
    /// MP4
    Mp4 = 1,
    /// DASH
    Dash = 16,
    /// HDR
    Hdr = 64,
    /// 4K
    FourK = 128,
    /// Dolby Audio
    DolbyAudio = 256,
    /// Dolby Vision
    DolbyVision = 512,
    /// 8K
    EightK = 1024,
    /// AV1
    Av1 = 2048,
}

impl Default for VideoFnval {
    fn default() -> Self {
        Self::Mp4
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VideoPlatform {
    PC,
    Html5,
}

impl Default for VideoPlatform {
    fn default() -> Self {
        Self::PC
    }
}

impl TryFrom<String> for VideoPlatform {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "pc" => Ok(Self::PC),
            "html5" => Ok(Self::Html5),
            _ => Err(value),
        }
    }
}

impl From<&VideoPlatform> for &str {
    fn from(value: &VideoPlatform) -> Self {
        match value {
            VideoPlatform::PC => "pc",
            VideoPlatform::Html5 => "html5",
        }
    }
}

make_serde! {
    pub mod serde_video_platform(VideoPlatform) {
        (deserializer) => {
            String::deserialize(deserializer)?.try_into().map_err(serde::de::Error::custom)
        },
        (platform, serializer) => {
           serializer.serialize_str(platform.into())
        },
    }
}

/// Video stream data
///
/// 视频流数据
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoStreamData {
    /// The quality of the video
    ///
    /// 视频清晰度
    pub quality: VideoQuality,
    /// The format of the video
    ///
    /// 视频格式
    pub format: String,
    /// The length of the video (ms)
    ///
    /// 视频时长（毫秒）
    pub timelength: u64,
    /// Supported formats
    ///
    /// 支持的格式列表
    pub accept_format: String,
    /// Description of supported formats
    ///
    /// 支持的格式描述
    pub accept_description: Vec<String>,
    /// Quality of supported formats
    ///
    /// 支持的清晰度列表
    pub accept_quality: Vec<VideoQuality>,
    /// Video codec ID
    ///
    /// 视频编码ID
    pub video_codecid: u32,
    /// Durl (for FLV/MP4)
    ///
    /// FLV/MP4 播放地址
    pub durl: Option<Vec<Durl>>,
    /// DASH (for DASH)
    ///
    /// DASH 播放信息
    pub dash: Option<Dash>,
    /// Support formats details
    ///
    /// 支持的格式详情
    pub support_formats: Vec<SupportFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoStreamDataResp {
    #[serde(default)]
    pub v_voucher: Option<String>,
    pub quality: Option<VideoQuality>,
    pub format: Option<String>,
    pub timelength: Option<u64>,
    pub accept_format: Option<String>,
    pub accept_description: Option<Vec<String>>,
    pub accept_quality: Option<Vec<VideoQuality>>,
    pub video_codecid: Option<u32>,
    pub durl: Option<Vec<Durl>>,
    pub dash: Option<Dash>,
    pub support_formats: Option<Vec<SupportFormat>>,
}

impl VideoStreamDataResp {
    pub fn into_result(self) -> APIResult<VideoStreamData> {
        match self.v_voucher {
            Some(..) => Err(crate::error::Error::Other("need arg gaia_source=view-card".to_string())),
            None => Ok(VideoStreamData {
                quality: self.quality.unwrap(),
                format: self.format.unwrap(),
                timelength: self.timelength.unwrap(),
                accept_format: self.accept_format.unwrap(),
                accept_description: self.accept_description.unwrap(),
                accept_quality: self.accept_quality.unwrap(),
                video_codecid: self.video_codecid.unwrap(),
                durl: self.durl,
                dash: self.dash,
                support_formats: self.support_formats.unwrap(),
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Durl {
    /// 视频分段序号
    ///
    /// 某些视频会分为多个片段（从1顺序增长）
    pub order: u32,
    /// 视频长度
    ///
    /// 单位为毫秒
    pub length: u64,
    /// 视频大小
    ///
    /// 单位为 Byte
    pub size: u64,
    /// 默认流 URL
    ///
    /// **注意 unicode 转义符**
    ///
    /// 有效时间为120min
    pub url: String,
    /// 备用视频流
    ///
    /// **注意 unicode 转义符**
    ///
    /// 有效时间为120min
    pub backup_url: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dash {
    pub duration: u64,
    pub min_buffer_time: f64,
    pub video: Vec<DashMedia>,
    pub audio: Option<Vec<DashMedia>>,
    pub dolby: Option<DashDolby>,
    pub flac: Option<DashFlac>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashMedia {
    pub id: u32,
    pub base_url: String,
    pub backup_url: Option<Vec<String>>,
    pub bandwidth: u64,
    pub mime_type: String,
    pub codecs: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<String>,
    pub sar: Option<String>,
    pub start_with_sap: Option<u32>,
    pub segment_base: Option<DashSegmentBase>,
    pub codecid: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashSegmentBase {
    pub initialization: String,
    pub index_range: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashDolby {
    pub r#type: u32,
    pub audio: Option<Vec<DashMedia>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashFlac {
    pub display: bool,
    pub audio: Option<DashMedia>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportFormat {
    pub quality: VideoQuality,
    pub format: String,
    pub new_description: String,
    pub display_desc: String,
    pub superscript: String,
    pub codecs: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VideoStreamArgs {
    /// view-card
    ///
    /// 无 Cookie(SESSDATA) 时需要此参数
    /// 有 Cookie(SESSDATA) 时不需要
    pub gaia_source: Option<String>,
    /// 视频清晰度选择
    ///
    /// 未登录默认 [`P480`](VideoQuality::P480)（480P），登录后默认 [`P720`](VideoQuality::P720)（720P）
    ///
    /// **DASH 格式时无效**
    pub qn: Option<VideoQuality>,
    /// 视频流格式标识
    pub fanval: VideoFnval,
    /// 0
    pub fnver: i32,
    /// 是否允许 4K 视频
    ///
    /// 画质最高 1080P：0（默认）
    ///
    /// 画质最高 4K：1
    pub fourk: u8,
    /// 从视频播放页的 HTML 中设置 window.\_\_playinfo\_\_ 处获取，
    /// 或者通过 buvid3 + 当前UNIX毫秒级时间戳 经过md5获取
    pub session: Option<String>,
    /// 固定为json
    pub otype: Option<String>,
    /// 目前为空
    pub r#type: Option<String>,
    /// pc：web播放（默认值，视频流存在 referer鉴权）
    ///
    /// html5：移动端 HTML5 播放（仅支持 MP4 格式，无 referer 鉴权可以直接使用video标签播放）
    #[serde(with = "serde_video_platform")]
    pub platform: VideoPlatform,
    /// 是否高画质
    ///
    /// platform=html5时，此值为1可使画质为1080p
    pub high_quality: u8,
    /// 未登录高画质
    ///
    /// 为 1 时可以不登录拉到 64 和 80 清晰度
    pub try_look: u8,
}

impl VideoStreamArgs {
    pub fn to_query(&self) -> [(&str, Option<String>); 11] {
        [
            ("gaia_source", self.gaia_source.clone()),
            ("qn", self.qn.map(|q| u32::from(&q).to_string())),
            ("fnval", Some(u32::from(&self.fanval).to_string())),
            ("fnver", Some(self.fnver.to_string())),
            ("fourk", Some(self.fourk.to_string())),
            ("session", self.session.clone()),
            ("otype", self.otype.clone()),
            ("type", self.r#type.clone()),
            (
                "platform",
                Some(<&VideoPlatform as Into<&str>>::into(&self.platform).to_string()),
            ),
            ("high_quality", Some(self.high_quality.to_string())),
            ("try_look", Some(self.try_look.to_string())),
        ]
    }
}

/// Get video stream
///
/// 获取视频流
pub async fn get_video_stream(
    client: &reqwest::Client,
    id: VideoID,
    cid: u64,
    args: &VideoStreamArgs,
    wbi_sign: Option<&WbiSign>,
) -> APIResult<VideoStreamData> {
    let mut params = id.to_query().to_vec();
    params.push(("cid", Some(cid.to_string())));
    params.append(&mut args.to_query().to_vec());
    if let Some(wbi) = wbi_sign {
        wbi.sign_option(&mut params)?;
    }
    let resp = client
        .get("https://api.bilibili.com/x/player/wbi/playurl")
        .headers(make_headers!())
        .query(&params)
        .send()
        .await?
        .json::<APIResponse<VideoStreamDataResp>>()
        .await?;

    resp.into_result().and_then(|r| r.into_result())
}
