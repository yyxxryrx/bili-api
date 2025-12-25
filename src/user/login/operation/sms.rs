pub mod web {
    use serde::{Deserialize, Serialize};

    use crate::{APIResponse, error::APIResult, util::international::InternationalDialingPrefix};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct SMSReqArgs {
        /// 国际冠字码
        #[serde(with = "crate::util::international::serde_international_dialing_prefix")]
        pub cid: InternationalDialingPrefix,
        /// 手机号码
        pub tel: u64,
        /// 登录来源
        pub source: String,
        /// 登录 API token
        pub token: String,
        /// 极验 challenge
        pub challenge: String,
        /// 极验 result
        pub validate: String,
        /// 极验 result + `|jordan`
        pub seccode: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct SMSRespData {
        /// 短信登录 token
        ///
        /// 在下方传参时需要，请备用
        pub captcha_key: String,
    }

    pub struct LoginSMS {
        pub captcha_key: String,
        pub created_time: std::time::Instant,
    }

    pub async fn send_sms_code(
        client: &reqwest::Client,
        args: &SMSReqArgs,
    ) -> APIResult<SMSRespData> {
        let response = client
            .post("https://passport.bilibili.com/x/passport-login/web/sms/send")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .json(args)
            .send()
            .await?;
        let json = response.json::<APIResponse<SMSRespData>>().await?;
        json.into_result()
    }

    pub async fn get_login_sms(client: &reqwest::Client, args: &SMSReqArgs) -> APIResult<LoginSMS> {
        let data = send_sms_code(client, args).await?;
        Ok(LoginSMS {
            captcha_key: data.captcha_key,
            created_time: std::time::Instant::now(),
        })
    }
}
