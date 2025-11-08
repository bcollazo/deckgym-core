use std::collections::HashMap;

// TODO: Probably best to generate this file from database.json via card_enum_generator.rs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbilityId {
    A1020VictreebelFragranceTrap,
    A1089GreninjaWaterShuriken,
    A1098MagnetonVoltCharge,
    A1177Weezing,
    A1007Butterfree,
    A1132Gardevoir,
    A1a006SerperiorJungleTotem,
    A2a010LeafeonExForestBreath,
    A2a071Arceus,
    A2110DarkraiExNightmareAura,
    A2b035GiratinaExBrokenSpaceBellow,
    A3066OricoricSafeguard,
    A3122SolgaleoExRisingRoad,
    A3141KomalaComatose,
    A3a021ZeraoraThunderclapFlash,
    A3a027ShiinoticIlluminate,
    A3a062CelesteelaUltraThrusters,
    A3b009FlareonExCombust,
    A3b034SylveonExHappyRibbon,
    A3b056EeveeExVeeveeVolve,
    A4083EspeonExPsychicHealing,
    A4a010EnteiExLegendaryPulse,
    A4a020SuicuneExLegendaryPulse,
    A4a022MiloticHealingRipples,
    A4a025RaikouExLegendaryPulse,
}

// Create a static HashMap for fast (pokemon, index) lookup
lazy_static::lazy_static! {
    static ref ABILITY_ID_MAP: HashMap<&'static str, AbilityId> = {
        let mut m = HashMap::new();
        m.insert("A1 007", AbilityId::A1007Butterfree);
        m.insert("A1 020", AbilityId::A1020VictreebelFragranceTrap);
        m.insert("A1 089", AbilityId::A1089GreninjaWaterShuriken);
        m.insert("A1 098", AbilityId::A1098MagnetonVoltCharge);
        m.insert("A1 177", AbilityId::A1177Weezing);
        m.insert("A1 132", AbilityId::A1132Gardevoir);
        m.insert("A1a 006", AbilityId::A1a006SerperiorJungleTotem);
        m.insert("A1a 070", AbilityId::A1a006SerperiorJungleTotem);
        m.insert("A2a 010", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 082", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 091", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A2a 071", AbilityId::A2a071Arceus);
        m.insert("A2a 086", AbilityId::A2a071Arceus);
        m.insert("A2a 095", AbilityId::A2a071Arceus);
        m.insert("A2a 096", AbilityId::A2a071Arceus);
        m.insert("A2 110", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("A2 187", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("A2 202", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("A2b 035", AbilityId::A2b035GiratinaExBrokenSpaceBellow);
        m.insert("A2b 083", AbilityId::A2b035GiratinaExBrokenSpaceBellow);
        m.insert("A2b 096", AbilityId::A2b035GiratinaExBrokenSpaceBellow);
        m.insert("A3 066", AbilityId::A3066OricoricSafeguard);
        m.insert("A3 122", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 141", AbilityId::A3141KomalaComatose);
        m.insert("A3 165", AbilityId::A3066OricoricSafeguard);
        m.insert("A3 179", AbilityId::A3141KomalaComatose);
        m.insert("A3 189", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 207", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3 239", AbilityId::A3122SolgaleoExRisingRoad);
        m.insert("A3a 021", AbilityId::A3a021ZeraoraThunderclapFlash);
        m.insert("A3a 027", AbilityId::A3a027ShiinoticIlluminate);
        m.insert("A3a 062", AbilityId::A3a062CelesteelaUltraThrusters);
        m.insert("A3a 075", AbilityId::A3a062CelesteelaUltraThrusters);
        m.insert("A3a 093", AbilityId::A1089GreninjaWaterShuriken);
        m.insert("A3b 009", AbilityId::A3b009FlareonExCombust);
        m.insert("A3b 034", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A3b 056", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A3b 079", AbilityId::A3b009FlareonExCombust);
        m.insert("A3b 081", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A3b 083", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A3b 087", AbilityId::A3b009FlareonExCombust);
        m.insert("A3b 089", AbilityId::A3b034SylveonExHappyRibbon);
        m.insert("A3b 092", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A4 083", AbilityId::A4083EspeonExPsychicHealing);
        m.insert("A4 190", AbilityId::A4083EspeonExPsychicHealing);
        m.insert("A4 205", AbilityId::A4083EspeonExPsychicHealing);
        m.insert("A4 218", AbilityId::A1098MagnetonVoltCharge);
        m.insert("A4 233", AbilityId::A2a010LeafeonExForestBreath);
        m.insert("A4a 010", AbilityId::A4a010EnteiExLegendaryPulse);
        m.insert("A4a 020", AbilityId::A4a020SuicuneExLegendaryPulse);
        m.insert("A4a 022", AbilityId::A4a022MiloticHealingRipples);
        m.insert("A4a 025", AbilityId::A4a025RaikouExLegendaryPulse);
        m.insert("A4a 072", AbilityId::A4a022MiloticHealingRipples);
        m.insert("A4a 079", AbilityId::A4a010EnteiExLegendaryPulse);
        m.insert("A4a 080", AbilityId::A4a020SuicuneExLegendaryPulse);
        m.insert("A4a 081", AbilityId::A4a025RaikouExLegendaryPulse);
        m.insert("A4a 087", AbilityId::A4a010EnteiExLegendaryPulse);
        m.insert("A4a 088", AbilityId::A4a025RaikouExLegendaryPulse);
        m.insert("A4a 090", AbilityId::A4a020SuicuneExLegendaryPulse);
        m.insert("A4b 066", AbilityId::A3b009FlareonExCombust);
        m.insert("A4b 135", AbilityId::A1098MagnetonVoltCharge);
        m.insert("A4b 136", AbilityId::A1098MagnetonVoltCharge);
        m.insert("A4b 146", AbilityId::A3066OricoricSafeguard);
        m.insert("A4b 147", AbilityId::A3066OricoricSafeguard);
        m.insert("A4b 149", AbilityId::A3a021ZeraoraThunderclapFlash);
        m.insert("A4b 150", AbilityId::A3a021ZeraoraThunderclapFlash);
        m.insert("A4b 160", AbilityId::A4083EspeonExPsychicHealing);
        m.insert("A4b 245", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("A4b 287", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A4b 304", AbilityId::A3a062CelesteelaUltraThrusters);
        m.insert("A4b 305", AbilityId::A3a062CelesteelaUltraThrusters);
        m.insert("A4b 370", AbilityId::A3b056EeveeExVeeveeVolve);
        m.insert("A4b 378", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("P-A 042", AbilityId::A2110DarkraiExNightmareAura);
        m.insert("P-A 110", AbilityId::A4a010EnteiExLegendaryPulse);
        m.insert("P-A 019", AbilityId::A1089GreninjaWaterShuriken);
        m.insert("P-A 104", AbilityId::A4a022MiloticHealingRipples);
        m
    };
}

impl AbilityId {
    pub fn from_pokemon_id(pokemon_id: &str) -> Option<Self> {
        ABILITY_ID_MAP.get(&pokemon_id).copied()
    }
}
