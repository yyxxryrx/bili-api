use crate::{APIResponse, error::APIResult, get_client, make_headers};
use serde::{Deserialize, Serialize};

/// Buvid data
///
/// Buvid 数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuvidData {
    /// buvid3
    pub b_3: String,
    /// buvid4
    pub b_4: String,
    /// b_nut
    #[serde(default)]
    pub b_nut: i64,
}

/// Get buvid3 / buvid4 / b_nut
///
/// 获取 buvid3 / buvid4 / b_nut
pub async fn get_buvid() -> APIResult<BuvidData> {
    let client = get_client!()?;
    let resp = client
        .get("https://api.bilibili.com/x/frontend/finger/spi")
        .headers(make_headers!())
        .send()
        .await?
        .json::<APIResponse<BuvidData>>()
        .await?;
    resp.into_result()
}
