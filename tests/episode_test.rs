use bili_api::APIResponse;
use bili_api::episode::info::SeasonInfoData;

#[test]
fn test_deserialize_season_info() {
    let json = r###"
{
  "code": 0,
  "message": "success",
  "result": {
    "activity": {
      "head_bg_url": "",
      "id": 0,
      "title": ""
    },
    "actors": "直枝 理树：堀江由衣\n笹濑川 佐佐美：徳井青空\n二木佳奈多：铃木惠子\n朱鹭戸沙耶：櫻井浩美\n能美 库特莉亚芙卡：若林直美\n三枝 叶留佳：铃木惠子\n神北 小毬：やなせなつみ\n枣 恭介：緑川光\n枣铃：..",
    "alias": "",
    "areas": [
      {
        "id": 2,
        "name": "日本"
      }
    ],
    "bkg_cover": "",
    "cover": "http://i0.hdslb.com/bfs/bangumi/18e4ebae9e5a2395aef71978d4a4184947581b82.png",
    "delivery_fragment_video": false,
    "enable_vt": false,
    "episodes": [
      {
        "aid": 31240728,
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "bvid": "BV1hW411d7UN",
        "cid": 54584652,
        "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
        "dimension": {
          "height": 0,
          "rotate": 0,
          "width": 0
        },
        "duration": 1427000,
        "enable_vt": false,
        "ep_id": 247669,
        "from": "bangumi",
        "id": 247669,
        "is_view_hide": false,
        "link": "https://www.bilibili.com/bangumi/play/ep247669",
        "long_title": "",
        "pub_time": 1536287160,
        "pv": 0,
        "release_date": "",
        "rights": {
          "allow_dm": 1,
          "allow_download": 1,
          "area_limit": 0,
          "cache_auth": 0
        },
        "section_type": 0,
        "share_copy": "《Little Busters! EX》第1话 ",
        "share_url": "https://www.bilibili.com/bangumi/play/ep247669",
        "short_link": "https://b23.tv/ep247669",
        "showDrmLoginDialog": false,
        "show_title": "1",
        "skip": {
          "ed": {
            "end": 1410,
            "start": 1320
          },
          "op": {
            "end": 268,
            "start": 178
          }
        },
        "status": 2,
        "subtitle": "已观看484.9万次",
        "title": "1",
        "vid": ""
      },
      {
        "aid": 719357189,
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "bvid": "BV1qQ4y1v7hQ",
        "cid": 54584699,
        "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
        "dimension": {
          "height": 0,
          "rotate": 0,
          "width": 0
        },
        "duration": 1427000,
        "enable_vt": false,
        "ep_id": 247670,
        "from": "bangumi",
        "id": 247670,
        "is_view_hide": false,
        "link": "https://www.bilibili.com/bangumi/play/ep247670",
        "long_title": "",
        "pub_time": 1536287160,
        "pv": 0,
        "release_date": "",
        "rights": {
          "allow_dm": 1,
          "allow_download": 1,
          "area_limit": 0,
          "cache_auth": 0
        },
        "section_type": 0,
        "share_copy": "《Little Busters! EX》第2话 ",
        "share_url": "https://www.bilibili.com/bangumi/play/ep247670",
        "short_link": "https://b23.tv/ep247670",
        "showDrmLoginDialog": false,
        "show_title": "2",
        "skip": {
          "ed": {
            "end": 1410,
            "start": 1320
          },
          "op": {
            "end": 126,
            "start": 36
          }
        },
        "status": 2,
        "subtitle": "已观看484.9万次",
        "title": "2",
        "vid": ""
      },
      {
        "aid": 336781705,
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "bvid": "BV1QR4y1t7gh",
        "cid": 54584746,
        "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
        "dimension": {
          "height": 0,
          "rotate": 0,
          "width": 0
        },
        "duration": 1427000,
        "enable_vt": false,
        "ep_id": 247671,
        "from": "bangumi",
        "id": 247671,
        "is_view_hide": false,
        "link": "https://www.bilibili.com/bangumi/play/ep247671",
        "long_title": "",
        "pub_time": 1536287160,
        "pv": 0,
        "release_date": "",
        "rights": {
          "allow_dm": 1,
          "allow_download": 1,
          "area_limit": 0,
          "cache_auth": 0
        },
        "section_type": 0,
        "share_copy": "《Little Busters! EX》第3话 ",
        "share_url": "https://www.bilibili.com/bangumi/play/ep247671",
        "short_link": "https://b23.tv/ep247671",
        "showDrmLoginDialog": false,
        "show_title": "3",
        "skip": {
          "ed": {
            "end": 1410,
            "start": 1320
          },
          "op": {
            "end": 90,
            "start": 0
          }
        },
        "status": 2,
        "subtitle": "已观看484.9万次",
        "title": "3",
        "vid": ""
      },
      {
        "aid": 891818940,
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "bvid": "BV1rP4y1G7cV",
        "cid": 54584784,
        "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
        "dimension": {
          "height": 0,
          "rotate": 0,
          "width": 0
        },
        "duration": 1427000,
        "enable_vt": false,
        "ep_id": 247672,
        "from": "bangumi",
        "id": 247672,
        "is_view_hide": false,
        "link": "https://www.bilibili.com/bangumi/play/ep247672",
        "long_title": "",
        "pub_time": 1536287160,
        "pv": 0,
        "release_date": "",
        "rights": {
          "allow_dm": 1,
          "allow_download": 1,
          "area_limit": 0,
          "cache_auth": 0
        },
        "section_type": 0,
        "share_copy": "《Little Busters! EX》第4话 ",
        "share_url": "https://www.bilibili.com/bangumi/play/ep247672",
        "short_link": "https://b23.tv/ep247672",
        "showDrmLoginDialog": false,
        "show_title": "4",
        "skip": {
          "ed": {
            "end": 1410,
            "start": 1320
          },
          "op": {
            "end": 289,
            "start": 199
          }
        },
        "status": 2,
        "subtitle": "已观看484.9万次",
        "title": "4",
        "vid": ""
      },
      {
        "aid": 209311296,
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "bvid": "BV1Lh41147ox",
        "cid": 54584866,
        "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
        "dimension": {
          "height": 0,
          "rotate": 0,
          "width": 0
        },
        "duration": 1427000,
        "enable_vt": false,
        "ep_id": 247673,
        "from": "bangumi",
        "id": 247673,
        "is_view_hide": false,
        "link": "https://www.bilibili.com/bangumi/play/ep247673",
        "long_title": "",
        "pub_time": 1536287160,
        "pv": 0,
        "release_date": "",
        "rights": {
          "allow_dm": 1,
          "allow_download": 1,
          "area_limit": 0,
          "cache_auth": 0
        },
        "section_type": 0,
        "share_copy": "《Little Busters! EX》第5话 ",
        "share_url": "https://www.bilibili.com/bangumi/play/ep247673",
        "short_link": "https://b23.tv/ep247673",
        "showDrmLoginDialog": false,
        "show_title": "5",
        "skip": {
          "ed": {
            "end": 1410,
            "start": 1320
          },
          "op": {
            "end": 90,
            "start": 0
          }
        },
        "status": 2,
        "subtitle": "已观看484.9万次",
        "title": "5",
        "vid": ""
      },
      {
        "aid": 891839004,
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "bvid": "BV1qP4y1G7Ei",
        "cid": 54584894,
        "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
        "dimension": {
          "height": 0,
          "rotate": 0,
          "width": 0
        },
        "duration": 1427000,
        "enable_vt": false,
        "ep_id": 247674,
        "from": "bangumi",
        "id": 247674,
        "is_view_hide": false,
        "link": "https://www.bilibili.com/bangumi/play/ep247674",
        "long_title": "",
        "pub_time": 1536287160,
        "pv": 0,
        "release_date": "",
        "rights": {
          "allow_dm": 1,
          "allow_download": 1,
          "area_limit": 0,
          "cache_auth": 0
        },
        "section_type": 0,
        "share_copy": "《Little Busters! EX》第6话 ",
        "share_url": "https://www.bilibili.com/bangumi/play/ep247674",
        "short_link": "https://b23.tv/ep247674",
        "showDrmLoginDialog": false,
        "show_title": "6",
        "skip": {
          "ed": {
            "end": 1410,
            "start": 1320
          },
          "op": {
            "end": 172,
            "start": 82
          }
        },
        "status": 2,
        "subtitle": "已观看484.9万次",
        "title": "6",
        "vid": ""
      },
      {
        "aid": 251791435,
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "bvid": "BV1wY41147aQ",
        "cid": 54584950,
        "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
        "dimension": {
          "height": 0,
          "rotate": 0,
          "width": 0
        },
        "duration": 1421000,
        "enable_vt": false,
        "ep_id": 247675,
        "from": "bangumi",
        "id": 247675,
        "is_view_hide": false,
        "link": "https://www.bilibili.com/bangumi/play/ep247675",
        "long_title": "",
        "pub_time": 1536287160,
        "pv": 0,
        "release_date": "",
        "rights": {
          "allow_dm": 1,
          "allow_download": 1,
          "area_limit": 0,
          "cache_auth": 0
        },
        "section_type": 0,
        "share_copy": "《Little Busters! EX》第7话 ",
        "share_url": "https://www.bilibili.com/bangumi/play/ep247675",
        "short_link": "https://b23.tv/ep247675",
        "showDrmLoginDialog": false,
        "show_title": "7",
        "skip": {
          "ed": {
            "end": 1410,
            "start": 1320
          },
          "op": {
            "end": 246,
            "start": 156
          }
        },
        "status": 2,
        "subtitle": "已观看484.9万次",
        "title": "7",
        "vid": ""
      },
      {
        "aid": 251866174,
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "bvid": "BV1sY41147gD",
        "cid": 54584997,
        "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
        "dimension": {
          "height": 0,
          "rotate": 0,
          "width": 0
        },
        "duration": 1417000,
        "enable_vt": false,
        "ep_id": 247676,
        "from": "bangumi",
        "id": 247676,
        "is_view_hide": false,
        "link": "https://www.bilibili.com/bangumi/play/ep247676",
        "long_title": "",
        "pub_time": 1536287160,
        "pv": 0,
        "release_date": "",
        "rights": {
          "allow_dm": 1,
          "allow_download": 1,
          "area_limit": 0,
          "cache_auth": 0
        },
        "section_type": 0,
        "share_copy": "《Little Busters! EX》第8话 ",
        "share_url": "https://www.bilibili.com/bangumi/play/ep247676",
        "short_link": "https://b23.tv/ep247676",
        "showDrmLoginDialog": false,
        "show_title": "8",
        "skip": {
          "ed": {
            "end": 0,
            "start": 0
          },
          "op": {
            "end": 0,
            "start": 0
          }
        },
        "status": 2,
        "subtitle": "已观看484.9万次",
        "title": "8",
        "vid": ""
      }
    ],
    "evaluate": "正篇讲述关系很亲密的5个人组成的小组织“Little Busters”的成员——少年直枝理树和领袖人物枣恭介为了享受最后的高中生活而组建了一支棒球队“Little Busters”，然后棒球队的成员们遇到了各种各样的烦恼的故事，本作作为特典动画剧情分别由佳奈多、佐佐美、沙耶这三名女性角色，主要会围绕她们各自的故事展开。",
    "freya": {
      "bubble_desc": "",
      "bubble_show_cnt": 0,
      "icon_show": 1
    },
    "hide_ep_vv_vt_dm": 1,
    "icon_font": {
      "name": "playdata-square-line@500",
      "text": "484.9万"
    },
    "jp_title": "",
    "link": "http://www.bilibili.com/bangumi/media/md3571/",
    "media_id": 3571,
    "mode": 2,
    "new_ep": {
      "desc": "已完结, 全8话",
      "id": 247676,
      "is_new": 0,
      "title": "8"
    },
    "play_strategy": {
      "strategies": [
        "common_section-formal_first_ep",
        "common_section-common_section",
        "common_section-next_season",
        "formal-finish-next_season",
        "formal-end-other_section",
        "formal-end-next_season",
        "ord"
      ]
    },
    "positive": {
      "id": 34730,
      "title": "正片"
    },
    "publish": {
      "is_finish": 1,
      "is_started": 1,
      "pub_time": "2014-01-29 00:00:01",
      "pub_time_show": "2014年1月29日",
      "unknow_pub_date": 0,
      "weekday": 0
    },
    "rating": {
      "count": 3130,
      "score": 9.8
    },
    "record": "",
    "rights": {
      "allow_bp": 1,
      "allow_bp_rank": 0,
      "allow_download": 1,
      "allow_review": 1,
      "area_limit": 317,
      "ban_area_show": 1,
      "can_watch": 1,
      "copyright": "bilibili",
      "forbid_pre": 0,
      "freya_white": 0,
      "is_cover_show": 0,
      "is_preview": 0,
      "is_sponsor": 0,
      "only_vip_download": 0,
      "resource": "",
      "watch_platform": 0
    },
    "season_id": 3571,
    "season_title": "Little Busters! EX",
    "seasons": [
      {
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "cover": "http://i0.hdslb.com/bfs/bangumi/image/e4d3dbad653a8091328143e262fac7c1d0da515d.png",
        "enable_vt": false,
        "horizontal_cover_1610": "",
        "horizontal_cover_169": "",
        "icon_font": {
          "name": "playdata-square-line@500",
          "text": "2260.5万"
        },
        "media_id": 3242,
        "new_ep": {
          "cover": "http://i0.hdslb.com/bfs/bangumi/c9a98b2ae5268e1bff4caf18ae5871014bf51885.jpg",
          "id": 82111,
          "index_show": "全26话"
        },
        "season_id": 3242,
        "season_title": "第一季",
        "season_type": 1,
        "stat": {
          "favorites": 1595224,
          "series_follow": 1761019,
          "views": 22605229,
          "vt": 0
        }
      },
      {
        "badge": "独家",
        "badge_info": {
          "bg_color": "#00C0FF",
          "bg_color_night": "#0B91BE",
          "text": "独家"
        },
        "badge_type": 1,
        "cover": "http://i0.hdslb.com/bfs/bangumi/94e6df21c816a824da404e466f73529761a75e68.jpg",
        "enable_vt": false,
        "horizontal_cover_1610": "",
        "horizontal_cover_169": "",
        "icon_font": {
          "name": "playdata-square-line@500",
          "text": "812.2万"
        },
        "media_id": 81292,
        "new_ep": {
          "cover": "http://i0.hdslb.com/bfs/archive/704f6f424f776a3713742a93b2172e8181ff1835.jpg",
          "id": 199818,
          "index_show": "全13话"
        },
        "season_id": 3570,
        "season_title": "第二季",
        "season_type": 1,
        "stat": {
          "favorites": 582559,
          "series_follow": 1761019,
          "views": 8122005,
          "vt": 0
        }
      },
      {
        "badge": "",
        "badge_info": {
          "bg_color": "#FB7299",
          "bg_color_night": "#D44E7D",
          "text": ""
        },
        "badge_type": 0,
        "cover": "http://i0.hdslb.com/bfs/bangumi/18e4ebae9e5a2395aef71978d4a4184947581b82.png",
        "enable_vt": false,
        "horizontal_cover_1610": "",
        "horizontal_cover_169": "",
        "icon_font": {
          "name": "playdata-square-line@500",
          "text": "484.9万"
        },
        "media_id": 3571,
        "new_ep": {
          "cover": "http://i0.hdslb.com/bfs/archive/df356838f3deb40246aac7ae9901a55ae20d1f6f.jpg",
          "id": 247676,
          "index_show": "全8话"
        },
        "season_id": 3571,
        "season_title": "OVA2",
        "season_type": 1,
        "stat": {
          "favorites": 407425,
          "series_follow": 1761019,
          "views": 4849188,
          "vt": 0
        }
      }
    ],
    "series": {
      "display_type": 0,
      "series_id": 2050,
      "series_title": "Little Busters!"
    },
    "share_copy": "《Little Busters! EX》寻找这个世界的秘密",
    "share_sub_title": "寻找这个世界的秘密",
    "share_url": "https://www.bilibili.com/bangumi/play/ss3571",
    "show": {
      "wide_screen": 0
    },
    "show_season_type": 1,
    "square_cover": "http://i0.hdslb.com/bfs/bangumi/6564a648c7cbd361d67e6cb06858dc7b40f30b18.png",
    "staff": "原作： Key、都乃河勇人\n导演： 山川吉樹\n脚本： 島田満、綾奈ゆにこ\n分镜： 櫻井親良、山川吉樹、古川知宏、鈴木洋平\n演出： 山川吉樹、鈴木洋平\n音乐： 麻枝准、PMMK、三輪学、戸越まごめ、折戸伸治\n人物原案： Na-Ga、樋上いたる\n人物设定： 飯塚晴子\n系列构成： 島田満\n色彩设计： 石川恭介\n总作画监督： 飯塚晴子、岩倉和憲\n作画监督： 飯塚晴子、西川絵奈、関口雅浩、萩原弘光、梶谷光春、音地正行、安野将人\n摄影监督： 中西智一\n剪辑： 西山茂\n主题歌演出： 鈴湯、Rita\n音响监督： 本山哲",
    "stat": {
      "coins": 29490,
      "danmakus": 94727,
      "favorite": 10306,
      "favorites": 407425,
      "follow_text": "176.1万系列追番",
      "likes": 47784,
      "reply": 3419,
      "share": 2680,
      "views": 4849188,
      "vt": 0
    },
    "status": 2,
    "styles": [
      "校园",
      "日常",
      "治愈",
      "励志"
    ],
    "subtitle": "已观看484.9万次",
    "title": "Little Busters! EX",
    "total": 8,
    "type": 1,
    "user_status": {
      "area_limit": 0,
      "ban_area_show": 0,
      "follow": 0,
      "follow_status": 0,
      "login": 0,
      "pay": 0,
      "pay_pack_paid": 0,
      "sponsor": 0
    }
  }
}
    "###;

    let resp: APIResponse<SeasonInfoData> = serde_json::from_str(json).unwrap();
    assert!(resp.is_success());
}
