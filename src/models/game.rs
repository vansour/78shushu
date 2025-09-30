use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Map {
    #[serde(rename = "零号大坝（常规）")]
    MapZeroDamRegular,
    #[serde(rename = "零号大坝（机密）")]
    MapZeroDamClassified,
    #[serde(rename = "零号大坝（前夜）")]
    MapZeroDamEveningBefore,
    #[serde(rename = "零号大坝（长夜）")]
    MapZeroDamLongNight,
    #[serde(rename = "零号大坝（终夜）")]
    MapZeroDamFinalNight,
    #[serde(rename = "长弓溪谷（常规）")]
    MapLongbowValleyRegular,
    #[serde(rename = "长弓溪谷（机密）")]
    MapLongbowValleyClassified,
    #[serde(rename = "巴克什（机密）")]
    MapBakshClassified,
    #[serde(rename = "巴克什（绝密）")]
    MapBakshTopSecret,
    #[serde(rename = "航天基地（机密）")]
    MapSpaceBaseClassified,
    #[serde(rename = "航天基地（绝密）")]
    MapSpaceBaseTopSecret,
    #[serde(rename = "潮汐监狱（绝密）")]
    MapTidalPrisonTopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "红狼")]
    AgentHongLang,
    #[serde(rename = "威龙")]
    AgentWeiLong,
    #[serde(rename = "无名")]
    AgentWuMing,
    #[serde(rename = "疾风")]
    AgentJiFeng,
    #[serde(rename = "蜂医")]
    AgentFengYi,
    #[serde(rename = "蛊")]
    AgentGu,
    #[serde(rename = "牧羊人")]
    AgentMuYangRen,
    #[serde(rename = "乌鲁鲁")]
    AgentWuLuLu,
    #[serde(rename = "深蓝")]
    AgentShenLan,
    #[serde(rename = "露娜")]
    AgentLuNa,
    #[serde(rename = "骇爪")]
    AgentHaiZhua,
    #[serde(rename = "银翼")]
    AgentYinYi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimaryWeapon {
    //步枪
    #[serde(rename = "MK47突击步枪")]
    RifleMK47AssaultRifle,
    #[serde(rename = "KC17突击步枪")]
    RifleKC17AssaultRifle,
    #[serde(rename = "K437突击步枪")]
    RifleK437AssaultRifle,
    #[serde(rename = "腾龙突击步枪")]
    RifleTengLongAssaultRifle,
    #[serde(rename = "AS Val突击步枪")]
    RifleASValAssaultRifle,
    #[serde(rename = "CAR-15突击步枪")]
    RifleCAR15AssaultRifle,
    #[serde(rename = "PTR-32突击步枪")]
    RiflePTR32AssaultRifle,
    #[serde(rename = "G3突击步枪")]
    RifleG3AssaultRifle,
    #[serde(rename = "SCAR-H突击步枪")]
    RifleSCARHAssaultRifle,
    #[serde(rename = "AK-12突击步枪")]
    RifleAK12AssaultRifle,
    #[serde(rename = "SG552突击步枪")]
    RifleSG552AssaultRifle,
    #[serde(rename = "M7突击步枪")]
    RifleM7AssaultRifle,
    #[serde(rename = "AUG突击步枪")]
    RifleAUGAssaultRifle,
    #[serde(rename = "M16A4突击步枪")]
    RifleM16A4AssaultRifle,
    #[serde(rename = "K416突击步枪")]
    RifleK416AssaultRifle,
    #[serde(rename = "AKS-74U突击步枪")]
    RifleAKS74UAssaultRifle,
    #[serde(rename = "QBZ95-1突击步枪")]
    RifleQBZ951AssaultRifle,
    #[serde(rename = "AKM突击步枪")]
    RifleAKMAssaultRifle,
    #[serde(rename = "M4A1突击步枪")]
    RifleM4A1AssaultRifle,

    //冲锋枪
    #[serde(rename = "QCQ171冲锋枪")]
    SubmachineQCQ171SubmachineGun,
    #[serde(rename = "MP7冲锋枪")]
    SubmachineMP7SubmachineGun,
    #[serde(rename = "勇士冲锋枪")]
    SubmachineYongShiSubmachineGun,
    #[serde(rename = "SR-3M紧凑突击步枪")]
    SubmachineSR3MCompactAssaultRifle,
    #[serde(rename = "SMG-45冲锋枪")]
    SubmachineSMG45SubmachineGun,
    #[serde(rename = "野牛冲锋枪")]
    SubmachineYeNiuSubmachineGun,
    #[serde(rename = "UZI冲锋枪")]
    SubmachineUZISubmachineGun,
    #[serde(rename = "Vector冲锋枪")]
    SubmachineVectorSubmachineGun,
    #[serde(rename = "P90冲锋枪")]
    SubmachineP90SubmachineGun,
    #[serde(rename = "MP5冲锋枪")]
    SubmachineMP5SubmachineGun,

    //霰弹枪
    #[serde(rename = "725双管霰弹枪")]
    Shotgun725DoubleBarreledShotgun,
    #[serde(rename = "M870霰弹枪")]
    ShotgunM870Shotgun,
    #[serde(rename = "S12K霰弹枪")]
    ShotgunS12KShotgun,
    #[serde(rename = "M1014霰弹枪")]
    ShotgunM1014Shotgun,

    //轻机枪
    #[serde(rename = "QJB201轻机枪")]
    LightMachineQJB201LightMachineGun,
    #[serde(rename = "M250轻机枪")]
    LightMachineM250LightMachineGun,
    #[serde(rename = "M249轻机枪")]
    LightMachineM249LightMachineGun,
    #[serde(rename = "PKM轻机枪")]
    LightMachinePKMLightMachineGun,

    //精确射手步枪
    #[serde(rename = "Marlin杠杆步枪")]
    PrecisionSniperRifleMarlinLeveredRifle,
    #[serde(rename = "PSG-1射手步枪")]
    PrecisionSniperRiflePSG1PrecisionSniperRifle,
    #[serde(rename = "SR9射手步枪")]
    PrecisionSniperRifleSR9PrecisionSniperRifle,
    #[serde(rename = "SR-25射手步枪")]
    PrecisionSniperRifleSR25PrecisionSniperRifle,
    #[serde(rename = "SKS射手步枪")]
    PrecisionSniperRifleSKSPrecisionSniperRifle,
    #[serde(rename = "M14射手步枪")]
    PrecisionSniperRifleM14PrecisionSniperRifle,
    #[serde(rename = "SVD狙击步枪")]
    PrecisionSniperRifleSVDSniperRifle,
    #[serde(rename = "VSS射手步枪")]
    PrecisionSniperRifleVSSPrecisionSniperRifle,
    #[serde(rename = "Mini-14射手步枪")]
    PrecisionSniperRifleMini14PrecisionSniperRifle,

    //狙击步枪
    #[serde(rename = "AWP狙击步枪")]
    SniperRifleAWPSniperRifle,
    #[serde(rename = "M700狙击步枪")]
    SniperRifleM700SniperRifle,
    #[serde(rename = "R93狙击步枪")]
    SniperRifleR93SniperRifle,
    #[serde(rename = "SV-98狙击步枪")]
    SniperRifleSV98SniperRifle,

    //手枪
    #[serde(rename = "M1911")]
    HandgunM1911,
    #[serde(rename = "G17")]
    HandgunG17,
    #[serde(rename = "G18")]
    HandgunG18,
    #[serde(rename = "93R")]
    Handgun93R,
    #[serde(rename = "沙漠之鹰")]
    HandgunDesertEagle,
    #[serde(rename = ".357左轮")]
    Handgun357Revolver,
    #[serde(rename = "QSZ92G")]
    HandgunQSZ92G,

    //特殊武器
    #[serde(rename = "复合弓")]
    SpecialWeaponsCompoundBow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Helmet {
    #[serde(rename = "老式钢盔")]
    HelmetOldSteelHelmet,
    #[serde(rename = "安保头盔")]
    HelmetSecurityHelmet,
    #[serde(rename = "奔尼帽")]
    HelmetBennyHat,
    #[serde(rename = "户外棒球帽")]
    HelmetOutdoorBaseballCap,
    #[serde(rename = "H01 战术头盔")]
    HelmetH01TacticalHelmet,
    #[serde(rename = "DRO 战术头盔")]
    HelmetDROTacticalHelmet,
    #[serde(rename = "复古摩托头盔")]
    HelmetVintageMotorcycleHelmet,
    #[serde(rename = "MC 防弹头盔")]
    HelmetMCBulletproofHelmet,
    #[serde(rename = "防暴头盔")]
    HelmetRiotHelmet,
    #[serde(rename = "H07 战术头盔")]
    HelmetH07TacticalHelmet,
    #[serde(rename = "DAS 防弹头盔")]
    HelmetDASBulletproofHelmet,
    #[serde(rename = "MC201 防弹头盔")]
    HelmetMC201BulletproofHelmet,
    #[serde(rename = "D6 战术头盔")]
    HelmetD6TacticalHelmet,
    #[serde(rename = "MHS 战术头盔")]
    HelmetMHSTacticalHelmet,
    #[serde(rename = "GT1战术头盔")]
    HelmetGT1TacticalHelmet,
    #[serde(rename = "DICH 训练头盔")]
    HelmetDICHTrainingHelmet,
    #[serde(rename = "GN 久战重型夜视头盔")]
    HelmetGNLongTermHeavyDutyNightVisionHelmet,
    #[serde(rename = "Mask-1铁臂头盔")]
    HelmetMask1IronArmHelmet,
    #[serde(rename = "H09 防暴头盔")]
    HelmetH09RiotHelmet,
    #[serde(rename = "GN 重型头盔")]
    HelmetGNHeavyHelmet,
    #[serde(rename = "DICH-1战术头盔")]
    HelmetDICH1TacticalHelmet,
    #[serde(rename = "GN重型夜视头盔")]
    HelmetGNHeavyNightVisionHelmet,
    #[serde(rename = "H70精英头盔")]
    HelmetH70EliteHelmet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Armor {
    #[serde(rename = "摩托马甲")]
    ArmorMotorVest,
    #[serde(rename = "安保防弹衣")]
    ArmorSecurityBodyArmor,
    #[serde(rename = "尼龙防弹衣")]
    ArmorNylonBodyArmor,
    #[serde(rename = "轻型防弹衣")]
    ArmorLightBodyArmor,
    #[serde(rename = "简易防刺服")]
    ArmorLightAntiStabVest,
    #[serde(rename = "HT战术背心")]
    ArmorHTTacticalVest,
    #[serde(rename = "TG战术防弹衣")]
    ArmorTGTacticalBodyArmor,
    #[serde(rename = "通用战术背心")]
    ArmorUniversalTacticalVest,
    #[serde(rename = "Hvk快拆防弹衣")]
    ArmorHvkQuickReleaseBodyArmor,
    #[serde(rename = "制式防弹背心")]
    ArmorStandardBodyArmorVest,
    #[serde(rename = "TG-H防弹衣")]
    ArmorTGHBodyArmor,
    #[serde(rename = "射手展示背心")]
    ArmorShooterDisplayVest,
    #[serde(rename = "HMP特勤防弹衣")]
    ArmorHMPSpecialServiceBodyArmor,
    #[serde(rename = "武士防弹背心")]
    ArmorSamuraiBodyArmorVest,
    #[serde(rename = "突击手防弹背心")]
    ArmorAssaulterBodyArmorVest,
    #[serde(rename = "DT-AVS防弹背心")]
    ArmorDTAVSBodyArmorVest,
    #[serde(rename = "MK-2战术背心")]
    ArmorMK2TacticalVest,
    #[serde(rename = "精英防弹背心")]
    ArmorEliteBodyArmorVest,
    #[serde(rename = "Hvk-2防弹衣")]
    ArmorHvk2BodyArmor,
    #[serde(rename = "FS复合防弹衣")]
    ArmorFSCompositeBodyArmor,
    #[serde(rename = "重型突击背心")]
    ArmorHeavyAssaultVest,
    #[serde(rename = "HA-2重型防弹衣")]
    ArmorHA2HeavyBodyArmor,
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
            Self::MapZeroDamRegular,
            Self::MapZeroDamClassified,
            Self::MapZeroDamEveningBefore,
            Self::MapZeroDamLongNight,
            Self::MapZeroDamFinalNight,
            Self::MapLongbowValleyRegular,
            Self::MapLongbowValleyClassified,
            Self::MapBakshClassified,
            Self::MapBakshTopSecret,
            Self::MapSpaceBaseClassified,
            Self::MapSpaceBaseTopSecret,
            Self::MapTidalPrisonTopSecret,
        ]
    }
}

impl Operator {
    pub fn all() -> Vec<Self> {
        vec![
            Self::AgentHongLang,
            Self::AgentWeiLong,
            Self::AgentWuMing,
            Self::AgentJiFeng,
            Self::AgentFengYi,
            Self::AgentGu,
            Self::AgentMuYangRen,
            Self::AgentWuLuLu,
            Self::AgentShenLan,
            Self::AgentLuNa,
            Self::AgentHaiZhua,
            Self::AgentYinYi,
        ]
    }
}

impl PrimaryWeapon {
    pub fn all() -> Vec<Self> {
        vec![
            Self::RifleMK47AssaultRifle,
            Self::RifleKC17AssaultRifle,
            Self::RifleK437AssaultRifle,
            Self::RifleTengLongAssaultRifle,
            Self::RifleASValAssaultRifle,
            Self::RifleCAR15AssaultRifle,
            Self::RiflePTR32AssaultRifle,
            Self::RifleG3AssaultRifle,
            Self::RifleSCARHAssaultRifle,
            Self::RifleAK12AssaultRifle,
            Self::RifleSG552AssaultRifle,
            Self::RifleM7AssaultRifle,
            Self::RifleAUGAssaultRifle,
            Self::RifleM16A4AssaultRifle,
            Self::RifleK416AssaultRifle,
            Self::RifleAKS74UAssaultRifle,
            Self::RifleQBZ951AssaultRifle,
            Self::RifleAKMAssaultRifle,
            Self::RifleM4A1AssaultRifle,
            Self::SubmachineQCQ171SubmachineGun,
            Self::SubmachineMP7SubmachineGun,
            Self::SubmachineYongShiSubmachineGun,
            Self::SubmachineSR3MCompactAssaultRifle,
            Self::SubmachineSMG45SubmachineGun,
            Self::SubmachineYeNiuSubmachineGun,
            Self::SubmachineUZISubmachineGun,
            Self::SubmachineVectorSubmachineGun,
            Self::SubmachineP90SubmachineGun,
            Self::SubmachineMP5SubmachineGun,
            Self::Shotgun725DoubleBarreledShotgun,
            Self::ShotgunM870Shotgun,
            Self::ShotgunS12KShotgun,
            Self::ShotgunM1014Shotgun,
            Self::LightMachineQJB201LightMachineGun,
            Self::LightMachineM250LightMachineGun,
            Self::LightMachineM249LightMachineGun,
            Self::LightMachinePKMLightMachineGun,
            Self::PrecisionSniperRifleMarlinLeveredRifle,
            Self::PrecisionSniperRiflePSG1PrecisionSniperRifle,
            Self::PrecisionSniperRifleSR9PrecisionSniperRifle,
            Self::PrecisionSniperRifleSR25PrecisionSniperRifle,
            Self::PrecisionSniperRifleSKSPrecisionSniperRifle,
            Self::PrecisionSniperRifleM14PrecisionSniperRifle,
            Self::PrecisionSniperRifleSVDSniperRifle,
            Self::PrecisionSniperRifleVSSPrecisionSniperRifle,
            Self::PrecisionSniperRifleMini14PrecisionSniperRifle,
            Self::SniperRifleAWPSniperRifle,
            Self::SniperRifleM700SniperRifle,
            Self::SniperRifleR93SniperRifle,
            Self::SniperRifleSV98SniperRifle,
            Self::HandgunM1911,
            Self::HandgunG17,
            Self::HandgunG18,
            Self::Handgun93R,
            Self::HandgunDesertEagle,
            Self::Handgun357Revolver,
            Self::HandgunQSZ92G,
            Self::SpecialWeaponsCompoundBow,
        ]
    }
}

impl Helmet {
    pub fn all() -> Vec<Self> {
        vec![
            Self::HelmetOldSteelHelmet,
            Self::HelmetSecurityHelmet,
            Self::HelmetBennyHat,
            Self::HelmetOutdoorBaseballCap,
            Self::HelmetH01TacticalHelmet,
            Self::HelmetDROTacticalHelmet,
            Self::HelmetVintageMotorcycleHelmet,
            Self::HelmetMCBulletproofHelmet,
            Self::HelmetRiotHelmet,
            Self::HelmetH07TacticalHelmet,
            Self::HelmetDASBulletproofHelmet,
            Self::HelmetMC201BulletproofHelmet,
            Self::HelmetD6TacticalHelmet,
            Self::HelmetMHSTacticalHelmet,
            Self::HelmetGT1TacticalHelmet,
            Self::HelmetDICHTrainingHelmet,
            Self::HelmetGNLongTermHeavyDutyNightVisionHelmet,
            Self::HelmetMask1IronArmHelmet,
            Self::HelmetH09RiotHelmet,
            Self::HelmetGNHeavyHelmet,
            Self::HelmetDICH1TacticalHelmet,
            Self::HelmetGNHeavyNightVisionHelmet,
            Self::HelmetH70EliteHelmet,
        ]
    }
}

impl Armor {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ArmorMotorVest,
            Self::ArmorSecurityBodyArmor,
            Self::ArmorNylonBodyArmor,
            Self::ArmorLightBodyArmor,
            Self::ArmorLightAntiStabVest,
            Self::ArmorHTTacticalVest,
            Self::ArmorTGTacticalBodyArmor,
            Self::ArmorUniversalTacticalVest,
            Self::ArmorHvkQuickReleaseBodyArmor,
            Self::ArmorStandardBodyArmorVest,
            Self::ArmorTGHBodyArmor,
            Self::ArmorShooterDisplayVest,
            Self::ArmorHMPSpecialServiceBodyArmor,
            Self::ArmorSamuraiBodyArmorVest,
            Self::ArmorAssaulterBodyArmorVest,
            Self::ArmorDTAVSBodyArmorVest,
            Self::ArmorMK2TacticalVest,
            Self::ArmorEliteBodyArmorVest,
            Self::ArmorHvk2BodyArmor,
            Self::ArmorFSCompositeBodyArmor,
            Self::ArmorHeavyAssaultVest,
            Self::ArmorHA2HeavyBodyArmor,
        ]
    }
}
