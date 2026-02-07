use std::collections::HashMap;

// TODO: Probably best to generate this file from database.json via card_enum_generator.rs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbilityId {
    A1020VictreebelFragranceTrap,
    A1061PoliwrathCounterattack,
    A1089GreninjaWaterShuriken,
    A1098MagnetonVoltCharge,
    A1123GengarExShadowySpellbind,
    A1177Weezing,
    A1188PidgeotDriveOff,
    A1007Butterfree,
    A1132Gardevoir,
    A1a006SerperiorJungleTotem,
    A1a046AerodactylExPrimevalLaw,
    A1a019VaporeonWashOut,
    A2a010LeafeonExForestBreath,
    A2a050CrobatCunningLink,
    A2a071Arceus,
    A2022ShayminFragrantFlowerGarden,
    A2072DusknoirShadowVoid,
    A2078GiratinaLevitate,
    A2092LucarioFightingCoach,
    A2a069ShayminSkySupport,
    A2110DarkraiExNightmareAura,
    A2b035GiratinaExBrokenSpaceBellow,
    A3066OricoricSafeguard,
    A3122SolgaleoExRisingRoad,
    A3141KomalaComatose,
    A3a015LuxrayIntimidatingFang,
    A3a021ZeraoraThunderclapFlash,
    A3a027ShiinoticIlluminate,
    A3a042NihilegoMorePoison,
    A3a062CelesteelaUltraThrusters,
    A3b009FlareonExCombust,
    A3b034SylveonExHappyRibbon,
    A3b056EeveeExVeeveeVolve,
    A3b057SnorlaxExFullMouthManner,
    A4083EspeonExPsychicHealing,
    A4112UmbreonExDarkChase,
    A4a010EnteiExLegendaryPulse,
    A4a020SuicuneExLegendaryPulse,
    A4a022MiloticHealingRipples,
    A4a025RaikouExLegendaryPulse,
    B1073GreninjaExShiftingStream,
    B1121IndeedeeExWatchOver,
    B1157HydreigonRoarInUnison,
    B1160DragalgeExPoisonPoint,
    B1184EeveeBoostedEvolution,
    B1172AegislashCursedMetal,
    B1177GoomyStickyMembrane,
    PA037CresseliaExLunarPlumage,
    B1a006AriadosTrapTerritory,
    B1a012CharmeleonIgnition,
    B1a018WartortleShellShield,
    B1a034ReuniclusInfiniteIncrease,
    B1a065FurfrouFurCoat,
    A4a032MisdreavusInfiltratingInspection,
}

// Create a static HashMap for fast (pokemon, index) lookup
lazy_static::lazy_static! {
    static ref ABILITY_ID_MAP: HashMap<&'static str, AbilityId> = {
        let mut m = HashMap::new();
        // A1
        m.insert("A1 020", AbilityId::A1020VictreebelFragranceTrap);
        m.insert("A1 061", AbilityId::A1061PoliwrathCounterattack);

        m.insert("A1 098", AbilityId::A1098MagnetonVoltCharge);
        m.insert("A1 123", AbilityId::A1123GengarExShadowySpellbind);
        m.insert("A1 177", AbilityId::A1177Weezing);
        m.insert("A1 188", AbilityId::A1188PidgeotDriveOff);
        m.insert("A1 245", AbilityId::A1188PidgeotDriveOff);
        m.insert("A1 132", AbilityId::A1132Gardevoir);
        m.insert("A1 261", AbilityId::A1123GengarExShadowySpellbind);
        m.insert("A1 277", AbilityId::A1123GengarExShadowySpellbind);
        // A1a
        m.insert("A1a 006", AbilityId::A1a006SerperiorJungleTotem);
        m.insert("A1a 019", AbilityId::A1a019VaporeonWashOut);
        m.insert("A1a 046", AbilityId::A1a046AerodactylExPrimevalLaw);
        m.insert("A1a 056", AbilityId::A1061PoliwrathCounterattack);
        m.insert("A1a 070", AbilityId::A1a006SerperiorJungleTotem);
        m.insert("A1a 072", AbilityId::A1a019VaporeonWashOut);
        m.insert("A1a 078", AbilityId::A1a046AerodactylExPrimevalLaw);
        m.insert("A1a 084", AbilityId::A1a046AerodactylExPrimevalLaw);
        // A2
        m.insert("A2 072", AbilityId::A2072DusknoirShadowVoid);
        m.insert("A2 078", AbilityId::A2078GiratinaLevitate);
        m.insert("A2 092", AbilityId::A2092LucarioFightingCoach);
        m.insert("A2 167", AbilityId::A2078GiratinaLevitate);
        m.insert("A2 170", AbilityId::A2092LucarioFightingCoach);
        m.insert("A2 110", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("A2 187", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("A2 202", AbilityId::A2110DarkraiExNightmareAura);
        // A2a
        m.insert("A2a 010", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 069", AbilityId::A2a069ShayminSkySupport);
        m.insert("A2a 081", AbilityId::A2a069ShayminSkySupport);
        m.insert("A2a 082", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 091", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 050", AbilityId::A2a050CrobatCunningLink);
        m.insert("A2a 071", AbilityId::A2a071Arceus);
        m.insert("A2a 086", AbilityId::A2a071Arceus);
        m.insert("A2a 095", AbilityId::A2a071Arceus);
        m.insert("A2a 096", AbilityId::A2a071Arceus);
        // A2b
        m.insert("A2b 028", AbilityId::A1061PoliwrathCounterattack);
        m.insert("A2b 035", AbilityId::A2b035GiratinaExBrokenSpaceBellow);
        m.insert("A2b 083", AbilityId::A2b035GiratinaExBrokenSpaceBellow);
        m.insert("A2b 096", AbilityId::A2b035GiratinaExBrokenSpaceBellow);
        // A3
        m.insert("A3 066", AbilityId::A3066OricoricSafeguard);
        m.insert("A3 122", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 141", AbilityId::A3141KomalaComatose);
        m.insert("A3 234", AbilityId::A1123GengarExShadowySpellbind);
        m.insert("A3 165", AbilityId::A3066OricoricSafeguard);
        m.insert("A3 179", AbilityId::A3141KomalaComatose);
        m.insert("A3 189", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 207", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 239", AbilityId::A3122SolgaleoExRisingRoad);
        // A3a
        m.insert("A3a 015", AbilityId::A3a015LuxrayIntimidatingFang);
        m.insert("A3a 021", AbilityId::A3a021ZeraoraThunderclapFlash);
        m.insert("A3a 027", AbilityId::A3a027ShiinoticIlluminate);
        m.insert("A3a 042", AbilityId::A3a042NihilegoMorePoison);
        m.insert("A3a 052", AbilityId::A1061PoliwrathCounterattack);
        m.insert("A3a 062", AbilityId::A3a062CelesteelaUltraThrusters);
        m.insert("A3a 075", AbilityId::A3a062CelesteelaUltraThrusters);

        m.insert("A3a 101", AbilityId::A1a046AerodactylExPrimevalLaw);
        m.insert("A3a 103", AbilityId::A3a042NihilegoMorePoison);
        m.insert("A3b 009", AbilityId::A3b009FlareonExCombust);
        m.insert("A3b 034", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A3b 056", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A3b 057", AbilityId::A3b057SnorlaxExFullMouthManner);
        m.insert("A3b 079", AbilityId::A3b009FlareonExCombust);
        m.insert("A3b 081", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A3b 083", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A3b 084", AbilityId::A3b057SnorlaxExFullMouthManner);
        m.insert("A3b 087", AbilityId::A3b009FlareonExCombust);
        m.insert("A3b 089", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A3b 091", AbilityId::A3b057SnorlaxExFullMouthManner);
        m.insert("A3b 092", AbilityId::A3b056EeveeExVeeveeVolve);
        // A4
        m.insert("A4 083", AbilityId::A4083EspeonExPsychicHealing);
        m.insert("A4 190", AbilityId::A4083EspeonExPsychicHealing);
        m.insert("A4 205", AbilityId::A4083EspeonExPsychicHealing);
        m.insert("A4 112", AbilityId::A4112UmbreonExDarkChase);
        m.insert("A4 193", AbilityId::A4112UmbreonExDarkChase);
        m.insert("A4 208", AbilityId::A4112UmbreonExDarkChase);
        m.insert("A4 218", AbilityId::A1098MagnetonVoltCharge);
        m.insert("A4 233", AbilityId::A2a010LeafeonExForestBreath);
        // A4a
        m.insert("A4a 010", AbilityId::A4a010EnteiExLegendaryPulse);
        m.insert("A4a 020", AbilityId::A4a020SuicuneExLegendaryPulse);
        m.insert("A4a 022", AbilityId::A4a022MiloticHealingRipples);
        m.insert("A4a 025", AbilityId::A4a025RaikouExLegendaryPulse);
        m.insert("A4a 065", AbilityId::A1061PoliwrathCounterattack);
        m.insert("A4a 072", AbilityId::A4a022MiloticHealingRipples);
        m.insert("A4a 079", AbilityId::A4a010EnteiExLegendaryPulse);
        m.insert("A4a 080", AbilityId::A4a020SuicuneExLegendaryPulse);
        m.insert("A4a 081", AbilityId::A4a025RaikouExLegendaryPulse);
        m.insert("A4a 087", AbilityId::A4a010EnteiExLegendaryPulse);
        m.insert("A4a 088", AbilityId::A4a025RaikouExLegendaryPulse);
        m.insert("A4a 090", AbilityId::A4a020SuicuneExLegendaryPulse);
        //A4b
        m.insert("A4b 066", AbilityId::A3b009FlareonExCombust);
        m.insert("A4b 099", AbilityId::A1a019VaporeonWashOut);
        m.insert("A4b 100", AbilityId::A1a019VaporeonWashOut);
        m.insert("A4b 135", AbilityId::A1098MagnetonVoltCharge);
        m.insert("A4b 136", AbilityId::A1098MagnetonVoltCharge);
        m.insert("A4b 146", AbilityId::A3066OricoricSafeguard);
        m.insert("A4b 155", AbilityId::A1123GengarExShadowySpellbind);
        m.insert("A4b 147", AbilityId::A3066OricoricSafeguard);
        m.insert("A4b 149", AbilityId::A3a021ZeraoraThunderclapFlash);
        m.insert("A4b 197", AbilityId::A1a046AerodactylExPrimevalLaw);
        m.insert("A4b 150", AbilityId::A3a021ZeraoraThunderclapFlash);
        m.insert("A4b 160", AbilityId::A4083EspeonExPsychicHealing);
        m.insert("A4b 212", AbilityId::A2092LucarioFightingCoach);
        m.insert("A4b 213", AbilityId::A2092LucarioFightingCoach);
        m.insert("A4b 212", AbilityId::A2092LucarioFightingCoach);
        m.insert("A4b 213", AbilityId::A2092LucarioFightingCoach);
        m.insert("A4b 230", AbilityId::A2a050CrobatCunningLink);
        m.insert("A4b 231", AbilityId::A2a050CrobatCunningLink);
        m.insert("A4b 241", AbilityId::A4112UmbreonExDarkChase);
        m.insert("A4b 245", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("A4b 246", AbilityId::A3a042NihilegoMorePoison);
        m.insert("A4b 247", AbilityId::A3a042NihilegoMorePoison);
        m.insert("A4b 259", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A4b 287", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A4b 288", AbilityId::A3b057SnorlaxExFullMouthManner);
        m.insert("A4b 297", AbilityId::A2a069ShayminSkySupport);
        m.insert("A4b 298", AbilityId::A2a069ShayminSkySupport);
        m.insert("A4b 304", AbilityId::A3a062CelesteelaUltraThrusters);
        m.insert("A4b 305", AbilityId::A3a062CelesteelaUltraThrusters);
        m.insert("A4b 369", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A4b 370", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A4b 378", AbilityId::A2110DarkraiExNightmareAura);
        // B1
        m.insert("B1 121", AbilityId::B1121IndeedeeExWatchOver);
        m.insert("B1 157", AbilityId::B1157HydreigonRoarInUnison);
        m.insert("B1 160", AbilityId::B1160DragalgeExPoisonPoint);
        m.insert("B1 172", AbilityId::B1172AegislashCursedMetal);
        m.insert("B1 177", AbilityId::B1177GoomyStickyMembrane);
        m.insert("B1 184", AbilityId::B1184EeveeBoostedEvolution);
        m.insert("B1 245", AbilityId::B1157HydreigonRoarInUnison);
        m.insert("B1 247", AbilityId::B1177GoomyStickyMembrane);
        m.insert("B1 260", AbilityId::B1121IndeedeeExWatchOver);
        m.insert("B1 263", AbilityId::B1160DragalgeExPoisonPoint);
        m.insert("B1 278", AbilityId::B1121IndeedeeExWatchOver);
        m.insert("B1 281", AbilityId::B1160DragalgeExPoisonPoint);
        m.insert("B1 289", AbilityId::A1020VictreebelFragranceTrap);
        m.insert("B1 297", AbilityId::A1061PoliwrathCounterattack);
        // B1a
        m.insert("B1a 006", AbilityId::B1a006AriadosTrapTerritory);
        m.insert("B1a 012", AbilityId::B1a012CharmeleonIgnition);
        m.insert("B1a 018", AbilityId::B1a018WartortleShellShield);
        m.insert("B1a 034", AbilityId::B1a034ReuniclusInfiniteIncrease);
        m.insert("B1a 070", AbilityId::B1a006AriadosTrapTerritory);  // Rare variant
        m.insert("B1a 072", AbilityId::B1a034ReuniclusInfiniteIncrease);  // Rare variant
        m.insert("B1a 101", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("B1a 102", AbilityId::B1172AegislashCursedMetal);
        // P-A

        m.insert("P-A 037", AbilityId::PA037CresseliaExLunarPlumage);
        m.insert("P-A 042", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("P-A 054", AbilityId::A1061PoliwrathCounterattack);
        m.insert("P-A 104", AbilityId::A4a022MiloticHealingRipples);
        m.insert("P-A 109", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("P-A 110", AbilityId::A4a010EnteiExLegendaryPulse);
        // P-B
        m.insert("P-B 020", AbilityId::B1a012CharmeleonIgnition);
        m
    };
}

impl AbilityId {
    pub fn from_pokemon_id(pokemon_id: &str) -> Option<Self> {
        ABILITY_ID_MAP.get(&pokemon_id).copied()
    }
}
