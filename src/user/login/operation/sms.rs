/// Web 端 API
pub mod web {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    use crate::{
        APIResponse, error::APIResult, user::login::LoginSource,
        util::international::InternationalDialingPrefix,
    };

    /// 发送短信验证码 API 参数
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SMSReqArgs {
        /// 国际冠字码
        #[serde(with = "crate::util::international::serde_international_dialing_prefix")]
        pub cid: InternationalDialingPrefix,
        /// 手机号码
        pub tel: u64,
        /// 登录来源
        #[serde(with = "crate::user::login::serde_login_source")]
        pub source: LoginSource,
        /// 登录 API token
        pub token: String,
        /// 极验 challenge
        pub challenge: String,
        /// 极验 result
        pub validate: String,
        /// 极验 result + `|jordan`
        pub seccode: String,
    }

    /// 使用短信验证码登录 API 参数
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SMSLoginArgs {
        /// 国际冠字码
        #[serde(with = "crate::util::international::serde_international_dialing_prefix")]
        pub cid: InternationalDialingPrefix,
        /// 手机号码
        pub tel: u64,
        /// 短信验证码
        pub code: u32,
        /// 登录来源
        #[serde(with = "crate::user::login::serde_login_source")]
        pub source: LoginSource,
        /// 短信登录 token
        pub captcha_key: String,
        /// 跳转 url (非必要)
        pub go_url: Option<String>,
        /// 是否记住登录 (非必要)
        pub keep: Option<bool>,
    }

    /// 发送短信验证码 API 返回数据
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SMSRespData {
        /// 短信登录 token
        ///
        /// 在下方传参时需要，请备用
        pub captcha_key: String,
    }

    /// 使用短信验证码登录 API 返回数据
    #[derive(Debug, Deserialize, Serialize)]
    pub struct SMSLoginData {
        /// 是否为新注册用户
        pub is_new: bool,
        /// 未知，可能0就是成功吧
        pub status: i32,
        /// 跳转 url，默认为 https://www.bilibili.com
        pub url: String,
    }

    /// 工具类，帮你传递重复参数
    pub struct LoginSMS {
        pub tel: u64,
        pub captcha_key: String,
        pub login_source: LoginSource,
        pub cid: InternationalDialingPrefix,
        pub created_time: std::time::Instant,
    }

    impl LoginSMS {
        /// 使用短信验证码登录
        pub async fn login(
            &self,
            client: &reqwest::Client,
            code: u32,
        ) -> APIResult<(HashMap<String, String>, SMSLoginData)> {
            if self.created_time.elapsed().as_secs() > 300 {
                return Err(crate::error::Error::Timeout);
            }
            let args = SMSLoginArgs {
                code,
                keep: None,
                go_url: None,
                cid: self.cid,
                tel: self.tel,
                source: self.login_source,
                captcha_key: self.captcha_key.clone(),
            };
            sms_login(client, &args).await
        }
    }

    /// 发送短信验证码
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

    /// 使用短信验证码登录
    pub async fn sms_login(
        client: &reqwest::Client,
        args: &SMSLoginArgs,
    ) -> APIResult<(HashMap<String, String>, SMSLoginData)> {
        let response = client
            .post("https://passport.bilibili.com/x/passport-login/web/login/sms")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .json(args)
            .send()
            .await?;
        let cookies = response.cookies().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c.name().to_string())
                .insert_entry(c.value().to_string());
            acc
        });
        let json = response.json::<APIResponse<SMSLoginData>>().await?;
        json.into_result().map(|v| (cookies, v))
    }

    /// 创建 `LoginSMS`
    pub async fn get_login_sms(client: &reqwest::Client, args: &SMSReqArgs) -> APIResult<LoginSMS> {
        let data = send_sms_code(client, args).await?;
        Ok(LoginSMS {
            cid: args.cid,
            tel: args.tel,
            login_source: args.source,
            captcha_key: data.captcha_key,
            created_time: std::time::Instant::now(),
        })
    }
}
