use bili_api::episode::info::{get_season_info, EpisodeQuery, SeasonInfoData};
use bili_api::APIResponse;

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
        "actors": "利姆鲁·特恩佩斯特：冈咲美保\\n大贤者：丰口惠美\\n维鲁 德拉·特恩佩斯特：前野智昭\\n井沢静江：花守由美里\\n岚牙：小林亲弘\\n红丸：古川慎\\n朱菜：千本木彩花\\n紫苑：M・A・O\\n苍影：江口拓也\\n白老：大塚芳忠\\n三上悟：寺岛拓笃\\n希兹：花守由美里\\n利古鲁德：山本兼平\\n利古鲁：石谷春贵\\n哥布塔：泊明日菜\\n凯金：斧敦\\n托蕾妮：田中理惠\\n艾莲：熊田茜音\\n卡巴尔：高梨谦吾\\n米莉姆：日高里菜",
        "alias": "",
        "areas": [
            {
                "id": 2,
                "name": "日本"
            }
        ],
        "bkg_cover": "",
        "cover": "http://i0.hdslb.com/bfs/bangumi/a4c0e0ccc44fe3949a734f546cf5bb07da925bad.png",
        "delivery_fragment_video": false,
        "enable_vt": false,
        "episodes": [
            {
                "aid": 32836144,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1YW411m7yV",
                "cid": 57467566,
                "cover": "http://i0.hdslb.com/bfs/archive/4994e631fef939e7f606c012682a96ce2d3bf7b8.jpg",
                "dimension": {
                    "height": 0,
                    "rotate": 0,
                    "width": 0
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250460,
                "from": "bangumi",
                "id": 250460,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250460",
                "long_title": "暴风龙维鲁德拉",
                "pub_time": 1538411400,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第1话 暴风龙维鲁德拉",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250460",
                "short_link": "https://b23.tv/ep250460",
                "showDrmLoginDialog": false,
                "show_title": "第1话 暴风龙维鲁德拉",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 0,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "1",
                "vid": ""
            },
            {
                "aid": 33424582,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1GW411174f",
                "cid": 58591077,
                "cover": "http://i0.hdslb.com/bfs/archive/cace285637cb09c5b143e494e3d4b56ad183d74c.jpg",
                "dimension": {
                    "height": 0,
                    "rotate": 0,
                    "width": 0
                },
                "duration": 1423000,
                "enable_vt": false,
                "ep_id": 250461,
                "from": "bangumi",
                "id": 250461,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250461",
                "long_title": " 与哥布林们的邂逅",
                "pub_time": 1539016200,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第2话 与哥布林们的邂逅",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250461",
                "short_link": "https://b23.tv/ep250461",
                "showDrmLoginDialog": false,
                "show_title": "第2话 与哥布林们的邂逅",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1326
                    },
                    "op": {
                        "end": 91,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "2",
                "vid": ""
            },
            {
                "aid": 33917020,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1et411Z7cf",
                "cid": 61556806,
                "cover": "http://i0.hdslb.com/bfs/archive/e35af6a4bc188b4604704f7829c8adc10da23b10.jpg",
                "dimension": {
                    "height": 0,
                    "rotate": 0,
                    "width": 0
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250462,
                "from": "bangumi",
                "id": 250462,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250462",
                "long_title": "哥布林村中的战斗",
                "pub_time": 1539621000,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第3话 哥布林村中的战斗",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250462",
                "short_link": "https://b23.tv/ep250462",
                "showDrmLoginDialog": false,
                "show_title": "第3话 哥布林村中的战斗",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 121,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "3",
                "vid": ""
            },
            {
                "aid": 34394751,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1bt411R7Up",
                "cid": 304152518,
                "cover": "http://i0.hdslb.com/bfs/archive/44a469c1dffbab32ebd943f5d7499eb3245b68d1.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1372000,
                "enable_vt": false,
                "ep_id": 250463,
                "from": "bangumi",
                "id": 250463,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250463",
                "long_title": "在矮人王国",
                "pub_time": 1540225800,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第4话 在矮人王国",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250463",
                "short_link": "https://b23.tv/ep250463",
                "showDrmLoginDialog": false,
                "show_title": "第4话 在矮人王国",
                "skip": {
                    "ed": {
                        "end": 1366,
                        "start": 1276
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "4",
                "vid": ""
            },
            {
                "aid": 34879533,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV18b411A731",
                "cid": 304157072,
                "cover": "http://i0.hdslb.com/bfs/archive/5dadd4ccdd6b247d07e3225758a52a9ebd15fd60.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1386000,
                "enable_vt": false,
                "ep_id": 250464,
                "from": "bangumi",
                "id": 250464,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250464",
                "long_title": "英雄王加泽尔·多瓦格",
                "pub_time": 1540830600,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第5话 英雄王加泽尔·多瓦格",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250464",
                "short_link": "https://b23.tv/ep250464",
                "showDrmLoginDialog": false,
                "show_title": "第5话 英雄王加泽尔·多瓦格",
                "skip": {
                    "ed": {
                        "end": 1379,
                        "start": 1290
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "5",
                "vid": ""
            },
            {
                "aid": 35367951,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV17b411A7Rd",
                "cid": 304165993,
                "cover": "http://i0.hdslb.com/bfs/archive/523e3b8faee592c683ea55cfdcdb26106651fdea.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250465,
                "from": "bangumi",
                "id": 250465,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250465",
                "long_title": "静",
                "pub_time": 1541435400,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第6话  静",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250465",
                "short_link": "https://b23.tv/ep250465",
                "showDrmLoginDialog": false,
                "show_title": "第6话 静",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "6",
                "vid": ""
            },
            {
                "aid": 35833132,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1st411U7eH",
                "cid": 62880801,
                "cover": "http://i0.hdslb.com/bfs/bangumi/793eb3cb7ff45e0c46b89049182c82d1cbe8af1a.jpg",
                "dimension": {
                    "height": 0,
                    "rotate": 0,
                    "width": 0
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250466,
                "from": "bangumi",
                "id": 250466,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250466",
                "long_title": "爆 炎支配者",
                "pub_time": 1542040200,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第7话 爆炎支配者",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250466",
                "short_link": "https://b23.tv/ep250466",
                "showDrmLoginDialog": false,
                "show_title": "第7话 爆炎支配者",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "7",
                "vid": ""
            },
            {
                "aid": 36328570,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1vt41117N6",
                "cid": 304166288,
                "cover": "http://i0.hdslb.com/bfs/archive/986e41f808b51b8a77613fde8715848c2fc1986c.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250467,
                "from": "bangumi",
                "id": 250467,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250467",
                "long_title": "继承的愿望",
                "pub_time": 1542645000,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第8话 继承的愿望",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250467",
                "short_link": "https://b23.tv/ep250467",
                "showDrmLoginDialog": false,
                "show_title": "第8话 继承的愿望",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "8",
                "vid": ""
            },
            {
                "aid": 36818011,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV14t41197PY",
                "cid": 304166496,
                "cover": "http://i0.hdslb.com/bfs/archive/4c130edd68baaac524e3b2fdb835c2b28db81e26.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250468,
                "from": "bangumi",
                "id": 250468,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250468",
                "long_title": "大鬼族的袭击",
                "pub_time": 1543249800,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第9话 大鬼族的袭击",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250468",
                "short_link": "https://b23.tv/ep250468",
                "showDrmLoginDialog": false,
                "show_title": "第9话 大鬼族的袭击",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "9",
                "vid": ""
            },
            {
                "aid": 37338903,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1Rt411D7FF",
                "cid": 304168362,
                "cover": "http://i0.hdslb.com/bfs/archive/f07e0cb2a1d5931bc0d640a3ba38b5bbd16352bf.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250469,
                "from": "bangumi",
                "id": 250469,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250469",
                "long_title": "猪头帝",
                "pub_time": 1543854600,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第10话 猪头帝",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250469",
                "short_link": "https://b23.tv/ep250469",
                "showDrmLoginDialog": false,
                "show_title": "第10话 猪头帝",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "10",
                "vid": ""
            },
            {
                "aid": 37873347,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1pt411q7T4",
                "cid": 304169098,
                "cover": "http://i0.hdslb.com/bfs/archive/c638600b2526728ff30475977e288efcd10824c8.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250470,
                "from": "bangumi",
                "id": 250470,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250470",
                "long_title": "加维鲁登场！",
                "pub_time": 1544459400,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第11话 加维鲁登场！",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250470",
                "short_link": "https://b23.tv/ep250470",
                "showDrmLoginDialog": false,
                "show_title": "第11话 加维鲁登场！",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "11",
                "vid": ""
            },
            {
                "aid": 38386513,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1mt411e7uS",
                "cid": 304172357,
                "cover": "http://i0.hdslb.com/bfs/archive/d67c4bf09c8b05dc2ffabea0c6e35a3418563b27.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250471,
                "from": "bangumi",
                "id": 250471,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250471",
                "long_title": "故障的齿轮",
                "pub_time": 1545064200,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第12话 故障的齿轮",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250471",
                "short_link": "https://b23.tv/ep250471",
                "showDrmLoginDialog": false,
                "show_title": "第12话 故障的齿轮",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 114,
                        "start": 24
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "12",
                "vid": ""
            },
            {
                "aid": 38925172,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV12t411k7tB",
                "cid": 68414415,
                "cover": "http://i0.hdslb.com/bfs/archive/bd951c4a7f59d89a0a58fbfdf05b18b33c8e3ae5.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250472,
                "from": "bangumi",
                "id": 250472,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250472",
                "long_title": "大冲突",
                "pub_time": 1545669000,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第13话 大冲突",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250472",
                "short_link": "https://b23.tv/ep250472",
                "showDrmLoginDialog": false,
                "show_title": "第13话 大冲突",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "13",
                "vid": ""
            },
            {
                "aid": 40158127,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1Rt411W7js",
                "cid": 304189879,
                "cover": "http://i0.hdslb.com/bfs/archive/20a525d42095f112e6801f0a023fb5a47fbece76.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250473,
                "from": "bangumi",
                "id": 250473,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250473",
                "long_title": "吞噬一切之人",
                "pub_time": 1546878600,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第14话 吞噬一切之人",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250473",
                "short_link": "https://b23.tv/ep250473",
                "showDrmLoginDialog": false,
                "show_title": "第14话 吞噬一切之人",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 99,
                        "start": 9
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "14",
                "vid": ""
            },
            {
                "aid": 40728086,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV14t411H7tL",
                "cid": 304190998,
                "cover": "http://i0.hdslb.com/bfs/archive/97338573d271f63f8bb353d3234e31259e38d55a.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250474,
                "from": "bangumi",
                "id": 250474,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250474",
                "long_title": "鸠拉森林大同盟",
                "pub_time": 1547483400,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第15话 鸠拉森林大同盟",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250474",
                "short_link": "https://b23.tv/ep250474",
                "showDrmLoginDialog": false,
                "show_title": "第15话 鸠拉森林大同盟",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 112,
                        "start": 22
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "15",
                "vid": ""
            },
            {
                "aid": 41317109,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1Dt4118714",
                "cid": 304191930,
                "cover": "http://i0.hdslb.com/bfs/archive/fc6c9a016704fd8455f3acfd4b03b6679d407993.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250475,
                "from": "bangumi",
                "id": 250475,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250475",
                "long_title": "魔王米莉姆来袭",
                "pub_time": 1548088200,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第16话 魔王米莉姆来袭",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250475",
                "short_link": "https://b23.tv/ep250475",
                "showDrmLoginDialog": false,
                "show_title": "第16话 魔王米莉姆来袭",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 61,
                        "start": 31
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "16",
                "vid": ""
            },
            {
                "aid": 41954051,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV14b411C7AQ",
                "cid": 73661368,
                "cover": "http://i0.hdslb.com/bfs/archive/8307bf0892c2149f3001bc263f284a2e38fccdf8.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250476,
                "from": "bangumi",
                "id": 250476,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250476",
                "long_title": "聚集而来之人",
                "pub_time": 1548693000,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第17话 聚集而来之人",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250476",
                "short_link": "https://b23.tv/ep250476",
                "showDrmLoginDialog": false,
                "show_title": "第17话 聚集而来之人",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 154,
                        "start": 64
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "17",
                "vid": ""
            },
            {
                "aid": 42635768,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1vb411r7Nf",
                "cid": 304192873,
                "cover": "http://i0.hdslb.com/bfs/archive/e80edea554187a5935fdee1fa07941fd7456a1cc.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250477,
                "from": "bangumi",
                "id": 250477,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250477",
                "long_title": "悄悄接近的恶意",
                "pub_time": 1549297800,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第18话 悄悄接近的恶意",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250477",
                "short_link": "https://b23.tv/ep250477",
                "showDrmLoginDialog": false,
                "show_title": "第18话 悄悄接近的恶意",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "18",
                "vid": ""
            },
            {
                "aid": 43231921,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1fb411S7DG",
                "cid": 304194003,
                "cover": "http://i0.hdslb.com/bfs/archive/40084ba9fdfa2f166d870edd532e2bd11b55d84f.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250478,
                "from": "bangumi",
                "id": 250478,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250478",
                "long_title": "暴风大妖涡",
                "pub_time": 1549902600,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第19话  暴风大妖涡",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250478",
                "short_link": "https://b23.tv/ep250478",
                "showDrmLoginDialog": false,
                "show_title": "第19话 暴风大妖涡",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 151,
                        "start": 61
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "19",
                "vid": ""
            },
            {
                "aid": 43977527,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV11b41117Fb",
                "cid": 304195119,
                "cover": "http://i0.hdslb.com/bfs/archive/7297f8d3c4faddf8182e4d91f0d3ca20bddc6c0d.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250479,
                "from": "bangumi",
                "id": 250479,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250479",
                "long_title": "优树·神乐坂",
                "pub_time": 1550507400,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第20话 优树·神乐坂",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250479",
                "short_link": "https://b23.tv/ep250479",
                "showDrmLoginDialog": false,
                "show_title": "第20话 优树·神乐坂",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "20",
                "vid": ""
            },
            {
                "aid": 44669063,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1bb41187ah",
                "cid": 78203647,
                "cover": "http://i0.hdslb.com/bfs/archive/1a539d8806b8df3165ede1e31778e4be8e5b49d7.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250480,
                "from": "bangumi",
                "id": 250480,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250480",
                "long_title": "静小姐的学生们",
                "pub_time": 1551112200,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第21话 静小姐的学生们",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250480",
                "short_link": "https://b23.tv/ep250480",
                "showDrmLoginDialog": false,
                "show_title": "第21话 静小姐的学生们",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "21",
                "vid": ""
            },
            {
                "aid": 45342449,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1Mb411z7cY",
                "cid": 79390717,
                "cover": "http://i0.hdslb.com/bfs/archive/bedbfc989c3670cf830a9a2f9fbbea376569d5d3.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250481,
                "from": "bangumi",
                "id": 250481,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250481",
                "long_title": "迷宫攻略",
                "pub_time": 1551717000,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《 关于我转生变成史莱姆这档事》第22话 迷宫攻略",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250481",
                "short_link": "https://b23.tv/ep250481",
                "showDrmLoginDialog": false,
                "show_title": "第22话 迷宫攻略",
                "skip": {
                    "ed": {
                        "end": 1415,
                        "start": 1325
                    },
                    "op": {
                        "end": 140,
                        "start": 50
                    }
                },
                "status": 13,
                "subtitle": "已观 看5.2亿次",
                "title": "22",
                "vid": ""
            },
            {
                "aid": 45991773,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1sb411e72G",
                "cid": 304195965,
                "cover": "http://i0.hdslb.com/bfs/archive/903719673ae9538a5be72ff5d4fbf07f9b1f0f9e.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1421000,
                "enable_vt": false,
                "ep_id": 250482,
                "from": "bangumi",
                "id": 250482,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250482",
                "long_title": "被拯救的灵魂",
                "pub_time": 1552321800,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第23话 被拯救的灵魂",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250482",
                "short_link": "https://b23.tv/ep250482",
                "showDrmLoginDialog": false,
                "show_title": "第23话 被拯救的灵魂",
                "skip": {
                    "ed": {
                        "end": 0,
                        "start": 0
                    },
                    "op": {
                        "end": 132,
                        "start": 42
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "23",
                "vid": ""
            },
            {
                "aid": 46653684,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1rb411n7of",
                "cid": 304197892,
                "cover": "http://i0.hdslb.com/bfs/archive/9135c44440b8cdb11b5b77264e3431f40b88514a.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1425000,
                "enable_vt": false,
                "ep_id": 250483,
                "from": "bangumi",
                "id": 250483,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep250483",
                "long_title": "外传：黑与面具",
                "pub_time": 1552926600,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》第24话 外传：黑与面具",
                "share_url": "https://www.bilibili.com/bangumi/play/ep250483",
                "short_link": "https://b23.tv/ep250483",
                "showDrmLoginDialog": false,
                "show_title": "第24话 外传：黑与面具",
                "skip": {
                    "ed": {
                        "end": 0,
                        "start": 0
                    },
                    "op": {
                        "end": 199,
                        "start": 109
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "24",
                "vid": ""
            },
            {
                "aid": 58368206,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1y4411F72o",
                "cid": 304199497,
                "cover": "http://i0.hdslb.com/bfs/archive/43670dfd218e455efebce12430dbeb2c8875987e.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1354000,
                "enable_vt": false,
                "ep_id": 277141,
                "from": "bangumi",
                "id": 277141,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep277141",
                "long_title": "外传：M的悲剧？",
                "pub_time": 1562598000,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》OAD02 外传：M的悲剧？",
                "share_url": "https://www.bilibili.com/bangumi/play/ep277141",
                "short_link": "https://b23.tv/ep277141",
                "showDrmLoginDialog": false,
                "show_title": "OAD02 外传：M的悲剧？",
                "skip": {
                    "ed": {
                        "end": 1353,
                        "start": 1263
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "OAD02",
                "vid": ""
            },
            {
                "aid": 882579111,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1mK4y1C7Th",
                "cid": 304200142,
                "cover": "http://i0.hdslb.com/bfs/archive/81d07d1a478ce3a6209b557e14df9b9c78c42abb.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1493000,
                "enable_vt": false,
                "ep_id": 316957,
                "from": "bangumi",
                "id": 316957,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep316957",
                "long_title": "外传：利姆鲁的华丽教师生活 其一",
                "pub_time": 1585238403,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》OAD03 外传：利姆鲁的华丽教师生活 其一",
                "share_url": "https://www.bilibili.com/bangumi/play/ep316957",
                "short_link": "https://b23.tv/ep316957",
                "showDrmLoginDialog": false,
                "show_title": "OAD03 外传：利姆鲁的华丽教师生活 其一",
                "skip": {
                    "ed": {
                        "end": 1492,
                        "start": 1402
                    },
                    "op": {
                        "end": 244,
                        "start": 154
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "OAD03",
                "vid": ""
            },
            {
                "aid": 583753748,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1vz4y1X7Jc",
                "cid": 210163125,
                "cover": "http://i0.hdslb.com/bfs/archive/89c839e9be005ed074a8efb2afeb1e0fb53c7850.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1429000,
                "enable_vt": false,
                "ep_id": 330026,
                "from": "bangumi",
                "id": 330026,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep330026",
                "long_title": "外传：利姆鲁的华丽教师生活 其二",
                "pub_time": 1594224001,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》OAD04 外传：利姆鲁的 华丽教师生活 其二",
                "share_url": "https://www.bilibili.com/bangumi/play/ep330026",
                "short_link": "https://b23.tv/ep330026",
                "showDrmLoginDialog": false,
                "show_title": "OAD04 外传：利姆鲁的华丽教师生活 其二",
                "skip": {
                    "ed": {
                        "end": 1428,
                        "start": 1337
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "OAD04",
                "vid": ""
            },
            {
                "aid": 415395804,
                "badge": "会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "会员"
                },
                "badge_type": 0,
                "bvid": "BV1zV411a7Mo",
                "cid": 259906541,
                "cover": "http://i0.hdslb.com/bfs/archive/88211e0d1618b8aa5b48267e77d91c1f25279d4c.jpg",
                "dimension": {
                    "height": 1080,
                    "rotate": 0,
                    "width": 1920
                },
                "duration": 1391000,
                "enable_vt": false,
                "ep_id": 366494,
                "from": "bangumi",
                "id": 366494,
                "is_view_hide": false,
                "link": "https://www.bilibili.com/bangumi/play/ep366494",
                "long_title": "外传：利姆鲁的华丽教师生活 其三",
                "pub_time": 1606406401,
                "pv": 0,
                "release_date": "",
                "rights": {
                    "allow_dm": 1,
                    "allow_download": 0,
                    "area_limit": 0
                },
                "section_type": 0,
                "share_copy": "《关于我转生变成史莱姆这档事》OAD05 外传：利姆鲁的华丽教师生活 其三",
                "share_url": "https://www.bilibili.com/bangumi/play/ep366494",
                "short_link": "https://b23.tv/ep366494",
                "showDrmLoginDialog": false,
                "show_title": "OAD05 外传：利姆鲁的华丽教师生活 其三",
                "skip": {
                    "ed": {
                        "end": 1390,
                        "start": 1300
                    },
                    "op": {
                        "end": 90,
                        "start": 0
                    }
                },
                "status": 13,
                "subtitle": "已观看5.2亿次",
                "title": "OAD05",
                "vid": ""
            }
        ],
        "evaluate": "史莱姆生活，开始了。\\n上班族的三上悟在道路上被歹徒给刺杀身亡后，回过神来发现自己转生到了异世界。\\n不过，自己居然是“史莱姆”！\\n他在得到 利姆鲁这个名字后开始了自己的史莱姆人生，随着与各个种族相处交流的过程中，他定下了一个目标——那就是“建立一个任何种族都能愉快地一起生活的国家”！",
        "freya": {
            "bubble_desc": "",
            "bubble_show_cnt": 0,
            "icon_show": 1
        },
        "hide_ep_vv_vt_dm": 1,
        "icon_font": {
            "name": "playdata-square-line@500",
            "text": "5.2亿"
        },
        "jp_title": "",
        "link": "http://www.bilibili.com/bangumi/media/md139252/",
        "media_id": 139252,
        "mode": 2,
        "new_ep": {
            "desc": "已完结, 全28话",
            "id": 366494,
            "is_new": 0,
            "title": "OAD05"
        },
        "payment": {
            "discount": 100,
            "pay_type": {
                "allow_discount": 0,
                "allow_pack": 0,
                "allow_ticket": 0,
                "allow_time_limit": 0,
                "allow_vip_discount": 0,
                "forbid_bb": 0
            },
            "price": "0.0",
            "promotion": "",
            "tip": "大会员专享观看特权哦~",
            "view_start_time": 0,
            "vip_discount": 100,
            "vip_first_promotion": "",
            "vip_price": "0",
            "vip_promotion": "开通大会员观看"
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
            "id": 34988,
            "title": "正片"
        },
        "publish": {
            "is_finish": 1,
            "is_started": 1,
            "pub_time": "2018-10-02 00:30:00",
            "pub_time_show": "2018年10月02日00:30",
            "unknow_pub_date": 0,
            "weekday": 0
        },
        "rating": {
            "count": 121584,
            "score": 9.4
        },
        "record": "登记号：10417060196812309",
        "rights": {
            "allow_bp": 0,
            "allow_bp_rank": 0,
            "allow_download": 0,
            "allow_review": 1,
            "area_limit": 328,
            "ban_area_show": 1,
            "can_watch": 1,
            "copyright": "bilibili",
            "forbid_pre": 0,
            "freya_white": 0,
            "is_cover_show": 0,
            "is_preview": 1,
            "is_sponsor": 0,
            "only_vip_download": 0,
            "resource": "",
            "watch_platform": 0
        },
        "season_id": 25739,
        "season_title": "关于我转生变成史莱姆这档事",
        "seasons": [
            {
                "badge": "大会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "大会员"
                },
                "badge_type": 0,
                "cover": "http://i0.hdslb.com/bfs/bangumi/a4c0e0ccc44fe3949a734f546cf5bb07da925bad.png",
                "enable_vt": false,
                "horizontal_cover_1610": "",
                "horizontal_cover_169": "",
                "icon_font": {
                    "name": "playdata-square-line@500",
                    "text": "5.2亿"
                },
                "media_id": 139252,
                "new_ep": {
                    "cover": "http://i0.hdslb.com/bfs/archive/88211e0d1618b8aa5b48267e77d91c1f25279d4c.jpg",
                    "id": 366494,
                    "index_show": "全28话"
                },
                "season_id": 25739,
                "season_title": "第一季",
                "season_type": 1,
                "stat": {
                    "favorites": 7943277,
                    "series_follow": 10889864,
                    "views": 517360693,
                    "vt": 0
                }
            },
            {
                "badge": "大会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "大会员"
                },
                "badge_type": 0,
                "cover": "http://i0.hdslb.com/bfs/bangumi/image/fd492888df64bbc3b821dac5d516dbc1c2fe5f08.png",
                "enable_vt": false,
                "horizontal_cover_1610": "http://i0.hdslb.com/bfs/bangumi/image/45a7d0290b40bf5c1554472b95d426123e881f44.png",
                "horizontal_cover_169": "http://i0.hdslb.com/bfs/bangumi/image/a8976272debc705bce52d1d9433e983ef4e18e50.png",
                "icon_font": {
                    "name": "playdata-square-line@500",
                    "text": "4.9亿"
                },
                "media_id": 28231812,
                "new_ep": {
                    "cover": "http://i0.hdslb.com/bfs/archive/41f6c69130a3159cb17df861e41d92b1a82e6186.png",
                    "id": 409826,
                    "index_show": "全26话"
                },
                "season_id": 36170,
                "season_title": "第二季",
                "season_type": 1,
                "stat": {
                    "favorites": 9196395,
                    "series_follow": 10889864,
                    "views": 485167927,
                    "vt": 0
                }
            },
            {
                "badge": "大会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "大会员"
                },
                "badge_type": 0,
                "cover": "https://i0.hdslb.com/bfs/bangumi/image/9c716a761afd055e6b65c96aac7880d2a960dd0b.png",
                "enable_vt": false,
                "horizontal_cover_1610": "https://i0.hdslb.com/bfs/bangumi/image/91fdee1d8ad02d38fd9c8434f50d880656d0b995.png",
                "horizontal_cover_169": "https://i0.hdslb.com/bfs/bangumi/image/af38a22c2862d19325bec6efe47d78c923a1f189.png",
                "icon_font": {
                    "name": "playdata-square-line@500",
                    "text": "7146.9万"
                },
                "media_id": 23154901,
                "new_ep": {
                    "cover": "http://i0.hdslb.com/bfs/archive/dd233a84d88196cbab7bee3c2ab4fc0a9aa98c49.jpg",
                    "id": 1396055,
                    "index_show": "全24话"
                },
                "season_id": 48852,
                "season_title": "第三季",
                "season_type": 1,
                "stat": {
                    "favorites": 10471155,
                    "series_follow": 10889864,
                    "views": 71469197,
                    "vt": 0
                }
            },
            {
                "badge": "大会员",
                "badge_info": {
                    "bg_color": "#FB7299",
                    "bg_color_night": "#D44E7D",
                    "text": "大会员"
                },
                "badge_type": 0,
                "cover": "http://i0.hdslb.com/bfs/bangumi/image/cac1699418451387f93572066b7e5d14d799d3cd.jpg",
                "enable_vt": false,
                "horizontal_cover_1610": "",
                "horizontal_cover_169": "",
                "icon_font": {
                    "name": "playdata-square-line@500",
                    "text": "1.3亿"
                },
                "media_id": 28233903,
                "new_ep": {
                    "cover": "http://i0.hdslb.com/bfs/archive/197b122dc9ebe6a0141c54673a803a9499076ce7.png",
                    "id": 395222,
                    "index_show": "全12话"
                },
                "season_id": 38221,
                "season_title": "外 传",
                "season_type": 1,
                "stat": {
                    "favorites": 8379536,
                    "series_follow": 10889864,
                    "views": 131116173,
                    "vt": 0
                }
            }
        ],
        "section": [
            {
                "attr": 0,
                "episode_id": 0,
                "episodes": [
                    {
                        "aid": 0,
                        "badge": "",
                        "badge_info": {
                            "bg_color": "",
                            "bg_color_night": "",
                            "text": ""
                        },
                        "cid": 0,
                        "cover": "http://i0.hdslb.com/bfs/bangumi/image/ef0677fa77cfe7f9891cdffb7219aaf1e5d185c6.jpg",
                        "enable_vt": false,
                        "ep_id": 0,
                        "id": 119602,
                        "is_view_hide": false,
                        "link": "https://www.bilibili.com/bangumi/play/ep328078",
                        "link_type": "WEB_PAGE",
                        "pub_time": 0,
                        "pv": 0,
                        "report": {
                            "aid": "0",
                            "ep_title": "关于我转生变成史莱姆这档事专访·下",
                            "position": "",
                            "season_id": "25739",
                            "season_type": "1",
                            "section_id": "48271",
                            "section_type": "5"
                        },
                        "section_type": 0,
                        "showDrmLoginDialog": false,
                        "status": 0,
                        "title": "关于我转生变成史莱姆这档事专访·下"
                    },
                    {
                        "aid": 0,
                        "badge": "",
                        "badge_info": {
                            "bg_color": "",
                            "bg_color_night": "",
                            "text": ""
                        },
                        "cid": 0,
                        "cover": "http://i0.hdslb.com/bfs/bangumi/image/71312d7c1c1215013ca0a35269e4710abdc32bf0.jpg",
                        "enable_vt": false,
                        "ep_id": 0,
                        "id": 119603,
                        "is_view_hide": false,
                        "link": "https://www.bilibili.com/bangumi/play/ep327218",
                        "link_type": "WEB_PAGE",
                        "pub_time": 0,
                        "pv": 0,
                        "report": {
                            "aid": "0",
                            "ep_title": "关于我转生变成史莱姆这档事专访·上",
                            "position": "",
                            "season_id": "25739",
                            "season_type": "1",
                            "section_id": "48271",
                            "section_type": "5"
                        },
                        "section_type": 0,
                        "showDrmLoginDialog": false,
                        "status": 0,
                        "title": "关于我转生变成史莱姆这档事专访·上"
                    }
                ],
                "id": 48271,
                "report": {
                    "season_id": "25739",
                    "season_type": "1",
                    "sec_title": "次元发电机专访",
                    "section_id": "48271",
                    "section_type": "5"
                },
                "title": "次元发电机专访",
                "type": 5,
                "type2": 0
            }
        ],
        "series": {
            "display_type": 0,
            "series_id": 4188,
            "series_title": "关于我转生变成史莱姆这档事"
        },
        "share_copy": "《关于我转生变成史莱姆这档事》参见萌王！",
        "share_sub_title": "参见萌王！",
        "share_url": "https://www.bilibili.com/bangumi/play/ss25739",
        "show": {
            "wide_screen": 0
        },
        "show_season_type": 1,
        "square_cover": "http://i0.hdslb.com/bfs/bangumi/8d9f5b4a566d0547bc2e3f6f733b732a09c0d3d4.jpg",
        "staff": "原作：川上泰树、伏濑、みっつばー《关于我转生变成史莱姆这档事》（讲谈社《月刊少年天狼星》连载）\\n监督：菊地康仁\\n副监督：中山敦史\\n系列构成：笔安一幸\\n角色设计：江畑谅真\\n怪物设计：岸田隆宏\\n美术监督：佐藤步\\n美术设定：藤濑智康、佐藤正浩\\n色彩设计：齐藤麻记\\n摄影监督：佐藤洋\\n图形设计：生原雄次\\n编辑：神宫司由美\\n音响监督：明田川仁\\n音乐：Elements Garden\\n动画制作：8bit（8-bit）",
        "stat": {
            "coins": 2161656,
            "danmakus": 5165571,
            "favorite": 108082,
            "favorites": 7943277,
            "follow_text": "1089万系列追番",
            "likes": 2703629,
            "reply": 470434,
            "share": 218739,
            "views": 517360693,
            "vt": 0
        },
        "status": 13,
        "styles": [
            "小说改",
            "奇幻",
            "战斗",
            "魔法"
        ],
        "subtitle": "已观看5.2亿次",
        "title": "关于我转生变成史莱姆这档事",
        "total": 28,
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
    let data = resp.data.unwrap();
    assert_eq!(data.season_id, 6001);
    assert_eq!(data.episodes.len(), 1);
    assert_eq!(data.episodes[0].bvid, "BV1xx411c7mD");
    assert_eq!(data.stat.unwrap().views, 10000);
}
