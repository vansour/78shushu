use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Map {
    #[serde(rename = "零号大坝（常规）")]
    ZeroDamRegular,
    #[serde(rename = "零号大坝（机密）")]
    ZeroDamClassified,
    #[serde(rename = "零号大坝（前夜）")]
    ZeroDamEveningBefore,
    #[serde(rename = "零号大坝（长夜）")]
    ZeroDamLongNight,
    #[serde(rename = "零号大坝（终夜）")]
    ZeroDamFinalNight,
    #[serde(rename = "长弓溪谷（常规）")]
    LongbowValleyRegular,
    #[serde(rename = "长弓溪谷（机密）")]
    LongbowValleyClassified,
    #[serde(rename = "巴克什（机密）")]
    BakshClassified,
    #[serde(rename = "巴克什（绝密）")]
    BakshTopSecret,
    #[serde(rename = "航天基地（机密）")]
    SpaceBaseClassified,
    #[serde(rename = "航天基地（绝密）")]
    SpaceBaseTopSecret,
    #[serde(rename = "潮汐监狱（绝密）")]
    TidalPrisonTopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "红狼")]
    HongLang,
    #[serde(rename = "威龙")]
    WeiLong,
    #[serde(rename = "无名")]
    WuMing,
    #[serde(rename = "疾风")]
    JiFeng,
    #[serde(rename = "蜂医")]
    FengYi,
    #[serde(rename = "蛊")]
    Gu,
    #[serde(rename = "牧羊人")]
    MuYangRen,
    #[serde(rename = "乌鲁鲁")]
    WuLuLu,
    #[serde(rename = "深蓝")]
    ShenLan,
    #[serde(rename = "露娜")]
    LuNa,
    #[serde(rename = "骇爪")]
    HaiZhua,
    #[serde(rename = "银翼")]
    YinYi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimaryWeapon {
    //步枪
    #[serde(rename = "MK47突击步枪")]
    MK47,
    #[serde(rename = "KC17突击步枪")]
    KC17,
    #[serde(rename = "K437突击步枪")]
    K437,
    #[serde(rename = "腾龙突击步枪")]
    TengLong,
    #[serde(rename = "AS Val突击步枪")]
    ASVal,
    #[serde(rename = "CAR-15突击步枪")]
    CAR15,
    #[serde(rename = "PTR-32突击步枪")]
    PTR32,
    #[serde(rename = "G3突击步枪")]
    G3,
    #[serde(rename = "SCAR-H突击步枪")]
    SCARH,
    #[serde(rename = "AK-12突击步枪")]
    AK12,
    #[serde(rename = "SG552突击步枪")]
    SG552,
    #[serde(rename = "M7突击步枪")]
    M7,
    #[serde(rename = "AUG突击步枪")]
    AUG,
    #[serde(rename = "M16A4突击步枪")]
    M16A4,
    #[serde(rename = "K416突击步枪")]
    K416,
    #[serde(rename = "AKS-74U突击步枪")]
    AKS74U,
    #[serde(rename = "QBZ95-1突击步枪")]
    QBZ951,
    #[serde(rename = "AKM突击步枪")]
    AKM,
    #[serde(rename = "M4A1突击步枪")]
    M4A1,

    //冲锋枪
    #[serde(rename = "QCQ171冲锋枪")]
    QCQ171,
    #[serde(rename = "MP7冲锋枪")]
    MP7,
    #[serde(rename = "勇士冲锋枪")]
    YongShi,
    #[serde(rename = "SR-3M紧凑突击步枪")]
    SR3M,
    #[serde(rename = "SMG-45冲锋枪")]
    SMG45,
    #[serde(rename = "野牛冲锋枪")]
    YeNiu,
    #[serde(rename = "UZI冲锋枪")]
    UZI,
    #[serde(rename = "Vector冲锋枪")]
    Vector,
    #[serde(rename = "P90冲锋枪")]
    P90,
    #[serde(rename = "MP5冲锋枪")]
    MP5,

    //霰弹枪
    #[serde(rename = "725双管霰弹枪")]
    O725,
    #[serde(rename = "M870霰弹枪")]
    M870,
    #[serde(rename = "S12K霰弹枪")]
    S12K,
    #[serde(rename = "M1014霰弹枪")]
    M1014,

    //轻机枪
    #[serde(rename = "QJB201轻机枪")]
    QJB201,
    #[serde(rename = "M250轻机枪")]
    M250,
    #[serde(rename = "M249轻机枪")]
    M249,
    #[serde(rename = "PKM轻机枪")]
    PKM,

    //精确射手步枪
    #[serde(rename = "Marlin杠杆步枪")]
    Marlin,
    #[serde(rename = "PSG-1射手步枪")]
    PSG1,
    #[serde(rename = "SR9射手步枪")]
    SR9,
    #[serde(rename = "SR-25射手步枪")]
    SR25,
    #[serde(rename = "SKS射手步枪")]
    SKS,
    #[serde(rename = "M14射手步枪")]
    M14,
    #[serde(rename = "SVD狙击步枪")]
    SVD,
    #[serde(rename = "VSS射手步枪")]
    VSS,
    #[serde(rename = "Mini-14射手步枪")]
    Mini14,

    //狙击步枪
    #[serde(rename = "AWP狙击步枪")]
    AWP,
    #[serde(rename = "M700狙击步枪")]
    M700,
    #[serde(rename = "R93狙击步枪")]
    R93,
    #[serde(rename = "SV-98狙击步枪")]
    SV98,

    //手枪
    #[serde(rename = "M1911")]
    M1911,
    #[serde(rename = "G17")]
    G17,
    #[serde(rename = "G18")]
    G18,
    #[serde(rename = "93R")]
    O93R,
    #[serde(rename = "沙漠之鹰")]
    DesertEagle,
    #[serde(rename = ".357左轮")]
    O357Revolver,
    #[serde(rename = "QSZ92G")]
    QSZ92G,

    //特殊武器
    #[serde(rename = "复合弓")]
    CompoundBow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Helmet {
    #[serde(rename = "老式钢盔")]
    OldSteelHelmet,
    #[serde(rename = "安保头盔")]
    SecurityHelmet,
    #[serde(rename = "奔尼帽")]
    BennyHat,
    #[serde(rename = "户外棒球帽")]
    OutdoorBaseballCap,
    #[serde(rename = "H01 战术头盔")]
    H01TacticalHelmet,
    #[serde(rename = "DRO 战术头盔")]
    DROTacticalHelmet,
    #[serde(rename = "复古摩托头盔")]
    VintageMotorcycleHelmet,
    #[serde(rename = "MC 防弹头盔")]
    MCBulletproofHelmet,
    #[serde(rename = "防暴头盔")]
    RiotHelmet,
    #[serde(rename = "H07 战术头盔")]
    H07TacticalHelmet,
    #[serde(rename = "DAS 防弹头盔")]
    DASBulletproofHelmet,
    #[serde(rename = "MC201 防弹头盔")]
    MC201BulletproofHelmet,
    #[serde(rename = "D6 战术头盔")]
    D6TacticalHelmet,
    #[serde(rename = "MHS 战术头盔")]
    MHSTacticalHelmet,
    #[serde(rename = "GT1战术头盔")]
    GT1TacticalHelmet,
    #[serde(rename = "DICH 训练头盔")]
    DICHTrainingHelmet,
    #[serde(rename = "GN 久站重型夜市头盔")]
    GNHeavyNightMarketHelmet,
    #[serde(rename = "Mask-1铁臂头盔")]
    Mask1IronArmHelmet,
    #[serde(rename = "H09 防暴头盔")]
    H09RiotHelmet,
    #[serde(rename = "GN 重型头盔")]
    GNHeavyHelmet,
    #[serde(rename = "DICH-1战术头盔")]
    DICH1TacticalHelmet,
    #[serde(rename = "GN重型夜视头盔")]
    GNHeavyNightVisionHelmet,
    #[serde(rename = "H70精英头盔")]
    H70EliteHelmet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Armor {
    #[serde(rename = "轻型护甲")]
    Light,
    #[serde(rename = "中型护甲")]
    Medium,
    #[serde(rename = "重型护甲")]
    Heavy,
    #[serde(rename = "战术背心")]
    TacticalVest,
    #[serde(rename = "防弹衣")]
    BodyArmor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RandomLoadout {
    pub map: Map,
    pub operator: Operator,
    pub primary_weapon: PrimaryWeapon,
    pub helmet: Helmet,
    pub armor: Armor,
}

impl Map {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ZeroDamRegular,
            Self::ZeroDamClassified,
            Self::ZeroDamEveningBefore,
            Self::ZeroDamLongNight,
            Self::ZeroDamFinalNight,
            Self::LongbowValleyRegular,
            Self::LongbowValleyClassified,
            Self::BakshClassified,
            Self::BakshTopSecret,
            Self::SpaceBaseClassified,
            Self::SpaceBaseTopSecret,
            Self::TidalPrisonTopSecret,
        ]
    }
}

impl Operator {
    pub fn all() -> Vec<Self> {
        vec![
            Self::HongLang,
            Self::WeiLong,
            Self::WuMing,
            Self::JiFeng,
            Self::FengYi,
            Self::Gu,
            Self::MuYangRen,
            Self::WuLuLu,
            Self::ShenLan,
            Self::LuNa,
            Self::HaiZhua,
            Self::YinYi,
        ]
    }
}

impl PrimaryWeapon {
    pub fn all() -> Vec<Self> {
        vec![
            Self::MK47,
            Self::KC17,
            Self::K437,
            Self::TengLong,
            Self::ASVal,
            Self::CAR15,
            Self::PTR32,
            Self::G3,
            Self::SCARH,
            Self::AK12,
            Self::SG552,
            Self::M7,
            Self::AUG,
            Self::M16A4,
            Self::K416,
            Self::AKS74U,
            Self::QBZ951,
            Self::AKM,
            Self::M4A1,
            Self::QCQ171,
            Self::MP7,
            Self::YongShi,
            Self::SR3M,
            Self::SMG45,
            Self::YeNiu,
            Self::UZI,
            Self::Vector,
            Self::P90,
            Self::MP5,
            Self::O725,
            Self::M870,
            Self::S12K,
            Self::M1014,
            Self::QJB201,
            Self::M250,
            Self::M249,
            Self::PKM,
            Self::Marlin,
            Self::PSG1,
            Self::SR9,
            Self::SR25,
            Self::SKS,
            Self::M14,
            Self::SVD,
            Self::VSS,
            Self::Mini14,
            Self::AWP,
            Self::M700,
            Self::R93,
            Self::SV98,
            Self::M1911,
            Self::G17,
            Self::G18,
            Self::O93R,
            Self::DesertEagle,
            Self::O357Revolver,
            Self::QSZ92G,
            Self::CompoundBow,
        ]
    }
}

impl Helmet {
    pub fn all() -> Vec<Self> {
        vec![
            Self::OldSteelHelmet,
            Self::SecurityHelmet,
            Self::BennyHat,
            Self::OutdoorBaseballCap,
            Self::H01TacticalHelmet,
            Self::DROTacticalHelmet,
            Self::VintageMotorcycleHelmet,
            Self::MCBulletproofHelmet,
            Self::RiotHelmet,
            Self::H07TacticalHelmet,
            Self::DASBulletproofHelmet,
            Self::MC201BulletproofHelmet,
            Self::D6TacticalHelmet,
            Self::MHSTacticalHelmet,
            Self::GT1TacticalHelmet,
            Self::DICHTrainingHelmet,
            Self::GNHeavyNightMarketHelmet,
            Self::Mask1IronArmHelmet,
            Self::H09RiotHelmet,
            Self::GNHeavyHelmet,
            Self::DICH1TacticalHelmet,
            Self::GNHeavyNightVisionHelmet,
            Self::H70EliteHelmet,
        ]
    }
}

impl Armor {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Light,
            Self::Medium,
            Self::Heavy,
            Self::TacticalVest,
            Self::BodyArmor,
        ]
    }
}