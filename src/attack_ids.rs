// TODO: Probably best to generate this file from database.json via card_enum_generator.rs.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttackId {
    A1003VenusaurMegaDrain,
    A1004VenusaurExGiantBloom,
    A1005CaterpieFindAFriend,
    A1013VileplumeSoothingScent,
    A1017VenomothPoisonPowder,
    A1022ExeggutorStomp,
    A1023ExeggutorExTropicalSwing,
    A1024TangelaAbsorb,
    A1026PinsirDoubleHorn,
    A1029PetililBlot,
    A1030LilligantLeafSupply,
    A1031SkiddoSurpriseAttack,
    A1033CharmanderEmber,
    A1035CharizardFireSpin,
    A1036CharizardExCrimsonStorm,
    A1038NinetalesFlamethrower,
    A1040ArcanineHeatTackle,
    A1041ArcanineExInfernoOnrush,
    A1045FlareonFlamethrower,
    A1046MoltresSkyAttack,
    A1047MoltresExInfernoDance,
    A1052CentiskorchFireBlast,
    A1055BlastoiseHydroPump,
    A1056BlastoiseExHydroBazooka,
    A1057PsyduckHeadache,
    A1063TentacruelPoisonTentacles,
    A1069KinglerKOCrab,
    A1071SeadraWaterArrow,
    A1073SeakingHornHazard,
    A1078GyaradosHyperBeam,
    A1079LaprasHydroPump,
    A1080VaporeonBubbleDrain,
    A1083ArticunoIceBeam,
    A1084ArticunoExBlizzard,
    A1091BruxishSecondStrike,
    A1093FrosmothPowderSnow,
    A1095RaichuThunderbolt,
    A1096PikachuExCircleCircuit,
    A1101ElectabuzzThunderPunch,
    A1102JolteonPinMissile,
    A1103ZapdosRagingThunder,
    A1104ZapdosExThunderingHurricane,
    A1106ZebstrikaThunderSpear,
    A1109EelektrossThunderFang,
    A1111HelioliskQuickAttack,
    A1112PincurchinThunderShock,
    A1115AbraTeleport,
    A1117AlakazamPsychic,
    A1126MrMimeBarrierAttack,
    A1127JynxPsychic,
    A1128MewtwoPowerBlast,
    A1129MewtwoExPsydrive,
    A1136GolurkDoubleLariat,
    A1142PrimeapeFightBack,
    A1149GolemDoubleEdge,
    A1153MarowakExBonemerang,
    A1154HitmonleeStretchKick,
    A1163GrapploctKnockBack,
    A1165ArbokCorner,
    A1171NidokingPoisonHorn,
    A1174GrimerPoisonGas,
    A1178MawileCrunch,
    A1181MeltanAmass,
    A1195WigglytuffExSleepySong,
    A1196MeowthPayDay,
    A1201LickitungContinuousLick,
    A1203KangaskhanDizzyPunch,
    A1213CinccinoDoTheWave,
    A1a001ExeggcuteGrowthSpurt,
    A1a002ExeggutorPsychic,
    A1a003CelebiExPowerfulBloom,
    A1a010PonytaStomp,
    A1a011RapidashRisingLunge,
    A1a017MagikarpLeapOut,
    A1a021LumineonAquaLiner,
    A1a026RaichuGigashock,
    A1a030DedenneThunderShock,
    A1a041MankeyFocusFist,
    A1a045GolemGuardPress,
    A1a061EeveeContinuousSteps,
    A2023MagmarStoke,
    A2035PiplupNap,
    A2049PalkiaExDimensionalStorm,
    A2050ManaphyOceanicGift,
    A2053MagnezoneThunderBlast,
    A2056ElectabuzzCharge,
    A2073DrifloonExpand,
    A2084GliscorAcrobatics,
    A2098SneaselDoubleScratch,
    A2111SkarmoryMetalArms,
    A2117BronzongGuardPress,
    A2118ProbopassTripleNose,
    A2119DialgaExMetallicTurbo,
    A2131AmbipomDoubleHit,
    A2141ChatotFuryAttack,
    A2a001HeracrossSingleHornThrow,
    A2a057ProbopassExDefensiveUnit,
    A2a071ArceusExUltimateForce,
    A2b001WeedleMultiply,
    A2b002KakunaStringShot,
    A2b003BeedrillExCrushingSpear,
    A2b005SprigatitoCryForHelp,
    A2b007MeowscaradaFightingClaws,
    A2b010CharizardExStoke,
    A2b022PikachuExThunderbolt,
    A2b031AlakazamPsychicSuppression,
    A2b032MrMimeJuggling,
    A2b035GiratinaExChaoticImpact,
    A2b044FlamigoDoubleKick,
    A3002AlolanExeggutorTropicalHammer,
    A3019SteeneeDoubleSpin,
    A3020TsareenaThreeKickCombo,
    A3037TurtonatorFireSpin,
    A3040AlolanVulpixCallForthCold,
    A3041AlolanNinetalesBlizzard,
    A3043CloysterGuardPress,
    A3071SpoinkPsycharge,
    A3085CosmogTeleport,
    A3086CosmoemStiffen,
    A3112AbsolUnseenClaw,
    A3116ToxapexSpikeCannon,
    A3122SolgaleoExSolBreaker,
    A3a003RowletFuryAttack,
    A3a006BuzzwoleExBigBeat,
    A3a007PheromosaJumpBlues,
    A3a019TapuKokoExPlasmaHurricane,
    A3a033LycanrocExLycanfang,
    A3a043GuzzlordExGrindcore,
    A3a044Poipole2Step,
    A3a045NaganadelElectroHouse,
    A3a047AlolanDugtrioExTripletHeadbutt,
    A3a053StakatakaBrassRock,
    A3a060TypeNullQuickBlow,
    A3a061SilvallyBraveBuddies,
    A3a062CelesteelaMoombahton,
    A3b009FlareonExFireSpin,
    A3b010TorkoalFlamethrower,
    A3b013IncineroarDarkestLariat,
    A3b014SalanditEmber,
    A3b020VanilluxeDoubleSpin,
    A3b053DragoniteExGigaImpact,
    A3b055EeveeCollect,
    A3b058AipomDoubleHit,
    A4021ShuckleExTripleSlap,
    A4026NinetalesScorchingBreath,
    A4032MagbyToastyToss,
    A4066PichuCracklyToss,
    A4077CleffaTwinklyCall,
    A4102HitmontopPiercingSpin,
    A4104PupitarGuardPress,
    A4105BinacleDualChop,
    A4120AbsolLeapOver,
    A4124SkarmoryExSteelWing,
    A4134EeveeFindAFriend,
    A4146UrsaringSwingAround,
    A4149LugiaExElementalBlast,
    A4a010EnteiExBlazingBeatdown,
    A4a020SuicuneExCrystalWaltz,
    A4a021FeebasLeapOut,
    A4a023MantykeSplashyToss,
    A4a025RaikouExVoltaicBullet,
    B1002MegaPinsirExCriticalScissors,
    B1031RapidashExSprintingFlare,
    B1035BlazikenBlazeKick,
    B1036MegaBlazikenExMegaBurning,
    B1050MagikarpWaterfallEvolution,
    B1052MegaGyaradosExMegaBlaster,
    B1085MegaAmpharosExLightningLancer,
    B1102MegaAltariaExMegaHarmony,
    B1150AbsolOminousClaw,
    B1151MegaAbsolExDarknessClaw,
    PA072AlolanGrimerPoisonGas,
    PA079DuskManeNecrozmaBlackMetal,
}

// Create a static HashMap for fast (pokemon, index) lookup
lazy_static::lazy_static! {
    static ref ATTACK_ID_MAP: HashMap<(&'static str, usize), AttackId> = {
        let mut m = HashMap::new();
        m.insert(("A1 003", 0), AttackId::A1003VenusaurMegaDrain);
        m.insert(("A1 004", 1), AttackId::A1004VenusaurExGiantBloom);
        m.insert(("A1 005", 0), AttackId::A1005CaterpieFindAFriend);
        m.insert(("A1 013", 0), AttackId::A1013VileplumeSoothingScent);
        m.insert(("A1 017", 0), AttackId::A1017VenomothPoisonPowder);
        m.insert(("A1 022", 0), AttackId::A1022ExeggutorStomp);
        m.insert(("A1 023", 0), AttackId::A1023ExeggutorExTropicalSwing);
        m.insert(("A1 024", 0), AttackId::A1024TangelaAbsorb);
        m.insert(("A1 026", 0), AttackId::A1026PinsirDoubleHorn);
        m.insert(("A1 029", 0), AttackId::A1029PetililBlot);
        m.insert(("A1 030", 0), AttackId::A1030LilligantLeafSupply);
        m.insert(("A1 031", 0), AttackId::A1031SkiddoSurpriseAttack);
        m.insert(("A1 033", 0), AttackId::A1033CharmanderEmber);
        m.insert(("A1 035", 0), AttackId::A1035CharizardFireSpin);
        m.insert(("A1 036", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A1 038", 0), AttackId::A1038NinetalesFlamethrower);
        m.insert(("A1 040", 0), AttackId::A1040ArcanineHeatTackle);
        m.insert(("A1 041", 0), AttackId::A1041ArcanineExInfernoOnrush);
        m.insert(("A1 045", 0), AttackId::A1045FlareonFlamethrower);
        m.insert(("A1 046", 0), AttackId::A1046MoltresSkyAttack);
        m.insert(("A1 047", 0), AttackId::A1047MoltresExInfernoDance);
        m.insert(("A1 052", 0), AttackId::A1052CentiskorchFireBlast);
        m.insert(("A1 055", 0), AttackId::A1055BlastoiseHydroPump);
        m.insert(("A1 056", 1), AttackId::A1056BlastoiseExHydroBazooka);
        m.insert(("A1 057", 0), AttackId::A1057PsyduckHeadache);
        m.insert(("A1 063", 0), AttackId::A1063TentacruelPoisonTentacles);
        m.insert(("A1 069", 0), AttackId::A1069KinglerKOCrab);
        m.insert(("A1 071", 0), AttackId::A1071SeadraWaterArrow);
        m.insert(("A1 073", 0), AttackId::A1073SeakingHornHazard);
        m.insert(("A1 078", 0), AttackId::A1078GyaradosHyperBeam);
        m.insert(("A1 079", 0), AttackId::A1079LaprasHydroPump);
        m.insert(("A1 080", 0), AttackId::A1080VaporeonBubbleDrain);
        m.insert(("A1 083", 0), AttackId::A1083ArticunoIceBeam);
        m.insert(("A1 084", 1), AttackId::A1084ArticunoExBlizzard);
        m.insert(("A1 091", 0), AttackId::A1091BruxishSecondStrike);
        m.insert(("A1 093", 0), AttackId::A1093FrosmothPowderSnow);
        m.insert(("A1 095", 0), AttackId::A1095RaichuThunderbolt);
        m.insert(("A1 096", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A1 101", 0), AttackId::A1101ElectabuzzThunderPunch);
        m.insert(("A1 102", 0), AttackId::A1102JolteonPinMissile);
        m.insert(("A1 103", 0), AttackId::A1103ZapdosRagingThunder);
        m.insert(("A1 104", 1), AttackId::A1104ZapdosExThunderingHurricane);
        m.insert(("A1 106", 0), AttackId::A1106ZebstrikaThunderSpear);
        m.insert(("A1 109", 0), AttackId::A1109EelektrossThunderFang);
        m.insert(("A1 111", 0), AttackId::A1111HelioliskQuickAttack);
        m.insert(("A1 112", 0), AttackId::A1112PincurchinThunderShock);
        m.insert(("A1 115", 0), AttackId::A1115AbraTeleport);
        m.insert(("A1 117", 0), AttackId::A1117AlakazamPsychic);
        m.insert(("A1 126", 0), AttackId::A1126MrMimeBarrierAttack);
        m.insert(("A1 127", 0), AttackId::A1127JynxPsychic);
        m.insert(("A1 128", 0), AttackId::A1128MewtwoPowerBlast);
        m.insert(("A1 129", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("A1 136", 0), AttackId::A1136GolurkDoubleLariat);
        m.insert(("A1 142", 0), AttackId::A1142PrimeapeFightBack);
        m.insert(("A1 149", 0), AttackId::A1149GolemDoubleEdge);
        m.insert(("A1 153", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A1 154", 0), AttackId::A1154HitmonleeStretchKick);
        m.insert(("A1 163", 0), AttackId::A1163GrapploctKnockBack);
        m.insert(("A1 165", 0), AttackId::A1165ArbokCorner);
        m.insert(("A1 171", 0), AttackId::A1171NidokingPoisonHorn);
        m.insert(("A1 174", 0), AttackId::A1174GrimerPoisonGas);
        m.insert(("A1 178", 0), AttackId::A1178MawileCrunch);
        m.insert(("A1 181", 0), AttackId::A1181MeltanAmass);
        m.insert(("A1 195", 0), AttackId::A1195WigglytuffExSleepySong);
        m.insert(("A1 196", 0), AttackId::A1196MeowthPayDay);
        m.insert(("A1 201", 0), AttackId::A1201LickitungContinuousLick);
        m.insert(("A1 203", 0), AttackId::A1203KangaskhanDizzyPunch);
        m.insert(("A1 213", 0), AttackId::A1213CinccinoDoTheWave);
        // Full Arts A1
        m.insert(("A1 229", 0), AttackId::A1026PinsirDoubleHorn);
        m.insert(("A1 230", 0), AttackId::A1033CharmanderEmber);
        m.insert(("A1 233", 0), AttackId::A1078GyaradosHyperBeam);
        m.insert(("A1 234", 0), AttackId::A1079LaprasHydroPump);
        m.insert(("A1 236", 0), AttackId::A1117AlakazamPsychic);
        m.insert(("A1 241", 0), AttackId::A1171NidokingPoisonHorn);
        m.insert(("A1 246", 0), AttackId::A1196MeowthPayDay);
        m.insert(("A1 251", 1), AttackId::A1004VenusaurExGiantBloom);
        m.insert(("A1 252", 0), AttackId::A1023ExeggutorExTropicalSwing);
        m.insert(("A1 253", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A1 254", 0), AttackId::A1041ArcanineExInfernoOnrush);
        m.insert(("A1 255", 0), AttackId::A1047MoltresExInfernoDance);
        m.insert(("A1 256", 1), AttackId::A1056BlastoiseExHydroBazooka);
        m.insert(("A1 258", 1), AttackId::A1084ArticunoExBlizzard);
        m.insert(("A1 259", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A1 260", 1), AttackId::A1104ZapdosExThunderingHurricane);
        m.insert(("A1 262", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("A1 264", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A1 265", 0), AttackId::A1195WigglytuffExSleepySong);
        m.insert(("A1 274", 0), AttackId::A1047MoltresExInfernoDance);
        m.insert(("A1 275", 1), AttackId::A1084ArticunoExBlizzard);
        m.insert(("A1 276", 1), AttackId::A1104ZapdosExThunderingHurricane);
        m.insert(("A1 279", 0), AttackId::A1195WigglytuffExSleepySong);
        m.insert(("A1 280", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A1 281", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A1 282", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("A1 284", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A1 285", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A1 286", 1), AttackId::A1129MewtwoExPsydrive);

        // A1a
        m.insert(("A1a 001", 0), AttackId::A1a001ExeggcuteGrowthSpurt);
        m.insert(("A1a 002", 0), AttackId::A1a002ExeggutorPsychic);
        m.insert(("A1a 003", 0), AttackId::A1a003CelebiExPowerfulBloom);
        m.insert(("A1a 010", 0), AttackId::A1a010PonytaStomp);
        m.insert(("A1a 011", 0), AttackId::A1a011RapidashRisingLunge);
        m.insert(("A1a 017", 0), AttackId::A1a017MagikarpLeapOut);
        m.insert(("A1a 021", 0), AttackId::A1a021LumineonAquaLiner);
        m.insert(("A1a 026", 0), AttackId::A1a026RaichuGigashock);
        m.insert(("A1a 030", 0), AttackId::A1a030DedenneThunderShock);
        m.insert(("A1a 041", 0), AttackId::A1a041MankeyFocusFist);
        m.insert(("A1a 045", 0), AttackId::A1a045GolemGuardPress);
        m.insert(("A1a 061", 0), AttackId::A1a061EeveeContinuousSteps);
        // Full Arts A1a
        m.insert(("A1a 069", 0), AttackId::A1a002ExeggutorPsychic);
        m.insert(("A1a 073", 0), AttackId::A1a030DedenneThunderShock);
        m.insert(("A1a 075", 0), AttackId::A1a003CelebiExPowerfulBloom);
        m.insert(("A1a 085", 0), AttackId::A1a003CelebiExPowerfulBloom);

        // A2
        m.insert(("A2 023", 0), AttackId::A2023MagmarStoke);
        m.insert(("A2 035", 0), AttackId::A2035PiplupNap);
        m.insert(("A2 049", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A2 050", 0), AttackId::A2050ManaphyOceanicGift);
        m.insert(("A2 053", 0), AttackId::A2053MagnezoneThunderBlast);
        m.insert(("A2 056", 0), AttackId::A2056ElectabuzzCharge);
        m.insert(("A2 073", 0), AttackId::A2073DrifloonExpand);
        m.insert(("A2 084", 0), AttackId::A2084GliscorAcrobatics);
        m.insert(("A2 098", 0), AttackId::A2098SneaselDoubleScratch);
        m.insert(("A2 111", 0), AttackId::A2111SkarmoryMetalArms);
        m.insert(("A2 117", 0), AttackId::A2117BronzongGuardPress);
        m.insert(("A2 118", 0), AttackId::A2118ProbopassTripleNose);
        m.insert(("A2 119", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("A2 131", 0), AttackId::A2131AmbipomDoubleHit);
        m.insert(("A2 141", 0), AttackId::A2141ChatotFuryAttack);
        m.insert(("A2 162", 0), AttackId::A2050ManaphyOceanicGift);
        m.insert(("A2 165", 0), AttackId::A2073DrifloonExpand);
        m.insert(("A2 182", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A2 188", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("A2 204", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A2 205", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("A2 206", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A2 207", 0), AttackId::A2119DialgaExMetallicTurbo);

        // A2a
        m.insert(("A2a 001", 0), AttackId::A2a001HeracrossSingleHornThrow);
        m.insert(("A2a 057", 0), AttackId::A2a057ProbopassExDefensiveUnit);
        m.insert(("A2a 071", 0), AttackId::A2a071ArceusExUltimateForce);
        m.insert(("A2a 085", 0), AttackId::A2a057ProbopassExDefensiveUnit);
        m.insert(("A2a 086", 0), AttackId::A2a071ArceusExUltimateForce);
        m.insert(("A2a 094", 0), AttackId::A2a057ProbopassExDefensiveUnit);
        m.insert(("A2a 095", 0), AttackId::A2a071ArceusExUltimateForce);
        m.insert(("A2a 096", 0), AttackId::A2a071ArceusExUltimateForce);

        // A2b
        m.insert(("A2b 001", 0), AttackId::A2b001WeedleMultiply);
        m.insert(("A2b 002", 0), AttackId::A2b002KakunaStringShot);
        m.insert(("A2b 003", 0), AttackId::A2b003BeedrillExCrushingSpear);
        m.insert(("A2b 005", 0), AttackId::A2b005SprigatitoCryForHelp);
        m.insert(("A2b 007", 0), AttackId::A2b007MeowscaradaFightingClaws);
        m.insert(("A2b 010", 0), AttackId::A2b010CharizardExStoke);
        m.insert(("A2b 022", 0), AttackId::A2b022PikachuExThunderbolt);
        m.insert(("A2b 031", 0), AttackId::A2b031AlakazamPsychicSuppression);
        m.insert(("A2b 032", 0), AttackId::A2b032MrMimeJuggling);
        m.insert(("A2b 035", 0), AttackId::A2b035GiratinaExChaoticImpact);
        m.insert(("A2b 044", 0), AttackId::A2b044FlamigoDoubleKick);
        m.insert(("A2b 073", 0), AttackId::A2b007MeowscaradaFightingClaws);
        m.insert(("A2b 079", 0), AttackId::A2b003BeedrillExCrushingSpear);
        m.insert(("A2b 080", 0), AttackId::A2b010CharizardExStoke);
        m.insert(("A2b 082", 0), AttackId::A2b022PikachuExThunderbolt);
        m.insert(("A2b 083", 0), AttackId::A2b035GiratinaExChaoticImpact);
        m.insert(("A2b 092", 0), AttackId::A2b022PikachuExThunderbolt);
        m.insert(("A2b 096", 0), AttackId::A2b035GiratinaExChaoticImpact);
        m.insert(("A2b 097", 0), AttackId::A2b001WeedleMultiply);
        m.insert(("A2b 098", 0), AttackId::A2b002KakunaStringShot);
        m.insert(("A2b 107", 0), AttackId::A2b003BeedrillExCrushingSpear);
        m.insert(("A2b 108", 0), AttackId::A2b010CharizardExStoke);

        // A3
        m.insert(("A3 002", 0), AttackId::A3002AlolanExeggutorTropicalHammer);
        m.insert(("A3 019", 0), AttackId::A3019SteeneeDoubleSpin);
        m.insert(("A3 020", 0), AttackId::A3020TsareenaThreeKickCombo);
        m.insert(("A3 037", 0), AttackId::A3037TurtonatorFireSpin);
        m.insert(("A3 040", 0), AttackId::A3040AlolanVulpixCallForthCold);
        m.insert(("A3 041", 0), AttackId::A3041AlolanNinetalesBlizzard);
        m.insert(("A3 043", 0), AttackId::A3043CloysterGuardPress);
        m.insert(("A3 071", 0), AttackId::A3071SpoinkPsycharge);
        m.insert(("A3 085", 0), AttackId::A3085CosmogTeleport);
        m.insert(("A3 086", 0), AttackId::A3086CosmoemStiffen);
        m.insert(("A3 112", 0), AttackId::A3112AbsolUnseenClaw);
        m.insert(("A3 116", 0), AttackId::A3116ToxapexSpikeCannon);
        m.insert(("A3 122", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A3 156", 0), AttackId::A3002AlolanExeggutorTropicalHammer);
        m.insert(("A3 158", 0), AttackId::A3020TsareenaThreeKickCombo);
        m.insert(("A3 161", 0), AttackId::A3037TurtonatorFireSpin);
        m.insert(("A3 162", 0), AttackId::A3040AlolanVulpixCallForthCold);
        m.insert(("A3 171", 0), AttackId::A3085CosmogTeleport);
        m.insert(("A3 189", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A3 207", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A3 212", 0), AttackId::A1003VenusaurMegaDrain);
        m.insert(("A3 214", 0), AttackId::A1a002ExeggutorPsychic);
        m.insert(("A3 217", 0), AttackId::A1055BlastoiseHydroPump);
        m.insert(("A3 230", 1), AttackId::A1004VenusaurExGiantBloom);
        m.insert(("A3 231", 0), AttackId::A1023ExeggutorExTropicalSwing);
        m.insert(("A3 232", 1), AttackId::A1056BlastoiseExHydroBazooka);
        m.insert(("A3 236", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A3 237", 0), AttackId::A1195WigglytuffExSleepySong);
        m.insert(("A3 239", 0), AttackId::A3122SolgaleoExSolBreaker);

        // A3a
        m.insert(("A3a 003", 0), AttackId::A3a003RowletFuryAttack);
        m.insert(("A3a 006", 1), AttackId::A3a006BuzzwoleExBigBeat);
        m.insert(("A3a 007", 0), AttackId::A3a007PheromosaJumpBlues);
        m.insert(("A3a 019", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("A3a 033", 0), AttackId::A3a033LycanrocExLycanfang);
        m.insert(("A3a 043", 0), AttackId::A3a043GuzzlordExGrindcore);
        m.insert(("A3a 044", 0), AttackId::A3a044Poipole2Step);
        m.insert(("A3a 045", 0), AttackId::A3a045NaganadelElectroHouse);
        m.insert(("A3a 047", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);
        m.insert(("A3a 053", 0), AttackId::A3a053StakatakaBrassRock);
        m.insert(("A3a 060", 0), AttackId::A3a060TypeNullQuickBlow);
        m.insert(("A3a 061", 0), AttackId::A3a061SilvallyBraveBuddies);
        m.insert(("A3a 062", 0), AttackId::A3a062CelesteelaMoombahton);
        m.insert(("A3a 070", 0), AttackId::A3a003RowletFuryAttack);
        m.insert(("A3a 071", 0), AttackId::A3a007PheromosaJumpBlues);
        m.insert(("A3a 074", 0), AttackId::A3a061SilvallyBraveBuddies);
        m.insert(("A3a 075", 0), AttackId::A3a062CelesteelaMoombahton);
        m.insert(("A3a 076", 1), AttackId::A3a006BuzzwoleExBigBeat);
        m.insert(("A3a 077", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("A3a 078", 0), AttackId::A3a033LycanrocExLycanfang);
        m.insert(("A3a 079", 0), AttackId::A3a043GuzzlordExGrindcore);
        m.insert(("A3a 080", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);
        m.insert(("A3a 084", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("A3a 085", 0), AttackId::A3a033LycanrocExLycanfang);
        m.insert(("A3a 086", 0), AttackId::A3a043GuzzlordExGrindcore);
        m.insert(("A3a 087", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);
        m.insert(("A3a 088", 1), AttackId::A3a006BuzzwoleExBigBeat);
        m.insert(("A3a 090", 0), AttackId::A1040ArcanineHeatTackle);
        m.insert(("A3a 094", 0), AttackId::A1127JynxPsychic);
        m.insert(("A3a 099", 0), AttackId::A1a003CelebiExPowerfulBloom);
        m.insert(("A3a 100", 0), AttackId::A1041ArcanineExInfernoOnrush);

        // A3b
        m.insert(("A3b 009", 0), AttackId::A3b009FlareonExFireSpin);
        m.insert(("A3b 010", 0), AttackId::A3b010TorkoalFlamethrower);
        m.insert(("A3b 013", 0), AttackId::A3b013IncineroarDarkestLariat);
        m.insert(("A3b 014", 0), AttackId::A3b014SalanditEmber);
        m.insert(("A3b 020", 0), AttackId::A3b020VanilluxeDoubleSpin);
        m.insert(("A3b 053", 0), AttackId::A3b053DragoniteExGigaImpact);
        m.insert(("A3b 055", 0), AttackId::A3b055EeveeCollect);
        m.insert(("A3b 058", 0), AttackId::A3b058AipomDoubleHit);
        m.insert(("A3b 078", 0), AttackId::A3b055EeveeCollect);
        m.insert(("A3b 079", 0), AttackId::A3b009FlareonExFireSpin);
        m.insert(("A3b 082", 0), AttackId::A3b053DragoniteExGigaImpact);
        m.insert(("A3b 087", 0), AttackId::A3b009FlareonExFireSpin);
        m.insert(("A3b 090", 0), AttackId::A3b053DragoniteExGigaImpact);
        m.insert(("A3b 094", 0), AttackId::A1079LaprasHydroPump);
        m.insert(("A3b 101", 0), AttackId::A1165ArbokCorner);
        m.insert(("A3b 103", 0), AttackId::A1047MoltresExInfernoDance);
        m.insert(("A3b 104", 1), AttackId::A1084ArticunoExBlizzard);
        m.insert(("A3b 105", 1), AttackId::A1104ZapdosExThunderingHurricane);

        // A4
        m.insert(("A4 021", 0), AttackId::A4021ShuckleExTripleSlap);
        m.insert(("A4 026", 0), AttackId::A4026NinetalesScorchingBreath);
        m.insert(("A4 032", 0), AttackId::A4032MagbyToastyToss);
        m.insert(("A4 066", 0), AttackId::A4066PichuCracklyToss);
        m.insert(("A4 077", 0), AttackId::A4077CleffaTwinklyCall);
        m.insert(("A4 102", 0), AttackId::A4102HitmontopPiercingSpin);
        m.insert(("A4 104", 0), AttackId::A4104PupitarGuardPress);
        m.insert(("A4 105", 0), AttackId::A4105BinacleDualChop);
        m.insert(("A4 120", 0), AttackId::A4120AbsolLeapOver);
        m.insert(("A4 124", 0), AttackId::A4124SkarmoryExSteelWing);
        m.insert(("A4 134", 0), AttackId::A4134EeveeFindAFriend);
        m.insert(("A4 146", 0), AttackId::A4146UrsaringSwingAround);
        m.insert(("A4 149", 0), AttackId::A4149LugiaExElementalBlast);
        m.insert(("A4 166", 0), AttackId::A4032MagbyToastyToss);
        m.insert(("A4 171", 0), AttackId::A4066PichuCracklyToss);
        m.insert(("A4 186", 0), AttackId::A4021ShuckleExTripleSlap);
        m.insert(("A4 194", 0), AttackId::A4124SkarmoryExSteelWing);
        m.insert(("A4 195", 0), AttackId::A4149LugiaExElementalBlast);
        m.insert(("A4 202", 0), AttackId::A4021ShuckleExTripleSlap);
        m.insert(("A4 209", 0), AttackId::A4124SkarmoryExSteelWing);
        m.insert(("A4 211", 0), AttackId::A4149LugiaExElementalBlast);
        m.insert(("A4 214", 0), AttackId::A1a017MagikarpLeapOut);
        m.insert(("A4 216", 0), AttackId::A1080VaporeonBubbleDrain);
        m.insert(("A4 221", 0), AttackId::A1a041MankeyFocusFist);
        m.insert(("A4 222", 0), AttackId::A1142PrimeapeFightBack);
        m.insert(("A4 228", 0), AttackId::A1171NidokingPoisonHorn);
        m.insert(("A4 231", 0), AttackId::A4134EeveeFindAFriend);
        m.insert(("A4 241", 0), AttackId::A4149LugiaExElementalBlast);

        // A4a
        m.insert(("A4a 010", 0), AttackId::A4a010EnteiExBlazingBeatdown);
        m.insert(("A4a 020", 0), AttackId::A4a020SuicuneExCrystalWaltz);
        m.insert(("A4a 021", 0), AttackId::A4a021FeebasLeapOut);
        m.insert(("A4a 023", 0), AttackId::A4a023MantykeSplashyToss);
        m.insert(("A4a 025", 0), AttackId::A4a025RaikouExVoltaicBullet);
        m.insert(("A4a 079", 0), AttackId::A4a010EnteiExBlazingBeatdown);
        m.insert(("A4a 080", 0), AttackId::A4a020SuicuneExCrystalWaltz);
        m.insert(("A4a 081", 0), AttackId::A4a025RaikouExVoltaicBullet);
        m.insert(("A4a 087", 0), AttackId::A4a010EnteiExBlazingBeatdown);
        m.insert(("A4a 088", 0), AttackId::A4a025RaikouExVoltaicBullet);
        m.insert(("A4a 090", 0), AttackId::A4a020SuicuneExCrystalWaltz);
        m.insert(("A4a 093", 0), AttackId::A1057PsyduckHeadache);
        m.insert(("A4a 096", 0), AttackId::A1069KinglerKOCrab);
        m.insert(("A4a 105", 0), AttackId::A4a023MantykeSplashyToss);

        // A4b
        m.insert(("A4b 005", 1), AttackId::A1004VenusaurExGiantBloom);
        m.insert(("A4b 006", 0), AttackId::A2b001WeedleMultiply);
        m.insert(("A4b 007", 0), AttackId::A2b001WeedleMultiply);
        m.insert(("A4b 008", 0), AttackId::A2b002KakunaStringShot);
        m.insert(("A4b 009", 0), AttackId::A2b002KakunaStringShot);
        m.insert(("A4b 010", 0), AttackId::A2b003BeedrillExCrushingSpear);
        m.insert(("A4b 013", 0), AttackId::A1023ExeggutorExTropicalSwing);
        m.insert(("A4b 023", 0), AttackId::A4021ShuckleExTripleSlap);
        m.insert(("A4b 024", 0), AttackId::A1a003CelebiExPowerfulBloom);
        m.insert(("A4b 044", 1), AttackId::A3a006BuzzwoleExBigBeat);
        m.insert(("A4b 045", 0), AttackId::A3a007PheromosaJumpBlues);
        m.insert(("A4b 046", 0), AttackId::A3a007PheromosaJumpBlues);
        m.insert(("A4b 049", 0), AttackId::A2b005SprigatitoCryForHelp);
        m.insert(("A4b 050", 0), AttackId::A2b005SprigatitoCryForHelp);
        m.insert(("A4b 053", 0), AttackId::A2b007MeowscaradaFightingClaws);
        m.insert(("A4b 054", 0), AttackId::A2b007MeowscaradaFightingClaws);
        m.insert(("A4b 055", 0), AttackId::A1033CharmanderEmber);
        m.insert(("A4b 056", 0), AttackId::A1033CharmanderEmber);
        m.insert(("A4b 059", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A4b 060", 0), AttackId::A2b010CharizardExStoke);
        m.insert(("A4b 063", 0), AttackId::A1041ArcanineExInfernoOnrush);
        m.insert(("A4b 066", 0), AttackId::A3b009FlareonExFireSpin);
        m.insert(("A4b 067", 0), AttackId::A1047MoltresExInfernoDance);
        m.insert(("A4b 069", 0), AttackId::A3b010TorkoalFlamethrower);
        m.insert(("A4b 070", 0), AttackId::A3b010TorkoalFlamethrower);
        m.insert(("A4b 087", 1), AttackId::A1056BlastoiseExHydroBazooka);
        m.insert(("A4b 096", 0), AttackId::A1a017MagikarpLeapOut);
        m.insert(("A4b 097", 0), AttackId::A1a017MagikarpLeapOut);
        m.insert(("A4b 101", 1), AttackId::A1084ArticunoExBlizzard);
        m.insert(("A4b 107", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A4b 108", 0), AttackId::A2050ManaphyOceanicGift);
        m.insert(("A4b 109", 0), AttackId::A2050ManaphyOceanicGift);
        m.insert(("A4b 131", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A4b 132", 0), AttackId::A2b022PikachuExThunderbolt);
        m.insert(("A4b 137", 0), AttackId::A2053MagnezoneThunderBlast);
        m.insert(("A4b 138", 0), AttackId::A2053MagnezoneThunderBlast);
        m.insert(("A4b 139", 1), AttackId::A1104ZapdosExThunderingHurricane);
        m.insert(("A4b 148", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("A4b 156", 0), AttackId::A1127JynxPsychic);
        m.insert(("A4b 157", 0), AttackId::A1127JynxPsychic);
        m.insert(("A4b 158", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("A4b 172", 0), AttackId::A2b035GiratinaExChaoticImpact);
        m.insert(("A4b 180", 0), AttackId::A3085CosmogTeleport);
        m.insert(("A4b 181", 0), AttackId::A3085CosmogTeleport);
        m.insert(("A4b 182", 0), AttackId::A3086CosmoemStiffen);
        m.insert(("A4b 183", 0), AttackId::A3086CosmoemStiffen);
        m.insert(("A4b 196", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A4b 222", 0), AttackId::A3a033LycanrocExLycanfang);
        m.insert(("A4b 242", 0), AttackId::A2098SneaselDoubleScratch);
        m.insert(("A4b 243", 0), AttackId::A2098SneaselDoubleScratch);
        m.insert(("A4b 248", 0), AttackId::A3a043GuzzlordExGrindcore);
        m.insert(("A4b 251", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);
        m.insert(("A4b 252", 0), AttackId::A4124SkarmoryExSteelWing);
        m.insert(("A4b 253", 0), AttackId::A2a057ProbopassExDefensiveUnit);
        m.insert(("A4b 254", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("A4b 259", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A4b 271", 0), AttackId::A3b053DragoniteExGigaImpact);
        m.insert(("A4b 279", 0), AttackId::A1195WigglytuffExSleepySong);
        m.insert(("A4b 285", 0), AttackId::A3b055EeveeCollect);
        m.insert(("A4b 286", 0), AttackId::A3b055EeveeCollect);
        m.insert(("A4b 289", 0), AttackId::A4149LugiaExElementalBlast);
        m.insert(("A4b 299", 0), AttackId::A2a071ArceusExUltimateForce);
        m.insert(("A4b 300", 0), AttackId::A3a060TypeNullQuickBlow);
        m.insert(("A4b 301", 0), AttackId::A3a060TypeNullQuickBlow);
        m.insert(("A4b 302", 0), AttackId::A3a061SilvallyBraveBuddies);
        m.insert(("A4b 303", 0), AttackId::A3a061SilvallyBraveBuddies);
        m.insert(("A4b 304", 0), AttackId::A3a062CelesteelaMoombahton);
        m.insert(("A4b 305", 0), AttackId::A3a062CelesteelaMoombahton);
        m.insert(("A4b 360", 1), AttackId::A3a006BuzzwoleExBigBeat);
        m.insert(("A4b 361", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A4b 363", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A4b 364", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A4b 365", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("A4b 368", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("A4b 369", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A4b 371", 0), AttackId::A4149LugiaExElementalBlast);
        m.insert(("A4b 372", 0), AttackId::A2a071ArceusExUltimateForce);
        m.insert(("A4b 376", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A4b 377", 0), AttackId::A2b035GiratinaExChaoticImpact);

        // B1
        m.insert(("B1 002", 0), AttackId::B1002MegaPinsirExCriticalScissors);
        m.insert(("B1 031", 0), AttackId::B1031RapidashExSprintingFlare);
        m.insert(("B1 035", 0), AttackId::B1035BlazikenBlazeKick);
        m.insert(("B1 036", 0), AttackId::B1036MegaBlazikenExMegaBurning);
        m.insert(("B1 050", 0), AttackId::B1050MagikarpWaterfallEvolution);
        m.insert(("B1 052", 0), AttackId::B1052MegaGyaradosExMegaBlaster);
        m.insert(("B1 085", 0), AttackId::B1085MegaAmpharosExLightningLancer);
        m.insert(("B1 102", 0), AttackId::B1102MegaAltariaExMegaHarmony);
        m.insert(("B1 150", 0), AttackId::B1150AbsolOminousClaw);
        m.insert(("B1 151", 0), AttackId::B1151MegaAbsolExDarknessClaw);
        m.insert(("B1 232", 0), AttackId::B1050MagikarpWaterfallEvolution);
        m.insert(("B1 251", 0), AttackId::B1002MegaPinsirExCriticalScissors);
        m.insert(("B1 253", 0), AttackId::B1031RapidashExSprintingFlare);
        m.insert(("B1 254", 0), AttackId::B1036MegaBlazikenExMegaBurning);
        m.insert(("B1 255", 0), AttackId::B1052MegaGyaradosExMegaBlaster);
        m.insert(("B1 258", 0), AttackId::B1085MegaAmpharosExLightningLancer);
        m.insert(("B1 259", 0), AttackId::B1102MegaAltariaExMegaHarmony);
        m.insert(("B1 262", 0), AttackId::B1151MegaAbsolExDarknessClaw);
        m.insert(("B1 272", 0), AttackId::B1002MegaPinsirExCriticalScissors);
        m.insert(("B1 274", 0), AttackId::B1031RapidashExSprintingFlare);
        m.insert(("B1 277", 0), AttackId::B1085MegaAmpharosExLightningLancer);
        m.insert(("B1 280", 0), AttackId::B1151MegaAbsolExDarknessClaw);
        m.insert(("B1 284", 0), AttackId::B1036MegaBlazikenExMegaBurning);
        m.insert(("B1 285", 0), AttackId::B1052MegaGyaradosExMegaBlaster);
        m.insert(("B1 286", 0), AttackId::B1102MegaAltariaExMegaHarmony);
        m.insert(("B1 292", 0), AttackId::A1046MoltresSkyAttack);
        m.insert(("B1 298", 0), AttackId::A1083ArticunoIceBeam);
        m.insert(("B1 299", 0), AttackId::A2050ManaphyOceanicGift);
        m.insert(("B1 302", 0), AttackId::A1103ZapdosRagingThunder);
        m.insert(("B1 309", 0), AttackId::A1a045GolemGuardPress);
        m.insert(("B1 312", 0), AttackId::A1196MeowthPayDay);
        m.insert(("B1 319", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("B1 321", 0), AttackId::A2b022PikachuExThunderbolt);
        m.insert(("B1 322", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("B1 323", 0), AttackId::A3a033LycanrocExLycanfang);
        m.insert(("B1 325", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);
        m.insert(("B1 326", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("B1 328", 0), AttackId::A2a071ArceusExUltimateForce);

        // Promo
        m.insert(("P-A 010", 0), AttackId::A1128MewtwoPowerBlast);
        m.insert(("P-A 012", 0), AttackId::A1196MeowthPayDay);
        m.insert(("P-A 018", 0), AttackId::A1003VenusaurMegaDrain);
        m.insert(("P-A 025", 0), AttackId::A1047MoltresExInfernoDance);
        m.insert(("P-A 029", 0), AttackId::A1055BlastoiseHydroPump);
        m.insert(("P-A 031", 0), AttackId::A1213CinccinoDoTheWave);
        m.insert(("P-A 032", 0), AttackId::A1033CharmanderEmber);
        m.insert(("P-A 034", 0), AttackId::A2035PiplupNap);
        m.insert(("P-A 039", 0), AttackId::A2111SkarmoryMetalArms);
        m.insert(("P-A 048", 0), AttackId::A2050ManaphyOceanicGift);
        m.insert(("P-A 050", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("P-A 052", 0), AttackId::A2b005SprigatitoCryForHelp);
        m.insert(("P-A 060", 0), AttackId::A1a001ExeggcuteGrowthSpurt);
        m.insert(("P-A 067", 0), AttackId::A3085CosmogTeleport);
        m.insert(("P-A 069", 0), AttackId::A3002AlolanExeggutorTropicalHammer);
        m.insert(("P-A 070", 0), AttackId::A3041AlolanNinetalesBlizzard);
        m.insert(("P-A 072", 0), AttackId::PA072AlolanGrimerPoisonGas);
        m.insert(("P-A 079", 0), AttackId::PA079DuskManeNecrozmaBlackMetal);
        m.insert(("P-A 080", 0), AttackId::A3a053StakatakaBrassRock);
        m.insert(("P-A 082", 0), AttackId::A3a044Poipole2Step);
        m.insert(("P-A 084", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("P-A 093", 0), AttackId::A4077CleffaTwinklyCall);
        m.insert(("P-A 110", 0), AttackId::A4a010EnteiExBlazingBeatdown);
        m.insert(("P-A 117", 0), AttackId::A4120AbsolLeapOver);

        m
    };
}

impl AttackId {
    // None if not found or implemented
    pub fn from_pokemon_index(pokemon_id: &str, index: usize) -> Option<Self> {
        ATTACK_ID_MAP.get(&(pokemon_id, index)).copied()
    }
}
