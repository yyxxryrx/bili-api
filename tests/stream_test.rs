use bili_api::video::stream::{VideoStreamData, VideoQuality};
use bili_api::APIResponse;

#[test]
fn test_deserialize_stream_response() {
    let json = r#"
    {
        "code": 0,
        "message": "0",
        "ttl": 1,
        "data": {
            "from": "local",
            "result": "suee",
            "message": "",
            "quality": 80,
            "format": "mp4",
            "timelength": 1000,
            "accept_format": "hdflv2,flv,flv720,flv480,mp4",
            "accept_description": ["高清 1080P+", "高清 1080P", "高清 720P", "清晰 480P", "流畅 360P"],
            "accept_quality": [112, 80, 64, 32, 16],
            "video_codecid": 7,
            "seek_param": "start",
            "seek_type": "offset",
            "durl": [
                {
                    "order": 1,
                    "length": 1000,
                    "size": 102400,
                    "url": "http://example.com/video.mp4",
                    "backup_url": ["http://backup.example.com/video.mp4"]
                }
            ],
            "support_formats": [
                {
                    "quality": 80,
                    "format": "mp4",
                    "new_description": "1080P",
                    "display_desc": "1080P",
                    "superscript": "",
                    "codecs": ["avc1.640032"]
                }
            ]
        }
    }
    "#;

    let resp: APIResponse<VideoStreamData> = serde_json::from_str(json).unwrap();
    assert!(resp.is_success());
    let data = resp.data.unwrap();
    assert_eq!(data.quality, VideoQuality::P1080);
    assert_eq!(data.format, "mp4");
    assert_eq!(data.accept_quality.len(), 5);
    assert!(data.durl.is_some());
    let durl = data.durl.as_ref().unwrap();
    assert_eq!(durl.len(), 1);
    assert_eq!(durl[0].url, "http://example.com/video.mp4");
}
