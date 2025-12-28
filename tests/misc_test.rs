use bili_api::APIResponse;
use bili_api::misc::get_buvid::BuvidData;

#[test]
fn test_deserialize_buvid_response() {
    let json = r#"
    {
        "code": 0,
        "message": "0",
        "ttl": 1,
        "data": {
            "b_3": "buvid3_value",
            "b_4": "buvid4_value",
            "b_nut": 123456789
        }
    }
    "#;

    let resp: APIResponse<BuvidData> = serde_json::from_str(json).unwrap();
    assert!(resp.is_success());
    let data = resp.data.unwrap();
    assert_eq!(data.b_3, "buvid3_value");
    assert_eq!(data.b_4, "buvid4_value");
    assert_eq!(data.b_nut, 123456789);
}
