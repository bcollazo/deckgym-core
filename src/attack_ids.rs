// TODO: Probably best to generate this file from database.json via card_enum_generator.rs.
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackId {
    A1003VenusaurMegaDrain,
    A1004VenusaurExGiantBloom,
    A1013VileplumeSoothingScent,
    A1017VenomothPoisonPowder,
    A1022ExeggutorStomp,
    A1023ExeggutorExTropicalSwing,
    A1024TangelaAbsorb,
    A1026PinsirDoubleHorn,
    A1029PetililBlot,
    A1030LilligantLeafSupply,
    A1031Skiddo,
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
    A1093FrosmothPowderSnow,
    A1095RaichuThunderbolt,
    A1096PikachuExCircleCircuit,
    A1101ElectabuzzThunderPunch,
    A1102JolteonPinMissile,
    A1104ZapdosExThunderingHurricane,
    A1106ZebstrikaThunderSpear,
    A1128MewtwoPowerBlast,
    A1129MewtwoExPsydrive,
    A1136GolurkDoubleLariat,
    A1149GolemDoubleEdge,
    A1153MarowakExBonemerang,
    A1154HitmonleeStretchKick,
    A1165ArbokCorner,
    A1171NidokingPoisonHorn,
    A1195WigglytuffSleepySong,
    A1196MeowthPayDay,
    A1201LickitungContinuousLick,
    A1203KangaskhanDizzyPunch,
    A1a026RaichuGigashock,
    A1a030DedenneThunderShock,
    A1a061EeveeContinuousSteps,
    A2049PalkiaDimensionalStorm,
    A2119DialgaExMetallicTurbo,
    A2a071ArceusExUltimateForce,
    // A2 Simple coin flip attacks
    A2016WormadamLeafCutter,
    A2019CarnivineFlop,
    A2030HeatRotomHeatBreath,
    A2039FloatzelJetScrew,
    A2126EeveeQuickAttack,
    // A2 Multiple coin flip attacks
    A2084GliscorAcrobatics,
    A2098SneaselDoubleScratch,
    A2106DrapionCrossPoison,
    A2118ProbopassTripleNose,
    A2131AmbipomDoubleHit,
    A2141ChatotFuryAttack,
    // A2 Variable coin flip attacks
    A2107CroagunkGroupBeatdown,
    A2108ToxicroakGroupBeatdown,
    // A2 Flip until tails attacks
    A2115WormadamIronHead,
    A2125LickilickyExLickingFury,
}

// Create a static HashMap for fast (pokemon, index) lookup
lazy_static::lazy_static! {
    static ref ATTACK_ID_MAP: HashMap<(&'static str, usize), AttackId> = {
        let mut m = HashMap::new();
        m.insert(("A1 003", 0), AttackId::A1003VenusaurMegaDrain);
        m.insert(("A1 004", 1), AttackId::A1004VenusaurExGiantBloom);
        m.insert(("A1 013", 0), AttackId::A1013VileplumeSoothingScent);
        m.insert(("A1 017", 0), AttackId::A1017VenomothPoisonPowder);
        m.insert(("A1 022", 0), AttackId::A1022ExeggutorStomp);
        m.insert(("A1 023", 0), AttackId::A1023ExeggutorExTropicalSwing);
        m.insert(("A1 024", 0), AttackId::A1024TangelaAbsorb);
        m.insert(("A1 026", 0), AttackId::A1026PinsirDoubleHorn);
        m.insert(("A1 029", 0), AttackId::A1029PetililBlot);
        m.insert(("A1 030", 0), AttackId::A1030LilligantLeafSupply);
        m.insert(("A1 031", 0), AttackId::A1031Skiddo);
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
        m.insert(("A1 233", 0), AttackId::A1078GyaradosHyperBeam); // Full art version
        m.insert(("A1 079", 0), AttackId::A1079LaprasHydroPump);
        m.insert(("A1 234", 0), AttackId::A1079LaprasHydroPump); // Full art version
        m.insert(("A1 080", 0), AttackId::A1080VaporeonBubbleDrain);
        m.insert(("A1 083", 0), AttackId::A1083ArticunoIceBeam);
        m.insert(("A1 093", 0), AttackId::A1093FrosmothPowderSnow);
        m.insert(("A1 095", 0), AttackId::A1095RaichuThunderbolt);
        m.insert(("A1 096", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A1 101", 0), AttackId::A1101ElectabuzzThunderPunch);
        m.insert(("A1 102", 0), AttackId::A1102JolteonPinMissile);
        m.insert(("A1 104", 1), AttackId::A1104ZapdosExThunderingHurricane);
        m.insert(("A1 106", 0), AttackId::A1106ZebstrikaThunderSpear);
        m.insert(("A1 128", 0), AttackId::A1128MewtwoPowerBlast);
        m.insert(("A1 129", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("A1 136", 0), AttackId::A1136GolurkDoubleLariat);
        m.insert(("A1 149", 0), AttackId::A1149GolemDoubleEdge);
        m.insert(("A1 153", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A1 154", 0), AttackId::A1154HitmonleeStretchKick);
        m.insert(("A1 165", 0), AttackId::A1165ArbokCorner);
        m.insert(("A1 171", 0), AttackId::A1171NidokingPoisonHorn);
        m.insert(("A1 195", 0), AttackId::A1195WigglytuffSleepySong);
        m.insert(("A1 196", 0), AttackId::A1196MeowthPayDay);
        m.insert(("A1 201", 0), AttackId::A1201LickitungContinuousLick);
        m.insert(("A1 203", 0), AttackId::A1203KangaskhanDizzyPunch);
        // Full Arts A1
        m.insert(("A1 229", 0), AttackId::A1026PinsirDoubleHorn);
        m.insert(("A1 230", 0), AttackId::A1033CharmanderEmber);
        m.insert(("A1 241", 0), AttackId::A1171NidokingPoisonHorn);
        m.insert(("A1 246", 0), AttackId::A1196MeowthPayDay);
        m.insert(("A1 251", 1), AttackId::A1004VenusaurExGiantBloom);
        m.insert(("A1 252", 0), AttackId::A1023ExeggutorExTropicalSwing);
        m.insert(("A1 253", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A1 254", 0), AttackId::A1041ArcanineExInfernoOnrush);
        m.insert(("A1 255", 0), AttackId::A1047MoltresExInfernoDance);
        m.insert(("A1 256", 1), AttackId::A1056BlastoiseExHydroBazooka);
        m.insert(("A1 259", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A1 260", 1), AttackId::A1104ZapdosExThunderingHurricane);
        m.insert(("A1 262", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("A1 264", 0), AttackId::A1153MarowakExBonemerang);
        m.insert(("A1 265", 0), AttackId::A1195WigglytuffSleepySong);
        m.insert(("A1 274", 0), AttackId::A1047MoltresExInfernoDance);
        m.insert(("A1 276", 1), AttackId::A1104ZapdosExThunderingHurricane);
        m.insert(("A1 279", 0), AttackId::A1195WigglytuffSleepySong);
        m.insert(("A1 280", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A1 281", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A1 282", 1), AttackId::A1129MewtwoExPsydrive);
        m.insert(("A1 284", 1), AttackId::A1036CharizardExCrimsonStorm);
        m.insert(("A1 285", 0), AttackId::A1096PikachuExCircleCircuit);
        m.insert(("A1 286", 1), AttackId::A1129MewtwoExPsydrive);
        // A1a
        m.insert(("A1a 026", 0), AttackId::A1a026RaichuGigashock);
        m.insert(("A1a 030", 0), AttackId::A1a030DedenneThunderShock);
        m.insert(("A1a 061", 0), AttackId::A1a061EeveeContinuousSteps);
        // Full Arts A1a
        m.insert(("A1a 073", 0), AttackId::A1a030DedenneThunderShock);

        // A2
        m.insert(("A2 049", 1), AttackId::A2049PalkiaDimensionalStorm);
        m.insert(("A2 182", 1), AttackId::A2049PalkiaDimensionalStorm);
        m.insert(("A2 204", 1), AttackId::A2049PalkiaDimensionalStorm);
        m.insert(("A2 206", 1), AttackId::A2049PalkiaDimensionalStorm);
        m.insert(("A2 119", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("A2 188", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("A2 205", 0), AttackId::A2119DialgaExMetallicTurbo);
        m.insert(("A2 207", 0), AttackId::A2119DialgaExMetallicTurbo);

        // A2 Simple coin flip attacks
        m.insert(("A2 016", 0), AttackId::A2016WormadamLeafCutter);
        m.insert(("A2 019", 0), AttackId::A2019CarnivineFlop);
        m.insert(("A2 030", 0), AttackId::A2030HeatRotomHeatBreath);
        m.insert(("A2 039", 0), AttackId::A2039FloatzelJetScrew);
        m.insert(("A2 126", 0), AttackId::A2126EeveeQuickAttack);
        m.insert(("A2 158", 0), AttackId::A2019CarnivineFlop); // Same attack as A2 019

        // A2 Multiple coin flip attacks
        m.insert(("A2 084", 0), AttackId::A2084GliscorAcrobatics);
        m.insert(("A2 098", 0), AttackId::A2098SneaselDoubleScratch);
        m.insert(("A2 106", 0), AttackId::A2106DrapionCrossPoison);
        m.insert(("A2 118", 0), AttackId::A2118ProbopassTripleNose);
        m.insert(("A2 131", 0), AttackId::A2131AmbipomDoubleHit);
        m.insert(("A2 141", 0), AttackId::A2141ChatotFuryAttack);

        // A2 Variable coin flip attacks
        m.insert(("A2 107", 0), AttackId::A2107CroagunkGroupBeatdown);
        m.insert(("A2 108", 0), AttackId::A2108ToxicroakGroupBeatdown);
        m.insert(("A2 173", 0), AttackId::A2107CroagunkGroupBeatdown); // Same attack as A2 107

        // A2 Flip until tails attacks
        m.insert(("A2 115", 0), AttackId::A2115WormadamIronHead);
        m.insert(("A2 125", 0), AttackId::A2125LickilickyExLickingFury);
        m.insert(("A2 189", 0), AttackId::A2125LickilickyExLickingFury); // Same attack as A2 125
        m.insert(("A2 203", 0), AttackId::A2125LickilickyExLickingFury); // Same attack as A2 125

        // A2a
        m.insert(("A2a 071", 0), AttackId::A2a071ArceusExUltimateForce);
        m.insert(("A2a 086", 0), AttackId::A2a071ArceusExUltimateForce);
        m.insert(("A2a 095", 0), AttackId::A2a071ArceusExUltimateForce);
        m.insert(("A2a 096", 0), AttackId::A2a071ArceusExUltimateForce);

        // Promo
        m.insert(("P-A 012", 0), AttackId::A1196MeowthPayDay);

        m
    };
}

impl AttackId {
    // None if not found or implemented
    pub fn from_pokemon_index(pokemon_id: &str, index: usize) -> Option<Self> {
        ATTACK_ID_MAP.get(&(pokemon_id, index)).copied()
    }
}
