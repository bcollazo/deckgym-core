// NOTE: This file should be treated as deprecated.
// The new way to implement attacks is with Mechanic enum.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttackId {
    A1115AbraTeleport,
    A1136GolurkDoubleLariat,
    A1149GolemDoubleEdge,
    A1153MarowakExBonemerang,
    A1163GrapploctKnockBack,
    A1178MawileCrunch,
    A1181MeltanAmass,
    A1196MeowthPayDay,
    A1201LickitungContinuousLick,
    A1203KangaskhanDizzyPunch,
    A1a001ExeggcuteGrowthSpurt,
    A1a010PonytaStomp,
    A1a011RapidashRisingLunge,
    A1a017MagikarpLeapOut,
    A1a026RaichuGigashock,
    A1a061EeveeContinuousSteps,
    A2023MagmarStoke,
    A2029InfernapeExFlareBlitz,
    A2049PalkiaExDimensionalStorm,
    A2056ElectabuzzCharge,
    A2060LuxrayVoltBolt,
    A2084GliscorAcrobatics,
    A2098SneaselDoubleScratch,
    A2118ProbopassTripleNose,
    A2131AmbipomDoubleHit,
    A2141ChatotFuryAttack,
    A2a001HeracrossSingleHornThrow,
    A2a063SnorlaxCollapse,
    A2b010CharizardExStoke,
    A2b032MrMimeJuggling,
    A2b044FlamigoDoubleKick,
    A3002AlolanExeggutorTropicalHammer,
    A3012DecidueyeExPierceThePain,
    A3019SteeneeDoubleSpin,
    A3020TsareenaThreeKickCombo,
    A3040AlolanVulpixCallForthCold,
    A3071SpoinkPsycharge,
    A3085CosmogTeleport,
    A3112AbsolUnseenClaw,
    A3116ToxapexSpikeCannon,
    A3122SolgaleoExSolBreaker,
    A3a003RowletFuryAttack,
    A3a019TapuKokoExPlasmaHurricane,
    A3a043GuzzlordExGrindcore,
    A3a044Poipole2Step,
    A3a047AlolanDugtrioExTripletHeadbutt,
    A3a060TypeNullQuickBlow,
    A3a061SilvallyBraveBuddies,
    A3a062CelesteelaMoombahton,
    A3b013IncineroarDarkestLariat,
    A3b020VanilluxeDoubleSpin,
    A3b055EeveeCollect,
    A3b057SnorlaxExFlopDownPunch,
    A3b058AipomDoubleHit,
    A4021ShuckleExTripleSlap,
    A4032MagbyToastyToss,
    A4066PichuCracklyToss,
    A4077CleffaTwinklyCall,
    A4105BinacleDualChop,
    A4134EeveeFindAFriend,
    A4146UrsaringSwingAround,
    A4a021FeebasLeapOut,
    A4a023MantykeSplashyToss,
    B1052MegaGyaradosExMegaBlaster,
    B1085MegaAmpharosExLightningLancer,
    B1101SableyeDirtyThrow,
    B1150AbsolOminousClaw,
    B1151MegaAbsolExDarknessClaw,
    B1a002IvysaurSynthesis,
}

// Create a static HashMap for fast (pokemon, index) lookup
lazy_static::lazy_static! {
    static ref ATTACK_ID_MAP: HashMap<(&'static str, usize), AttackId> = {
        let mut m = HashMap::new();
        m.insert(("A1 115", 0), AttackId::A1115AbraTeleport);
        m.insert(("A1 136", 0), AttackId::A1136GolurkDoubleLariat);
        m.insert(("A1 149", 0), AttackId::A1149GolemDoubleEdge);
        m.insert(("A1 153", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A1 163", 0), AttackId::A1163GrapploctKnockBack);
        m.insert(("A1 178", 0), AttackId::A1178MawileCrunch);
        m.insert(("A1 181", 0), AttackId::A1181MeltanAmass);
        m.insert(("A1 196", 0), AttackId::A1196MeowthPayDay);
        m.insert(("A1 201", 0), AttackId::A1201LickitungContinuousLick);
        m.insert(("A1 203", 0), AttackId::A1203KangaskhanDizzyPunch);
        // Full Arts A1
        m.insert(("A1 246", 0), AttackId::A1196MeowthPayDay);
        m.insert(("A1 264", 0), AttackId::A1153MarowakExBonemerang);

        // A1a
        m.insert(("A1a 001", 0), AttackId::A1a001ExeggcuteGrowthSpurt);
        m.insert(("A1a 010", 0), AttackId::A1a010PonytaStomp);
        m.insert(("A1a 011", 0), AttackId::A1a011RapidashRisingLunge);
        m.insert(("A1a 017", 0), AttackId::A1a017MagikarpLeapOut);
        m.insert(("A1a 026", 0), AttackId::A1a026RaichuGigashock);
        m.insert(("A1a 061", 0), AttackId::A1a061EeveeContinuousSteps);
        // Full Arts A1a

        // A2
        m.insert(("A2 023", 0), AttackId::A2023MagmarStoke);
        m.insert(("A2 029", 0), AttackId::A2029InfernapeExFlareBlitz);
        m.insert(("A2 049", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A2 056", 0), AttackId::A2056ElectabuzzCharge);
        m.insert(("A2 060", 0), AttackId::A2060LuxrayVoltBolt);
        m.insert(("A2 084", 0), AttackId::A2084GliscorAcrobatics);
        m.insert(("A2 098", 0), AttackId::A2098SneaselDoubleScratch);
        m.insert(("A2 118", 0), AttackId::A2118ProbopassTripleNose);
        m.insert(("A2 131", 0), AttackId::A2131AmbipomDoubleHit);
        m.insert(("A2 141", 0), AttackId::A2141ChatotFuryAttack);
        m.insert(("A2 181", 0), AttackId::A2029InfernapeExFlareBlitz);
        m.insert(("A2 182", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A2 197", 0), AttackId::A2029InfernapeExFlareBlitz);
        m.insert(("A2 204", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A2 206", 1), AttackId::A2049PalkiaExDimensionalStorm);

        // A2a
        m.insert(("A2a 001", 0), AttackId::A2a001HeracrossSingleHornThrow);
        m.insert(("A2a 063", 0), AttackId::A2a063SnorlaxCollapse);

        // A2b
        m.insert(("A2b 010", 0), AttackId::A2b010CharizardExStoke);
        m.insert(("A2b 032", 0), AttackId::A2b032MrMimeJuggling);
        m.insert(("A2b 044", 0), AttackId::A2b044FlamigoDoubleKick);
        m.insert(("A2b 080", 0), AttackId::A2b010CharizardExStoke);
        m.insert(("A2b 108", 0), AttackId::A2b010CharizardExStoke);

        // A3
        m.insert(("A3 002", 0), AttackId::A3002AlolanExeggutorTropicalHammer);
        m.insert(("A3 012", 0), AttackId::A3012DecidueyeExPierceThePain);
        m.insert(("A3 019", 0), AttackId::A3019SteeneeDoubleSpin);
        m.insert(("A3 020", 0), AttackId::A3020TsareenaThreeKickCombo);
        m.insert(("A3 040", 0), AttackId::A3040AlolanVulpixCallForthCold);
        m.insert(("A3 071", 0), AttackId::A3071SpoinkPsycharge);
        m.insert(("A3 085", 0), AttackId::A3085CosmogTeleport);
        m.insert(("A3 112", 0), AttackId::A3112AbsolUnseenClaw);
        m.insert(("A3 116", 0), AttackId::A3116ToxapexSpikeCannon);
        m.insert(("A3 122", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A3 156", 0), AttackId::A3002AlolanExeggutorTropicalHammer);
        m.insert(("A3 158", 0), AttackId::A3020TsareenaThreeKickCombo);
        m.insert(("A3 162", 0), AttackId::A3040AlolanVulpixCallForthCold);
        m.insert(("A3 171", 0), AttackId::A3085CosmogTeleport);
        m.insert(("A3 180", 0), AttackId::A3012DecidueyeExPierceThePain);
        m.insert(("A3 189", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A3 198", 0), AttackId::A3012DecidueyeExPierceThePain);
        m.insert(("A3 207", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A3 236", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A3 239", 0), AttackId::A3122SolgaleoExSolBreaker);

        // A3a
        m.insert(("A3a 003", 0), AttackId::A3a003RowletFuryAttack);
        m.insert(("A3a 019", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("A3a 043", 0), AttackId::A3a043GuzzlordExGrindcore);
        m.insert(("A3a 044", 0), AttackId::A3a044Poipole2Step);
        m.insert(("A3a 047", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);
        m.insert(("A3a 060", 0), AttackId::A3a060TypeNullQuickBlow);
        m.insert(("A3a 061", 0), AttackId::A3a061SilvallyBraveBuddies);
        m.insert(("A3a 062", 0), AttackId::A3a062CelesteelaMoombahton);
        m.insert(("A3a 070", 0), AttackId::A3a003RowletFuryAttack);
        m.insert(("A3a 074", 0), AttackId::A3a061SilvallyBraveBuddies);
        m.insert(("A3a 075", 0), AttackId::A3a062CelesteelaMoombahton);
        m.insert(("A3a 077", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("A3a 079", 0), AttackId::A3a043GuzzlordExGrindcore);
        m.insert(("A3a 080", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);
        m.insert(("A3a 084", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("A3a 086", 0), AttackId::A3a043GuzzlordExGrindcore);
        m.insert(("A3a 087", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);

        // A3b
        m.insert(("A3b 013", 0), AttackId::A3b013IncineroarDarkestLariat);
        m.insert(("A3b 020", 0), AttackId::A3b020VanilluxeDoubleSpin);
        m.insert(("A3b 055", 0), AttackId::A3b055EeveeCollect);
        m.insert(("A3b 057", 0), AttackId::A3b057SnorlaxExFlopDownPunch);
        m.insert(("A3b 058", 0), AttackId::A3b058AipomDoubleHit);
        m.insert(("A3b 078", 0), AttackId::A3b055EeveeCollect);
        m.insert(("A3b 084", 0), AttackId::A3b057SnorlaxExFlopDownPunch);
        m.insert(("A3b 091", 0), AttackId::A3b057SnorlaxExFlopDownPunch);

        // A4
        m.insert(("A4 021", 0), AttackId::A4021ShuckleExTripleSlap);
        m.insert(("A4 032", 0), AttackId::A4032MagbyToastyToss);
        m.insert(("A4 066", 0), AttackId::A4066PichuCracklyToss);
        m.insert(("A4 077", 0), AttackId::A4077CleffaTwinklyCall);
        m.insert(("A4 105", 0), AttackId::A4105BinacleDualChop);
        m.insert(("A4 134", 0), AttackId::A4134EeveeFindAFriend);
        m.insert(("A4 146", 0), AttackId::A4146UrsaringSwingAround);
        m.insert(("A4 166", 0), AttackId::A4032MagbyToastyToss);
        m.insert(("A4 171", 0), AttackId::A4066PichuCracklyToss);
        m.insert(("A4 186", 0), AttackId::A4021ShuckleExTripleSlap);
        m.insert(("A4 202", 0), AttackId::A4021ShuckleExTripleSlap);
        m.insert(("A4 214", 0), AttackId::A1a017MagikarpLeapOut);
        m.insert(("A4 231", 0), AttackId::A4134EeveeFindAFriend);

        // A4a
        m.insert(("A4a 021", 0), AttackId::A4a021FeebasLeapOut);
        m.insert(("A4a 023", 0), AttackId::A4a023MantykeSplashyToss);
        m.insert(("A4a 101", 0), AttackId::A2029InfernapeExFlareBlitz);
        m.insert(("A4a 105", 0), AttackId::A4a023MantykeSplashyToss);

        // A4b
        m.insert(("A4b 023", 0), AttackId::A4021ShuckleExTripleSlap);
        m.insert(("A4b 042", 0), AttackId::A3012DecidueyeExPierceThePain);
        m.insert(("A4b 060", 0), AttackId::A2b010CharizardExStoke);
        m.insert(("A4b 075", 0), AttackId::A2029InfernapeExFlareBlitz);
        m.insert(("A4b 096", 0), AttackId::A1a017MagikarpLeapOut);
        m.insert(("A4b 097", 0), AttackId::A1a017MagikarpLeapOut);
        m.insert(("A4b 107", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A4b 148", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("A4b 180", 0), AttackId::A3085CosmogTeleport);
        m.insert(("A4b 181", 0), AttackId::A3085CosmogTeleport);
        m.insert(("A4b 196", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A4b 242", 0), AttackId::A2098SneaselDoubleScratch);
        m.insert(("A4b 243", 0), AttackId::A2098SneaselDoubleScratch);
        m.insert(("A4b 248", 0), AttackId::A3a043GuzzlordExGrindcore);
        m.insert(("A4b 251", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);
        m.insert(("A4b 259", 0), AttackId::A3122SolgaleoExSolBreaker);
        m.insert(("A4b 285", 0), AttackId::A3b055EeveeCollect);
        m.insert(("A4b 286", 0), AttackId::A3b055EeveeCollect);
        m.insert(("A4b 288", 0), AttackId::A3b057SnorlaxExFlopDownPunch);
        m.insert(("A4b 300", 0), AttackId::A3a060TypeNullQuickBlow);
        m.insert(("A4b 301", 0), AttackId::A3a060TypeNullQuickBlow);
        m.insert(("A4b 302", 0), AttackId::A3a061SilvallyBraveBuddies);
        m.insert(("A4b 303", 0), AttackId::A3a061SilvallyBraveBuddies);
        m.insert(("A4b 304", 0), AttackId::A3a062CelesteelaMoombahton);
        m.insert(("A4b 305", 0), AttackId::A3a062CelesteelaMoombahton);
        m.insert(("A4b 363", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("A4b 369", 0), AttackId::A3122SolgaleoExSolBreaker);

        // B1a
        m.insert(("B1a 002", 0), AttackId::B1a002IvysaurSynthesis);
        m.insert(("B1a 097", 0), AttackId::A3a061SilvallyBraveBuddies);

        // B1
        m.insert(("B1 052", 0), AttackId::B1052MegaGyaradosExMegaBlaster);
        m.insert(("B1 085", 0), AttackId::B1085MegaAmpharosExLightningLancer);
        m.insert(("B1 101", 0), AttackId::B1101SableyeDirtyThrow);
        m.insert(("B1 150", 0), AttackId::B1150AbsolOminousClaw);
        m.insert(("B1 151", 0), AttackId::B1151MegaAbsolExDarknessClaw);
        m.insert(("B1 255", 0), AttackId::B1052MegaGyaradosExMegaBlaster);
        m.insert(("B1 258", 0), AttackId::B1085MegaAmpharosExLightningLancer);
        m.insert(("B1 262", 0), AttackId::B1151MegaAbsolExDarknessClaw);
        m.insert(("B1 277", 0), AttackId::B1085MegaAmpharosExLightningLancer);
        m.insert(("B1 280", 0), AttackId::B1151MegaAbsolExDarknessClaw);
        m.insert(("B1 285", 0), AttackId::B1052MegaGyaradosExMegaBlaster);
        m.insert(("B1 312", 0), AttackId::A1196MeowthPayDay);
        m.insert(("B1 317", 0), AttackId::A3012DecidueyeExPierceThePain);
        m.insert(("B1 319", 1), AttackId::A2049PalkiaExDimensionalStorm);
        m.insert(("B1 322", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("B1 325", 0), AttackId::A3a047AlolanDugtrioExTripletHeadbutt);

        // Promo
        m.insert(("P-A 012", 0), AttackId::A1196MeowthPayDay);
        m.insert(("P-A 049", 0), AttackId::A2a063SnorlaxCollapse);
        m.insert(("P-A 060", 0), AttackId::A1a001ExeggcuteGrowthSpurt);
        m.insert(("P-A 067", 0), AttackId::A3085CosmogTeleport);
        m.insert(("P-A 069", 0), AttackId::A3002AlolanExeggutorTropicalHammer);
        m.insert(("P-A 082", 0), AttackId::A3a044Poipole2Step);
        m.insert(("P-A 084", 0), AttackId::A3a019TapuKokoExPlasmaHurricane);
        m.insert(("P-A 093", 0), AttackId::A4077CleffaTwinklyCall);

        m
    };
}

impl AttackId {
    // None if not found or implemented
    pub fn from_pokemon_index(pokemon_id: &str, index: usize) -> Option<Self> {
        ATTACK_ID_MAP.get(&(pokemon_id, index)).copied()
    }
}
