#[derive(Debug, Clone, Copy)]
pub enum InternationalDialingPrefix {
    /// 中国大陆
    ChineseMainland,
    /// 中国香港特别行政区
    HongKongSpecialAdministrativeRegionofChina,
    /// 中国澳门特别行政区
    MacaoSpecialAdministrativeRegionofChina,
    /// 中国台湾
    TaiwanChina,
    /// 美国或加拿大
    TheUnitedStatesOrCanada,
    /// 比利时
    Belgium,
    /// 澳大利亚
    Australia,
    /// 法国
    France,
    /// 日本
    Japan,
    /// 新加坡
    Singapore,
    /// 韩国
    SouthKorea,
    /// 马来西亚
    Malaysia,
    /// 英国
    TheUnitedKingdom,
    /// 意大利
    Italy,
    /// 德国
    Germany,
    /// 俄罗斯
    Russia,
    /// 新西兰
    NewZealand,
    /// 瓦利斯群岛和富图纳群岛
    WallisIslandsandFutunaIslands,
    /// 葡萄牙
    Portugal,
    /// 帕劳
    Palau,
    /// 诺福克岛
    NorfolkIsland,
    /// 挪威
    Norway,
    /// 纽埃岛
    NiueIsland,
    /// 尼日利亚
    Nigeria,
    /// 尼日尔
    Niger,
    /// 尼加拉瓜
    Nicaragua,
    /// 尼泊尔
    Nepal,
    /// 瑙鲁
    Nauru,
    /// 格鲁吉亚
    Georgia,
    /// 瑞典
    Sweden,
    /// 沙特阿拉伯
    SaudiArabia,
    /// 桑给巴尔岛
    ZanzibarIsland,
    /// 塞舌尔共和国
    RepublicofSeychelles,
    /// 塞浦路斯
    Cyprus,
    /// 塞内加尔
    Senegal,
    /// 塞拉利昂
    SierraLeone,
    /// 萨摩亚，东部
    SamoaEastern,
    /// 萨摩亚，西部
    SamoaWest,
    /// 萨尔瓦多
    ElSalvador,
    /// 瑞士
    Switzerland,
    /// 圣多美和普林西比
    SaoTomeandPrincipe,
    /// 塞尔维亚
    Serbia,
    /// 南非
    SouthAfrica,
    /// 毛里塔尼亚
    Mauritania,
    /// 毛里求斯
    Mauritius,
    /// 马歇尔岛
    Marshallislands,
    /// 马提尼克岛
    MartiniqueIsland,
    /// 马其顿
    Macedonia,
    /// 马里亚纳岛
    MarianaIslands,
    /// 马里
    Mali,
    /// 马拉维
    Malawi,
    /// 马耳他
    Malta,
    /// 马尔代夫
    Maldives,
    /// 蒙古
    Mongolia,
    /// 蒙特塞拉特岛
    MontserratIsland,
    /// 纳米比亚
    Namibia,
    /// 墨西哥
    Mexico,
    /// 莫桑比克
    Mozambique,
    /// 摩纳哥
    Monaco,
    /// 摩洛哥
    Morocco,
    /// 摩尔多瓦
    Moldova,
    /// 缅甸
    Myanmar,
    /// 密克罗尼西亚
    Micronesia,
    /// 秘鲁
    Peru,
    /// 孟加拉国
    Bangladesh,
    /// 马达加斯加
    Madagascar,
    /// 圣卢西亚
    SaintLucia,
    /// 智利
    Chile,
    /// 牙买加
    Jamaica,
    /// 叙利亚
    Syria,
    /// 匈牙利
    Hungary,
    /// 科特迪瓦
    IvoryCoast,
    /// 希腊
    Greece,
    /// 西班牙
    Spain,
    /// 乌兹别克斯坦
    Uzbekistan,
    /// 乌拉圭
    Uruguay,
    /// 乌克兰
    Ukraine,
    /// 乌干达
    Uganda,
    /// 亚美尼亚
    Armenia,
    /// 也门
    Yemen,
    /// 直布罗陀
    Gibraltar,
    /// 乍得
    Chad,
    /// 赞比亚
    Zambia,
    /// 越南
    Vietnam,
    /// 约旦
    Jordan,
    /// 印尼
    Indonesia,
    /// 印度
    India,
    /// 以色列
    Israel,
    /// 伊朗
    Iran,
    /// 伊拉克
    Iraq,
    /// 文莱
    Brunei,
    /// 委内瑞拉
    Venezuela,
    /// 维珍群岛(英属)
    VirginIslandsBritish,
    /// 泰国
    Thailand,
    /// 索马里
    Somalia,
    /// 所罗门群岛
    SolomonIslands,
    /// 苏里南
    Suriname,
    /// 苏丹
    Sudan,
    /// 斯威士兰
    Eswatini,
    /// 斯洛文尼亚
    Slovenia,
    /// 斯洛伐克
    Slovakia,
    /// 斯里兰卡
    SriLanka,
    /// 圣皮埃尔和密克隆群岛
    SaintPierreandMiquelonIslands,
    /// 坦桑尼亚
    Tanzania,
    /// 汤加
    Tonga,
    /// 维珍群岛(美属)
    VirginIslandsUnitedStates,
    /// 瓦努阿图
    Vanuatu,
    /// 托克劳岛
    TokelauIsland,
    /// 土库曼斯坦
    Turkmenistan,
    /// 土耳其
    Turkiye,
    /// 图瓦卢
    Tuvalu,
    /// 突尼斯
    Tunisia,
    /// 阿森松岛
    AscensionIsland,
    /// 特立尼达和多巴哥
    TrinidadandTobago,
    /// 特克斯和凯科斯
    TurksandCaicos,
    /// 圣马力诺
    SanMarino,
    /// 法属圭亚那
    FrenchGuiana,
    /// 不丹
    Bhutan,
    /// 博茨瓦纳
    Botswana,
    /// 伯利兹
    Belize,
    /// 玻利维亚
    Bolivia,
    /// 波兰
    Poland,
    /// 波黑
    BosniaandHerzegovina,
    /// 波多黎各
    PuertoRico,
    /// 冰岛
    Iceland,
    /// 贝宁
    Benin,
    /// 保加利亚
    Bulgaria,
    /// 布基纳法索
    BurkinaFaso,
    /// 布隆迪
    Burundi,
    /// 法属波利尼西亚
    FrenchPolynesia,
    /// 法罗岛
    Faro,
    /// 厄立特里亚
    Eritrea,
    /// 厄瓜多尔
    Ecuador,
    /// 多米尼加代表
    DominicanRep,
    /// 多米尼加
    Dominican,
    /// 多哥
    Togo,
    /// 迪戈加西亚岛
    DiegoGarcia,
    /// 丹麦
    Denmark,
    /// 赤道几内亚
    EquatorialGuinea,
    /// 百慕大群岛
    BermudaIslands,
    /// 白俄罗斯
    Belarus,
    /// 巴西
    Brazil,
    /// 爱尔兰
    Ireland,
    /// 埃塞俄比亚
    Ethiopia,
    /// 埃及
    Egypt,
    /// 阿塞拜疆
    Azerbaijan,
    /// 阿曼
    Oman,
    /// 阿联酋
    UnitedArabEmirates,
    /// 阿根廷
    Argentina,
    /// 阿富汗
    Afghanistan,
    /// 阿尔及利亚
    Algeria,
    /// 阿尔巴尼亚
    Albania,
    /// 爱沙尼亚
    Estonia,
    /// 安道尔
    Andorra,
    /// 巴拿马
    Panama,
    /// 巴林
    Bahrain,
    /// 巴拉圭
    Paraguay,
    /// 巴基斯坦
    Pakistan,
    /// 巴哈马群岛
    BahamasIslands,
    /// 巴布亚新几内亚
    Papuanewguinea,
    /// 巴巴多斯
    Barbados,
    /// 奥地利
    Austria,
    /// 安提瓜岛和巴布达
    AntiguaandBarbuda,
    /// 安哥拉
    Angola,
    /// 非洲中部
    CentralAfrica,
    /// 罗马尼亚
    Romania,
    /// 科威特
    Kuwait,
    /// 科摩罗
    Comoros,
    /// 开曼群岛
    CaymanIslands,
    /// 卡塔尔
    Qatar,
    /// 喀麦隆
    Cameroon,
    /// 聚会岛
    GatheringIsland,
    /// 津巴布韦
    Zimbabwe,
    /// 捷克
    Czech,
    /// 柬埔寨
    Cambodia,
    /// 加蓬
    Gabon,
    /// 克罗地亚
    Croatia,
    /// 肯尼亚
    Kenya,
    /// 卢旺达
    Rwanda,
    /// 卢森堡
    Luxembourg,
    /// 利比亚
    Libya,
    /// 利比里亚
    Liberia,
    /// 立陶宛
    Lithuania,
    /// 黎巴嫩
    Lebanon,
    /// 老挝
    Laos,
    /// 莱索托
    Lesotho,
    /// 拉脱维亚
    Latvia,
    /// 库克岛
    CookIsland,
    /// 加纳
    Ghana,
    /// 几内亚比绍
    GuineaBissau,
    /// 几内亚
    Guinea,
    /// 格林纳达
    Grenada,
    /// 哥斯达黎加
    CostaRica,
    /// 哥伦比亚
    Colombia,
    /// 刚果(金)
    DemocraticRepublicoftheCongo,
    /// 刚果
    Congo,
    /// 冈比亚
    TheGambia,
    /// 福克兰岛
    FalklandIslands,
    /// 佛得角
    CapeVerde,
    /// 芬兰
    Finland,
    /// 斐济
    Fiji,
    /// 格陵兰岛
    Greenland,
    /// 古巴
    Cuba,
    /// 吉尔吉斯斯坦
    Kyrgyzstan,
    /// 吉布提
    Djibouti,
    /// 基里巴斯
    Kiribati,
    /// 维克岛
    WakeIsland,
    /// 洪都拉斯
    Honduras,
    /// 荷兰
    Netherlands,
    /// 朝鲜
    NorthKorea,
    /// 海地
    Haiti,
    /// 关岛
    Guam,
    /// 瓜德罗普岛
    GuadeloupeIsland,
    /// 菲律宾
    Philippines,
    /// 其他
    Other(u16),
}

impl From<u16> for InternationalDialingPrefix {
    fn from(code: u16) -> Self {
        match code {
            86 => Self::ChineseMainland,
            852 => Self::HongKongSpecialAdministrativeRegionofChina,
            853 => Self::MacaoSpecialAdministrativeRegionofChina,
            886 => Self::TaiwanChina,
            1 => Self::TheUnitedStatesOrCanada,
            32 => Self::Belgium,
            61 => Self::Australia,
            33 => Self::France,
            81 => Self::Japan,
            65 => Self::Singapore,
            82 => Self::SouthKorea,
            60 => Self::Malaysia,
            44 => Self::TheUnitedKingdom,
            39 => Self::Italy,
            49 => Self::Germany,
            7 => Self::Russia,
            64 => Self::NewZealand,
            1681 => Self::WallisIslandsandFutunaIslands,
            351 => Self::Portugal,
            680 => Self::Palau,
            672 => Self::NorfolkIsland,
            47 => Self::Norway,
            683 => Self::NiueIsland,
            234 => Self::Nigeria,
            227 => Self::Niger,
            505 => Self::Nicaragua,
            977 => Self::Nepal,
            674 => Self::Nauru,
            995 => Self::Georgia,
            46 => Self::Sweden,
            966 => Self::SaudiArabia,
            259 => Self::ZanzibarIsland,
            248 => Self::RepublicofSeychelles,
            357 => Self::Cyprus,
            221 => Self::Senegal,
            232 => Self::SierraLeone,
            684 => Self::SamoaEastern,
            685 => Self::SamoaWest,
            503 => Self::ElSalvador,
            41 => Self::Switzerland,
            239 => Self::SaoTomeandPrincipe,
            381 => Self::Serbia,
            27 => Self::SouthAfrica,
            222 => Self::Mauritania,
            230 => Self::Mauritius,
            692 => Self::Marshallislands,
            596 => Self::MartiniqueIsland,
            389 => Self::Macedonia,
            1670 => Self::MarianaIslands,
            223 => Self::Mali,
            265 => Self::Malawi,
            356 => Self::Malta,
            960 => Self::Maldives,
            976 => Self::Mongolia,
            1664 => Self::MontserratIsland,
            264 => Self::Namibia,
            52 => Self::Mexico,
            258 => Self::Mozambique,
            377 => Self::Monaco,
            212 => Self::Morocco,
            373 => Self::Moldova,
            95 => Self::Myanmar,
            691 => Self::Micronesia,
            51 => Self::Peru,
            880 => Self::Bangladesh,
            261 => Self::Madagascar,
            1784 => Self::SaintLucia,
            56 => Self::Chile,
            1876 => Self::Jamaica,
            963 => Self::Syria,
            36 => Self::Hungary,
            225 => Self::IvoryCoast,
            30 => Self::Greece,
            34 => Self::Spain,
            998 => Self::Uzbekistan,
            598 => Self::Uruguay,
            380 => Self::Ukraine,
            256 => Self::Uganda,
            374 => Self::Armenia,
            967 => Self::Yemen,
            350 => Self::Gibraltar,
            235 => Self::Chad,
            260 => Self::Zambia,
            84 => Self::Vietnam,
            962 => Self::Jordan,
            62 => Self::Indonesia,
            91 => Self::India,
            972 => Self::Israel,
            98 => Self::Iran,
            964 => Self::Iraq,
            673 => Self::Brunei,
            58 => Self::Venezuela,
            1284 => Self::VirginIslandsBritish,
            66 => Self::Thailand,
            252 => Self::Somalia,
            677 => Self::SolomonIslands,
            597 => Self::Suriname,
            249 => Self::Sudan,
            268 => Self::Eswatini,
            386 => Self::Slovenia,
            421 => Self::Slovakia,
            94 => Self::SriLanka,
            508 => Self::SaintPierreandMiquelonIslands,
            255 => Self::Tanzania,
            676 => Self::Tonga,
            1340 => Self::VirginIslandsUnitedStates,
            678 => Self::Vanuatu,
            690 => Self::TokelauIsland,
            993 => Self::Turkmenistan,
            90 => Self::Turkiye,
            688 => Self::Tuvalu,
            216 => Self::Tunisia,
            247 => Self::AscensionIsland,
            1868 => Self::TrinidadandTobago,
            1649 => Self::TurksandCaicos,
            378 => Self::SanMarino,
            594 => Self::FrenchGuiana,
            975 => Self::Bhutan,
            267 => Self::Botswana,
            501 => Self::Belize,
            591 => Self::Bolivia,
            48 => Self::Poland,
            387 => Self::BosniaandHerzegovina,
            1787 => Self::PuertoRico,
            354 => Self::Iceland,
            229 => Self::Benin,
            359 => Self::Bulgaria,
            226 => Self::BurkinaFaso,
            257 => Self::Burundi,
            689 => Self::FrenchPolynesia,
            298 => Self::Faro,
            291 => Self::Eritrea,
            593 => Self::Ecuador,
            1809 => Self::DominicanRep,
            1767 => Self::Dominican,
            228 => Self::Togo,
            246 => Self::DiegoGarcia,
            45 => Self::Denmark,
            240 => Self::EquatorialGuinea,
            1441 => Self::BermudaIslands,
            375 => Self::Belarus,
            55 => Self::Brazil,
            353 => Self::Ireland,
            251 => Self::Ethiopia,
            20 => Self::Egypt,
            994 => Self::Azerbaijan,
            968 => Self::Oman,
            971 => Self::UnitedArabEmirates,
            54 => Self::Argentina,
            93 => Self::Afghanistan,
            213 => Self::Algeria,
            355 => Self::Albania,
            372 => Self::Estonia,
            376 => Self::Andorra,
            507 => Self::Panama,
            973 => Self::Bahrain,
            595 => Self::Paraguay,
            92 => Self::Pakistan,
            1242 => Self::BahamasIslands,
            675 => Self::Papuanewguinea,
            1246 => Self::Barbados,
            43 => Self::Austria,
            1268 => Self::AntiguaandBarbuda,
            244 => Self::Angola,
            236 => Self::CentralAfrica,
            40 => Self::Romania,
            965 => Self::Kuwait,
            269 => Self::Comoros,
            1345 => Self::CaymanIslands,
            974 => Self::Qatar,
            237 => Self::Cameroon,
            262 => Self::GatheringIsland,
            263 => Self::Zimbabwe,
            420 => Self::Czech,
            855 => Self::Cambodia,
            241 => Self::Gabon,
            385 => Self::Croatia,
            254 => Self::Kenya,
            250 => Self::Rwanda,
            352 => Self::Luxembourg,
            218 => Self::Libya,
            231 => Self::Liberia,
            370 => Self::Lithuania,
            961 => Self::Lebanon,
            856 => Self::Laos,
            266 => Self::Lesotho,
            371 => Self::Latvia,
            682 => Self::CookIsland,
            233 => Self::Ghana,
            245 => Self::GuineaBissau,
            224 => Self::Guinea,
            1473 => Self::Grenada,
            506 => Self::CostaRica,
            57 => Self::Colombia,
            243 => Self::DemocraticRepublicoftheCongo,
            242 => Self::Congo,
            220 => Self::TheGambia,
            500 => Self::FalklandIslands,
            238 => Self::CapeVerde,
            358 => Self::Finland,
            679 => Self::Fiji,
            299 => Self::Greenland,
            53 => Self::Cuba,
            996 => Self::Kyrgyzstan,
            253 => Self::Djibouti,
            686 => Self::Kiribati,
            1808 => Self::WakeIsland,
            504 => Self::Honduras,
            31 => Self::Netherlands,
            850 => Self::NorthKorea,
            509 => Self::Haiti,
            1671 => Self::Guam,
            590 => Self::GuadeloupeIsland,
            63 => Self::Philippines,
            _ => Self::Other(code),
        }
    }
}

impl From<InternationalDialingPrefix> for u16 {
    fn from(c: InternationalDialingPrefix) -> Self {
        match c {
            InternationalDialingPrefix::ChineseMainland => 86,
            InternationalDialingPrefix::HongKongSpecialAdministrativeRegionofChina => 852,
            InternationalDialingPrefix::MacaoSpecialAdministrativeRegionofChina => 853,
            InternationalDialingPrefix::TaiwanChina => 886,
            InternationalDialingPrefix::TheUnitedStatesOrCanada => 1,
            InternationalDialingPrefix::Belgium => 32,
            InternationalDialingPrefix::Australia => 61,
            InternationalDialingPrefix::France => 33,
            InternationalDialingPrefix::Japan => 81,
            InternationalDialingPrefix::Singapore => 65,
            InternationalDialingPrefix::SouthKorea => 82,
            InternationalDialingPrefix::Malaysia => 60,
            InternationalDialingPrefix::TheUnitedKingdom => 44,
            InternationalDialingPrefix::Italy => 39,
            InternationalDialingPrefix::Germany => 49,
            InternationalDialingPrefix::Russia => 7,
            InternationalDialingPrefix::NewZealand => 64,
            InternationalDialingPrefix::WallisIslandsandFutunaIslands => 1681,
            InternationalDialingPrefix::Portugal => 351,
            InternationalDialingPrefix::Palau => 680,
            InternationalDialingPrefix::NorfolkIsland => 672,
            InternationalDialingPrefix::Norway => 47,
            InternationalDialingPrefix::NiueIsland => 683,
            InternationalDialingPrefix::Nigeria => 234,
            InternationalDialingPrefix::Niger => 227,
            InternationalDialingPrefix::Nicaragua => 505,
            InternationalDialingPrefix::Nepal => 977,
            InternationalDialingPrefix::Nauru => 674,
            InternationalDialingPrefix::Georgia => 995,
            InternationalDialingPrefix::Sweden => 46,
            InternationalDialingPrefix::SaudiArabia => 966,
            InternationalDialingPrefix::ZanzibarIsland => 259,
            InternationalDialingPrefix::RepublicofSeychelles => 248,
            InternationalDialingPrefix::Cyprus => 357,
            InternationalDialingPrefix::Senegal => 221,
            InternationalDialingPrefix::SierraLeone => 232,
            InternationalDialingPrefix::SamoaEastern => 684,
            InternationalDialingPrefix::SamoaWest => 685,
            InternationalDialingPrefix::ElSalvador => 503,
            InternationalDialingPrefix::Switzerland => 41,
            InternationalDialingPrefix::SaoTomeandPrincipe => 239,
            InternationalDialingPrefix::Serbia => 381,
            InternationalDialingPrefix::SouthAfrica => 27,
            InternationalDialingPrefix::Mauritania => 222,
            InternationalDialingPrefix::Mauritius => 230,
            InternationalDialingPrefix::Marshallislands => 692,
            InternationalDialingPrefix::MartiniqueIsland => 596,
            InternationalDialingPrefix::Macedonia => 389,
            InternationalDialingPrefix::MarianaIslands => 1670,
            InternationalDialingPrefix::Mali => 223,
            InternationalDialingPrefix::Malawi => 265,
            InternationalDialingPrefix::Malta => 356,
            InternationalDialingPrefix::Maldives => 960,
            InternationalDialingPrefix::Mongolia => 976,
            InternationalDialingPrefix::MontserratIsland => 1664,
            InternationalDialingPrefix::Namibia => 264,
            InternationalDialingPrefix::Mexico => 52,
            InternationalDialingPrefix::Mozambique => 258,
            InternationalDialingPrefix::Monaco => 377,
            InternationalDialingPrefix::Morocco => 212,
            InternationalDialingPrefix::Moldova => 373,
            InternationalDialingPrefix::Myanmar => 95,
            InternationalDialingPrefix::Micronesia => 691,
            InternationalDialingPrefix::Peru => 51,
            InternationalDialingPrefix::Bangladesh => 880,
            InternationalDialingPrefix::Madagascar => 261,
            InternationalDialingPrefix::SaintLucia => 1784,
            InternationalDialingPrefix::Chile => 56,
            InternationalDialingPrefix::Jamaica => 1876,
            InternationalDialingPrefix::Syria => 963,
            InternationalDialingPrefix::Hungary => 36,
            InternationalDialingPrefix::IvoryCoast => 225,
            InternationalDialingPrefix::Greece => 30,
            InternationalDialingPrefix::Spain => 34,
            InternationalDialingPrefix::Uzbekistan => 998,
            InternationalDialingPrefix::Uruguay => 598,
            InternationalDialingPrefix::Ukraine => 380,
            InternationalDialingPrefix::Uganda => 256,
            InternationalDialingPrefix::Armenia => 374,
            InternationalDialingPrefix::Yemen => 967,
            InternationalDialingPrefix::Gibraltar => 350,
            InternationalDialingPrefix::Chad => 235,
            InternationalDialingPrefix::Zambia => 260,
            InternationalDialingPrefix::Vietnam => 84,
            InternationalDialingPrefix::Jordan => 962,
            InternationalDialingPrefix::Indonesia => 62,
            InternationalDialingPrefix::India => 91,
            InternationalDialingPrefix::Israel => 972,
            InternationalDialingPrefix::Iran => 98,
            InternationalDialingPrefix::Iraq => 964,
            InternationalDialingPrefix::Brunei => 673,
            InternationalDialingPrefix::Venezuela => 58,
            InternationalDialingPrefix::VirginIslandsBritish => 1284,
            InternationalDialingPrefix::Thailand => 66,
            InternationalDialingPrefix::Somalia => 252,
            InternationalDialingPrefix::SolomonIslands => 677,
            InternationalDialingPrefix::Suriname => 597,
            InternationalDialingPrefix::Sudan => 249,
            InternationalDialingPrefix::Eswatini => 268,
            InternationalDialingPrefix::Slovenia => 386,
            InternationalDialingPrefix::Slovakia => 421,
            InternationalDialingPrefix::SriLanka => 94,
            InternationalDialingPrefix::SaintPierreandMiquelonIslands => 508,
            InternationalDialingPrefix::Tanzania => 255,
            InternationalDialingPrefix::Tonga => 676,
            InternationalDialingPrefix::VirginIslandsUnitedStates => 1340,
            InternationalDialingPrefix::Vanuatu => 678,
            InternationalDialingPrefix::TokelauIsland => 690,
            InternationalDialingPrefix::Turkmenistan => 993,
            InternationalDialingPrefix::Turkiye => 90,
            InternationalDialingPrefix::Tuvalu => 688,
            InternationalDialingPrefix::Tunisia => 216,
            InternationalDialingPrefix::AscensionIsland => 247,
            InternationalDialingPrefix::TrinidadandTobago => 1868,
            InternationalDialingPrefix::TurksandCaicos => 1649,
            InternationalDialingPrefix::SanMarino => 378,
            InternationalDialingPrefix::FrenchGuiana => 594,
            InternationalDialingPrefix::Bhutan => 975,
            InternationalDialingPrefix::Botswana => 267,
            InternationalDialingPrefix::Belize => 501,
            InternationalDialingPrefix::Bolivia => 591,
            InternationalDialingPrefix::Poland => 48,
            InternationalDialingPrefix::BosniaandHerzegovina => 387,
            InternationalDialingPrefix::PuertoRico => 1787,
            InternationalDialingPrefix::Iceland => 354,
            InternationalDialingPrefix::Benin => 229,
            InternationalDialingPrefix::Bulgaria => 359,
            InternationalDialingPrefix::BurkinaFaso => 226,
            InternationalDialingPrefix::Burundi => 257,
            InternationalDialingPrefix::FrenchPolynesia => 689,
            InternationalDialingPrefix::Faro => 298,
            InternationalDialingPrefix::Eritrea => 291,
            InternationalDialingPrefix::Ecuador => 593,
            InternationalDialingPrefix::DominicanRep => 1809,
            InternationalDialingPrefix::Dominican => 1767,
            InternationalDialingPrefix::Togo => 228,
            InternationalDialingPrefix::DiegoGarcia => 246,
            InternationalDialingPrefix::Denmark => 45,
            InternationalDialingPrefix::EquatorialGuinea => 240,
            InternationalDialingPrefix::BermudaIslands => 1441,
            InternationalDialingPrefix::Belarus => 375,
            InternationalDialingPrefix::Brazil => 55,
            InternationalDialingPrefix::Ireland => 353,
            InternationalDialingPrefix::Ethiopia => 251,
            InternationalDialingPrefix::Egypt => 20,
            InternationalDialingPrefix::Azerbaijan => 994,
            InternationalDialingPrefix::Oman => 968,
            InternationalDialingPrefix::UnitedArabEmirates => 971,
            InternationalDialingPrefix::Argentina => 54,
            InternationalDialingPrefix::Afghanistan => 93,
            InternationalDialingPrefix::Algeria => 213,
            InternationalDialingPrefix::Albania => 355,
            InternationalDialingPrefix::Estonia => 372,
            InternationalDialingPrefix::Andorra => 376,
            InternationalDialingPrefix::Panama => 507,
            InternationalDialingPrefix::Bahrain => 973,
            InternationalDialingPrefix::Paraguay => 595,
            InternationalDialingPrefix::Pakistan => 92,
            InternationalDialingPrefix::BahamasIslands => 1242,
            InternationalDialingPrefix::Papuanewguinea => 675,
            InternationalDialingPrefix::Barbados => 1246,
            InternationalDialingPrefix::Austria => 43,
            InternationalDialingPrefix::AntiguaandBarbuda => 1268,
            InternationalDialingPrefix::Angola => 244,
            InternationalDialingPrefix::CentralAfrica => 236,
            InternationalDialingPrefix::Romania => 40,
            InternationalDialingPrefix::Kuwait => 965,
            InternationalDialingPrefix::Comoros => 269,
            InternationalDialingPrefix::CaymanIslands => 1345,
            InternationalDialingPrefix::Qatar => 974,
            InternationalDialingPrefix::Cameroon => 237,
            InternationalDialingPrefix::GatheringIsland => 262,
            InternationalDialingPrefix::Zimbabwe => 263,
            InternationalDialingPrefix::Czech => 420,
            InternationalDialingPrefix::Cambodia => 855,
            InternationalDialingPrefix::Gabon => 241,
            InternationalDialingPrefix::Croatia => 385,
            InternationalDialingPrefix::Kenya => 254,
            InternationalDialingPrefix::Rwanda => 250,
            InternationalDialingPrefix::Luxembourg => 352,
            InternationalDialingPrefix::Libya => 218,
            InternationalDialingPrefix::Liberia => 231,
            InternationalDialingPrefix::Lithuania => 370,
            InternationalDialingPrefix::Lebanon => 961,
            InternationalDialingPrefix::Laos => 856,
            InternationalDialingPrefix::Lesotho => 266,
            InternationalDialingPrefix::Latvia => 371,
            InternationalDialingPrefix::CookIsland => 682,
            InternationalDialingPrefix::Ghana => 233,
            InternationalDialingPrefix::GuineaBissau => 245,
            InternationalDialingPrefix::Guinea => 224,
            InternationalDialingPrefix::Grenada => 1473,
            InternationalDialingPrefix::CostaRica => 506,
            InternationalDialingPrefix::Colombia => 57,
            InternationalDialingPrefix::DemocraticRepublicoftheCongo => 243,
            InternationalDialingPrefix::Congo => 242,
            InternationalDialingPrefix::TheGambia => 220,
            InternationalDialingPrefix::FalklandIslands => 500,
            InternationalDialingPrefix::CapeVerde => 238,
            InternationalDialingPrefix::Finland => 358,
            InternationalDialingPrefix::Fiji => 679,
            InternationalDialingPrefix::Greenland => 299,
            InternationalDialingPrefix::Cuba => 53,
            InternationalDialingPrefix::Kyrgyzstan => 996,
            InternationalDialingPrefix::Djibouti => 253,
            InternationalDialingPrefix::Kiribati => 686,
            InternationalDialingPrefix::WakeIsland => 1808,
            InternationalDialingPrefix::Honduras => 504,
            InternationalDialingPrefix::Netherlands => 31,
            InternationalDialingPrefix::NorthKorea => 850,
            InternationalDialingPrefix::Haiti => 509,
            InternationalDialingPrefix::Guam => 1671,
            InternationalDialingPrefix::GuadeloupeIsland => 590,
            InternationalDialingPrefix::Philippines => 63,
            InternationalDialingPrefix::Other(code) => code,
        }
    }
}

pub mod serde_international_dialing_prefix {
    use super::InternationalDialingPrefix;

    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<InternationalDialingPrefix, D::Error>
    where
        D: Deserializer<'de>,
    {
        let t = u16::deserialize(deserializer)?;
        Ok(t.into())
    }

    pub fn serialize<S>(
        international_dialing_prefix: &InternationalDialingPrefix,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(From::from(*international_dialing_prefix))
    }
}
