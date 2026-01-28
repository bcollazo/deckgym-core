// This is code generated from the database.json by card_enum_generator.rs. Do not edit manually.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum CardId {
    A1001Bulbasaur,
    A1002Ivysaur,
    A1003Venusaur,
    A1004VenusaurEx,
    A1005Caterpie,
    A1006Metapod,
    A1007Butterfree,
    A1008Weedle,
    A1009Kakuna,
    A1010Beedrill,
    A1011Oddish,
    A1012Gloom,
    A1013Vileplume,
    A1014Paras,
    A1015Parasect,
    A1016Venonat,
    A1017Venomoth,
    A1018Bellsprout,
    A1019Weepinbell,
    A1020Victreebel,
    A1021Exeggcute,
    A1022Exeggutor,
    A1023ExeggutorEx,
    A1024Tangela,
    A1025Scyther,
    A1026Pinsir,
    A1027Cottonee,
    A1028Whimsicott,
    A1029Petilil,
    A1030Lilligant,
    A1031Skiddo,
    A1032Gogoat,
    A1033Charmander,
    A1034Charmeleon,
    A1035Charizard,
    A1036CharizardEx,
    A1037Vulpix,
    A1038Ninetales,
    A1039Growlithe,
    A1040Arcanine,
    A1041ArcanineEx,
    A1042Ponyta,
    A1043Rapidash,
    A1044Magmar,
    A1045Flareon,
    A1046Moltres,
    A1047MoltresEx,
    A1048Heatmor,
    A1049Salandit,
    A1050Salazzle,
    A1051Sizzlipede,
    A1052Centiskorch,
    A1053Squirtle,
    A1054Wartortle,
    A1055Blastoise,
    A1056BlastoiseEx,
    A1057Psyduck,
    A1058Golduck,
    A1059Poliwag,
    A1060Poliwhirl,
    A1061Poliwrath,
    A1062Tentacool,
    A1063Tentacruel,
    A1064Seel,
    A1065Dewgong,
    A1066Shellder,
    A1067Cloyster,
    A1068Krabby,
    A1069Kingler,
    A1070Horsea,
    A1071Seadra,
    A1072Goldeen,
    A1073Seaking,
    A1074Staryu,
    A1075Starmie,
    A1076StarmieEx,
    A1077Magikarp,
    A1078Gyarados,
    A1079Lapras,
    A1080Vaporeon,
    A1081Omanyte,
    A1082Omastar,
    A1083Articuno,
    A1084ArticunoEx,
    A1085Ducklett,
    A1086Swanna,
    A1087Froakie,
    A1088Frogadier,
    A1089Greninja,
    A1090Pyukumuku,
    A1091Bruxish,
    A1092Snom,
    A1093Frosmoth,
    A1094Pikachu,
    A1095Raichu,
    A1096PikachuEx,
    A1097Magnemite,
    A1098Magneton,
    A1099Voltorb,
    A1100Electrode,
    A1101Electabuzz,
    A1102Jolteon,
    A1103Zapdos,
    A1104ZapdosEx,
    A1105Blitzle,
    A1106Zebstrika,
    A1107Tynamo,
    A1108Eelektrik,
    A1109Eelektross,
    A1110Helioptile,
    A1111Heliolisk,
    A1112Pincurchin,
    A1113Clefairy,
    A1114Clefable,
    A1115Abra,
    A1116Kadabra,
    A1117Alakazam,
    A1118Slowpoke,
    A1119Slowbro,
    A1120Gastly,
    A1121Haunter,
    A1122Gengar,
    A1123GengarEx,
    A1124Drowzee,
    A1125Hypno,
    A1126MrMime,
    A1127Jynx,
    A1128Mewtwo,
    A1129MewtwoEx,
    A1130Ralts,
    A1131Kirlia,
    A1132Gardevoir,
    A1133Woobat,
    A1134Swoobat,
    A1135Golett,
    A1136Golurk,
    A1137Sandshrew,
    A1138Sandslash,
    A1139Diglett,
    A1140Dugtrio,
    A1141Mankey,
    A1142Primeape,
    A1143Machop,
    A1144Machoke,
    A1145Machamp,
    A1146MachampEx,
    A1147Geodude,
    A1148Graveler,
    A1149Golem,
    A1150Onix,
    A1151Cubone,
    A1152Marowak,
    A1153MarowakEx,
    A1154Hitmonlee,
    A1155Hitmonchan,
    A1156Rhyhorn,
    A1157Rhydon,
    A1158Kabuto,
    A1159Kabutops,
    A1160Mienfoo,
    A1161Mienshao,
    A1162Clobbopus,
    A1163Grapploct,
    A1164Ekans,
    A1165Arbok,
    A1166NidoranF,
    A1167Nidorina,
    A1168Nidoqueen,
    A1169NidoranM,
    A1170Nidorino,
    A1171Nidoking,
    A1172Zubat,
    A1173Golbat,
    A1174Grimer,
    A1175Muk,
    A1176Koffing,
    A1177Weezing,
    A1178Mawile,
    A1179Pawniard,
    A1180Bisharp,
    A1181Meltan,
    A1182Melmetal,
    A1183Dratini,
    A1184Dragonair,
    A1185Dragonite,
    A1186Pidgey,
    A1187Pidgeotto,
    A1188Pidgeot,
    A1189Rattata,
    A1190Raticate,
    A1191Spearow,
    A1192Fearow,
    A1193Jigglypuff,
    A1194Wigglytuff,
    A1195WigglytuffEx,
    A1196Meowth,
    A1197Persian,
    A1198Farfetchd,
    A1199Doduo,
    A1200Dodrio,
    A1201Lickitung,
    A1202Chansey,
    A1203Kangaskhan,
    A1204Tauros,
    A1205Ditto,
    A1206Eevee,
    A1207Eevee,
    A1208Eevee,
    A1209Porygon,
    A1210Aerodactyl,
    A1211Snorlax,
    A1212Minccino,
    A1213Cinccino,
    A1214Wooloo,
    A1215Dubwool,
    A1216HelixFossil,
    A1217DomeFossil,
    A1218OldAmber,
    A1219Erika,
    A1220Misty,
    A1221Blaine,
    A1222Koga,
    A1223Giovanni,
    A1224Brock,
    A1225Sabrina,
    A1226LtSurge,
    A1227Bulbasaur,
    A1228Gloom,
    A1229Pinsir,
    A1230Charmander,
    A1231Rapidash,
    A1232Squirtle,
    A1233Gyarados,
    A1234Lapras,
    A1235Electrode,
    A1236Alakazam,
    A1237Slowpoke,
    A1238Diglett,
    A1239Cubone,
    A1240Nidoqueen,
    A1241Nidoking,
    A1242Golbat,
    A1243Weezing,
    A1244Dragonite,
    A1245Pidgeot,
    A1246Meowth,
    A1247Ditto,
    A1248Eevee,
    A1249Porygon,
    A1250Snorlax,
    A1251VenusaurEx,
    A1252ExeggutorEx,
    A1253CharizardEx,
    A1254ArcanineEx,
    A1255MoltresEx,
    A1256BlastoiseEx,
    A1257StarmieEx,
    A1258ArticunoEx,
    A1259PikachuEx,
    A1260ZapdosEx,
    A1261GengarEx,
    A1262MewtwoEx,
    A1263MachampEx,
    A1264MarowakEx,
    A1265WigglytuffEx,
    A1266Erika,
    A1267Misty,
    A1268Blaine,
    A1269Koga,
    A1270Giovanni,
    A1271Brock,
    A1272Sabrina,
    A1273LtSurge,
    A1274MoltresEx,
    A1275ArticunoEx,
    A1276ZapdosEx,
    A1277GengarEx,
    A1278MachampEx,
    A1279WigglytuffEx,
    A1280CharizardEx,
    A1281PikachuEx,
    A1282MewtwoEx,
    A1283Mew,
    A1284CharizardEx,
    A1285PikachuEx,
    A1286MewtwoEx,
    A1a001Exeggcute,
    A1a002Exeggutor,
    A1a003CelebiEx,
    A1a004Snivy,
    A1a005Servine,
    A1a006Serperior,
    A1a007Morelull,
    A1a008Shiinotic,
    A1a009Dhelmise,
    A1a010Ponyta,
    A1a011Rapidash,
    A1a012Magmar,
    A1a013Larvesta,
    A1a014Volcarona,
    A1a015Salandit,
    A1a016Salazzle,
    A1a017Magikarp,
    A1a018GyaradosEx,
    A1a019Vaporeon,
    A1a020Finneon,
    A1a021Lumineon,
    A1a022Chewtle,
    A1a023Drednaw,
    A1a024Cramorant,
    A1a025Pikachu,
    A1a026Raichu,
    A1a027Electabuzz,
    A1a028Joltik,
    A1a029Galvantula,
    A1a030Dedenne,
    A1a031Mew,
    A1a032MewEx,
    A1a033Sigilyph,
    A1a034Elgyem,
    A1a035Beheeyem,
    A1a036Flabebe,
    A1a037Floette,
    A1a038Florges,
    A1a039Swirlix,
    A1a040Slurpuff,
    A1a041Mankey,
    A1a042Primeape,
    A1a043Geodude,
    A1a044Graveler,
    A1a045Golem,
    A1a046AerodactylEx,
    A1a047Marshadow,
    A1a048Stonjourner,
    A1a049Koffing,
    A1a050Weezing,
    A1a051Purrloin,
    A1a052Liepard,
    A1a053Venipede,
    A1a054Whirlipede,
    A1a055Scolipede,
    A1a056Druddigon,
    A1a057Pidgey,
    A1a058Pidgeotto,
    A1a059PidgeotEx,
    A1a060Tauros,
    A1a061Eevee,
    A1a062Chatot,
    A1a063OldAmber,
    A1a064PokemonFlute,
    A1a065MythicalSlab,
    A1a066BuddingExpeditioner,
    A1a067Blue,
    A1a068Leaf,
    A1a069Exeggutor,
    A1a070Serperior,
    A1a071Salandit,
    A1a072Vaporeon,
    A1a073Dedenne,
    A1a074Marshadow,
    A1a075CelebiEx,
    A1a076GyaradosEx,
    A1a077MewEx,
    A1a078AerodactylEx,
    A1a079PidgeotEx,
    A1a080BuddingExpeditioner,
    A1a081Blue,
    A1a082Leaf,
    A1a083MewEx,
    A1a084AerodactylEx,
    A1a085CelebiEx,
    A1a086MewEx,
    A2001Oddish,
    A2002Gloom,
    A2003Bellossom,
    A2004Tangela,
    A2005Tangrowth,
    A2006Yanma,
    A2007YanmegaEx,
    A2008Roselia,
    A2009Roserade,
    A2010Turtwig,
    A2011Grotle,
    A2012Torterra,
    A2013Kricketot,
    A2014Kricketune,
    A2015Burmy,
    A2016Wormadam,
    A2017Combee,
    A2018Vespiquen,
    A2019Carnivine,
    A2020Leafeon,
    A2021MowRotom,
    A2022Shaymin,
    A2023Magmar,
    A2024Magmortar,
    A2025Slugma,
    A2026Magcargo,
    A2027Chimchar,
    A2028Monferno,
    A2029InfernapeEx,
    A2030HeatRotom,
    A2031Swinub,
    A2032Piloswine,
    A2033Mamoswine,
    A2034Regice,
    A2035Piplup,
    A2036Prinplup,
    A2037Empoleon,
    A2038Buizel,
    A2039Floatzel,
    A2040Shellos,
    A2041Gastrodon,
    A2042Finneon,
    A2043Lumineon,
    A2044Snover,
    A2045Abomasnow,
    A2046Glaceon,
    A2047WashRotom,
    A2048FrostRotom,
    A2049PalkiaEx,
    A2050Manaphy,
    A2051Magnemite,
    A2052Magneton,
    A2053Magnezone,
    A2054Voltorb,
    A2055Electrode,
    A2056Electabuzz,
    A2057Electivire,
    A2058Shinx,
    A2059Luxio,
    A2060Luxray,
    A2061PachirisuEx,
    A2062Rotom,
    A2063Togepi,
    A2064Togetic,
    A2065Togekiss,
    A2066Misdreavus,
    A2067MismagiusEx,
    A2068Ralts,
    A2069Kirlia,
    A2070Duskull,
    A2071Dusclops,
    A2072Dusknoir,
    A2073Drifloon,
    A2074Drifblim,
    A2075Uxie,
    A2076Mesprit,
    A2077Azelf,
    A2078Giratina,
    A2079Cresselia,
    A2080Rhyhorn,
    A2081Rhydon,
    A2082Rhyperior,
    A2083Gligar,
    A2084Gliscor,
    A2085Hitmontop,
    A2086Nosepass,
    A2087Regirock,
    A2088Cranidos,
    A2089Rampardos,
    A2090Wormadam,
    A2091Riolu,
    A2092Lucario,
    A2093Hippopotas,
    A2094Hippowdon,
    A2095GalladeEx,
    A2096Murkrow,
    A2097Honchkrow,
    A2098Sneasel,
    A2099WeavileEx,
    A2100Poochyena,
    A2101Mightyena,
    A2102Stunky,
    A2103Skuntank,
    A2104Spiritomb,
    A2105Skorupi,
    A2106Drapion,
    A2107Croagunk,
    A2108Toxicroak,
    A2109Darkrai,
    A2110DarkraiEx,
    A2111Skarmory,
    A2112Registeel,
    A2113Shieldon,
    A2114Bastiodon,
    A2115Wormadam,
    A2116Bronzor,
    A2117Bronzong,
    A2118Probopass,
    A2119DialgaEx,
    A2120Heatran,
    A2121Gible,
    A2122Gabite,
    A2123Garchomp,
    A2124Lickitung,
    A2125LickilickyEx,
    A2126Eevee,
    A2127Porygon,
    A2128Porygon2,
    A2129PorygonZ,
    A2130Aipom,
    A2131Ambipom,
    A2132Starly,
    A2133Staravia,
    A2134Staraptor,
    A2135Bidoof,
    A2136Bibarel,
    A2137Buneary,
    A2138Lopunny,
    A2139Glameow,
    A2140Purugly,
    A2141Chatot,
    A2142FanRotom,
    A2143Regigigas,
    A2144SkullFossil,
    A2145ArmorFossil,
    A2146PokemonCommunication,
    A2147GiantCape,
    A2148RockyHelmet,
    A2149LumBerry,
    A2150Cyrus,
    A2151TeamGalacticGrunt,
    A2152Cynthia,
    A2153Volkner,
    A2154Dawn,
    A2155Mars,
    A2156Tangrowth,
    A2157Combee,
    A2158Carnivine,
    A2159Shaymin,
    A2160Mamoswine,
    A2161Gastrodon,
    A2162Manaphy,
    A2163Shinx,
    A2164Rotom,
    A2165Drifloon,
    A2166Mesprit,
    A2167Giratina,
    A2168Cresselia,
    A2169Rhyperior,
    A2170Lucario,
    A2171Hippopotas,
    A2172Spiritomb,
    A2173Croagunk,
    A2174Heatran,
    A2175Garchomp,
    A2176Staraptor,
    A2177Bidoof,
    A2178Glameow,
    A2179Regigigas,
    A2180YanmegaEx,
    A2181InfernapeEx,
    A2182PalkiaEx,
    A2183PachirisuEx,
    A2184MismagiusEx,
    A2185GalladeEx,
    A2186WeavileEx,
    A2187DarkraiEx,
    A2188DialgaEx,
    A2189LickilickyEx,
    A2190Cyrus,
    A2191TeamGalacticGrunt,
    A2192Cynthia,
    A2193Volkner,
    A2194Dawn,
    A2195Mars,
    A2196YanmegaEx,
    A2197InfernapeEx,
    A2198PachirisuEx,
    A2199MismagiusEx,
    A2200GalladeEx,
    A2201WeavileEx,
    A2202DarkraiEx,
    A2203LickilickyEx,
    A2204PalkiaEx,
    A2205DialgaEx,
    A2206PalkiaEx,
    A2207DialgaEx,
    A2a001Heracross,
    A2a002Burmy,
    A2a003Mothim,
    A2a004Combee,
    A2a005Vespiquen,
    A2a006Cherubi,
    A2a007Cherrim,
    A2a008Cherrim,
    A2a009Carnivine,
    A2a010LeafeonEx,
    A2a011Houndour,
    A2a012Houndoom,
    A2a013Heatran,
    A2a014Marill,
    A2a015Azumarill,
    A2a016Barboach,
    A2a017Whiscash,
    A2a018Snorunt,
    A2a019Froslass,
    A2a020Snover,
    A2a021Abomasnow,
    A2a022GlaceonEx,
    A2a023OriginFormePalkia,
    A2a024Phione,
    A2a025Pikachu,
    A2a026Raichu,
    A2a027Electrike,
    A2a028Manectric,
    A2a029Clefairy,
    A2a030Clefable,
    A2a031Gastly,
    A2a032Haunter,
    A2a033Gengar,
    A2a034Unown,
    A2a035Rotom,
    A2a036Sudowoodo,
    A2a037Phanpy,
    A2a038Donphan,
    A2a039Larvitar,
    A2a040Pupitar,
    A2a041Tyranitar,
    A2a042Nosepass,
    A2a043Meditite,
    A2a044Medicham,
    A2a045Gible,
    A2a046Gabite,
    A2a047GarchompEx,
    A2a048Zubat,
    A2a049Golbat,
    A2a050Crobat,
    A2a051Croagunk,
    A2a052Toxicroak,
    A2a053Magnemite,
    A2a054Magneton,
    A2a055Magnezone,
    A2a056Mawile,
    A2a057ProbopassEx,
    A2a058Bronzor,
    A2a059Bronzong,
    A2a060OriginFormeDialga,
    A2a061Giratina,
    A2a062Eevee,
    A2a063Snorlax,
    A2a064Hoothoot,
    A2a065Noctowl,
    A2a066Starly,
    A2a067Staravia,
    A2a068Staraptor,
    A2a069Shaymin,
    A2a070Arceus,
    A2a071ArceusEx,
    A2a072Irida,
    A2a073CelesticTownElder,
    A2a074Barry,
    A2a075Adaman,
    A2a076Houndoom,
    A2a077Marill,
    A2a078Unown,
    A2a079Sudowoodo,
    A2a080Magnemite,
    A2a081Shaymin,
    A2a082LeafeonEx,
    A2a083GlaceonEx,
    A2a084GarchompEx,
    A2a085ProbopassEx,
    A2a086ArceusEx,
    A2a087Irida,
    A2a088CelesticTownElder,
    A2a089Barry,
    A2a090Adaman,
    A2a091LeafeonEx,
    A2a092GlaceonEx,
    A2a093GarchompEx,
    A2a094ProbopassEx,
    A2a095ArceusEx,
    A2a096ArceusEx,
    A2b001Weedle,
    A2b002Kakuna,
    A2b003BeedrillEx,
    A2b004Pinsir,
    A2b005Sprigatito,
    A2b006Floragato,
    A2b007Meowscarada,
    A2b008Charmander,
    A2b009Charmeleon,
    A2b010CharizardEx,
    A2b011Magmar,
    A2b012Magmortar,
    A2b013PaldeanTauros,
    A2b014Tentacool,
    A2b015Tentacruel,
    A2b016Buizel,
    A2b017Floatzel,
    A2b018Wiglett,
    A2b019WugtrioEx,
    A2b020Dondozo,
    A2b021Tatsugiri,
    A2b022PikachuEx,
    A2b023Voltorb,
    A2b024Electrode,
    A2b025Pachirisu,
    A2b026Pawmi,
    A2b027Pawmo,
    A2b028Pawmot,
    A2b029Abra,
    A2b030Kadabra,
    A2b031Alakazam,
    A2b032MrMime,
    A2b033Drifloon,
    A2b034Drifblim,
    A2b035GiratinaEx,
    A2b036Gimmighoul,
    A2b037Machop,
    A2b038Machoke,
    A2b039Machamp,
    A2b040Hitmonlee,
    A2b041Hitmonchan,
    A2b042Riolu,
    A2b043LucarioEx,
    A2b044Flamigo,
    A2b045Ekans,
    A2b046Arbok,
    A2b047PaldeanWooper,
    A2b048PaldeanClodsireEx,
    A2b049Spiritomb,
    A2b050Shroodle,
    A2b051Grafaiai,
    A2b052Tinkatink,
    A2b053Tinkatuff,
    A2b054TinkatonEx,
    A2b055Varoom,
    A2b056Revavroom,
    A2b057Gholdengo,
    A2b058Rattata,
    A2b059Raticate,
    A2b060Jigglypuff,
    A2b061Wigglytuff,
    A2b062Lickitung,
    A2b063Lickilicky,
    A2b064Bidoof,
    A2b065BibarelEx,
    A2b066Buneary,
    A2b067Lopunny,
    A2b068Cyclizar,
    A2b069Iono,
    A2b070PokemonCenterLady,
    A2b071Red,
    A2b072TeamRocketGrunt,
    A2b073Meowscarada,
    A2b074Buizel,
    A2b075Tatsugiri,
    A2b076Grafaiai,
    A2b077Gholdengo,
    A2b078Wigglytuff,
    A2b079BeedrillEx,
    A2b080CharizardEx,
    A2b081WugtrioEx,
    A2b082PikachuEx,
    A2b083GiratinaEx,
    A2b084LucarioEx,
    A2b085PaldeanClodsireEx,
    A2b086TinkatonEx,
    A2b087BibarelEx,
    A2b088Iono,
    A2b089PokemonCenterLady,
    A2b090Red,
    A2b091TeamRocketGrunt,
    A2b092PikachuEx,
    A2b093PaldeanClodsireEx,
    A2b094TinkatonEx,
    A2b095BibarelEx,
    A2b096GiratinaEx,
    A2b097Weedle,
    A2b098Kakuna,
    A2b099Charmander,
    A2b100Charmeleon,
    A2b101Wiglett,
    A2b102Dondozo,
    A2b103Pachirisu,
    A2b104Riolu,
    A2b105Varoom,
    A2b106Revavroom,
    A2b107BeedrillEx,
    A2b108CharizardEx,
    A2b109WugtrioEx,
    A2b110LucarioEx,
    A2b111PokeBall,
    A3001Exeggcute,
    A3002AlolanExeggutor,
    A3003Surskit,
    A3004Masquerain,
    A3005Maractus,
    A3006Karrablast,
    A3007Phantump,
    A3008Trevenant,
    A3009Rowlet,
    A3010Rowlet,
    A3011Dartrix,
    A3012DecidueyeEx,
    A3013Grubbin,
    A3014Fomantis,
    A3015Lurantis,
    A3016Morelull,
    A3017Shiinotic,
    A3018Bounsweet,
    A3019Steenee,
    A3020Tsareena,
    A3021Wimpod,
    A3022Golisopod,
    A3023DhelmiseEx,
    A3024TapuBulu,
    A3025Growlithe,
    A3026Arcanine,
    A3027AlolanMarowak,
    A3028Fletchinder,
    A3029Talonflame,
    A3030Litten,
    A3031Litten,
    A3032Torracat,
    A3033IncineroarEx,
    A3034Oricorio,
    A3035Salandit,
    A3036Salazzle,
    A3037Turtonator,
    A3038AlolanSandshrew,
    A3039AlolanSandslash,
    A3040AlolanVulpix,
    A3041AlolanNinetales,
    A3042Shellder,
    A3043Cloyster,
    A3044Lapras,
    A3045Popplio,
    A3046Popplio,
    A3047Brionne,
    A3048Primarina,
    A3049CrabominableEx,
    A3050Wishiwashi,
    A3051WishiwashiEx,
    A3052Dewpider,
    A3053Araquanid,
    A3054Pyukumuku,
    A3055Bruxish,
    A3056TapuFini,
    A3057Pikachu,
    A3058AlolanRaichuEx,
    A3059AlolanGeodude,
    A3060AlolanGraveler,
    A3061AlolanGolem,
    A3062Helioptile,
    A3063Heliolisk,
    A3064Charjabug,
    A3065Vikavolt,
    A3066Oricorio,
    A3067Togedemaru,
    A3068TapuKoko,
    A3069MrMime,
    A3070Sableye,
    A3071Spoink,
    A3072Grumpig,
    A3073Lunatone,
    A3074Shuppet,
    A3075Banette,
    A3076Oricorio,
    A3077Oricorio,
    A3078Cutiefly,
    A3079Ribombee,
    A3080Comfey,
    A3081Sandygast,
    A3082Palossand,
    A3083Mimikyu,
    A3084TapuLele,
    A3085Cosmog,
    A3086Cosmoem,
    A3087LunalaEx,
    A3088Necrozma,
    A3089Cubone,
    A3090Makuhita,
    A3091Hariyama,
    A3092Solrock,
    A3093Drilbur,
    A3094Timburr,
    A3095Gurdurr,
    A3096Conkeldurr,
    A3097Crabrawler,
    A3098Rockruff,
    A3099Rockruff,
    A3100Lycanroc,
    A3101Lycanroc,
    A3102Mudbray,
    A3103Mudsdale,
    A3104PassimianEx,
    A3105Minior,
    A3106AlolanRattata,
    A3107AlolanRaticate,
    A3108AlolanMeowth,
    A3109AlolanPersian,
    A3110AlolanGrimer,
    A3111AlolanMukEx,
    A3112Absol,
    A3113Trubbish,
    A3114Garbodor,
    A3115Mareanie,
    A3116ToxapEx,
    A3117AlolanDiglett,
    A3118AlolanDugtrio,
    A3119Excadrill,
    A3120Escavalier,
    A3121Klefki,
    A3122SolgaleoEx,
    A3123Magearna,
    A3124Drampa,
    A3125Jangmoo,
    A3126Hakamoo,
    A3127Kommoo,
    A3128Tauros,
    A3129Skitty,
    A3130Delcatty,
    A3131Fletchling,
    A3132Hawlucha,
    A3133Pikipek,
    A3134Trumbeak,
    A3135Toucannon,
    A3136Yungoos,
    A3137Gumshoos,
    A3138Stufful,
    A3139Bewear,
    A3140Oranguru,
    A3141Komala,
    A3142BigMalasada,
    A3143FishingNet,
    A3144RareCandy,
    A3145RotomDEx,
    A3146PoisonBarb,
    A3147LeafCape,
    A3148Acerola,
    A3149Ilima,
    A3150Kiawe,
    A3151Guzma,
    A3152Lana,
    A3153Sophocles,
    A3154Mallow,
    A3155Lillie,
    A3156AlolanExeggutor,
    A3157Morelull,
    A3158Tsareena,
    A3159TapuBulu,
    A3160AlolanMarowak,
    A3161Turtonator,
    A3162AlolanVulpix,
    A3163Pyukumuku,
    A3164TapuFini,
    A3165Oricorio,
    A3166TapuKoko,
    A3167Cutiefly,
    A3168Comfey,
    A3169Sandygast,
    A3170TapuLele,
    A3171Cosmog,
    A3172Rockruff,
    A3173Mudsdale,
    A3174Minior,
    A3175Magearna,
    A3176Drampa,
    A3177Pikipek,
    A3178Bewear,
    A3179Komala,
    A3180DecidueyeEx,
    A3181DhelmiseEx,
    A3182IncineroarEx,
    A3183CrabominableEx,
    A3184WishiwashiEx,
    A3185AlolanRaichuEx,
    A3186LunalaEx,
    A3187PassimianEx,
    A3188AlolanMukEx,
    A3189SolgaleoEx,
    A3190Acerola,
    A3191Ilima,
    A3192Kiawe,
    A3193Guzma,
    A3194Lana,
    A3195Sophocles,
    A3196Mallow,
    A3197Lillie,
    A3198DecidueyeEx,
    A3199DhelmiseEx,
    A3200IncineroarEx,
    A3201CrabominableEx,
    A3202WishiwashiEx,
    A3203AlolanRaichuEx,
    A3204LunalaEx,
    A3205PassimianEx,
    A3206AlolanMukEx,
    A3207SolgaleoEx,
    A3208Guzma,
    A3209Lillie,
    A3210Bulbasaur,
    A3211Ivysaur,
    A3212Venusaur,
    A3213Exeggcute,
    A3214Exeggutor,
    A3215Squirtle,
    A3216Wartortle,
    A3217Blastoise,
    A3218Staryu,
    A3219Starmie,
    A3220Gastly,
    A3221Haunter,
    A3222Gengar,
    A3223Machop,
    A3224Machoke,
    A3225Machamp,
    A3226Cubone,
    A3227Marowak,
    A3228Jigglypuff,
    A3229Wigglytuff,
    A3230VenusaurEx,
    A3231ExeggutorEx,
    A3232BlastoiseEx,
    A3233StarmieEx,
    A3234GengarEx,
    A3235MachampEx,
    A3236MarowakEx,
    A3237WigglytuffEx,
    A3238LunalaEx,
    A3239SolgaleoEx,
    A3a001Petilil,
    A3a002Lilligant,
    A3a003Rowlet,
    A3a004Dartrix,
    A3a005Decidueye,
    A3a006BuzzwoleEx,
    A3a007Pheromosa,
    A3a008Kartana,
    A3a009Blacephalon,
    A3a010Mantine,
    A3a011Carvanha,
    A3a012Sharpedo,
    A3a013Shinx,
    A3a014Luxio,
    A3a015Luxray,
    A3a016Blitzle,
    A3a017Zebstrika,
    A3a018Emolga,
    A3a019TapuKokoEx,
    A3a020Xurkitree,
    A3a021Zeraora,
    A3a022Clefairy,
    A3a023Clefable,
    A3a024Phantump,
    A3a025Trevenant,
    A3a026Morelull,
    A3a027Shiinotic,
    A3a028Meditite,
    A3a029Medicham,
    A3a030Baltoy,
    A3a031Claydol,
    A3a032Rockruff,
    A3a033LycanrocEx,
    A3a034Passimian,
    A3a035Sandygast,
    A3a036Palossand,
    A3a037AlolanMeowth,
    A3a038AlolanPersian,
    A3a039Sandile,
    A3a040Krokorok,
    A3a041Krookodile,
    A3a042Nihilego,
    A3a043GuzzlordEx,
    A3a044Poipole,
    A3a045Naganadel,
    A3a046AlolanDiglett,
    A3a047AlolanDugtrioEx,
    A3a048Aron,
    A3a049Lairon,
    A3a050Aggron,
    A3a051Ferroseed,
    A3a052Ferrothorn,
    A3a053Stakataka,
    A3a054Lillipup,
    A3a055Herdier,
    A3a056Stoutland,
    A3a057Stufful,
    A3a058Bewear,
    A3a059Oranguru,
    A3a060TypeNull,
    A3a061Silvally,
    A3a062Celesteela,
    A3a063BeastWall,
    A3a064Repel,
    A3a065ElectricalCord,
    A3a066Beastite,
    A3a067Gladion,
    A3a068Looker,
    A3a069Lusamine,
    A3a070Rowlet,
    A3a071Pheromosa,
    A3a072Blacephalon,
    A3a073AlolanMeowth,
    A3a074Silvally,
    A3a075Celesteela,
    A3a076BuzzwoleEx,
    A3a077TapuKokoEx,
    A3a078LycanrocEx,
    A3a079GuzzlordEx,
    A3a080AlolanDugtrioEx,
    A3a081Gladion,
    A3a082Looker,
    A3a083Lusamine,
    A3a084TapuKokoEx,
    A3a085LycanrocEx,
    A3a086GuzzlordEx,
    A3a087AlolanDugtrioEx,
    A3a088BuzzwoleEx,
    A3a089Growlithe,
    A3a090Arcanine,
    A3a091Froakie,
    A3a092Frogadier,
    A3a093Greninja,
    A3a094Jynx,
    A3a095Pidgey,
    A3a096Pidgeotto,
    A3a097Pidgeot,
    A3a098Aerodactyl,
    A3a099CelebiEx,
    A3a100ArcanineEx,
    A3a101AerodactylEx,
    A3a102PidgeotEx,
    A3a103Nihilego,
    A3b001Tropius,
    A3b002Leafeon,
    A3b003Bounsweet,
    A3b004Steenee,
    A3b005Tsareena,
    A3b006Applin,
    A3b007Appletun,
    A3b008Flareon,
    A3b009FlareonEx,
    A3b010Torkoal,
    A3b011Litten,
    A3b012Torracat,
    A3b013Incineroar,
    A3b014Salandit,
    A3b015Salazzle,
    A3b016Vaporeon,
    A3b017Glaceon,
    A3b018Vanillite,
    A3b019Vanillish,
    A3b020Vanilluxe,
    A3b021Alomomola,
    A3b022Popplio,
    A3b023Brionne,
    A3b024PrimarinaEx,
    A3b025Jolteon,
    A3b026Joltik,
    A3b027Galvantula,
    A3b028Espeon,
    A3b029Woobat,
    A3b030Swoobat,
    A3b031Swirlix,
    A3b032Slurpuff,
    A3b033Sylveon,
    A3b034SylveonEx,
    A3b035Mimikyu,
    A3b036Milcery,
    A3b037Alcremie,
    A3b038Barboach,
    A3b039Whiscash,
    A3b040Mienfoo,
    A3b041Mienshao,
    A3b042Carbink,
    A3b043Umbreon,
    A3b044Sableye,
    A3b045Purrloin,
    A3b046Liepard,
    A3b047Mawile,
    A3b048Togedemaru,
    A3b049Meltan,
    A3b050Melmetal,
    A3b051Dratini,
    A3b052Dragonair,
    A3b053DragoniteEx,
    A3b054Drampa,
    A3b055Eevee,
    A3b056EeveeEx,
    A3b057SnorlaxEx,
    A3b058Aipom,
    A3b059Ambipom,
    A3b060Chatot,
    A3b061Audino,
    A3b062Minccino,
    A3b063Cinccino,
    A3b064Skwovet,
    A3b065Greedent,
    A3b066EeveeBag,
    A3b067Leftovers,
    A3b068Hau,
    A3b069Penny,
    A3b070Leafeon,
    A3b071Flareon,
    A3b072Vaporeon,
    A3b073Glaceon,
    A3b074Jolteon,
    A3b075Espeon,
    A3b076Sylveon,
    A3b077Umbreon,
    A3b078Eevee,
    A3b079FlareonEx,
    A3b080PrimarinaEx,
    A3b081SylveonEx,
    A3b082DragoniteEx,
    A3b083EeveeEx,
    A3b084SnorlaxEx,
    A3b085Hau,
    A3b086Penny,
    A3b087FlareonEx,
    A3b088PrimarinaEx,
    A3b089SylveonEx,
    A3b090DragoniteEx,
    A3b091SnorlaxEx,
    A3b092EeveeEx,
    A3b093Pinsir,
    A3b094Lapras,
    A3b095Voltorb,
    A3b096Electrode,
    A3b097Ralts,
    A3b098Kirlia,
    A3b099Gardevoir,
    A3b100Ekans,
    A3b101Arbok,
    A3b102Farfetchd,
    A3b103MoltresEx,
    A3b104ArticunoEx,
    A3b105ZapdosEx,
    A3b106GalladeEx,
    A3b107EeveeBag,
    A4001Oddish,
    A4002Gloom,
    A4003Bellossom,
    A4004Tangela,
    A4005Tangrowth,
    A4006Scyther,
    A4007Pinsir,
    A4008Chikorita,
    A4009Bayleef,
    A4010Meganium,
    A4011Ledyba,
    A4012Ledian,
    A4013Hoppip,
    A4014Skiploom,
    A4015Jumpluff,
    A4016Sunkern,
    A4017Sunflora,
    A4018Yanma,
    A4019Yanmega,
    A4020Pineco,
    A4021ShuckleEx,
    A4022Heracross,
    A4023Cherubi,
    A4024Cherrim,
    A4025Vulpix,
    A4026Ninetales,
    A4027Cyndaquil,
    A4028Quilava,
    A4029Typhlosion,
    A4030Slugma,
    A4031Magcargo,
    A4032Magby,
    A4033Entei,
    A4034HoOhEx,
    A4035Darumaka,
    A4036Darmanitan,
    A4037Heatmor,
    A4038Poliwag,
    A4039Poliwhirl,
    A4040Politoed,
    A4041Horsea,
    A4042Seadra,
    A4043KingdraEx,
    A4044Magikarp,
    A4045Gyarados,
    A4046Totodile,
    A4047Croconaw,
    A4048Feraligatr,
    A4049Marill,
    A4050Azumarill,
    A4051Wooper,
    A4052Quagsire,
    A4053Qwilfish,
    A4054Corsola,
    A4055Remoraid,
    A4056Octillery,
    A4057Delibird,
    A4058Mantine,
    A4059Suicune,
    A4060Corphish,
    A4061Crawdaunt,
    A4062Ducklett,
    A4063Swanna,
    A4064Chinchou,
    A4065LanturnEx,
    A4066Pichu,
    A4067Mareep,
    A4068Flaaffy,
    A4069Ampharos,
    A4070Elekid,
    A4071Raikou,
    A4072Emolga,
    A4073Slowpoke,
    A4074Slowking,
    A4075Smoochum,
    A4076Jynx,
    A4077Cleffa,
    A4078Togepi,
    A4079Togetic,
    A4080Togekiss,
    A4081Natu,
    A4082Xatu,
    A4083EspeonEx,
    A4084Unown,
    A4085Unown,
    A4086Wobbuffet,
    A4087Girafarig,
    A4088Snubbull,
    A4089Granbull,
    A4090Munna,
    A4091Musharna,
    A4092Onix,
    A4093Sudowoodo,
    A4094Gligar,
    A4095Gliscor,
    A4096Swinub,
    A4097Piloswine,
    A4098Mamoswine,
    A4099Phanpy,
    A4100DonphanEx,
    A4101Tyrogue,
    A4102Hitmontop,
    A4103Larvitar,
    A4104Pupitar,
    A4105Binacle,
    A4106Barbaracle,
    A4107Zubat,
    A4108Golbat,
    A4109CrobatEx,
    A4110Spinarak,
    A4111Ariados,
    A4112UmbreonEx,
    A4113Murkrow,
    A4114Honchkrow,
    A4115Sneasel,
    A4116Weavile,
    A4117Houndour,
    A4118Houndoom,
    A4119Tyranitar,
    A4120Absol,
    A4121Forretress,
    A4122Steelix,
    A4123Scizor,
    A4124SkarmoryEx,
    A4125Mawile,
    A4126Klink,
    A4127Klang,
    A4128Klinklang,
    A4129Spearow,
    A4130Fearow,
    A4131Chansey,
    A4132Blissey,
    A4133Kangaskhan,
    A4134Eevee,
    A4135Porygon,
    A4136Porygon2,
    A4137PorygonZ,
    A4138Sentret,
    A4139Furret,
    A4140Hoothoot,
    A4141Noctowl,
    A4142Aipom,
    A4143Ambipom,
    A4144Dunsparce,
    A4145Teddiursa,
    A4146Ursaring,
    A4147Stantler,
    A4148Smeargle,
    A4149LugiaEx,
    A4150Bouffalant,
    A4151ElementalSwitch,
    A4152SquirtBottle,
    A4153SteelApron,
    A4154DarkPendant,
    A4155RescueScarf,
    A4156Will,
    A4157Lyra,
    A4158Silver,
    A4159Fisher,
    A4160Jasmine,
    A4161Hiker,
    A4162Chikorita,
    A4163Bellossom,
    A4164Heracross,
    A4165Cyndaquil,
    A4166Magby,
    A4167Totodile,
    A4168Qwilfish,
    A4169Octillery,
    A4170Delibird,
    A4171Pichu,
    A4172Ampharos,
    A4173Togepi,
    A4174Xatu,
    A4175Wobbuffet,
    A4176Gligar,
    A4177Spinarak,
    A4178Murkrow,
    A4179Tyranitar,
    A4180Scizor,
    A4181Sentret,
    A4182Hoothoot,
    A4183Stantler,
    A4184Smeargle,
    A4185Blissey,
    A4186ShuckleEx,
    A4187HoOhEx,
    A4188KingdraEx,
    A4189LanturnEx,
    A4190EspeonEx,
    A4191DonphanEx,
    A4192CrobatEx,
    A4193UmbreonEx,
    A4194SkarmoryEx,
    A4195LugiaEx,
    A4196Will,
    A4197Lyra,
    A4198Silver,
    A4199Fisher,
    A4200Jasmine,
    A4201Hiker,
    A4202ShuckleEx,
    A4203KingdraEx,
    A4204LanturnEx,
    A4205EspeonEx,
    A4206DonphanEx,
    A4207CrobatEx,
    A4208UmbreonEx,
    A4209SkarmoryEx,
    A4210HoOhEx,
    A4211LugiaEx,
    A4212Yanma,
    A4213Flareon,
    A4214Magikarp,
    A4215Gyarados,
    A4216Vaporeon,
    A4217Magnemite,
    A4218Magneton,
    A4219Jolteon,
    A4220Misdreavus,
    A4221Mankey,
    A4222Primeape,
    A4223NidoranF,
    A4224Nidorina,
    A4225Nidoqueen,
    A4226NidoranM,
    A4227Nidorino,
    A4228Nidoking,
    A4229Sneasel,
    A4230Lickitung,
    A4231Eevee,
    A4232YanmegaEx,
    A4233LeafeonEx,
    A4234GyaradosEx,
    A4235GlaceonEx,
    A4236PachirisuEx,
    A4237MismagiusEx,
    A4238WeavileEx,
    A4239LickilickyEx,
    A4240HoOhEx,
    A4241LugiaEx,
    A4a001Hoppip,
    A4a002Skiploom,
    A4a003JumpluffEx,
    A4a004Sunkern,
    A4a005Sunflora,
    A4a006Celebi,
    A4a007Durant,
    A4a008Slugma,
    A4a009Magcargo,
    A4a010EnteiEx,
    A4a011Fletchinder,
    A4a012Talonflame,
    A4a013Poliwag,
    A4a014Poliwhirl,
    A4a015Tentacool,
    A4a016Tentacruel,
    A4a017Slowpoke,
    A4a018Slowking,
    A4a019Jynx,
    A4a020SuicuneEx,
    A4a021Feebas,
    A4a022Milotic,
    A4a023Mantyke,
    A4a024Cryogonal,
    A4a025RaikouEx,
    A4a026Tynamo,
    A4a027Eelektrik,
    A4a028Eelektross,
    A4a029Stunfisk,
    A4a030Yamper,
    A4a031Boltund,
    A4a032Misdreavus,
    A4a033Mismagius,
    A4a034GalarianCorsola,
    A4a035GalarianCursola,
    A4a036Latias,
    A4a037Latios,
    A4a038Frillish,
    A4a039Jellicent,
    A4a040Diglett,
    A4a041Dugtrio,
    A4a042PoliwrathEx,
    A4a043Phanpy,
    A4a044Donphan,
    A4a045Relicanth,
    A4a046Dwebble,
    A4a047Crustle,
    A4a048Seviper,
    A4a049Zorua,
    A4a050Zoroark,
    A4a051Inkay,
    A4a052Malamar,
    A4a053Skrelp,
    A4a054Dragalge,
    A4a055Altaria,
    A4a056Farfetchd,
    A4a057Lickitung,
    A4a058Lickilicky,
    A4a059Igglybuff,
    A4a060Teddiursa,
    A4a061Ursaring,
    A4a062Miltank,
    A4a063Azurill,
    A4a064Swablu,
    A4a065Zangoose,
    A4a066Fletchling,
    A4a067InflatableBoat,
    A4a068MemoryLight,
    A4a069Whitney,
    A4a070TravelingMerchant,
    A4a071Morty,
    A4a072Milotic,
    A4a073Stunfisk,
    A4a074Yamper,
    A4a075Latios,
    A4a076Phanpy,
    A4a077Azurill,
    A4a078JumpluffEx,
    A4a079EnteiEx,
    A4a080SuicuneEx,
    A4a081RaikouEx,
    A4a082PoliwrathEx,
    A4a083Whitney,
    A4a084TravelingMerchant,
    A4a085Morty,
    A4a086JumpluffEx,
    A4a087EnteiEx,
    A4a088RaikouEx,
    A4a089PoliwrathEx,
    A4a090SuicuneEx,
    A4a091Chimchar,
    A4a092Monferno,
    A4a093Psyduck,
    A4a094Golduck,
    A4a095Krabby,
    A4a096Kingler,
    A4a097Pyukumuku,
    A4a098Gible,
    A4a099Gabite,
    A4a100PaldeanWooper,
    A4a101InfernapeEx,
    A4a102MewEx,
    A4a103GarchompEx,
    A4a104PaldeanClodsireEx,
    A4a105Mantyke,
    A4b001Bulbasaur,
    A4b002Bulbasaur,
    A4b003Ivysaur,
    A4b004Ivysaur,
    A4b005VenusaurEx,
    A4b006Weedle,
    A4b007Weedle,
    A4b008Kakuna,
    A4b009Kakuna,
    A4b010BeedrillEx,
    A4b011Exeggcute,
    A4b012Exeggcute,
    A4b013ExeggutorEx,
    A4b014Hoppip,
    A4b015Hoppip,
    A4b016Skiploom,
    A4b017Skiploom,
    A4b018Jumpluff,
    A4b019Jumpluff,
    A4b020Yanma,
    A4b021Yanma,
    A4b022YanmegaEx,
    A4b023ShuckleEx,
    A4b024CelebiEx,
    A4b025Cherubi,
    A4b026Cherubi,
    A4b027Cherrim,
    A4b028Cherrim,
    A4b029LeafeonEx,
    A4b030Shaymin,
    A4b031Shaymin,
    A4b032Snivy,
    A4b033Snivy,
    A4b034Servine,
    A4b035Servine,
    A4b036Serperior,
    A4b037Serperior,
    A4b038Rowlet,
    A4b039Rowlet,
    A4b040Dartrix,
    A4b041Dartrix,
    A4b042DecidueyeEx,
    A4b043DhelmiseEx,
    A4b044BuzzwoleEx,
    A4b045Pheromosa,
    A4b046Pheromosa,
    A4b047Kartana,
    A4b048Kartana,
    A4b049Sprigatito,
    A4b050Sprigatito,
    A4b051Floragato,
    A4b052Floragato,
    A4b053Meowscarada,
    A4b054Meowscarada,
    A4b055Charmander,
    A4b056Charmander,
    A4b057Charmeleon,
    A4b058Charmeleon,
    A4b059CharizardEx,
    A4b060CharizardEx,
    A4b061Growlithe,
    A4b062Growlithe,
    A4b063ArcanineEx,
    A4b064Flareon,
    A4b065Flareon,
    A4b066FlareonEx,
    A4b067MoltresEx,
    A4b068HoOhEx,
    A4b069Torkoal,
    A4b070Torkoal,
    A4b071Chimchar,
    A4b072Chimchar,
    A4b073Monferno,
    A4b074Monferno,
    A4b075InfernapeEx,
    A4b076Heatran,
    A4b077Heatran,
    A4b078Litten,
    A4b079Litten,
    A4b080Torracat,
    A4b081Torracat,
    A4b082IncineroarEx,
    A4b083Squirtle,
    A4b084Squirtle,
    A4b085Wartortle,
    A4b086Wartortle,
    A4b087BlastoiseEx,
    A4b088Horsea,
    A4b089Horsea,
    A4b090Seadra,
    A4b091Seadra,
    A4b092KingdraEx,
    A4b093Staryu,
    A4b094Staryu,
    A4b095StarmieEx,
    A4b096Magikarp,
    A4b097Magikarp,
    A4b098GyaradosEx,
    A4b099Vaporeon,
    A4b100Vaporeon,
    A4b101ArticunoEx,
    A4b102Corphish,
    A4b103Corphish,
    A4b104Crawdaunt,
    A4b105Crawdaunt,
    A4b106GlaceonEx,
    A4b107PalkiaEx,
    A4b108Manaphy,
    A4b109Manaphy,
    A4b110Froakie,
    A4b111Froakie,
    A4b112Frogadier,
    A4b113Frogadier,
    A4b114Greninja,
    A4b115Greninja,
    A4b116Popplio,
    A4b117Popplio,
    A4b118Brionne,
    A4b119Brionne,
    A4b120PrimarinaEx,
    A4b121CrabominableEx,
    A4b122Wishiwashi,
    A4b123Wishiwashi,
    A4b124WishiwashiEx,
    A4b125Wiglett,
    A4b126Wiglett,
    A4b127WugtrioEx,
    A4b128Pikachu,
    A4b129Pikachu,
    A4b130AlolanRaichuEx,
    A4b131PikachuEx,
    A4b132PikachuEx,
    A4b133Magnemite,
    A4b134Magnemite,
    A4b135Magneton,
    A4b136Magneton,
    A4b137Magnezone,
    A4b138Magnezone,
    A4b139ZapdosEx,
    A4b140Chinchou,
    A4b141Chinchou,
    A4b142LanturnEx,
    A4b143Pachirisu,
    A4b144Pachirisu,
    A4b145PachirisuEx,
    A4b146Oricorio,
    A4b147Oricorio,
    A4b148TapuKokoEx,
    A4b149Zeraora,
    A4b150Zeraora,
    A4b151Gastly,
    A4b152Gastly,
    A4b153Haunter,
    A4b154Haunter,
    A4b155GengarEx,
    A4b156Jynx,
    A4b157Jynx,
    A4b158MewtwoEx,
    A4b159MewEx,
    A4b160EspeonEx,
    A4b161Misdreavus,
    A4b162Misdreavus,
    A4b163MismagiusEx,
    A4b164Ralts,
    A4b165Ralts,
    A4b166Kirlia,
    A4b167Kirlia,
    A4b168Gardevoir,
    A4b169Gardevoir,
    A4b170Giratina,
    A4b171Giratina,
    A4b172GiratinaEx,
    A4b173Swirlix,
    A4b174Swirlix,
    A4b175Slurpuff,
    A4b176Slurpuff,
    A4b177SylveonEx,
    A4b178Oricorio,
    A4b179Oricorio,
    A4b180Cosmog,
    A4b181Cosmog,
    A4b182Cosmoem,
    A4b183Cosmoem,
    A4b184LunalaEx,
    A4b185Milcery,
    A4b186Milcery,
    A4b187Alcremie,
    A4b188Alcremie,
    A4b189Machop,
    A4b190Machop,
    A4b191Machoke,
    A4b192Machoke,
    A4b193MachampEx,
    A4b194Cubone,
    A4b195Cubone,
    A4b196MarowakEx,
    A4b197AerodactylEx,
    A4b198Sudowoodo,
    A4b199Sudowoodo,
    A4b200Phanpy,
    A4b201Phanpy,
    A4b202DonphanEx,
    A4b203Nosepass,
    A4b204Nosepass,
    A4b205Gible,
    A4b206Gible,
    A4b207Gabite,
    A4b208Gabite,
    A4b209GarchompEx,
    A4b210Riolu,
    A4b211Riolu,
    A4b212Lucario,
    A4b213Lucario,
    A4b214LucarioEx,
    A4b215GalladeEx,
    A4b216Drilbur,
    A4b217Drilbur,
    A4b218Crabrawler,
    A4b219Crabrawler,
    A4b220Rockruff,
    A4b221Rockruff,
    A4b222LycanrocEx,
    A4b223PassimianEx,
    A4b224Marshadow,
    A4b225Marshadow,
    A4b226Zubat,
    A4b227Zubat,
    A4b228Golbat,
    A4b229Golbat,
    A4b230Crobat,
    A4b231Crobat,
    A4b232CrobatEx,
    A4b233AlolanGrimer,
    A4b234AlolanGrimer,
    A4b235AlolanMukEx,
    A4b236PaldeanWooper,
    A4b237PaldeanWooper,
    A4b238PaldeanClodsireEx,
    A4b239Umbreon,
    A4b240Umbreon,
    A4b241UmbreonEx,
    A4b242Sneasel,
    A4b243Sneasel,
    A4b244WeavileEx,
    A4b245DarkraiEx,
    A4b246Nihilego,
    A4b247Nihilego,
    A4b248GuzzlordEx,
    A4b249AlolanDiglett,
    A4b250AlolanDiglett,
    A4b251AlolanDugtrioEx,
    A4b252SkarmoryEx,
    A4b253ProbopassEx,
    A4b254DialgaEx,
    A4b255Excadrill,
    A4b256Excadrill,
    A4b257Klefki,
    A4b258Klefki,
    A4b259SolgaleoEx,
    A4b260Magearna,
    A4b261Magearna,
    A4b262Tinkatink,
    A4b263Tinkatink,
    A4b264Tinkatuff,
    A4b265Tinkatuff,
    A4b266TinkatonEx,
    A4b267Dratini,
    A4b268Dratini,
    A4b269Dragonair,
    A4b270Dragonair,
    A4b271DragoniteEx,
    A4b272Pidgey,
    A4b273Pidgey,
    A4b274Pidgeotto,
    A4b275Pidgeotto,
    A4b276PidgeotEx,
    A4b277Jigglypuff,
    A4b278Jigglypuff,
    A4b279WigglytuffEx,
    A4b280Farfetchd,
    A4b281Farfetchd,
    A4b282Lickitung,
    A4b283Lickitung,
    A4b284LickilickyEx,
    A4b285Eevee,
    A4b286Eevee,
    A4b287EeveeEx,
    A4b288SnorlaxEx,
    A4b289LugiaEx,
    A4b290Skitty,
    A4b291Skitty,
    A4b292Delcatty,
    A4b293Delcatty,
    A4b294Bidoof,
    A4b295Bidoof,
    A4b296BibarelEx,
    A4b297Shaymin,
    A4b298Shaymin,
    A4b299ArceusEx,
    A4b300TypeNull,
    A4b301TypeNull,
    A4b302Silvally,
    A4b303Silvally,
    A4b304Celesteela,
    A4b305Celesteela,
    A4b306Cyclizar,
    A4b307Cyclizar,
    A4b308EeveeBag,
    A4b309EeveeBag,
    A4b310ElementalSwitch,
    A4b311ElementalSwitch,
    A4b312OldAmber,
    A4b313OldAmber,
    A4b314RareCandy,
    A4b315RareCandy,
    A4b316PokemonCommunication,
    A4b317PokemonCommunication,
    A4b318ElectricalCord,
    A4b319ElectricalCord,
    A4b320GiantCape,
    A4b321GiantCape,
    A4b322RockyHelmet,
    A4b323RockyHelmet,
    A4b324LeafCape,
    A4b325LeafCape,
    A4b326Cyrus,
    A4b327Cyrus,
    A4b328Erika,
    A4b329Erika,
    A4b330Irida,
    A4b331Irida,
    A4b332Lyra,
    A4b333Lyra,
    A4b334Giovanni,
    A4b335Giovanni,
    A4b336Silver,
    A4b337Silver,
    A4b338Sabrina,
    A4b339Sabrina,
    A4b340Iono,
    A4b341Iono,
    A4b342Dawn,
    A4b343Dawn,
    A4b344Mars,
    A4b345Mars,
    A4b346Leaf,
    A4b347Leaf,
    A4b348Lillie,
    A4b349Lillie,
    A4b350Lusamine,
    A4b351Lusamine,
    A4b352Red,
    A4b353Red,
    A4b354Floragato,
    A4b355Crawdaunt,
    A4b356Greninja,
    A4b357Gardevoir,
    A4b358Slurpuff,
    A4b359Farfetchd,
    A4b360BuzzwoleEx,
    A4b361CharizardEx,
    A4b362HoOhEx,
    A4b363PalkiaEx,
    A4b364PikachuEx,
    A4b365MewtwoEx,
    A4b366MewEx,
    A4b367LunalaEx,
    A4b368DialgaEx,
    A4b369SolgaleoEx,
    A4b370EeveeEx,
    A4b371LugiaEx,
    A4b372ArceusEx,
    A4b373ProfessorsResearch,
    A4b374Lillie,
    A4b375Lusamine,
    A4b376PikachuEx,
    A4b377GiratinaEx,
    A4b378DarkraiEx,
    A4b379RareCandy,
    B1001Pinsir,
    B1002MegaPinsirEx,
    B1003Wurmple,
    B1004Silcoon,
    B1005Beautifly,
    B1006Cascoon,
    B1007Dustox,
    B1008Seedot,
    B1009Nuzleaf,
    B1010Shiftry,
    B1011Shroomish,
    B1012Breloom,
    B1013Pansage,
    B1014Simisage,
    B1015Cottonee,
    B1016WhimsicottEx,
    B1017Petilil,
    B1018Lilligant,
    B1019Maractus,
    B1020Virizion,
    B1021Skiddo,
    B1022Gogoat,
    B1023Phantump,
    B1024Trevenant,
    B1025Grookey,
    B1026Thwackey,
    B1027Rillaboom,
    B1028Growlithe,
    B1029Arcanine,
    B1030Ponyta,
    B1031RapidashEx,
    B1032HoOh,
    B1033Torchic,
    B1034Combusken,
    B1035Blaziken,
    B1036MegaBlazikenEx,
    B1037Pansear,
    B1038Simisear,
    B1039Darumaka,
    B1040Darmanitan,
    B1041Litwick,
    B1042Lampent,
    B1043Chandelure,
    B1044Heatmor,
    B1045Litleo,
    B1046Pyroar,
    B1047Turtonator,
    B1048Psyduck,
    B1049Golduck,
    B1050Magikarp,
    B1051Gyarados,
    B1052MegaGyaradosEx,
    B1053Lotad,
    B1054Lombre,
    B1055Ludicolo,
    B1056Wailmer,
    B1057Wailord,
    B1058Corphish,
    B1059Crawdaunt,
    B1060Luvdisc,
    B1061Panpour,
    B1062Simipour,
    B1063Tympole,
    B1064Palpitoad,
    B1065Seismitoad,
    B1066Tirtouga,
    B1067Carracosta,
    B1068Frillish,
    B1069Jellicent,
    B1070Keldeo,
    B1071Froakie,
    B1072Frogadier,
    B1073GreninjaEx,
    B1074Bergmite,
    B1075Avalugg,
    B1076Chewtle,
    B1077Drednaw,
    B1078Arrokuda,
    B1079Barraskewda,
    B1080Eiscue,
    B1081JolteonEx,
    B1082Mareep,
    B1083Flaaffy,
    B1084Ampharos,
    B1085MegaAmpharosEx,
    B1086Shinx,
    B1087Luxio,
    B1088Luxray,
    B1089Pachirisu,
    B1090Blitzle,
    B1091Zebstrika,
    B1092Joltik,
    B1093Galvantula,
    B1094Dedenne,
    B1095Yamper,
    B1096Boltund,
    B1097Natu,
    B1098Xatu,
    B1099Misdreavus,
    B1100Mismagius,
    B1101Sableye,
    B1102MegaAltariaEx,
    B1103Duskull,
    B1104Dusclops,
    B1105Dusknoir,
    B1106Jirachi,
    B1107Drifloon,
    B1108Drifblim,
    B1109Chingling,
    B1110Yamask,
    B1111Cofagrigus,
    B1112Gothita,
    B1113Gothorita,
    B1114Gothitelle,
    B1115Spritzee,
    B1116Aromatisse,
    B1117Swirlix,
    B1118Slurpuff,
    B1119Carbink,
    B1120Klefki,
    B1121IndeedeeEx,
    B1122Sandshrew,
    B1123Sandslash,
    B1124HitmonchanEx,
    B1125Sudowoodo,
    B1126Makuhita,
    B1127Hariyama,
    B1128Hippopotas,
    B1129Hippowdon,
    B1130Sandile,
    B1131Krokorok,
    B1132Krookodile,
    B1133Archen,
    B1134Archeops,
    B1135Golett,
    B1136Golurk,
    B1137Terrakion,
    B1138Pancham,
    B1139Crabrawler,
    B1140Crabominable,
    B1141Stufful,
    B1142Bewear,
    B1143Sandygast,
    B1144Palossand,
    B1145Rolycoly,
    B1146Carkol,
    B1147Coalossal,
    B1148Murkrow,
    B1149Honchkrow,
    B1150Absol,
    B1151MegaAbsolEx,
    B1152Skorupi,
    B1153Drapion,
    B1154Darkrai,
    B1155Deino,
    B1156Zweilous,
    B1157Hydreigon,
    B1158Pangoro,
    B1159Skrelp,
    B1160DragalgeEx,
    B1161Mareanie,
    B1162ToxapEx,
    B1163Impidimp,
    B1164Morgrem,
    B1165Grimmsnarl,
    B1166Ferroseed,
    B1167Ferrothorn,
    B1168Durant,
    B1169Cobalion,
    B1170Honedge,
    B1171Doublade,
    B1172Aegislash,
    B1173Meltan,
    B1174MelmetalEx,
    B1175Corviknight,
    B1176Druddigon,
    B1177Goomy,
    B1178Sliggoo,
    B1179Goodra,
    B1180Pidgey,
    B1181Pidgeotto,
    B1182Pidgeot,
    B1183TaurosEx,
    B1184Eevee,
    B1185Aipom,
    B1186Ambipom,
    B1187Miltank,
    B1188Zigzagoon,
    B1189Linoone,
    B1190Whismur,
    B1191Loudred,
    B1192Exploud,
    B1193Skitty,
    B1194Delcatty,
    B1195Spinda,
    B1196Swablu,
    B1197Altaria,
    B1198Chatot,
    B1199Patrat,
    B1200Watchog,
    B1201Lillipup,
    B1202Herdier,
    B1203Stoutland,
    B1204Rufflet,
    B1205Braviary,
    B1206Furfrou,
    B1207Furfrou,
    B1208Furfrou,
    B1209Rookidee,
    B1210Corvisquire,
    B1211Wooloo,
    B1212Dubwool,
    B1213PrankSpinner,
    B1214PlumeFossil,
    B1215HittingHammer,
    B1216CoverFossil,
    B1217FlamePatch,
    B1218SitrusBerry,
    B1219HeavyHelmet,
    B1220LuckyMittens,
    B1221Marlon,
    B1222Hala,
    B1223May,
    B1224Fantina,
    B1225Copycat,
    B1226Lisia,
    B1227Beautifly,
    B1228Skiddo,
    B1229Rillaboom,
    B1230Growlithe,
    B1231Chandelure,
    B1232Magikarp,
    B1233Ludicolo,
    B1234Jellicent,
    B1235Keldeo,
    B1236Eiscue,
    B1237Luxray,
    B1238Cofagrigus,
    B1239Gothita,
    B1240Makuhita,
    B1241Hippowdon,
    B1242Archeops,
    B1243Pancham,
    B1244Honchkrow,
    B1245Hydreigon,
    B1246Corviknight,
    B1247Goomy,
    B1248Delcatty,
    B1249Stoutland,
    B1250Rufflet,
    B1251MegaPinsirEx,
    B1252WhimsicottEx,
    B1253RapidashEx,
    B1254MegaBlazikenEx,
    B1255MegaGyaradosEx,
    B1256GreninjaEx,
    B1257JolteonEx,
    B1258MegaAmpharosEx,
    B1259MegaAltariaEx,
    B1260IndeedeeEx,
    B1261HitmonchanEx,
    B1262MegaAbsolEx,
    B1263DragalgeEx,
    B1264MelmetalEx,
    B1265TaurosEx,
    B1266Marlon,
    B1267Hala,
    B1268May,
    B1269Fantina,
    B1270Copycat,
    B1271Lisia,
    B1272MegaPinsirEx,
    B1273WhimsicottEx,
    B1274RapidashEx,
    B1275GreninjaEx,
    B1276JolteonEx,
    B1277MegaAmpharosEx,
    B1278IndeedeeEx,
    B1279HitmonchanEx,
    B1280MegaAbsolEx,
    B1281DragalgeEx,
    B1282MelmetalEx,
    B1283TaurosEx,
    B1284MegaBlazikenEx,
    B1285MegaGyaradosEx,
    B1286MegaAltariaEx,
    B1287Bellsprout,
    B1288Weepinbell,
    B1289Victreebel,
    B1290Rowlet,
    B1291Dartrix,
    B1292Moltres,
    B1293Litten,
    B1294Torracat,
    B1295Poliwag,
    B1296Poliwhirl,
    B1297Poliwrath,
    B1298Articuno,
    B1299Manaphy,
    B1300Popplio,
    B1301Brionne,
    B1302Zapdos,
    B1303Oricorio,
    B1304Zeraora,
    B1305Drowzee,
    B1306Hypno,
    B1307Geodude,
    B1308Graveler,
    B1309Golem,
    B1310Rockruff,
    B1311AlolanDiglett,
    B1312Meowth,
    B1313Persian,
    B1314Doduo,
    B1315Dodrio,
    B1316Bidoof,
    B1317DecidueyeEx,
    B1318IncineroarEx,
    B1319PalkiaEx,
    B1320PrimarinaEx,
    B1321PikachuEx,
    B1322TapuKokoEx,
    B1323LycanrocEx,
    B1324PassimianEx,
    B1325AlolanDugtrioEx,
    B1326DialgaEx,
    B1327BibarelEx,
    B1328ArceusEx,
    B1329Lilligant,
    B1330Klefki,
    B1331FlamePatch,
    B1a001Bulbasaur,
    B1a002Ivysaur,
    B1a003Venusaur,
    B1a004MegaVenusaurEx,
    B1a005Spinarak,
    B1a006Ariados,
    B1a007Sunkern,
    B1a008Sunflora,
    B1a009Burmy,
    B1a010Mothim,
    B1a011Charmander,
    B1a012Charmeleon,
    B1a013Charizard,
    B1a014MegaCharizardYEx,
    B1a015Houndour,
    B1a016Houndoom,
    B1a017Squirtle,
    B1a018Wartortle,
    B1a019Blastoise,
    B1a020MegaBlastoiseEx,
    B1a021Basculin,
    B1a022Clauncher,
    B1a023Clawitzer,
    B1a024Magnemite,
    B1a025Magneton,
    B1a026Magnezone,
    B1a027Emolga,
    B1a028Helioptile,
    B1a029Heliolisk,
    B1a030Misdreavus,
    B1a031Mismagius,
    B1a032Solosis,
    B1a033Duosion,
    B1a034Reuniclus,
    B1a035Spritzee,
    B1a036Aromatisse,
    B1a037Xerneas,
    B1a038Onix,
    B1a039Makuhita,
    B1a040Hariyama,
    B1a041Nosepass,
    B1a042MegaLopunnyEx,
    B1a043Mienfoo,
    B1a044Mienshao,
    B1a045Grimer,
    B1a046Muk,
    B1a047Purrloin,
    B1a048Liepard,
    B1a049Trubbish,
    B1a050Garbodor,
    B1a051Steelix,
    B1a052MegaSteelixEx,
    B1a053Probopass,
    B1a054Genesect,
    B1a055Ditto,
    B1a056Porygon,
    B1a057Porygon2,
    B1a058PorygonZ,
    B1a059Starly,
    B1a060Staravia,
    B1a061Staraptor,
    B1a062Buneary,
    B1a063Lopunny,
    B1a064Bouffalant,
    B1a065Furfrou,
    B1a066ClemontsBackpack,
    B1a067QuickGrowExtract,
    B1a068Clemont,
    B1a069Serena,
    B1a070Ariados,
    B1a071Sunflora,
    B1a072Reuniclus,
    B1a073Xerneas,
    B1a074Trubbish,
    B1a075Buneary,
    B1a076MegaVenusaurEx,
    B1a077MegaCharizardYEx,
    B1a078MegaBlastoiseEx,
    B1a079MegaLopunnyEx,
    B1a080MegaSteelixEx,
    B1a081Clemont,
    B1a082Serena,
    B1a083MegaVenusaurEx,
    B1a084MegaBlastoiseEx,
    B1a085MegaLopunnyEx,
    B1a086MegaSteelixEx,
    B1a087MegaCharizardYEx,
    B1a088Oddish,
    B1a089Gloom,
    B1a090Vileplume,
    B1a091Charizard,
    B1a092Shellder,
    B1a093Cloyster,
    B1a094Sandshrew,
    B1a095Sandslash,
    B1a096TypeNull,
    B1a097Silvally,
    B1a098BuzzwoleEx,
    B1a099LunalaEx,
    B1a100GuzzlordEx,
    B1a101SolgaleoEx,
    B1a102Aegislash,
    B1a103QuickGrowExtract,
    B2001Ledyba,
    B2002Ledian,
    B2003Shuckle,
    B2004Roselia,
    B2005Roserade,
    B2006Cacnea,
    B2007Cacturne,
    B2008Chespin,
    B2009Quilladin,
    B2010Chesnaught,
    B2011Scatterbug,
    B2012Spewpa,
    B2013Vivillon,
    B2014Buzzwole,
    B2015Gossifleur,
    B2016Eldegoss,
    B2017TealMaskOgerponEx,
    B2018AlolanMarowak,
    B2019Reshiram,
    B2020Litleo,
    B2021Pyroar,
    B2022Oricorio,
    B2023BlacephalonEx,
    B2024Scorbunny,
    B2025Raboot,
    B2026Cinderace,
    B2027HearthflameMaskOgerpon,
    B2028AlolanVulpix,
    B2029AlolanNinetalesEx,
    B2030GalarianMrMime,
    B2031GalarianMrRime,
    B2032Delibird,
    B2033Mudkip,
    B2034Marshtomp,
    B2035Swampert,
    B2036MegaSwampertEx,
    B2037Vanillite,
    B2038Vanillish,
    B2039Vanilluxe,
    B2040Cryogonal,
    B2041Amaura,
    B2042Aurorus,
    B2043Chewtle,
    B2044Drednaw,
    B2045Cramorant,
    B2046Arrokuda,
    B2047Barraskewda,
    B2048WellspringMaskOgerpon,
    B2049Pikachu,
    B2050AlolanRaichu,
    B2051Zapdos,
    B2052Plusle,
    B2053Minun,
    B2054Toxel,
    B2055ToxtricityEx,
    B2056Tadbulb,
    B2057Bellibolt,
    B2058GalarianPonyta,
    B2059GalarianRapidash,
    B2060Wobbuffet,
    B2061Snubbull,
    B2062Granbull,
    B2063Ralts,
    B2064Kirlia,
    B2065Gardevoir,
    B2066MegaGardevoirEx,
    B2067Litwick,
    B2068Lampent,
    B2069Chandelure,
    B2070Meloetta,
    B2071Pumpkaboo,
    B2072Gourgeist,
    B2073MimikyuEx,
    B2074Sinistea,
    B2075Polteageist,
    B2076Indeedee,
    B2077Sandshrew,
    B2078Sandslash,
    B2079Machop,
    B2080Machoke,
    B2081Machamp,
    B2082Cubone,
    B2083Meditite,
    B2084Medicham,
    B2085Roggenrola,
    B2086Boldore,
    B2087GigalithEx,
    B2088Drilbur,
    B2089Tyrunt,
    B2090Tyrantrum,
    B2091Passimian,
    B2093CornerstoneMaskOgerpon,
    B2094AlolanMeowth,
    B2095AlolanPersian,
    B2096AlolanGrimer,
    B2097AlolanMuk,
    B2098GalarianZigzagoon,
    B2099GalarianLinoone,
    B2100GalarianObstagoon,
    B2101Stunky,
    B2102Skuntank,
    B2103Spiritomb,
    B2104Purrloin,
    B2105Liepard,
    B2106Scraggy,
    B2107Scrafty,
    B2108Yveltal,
    B2109Guzzlord,
    B2110GalarianMeowth,
    B2111GalarianPerrserker,
    B2112Mawile,
    B2113MegaMawileEx,
    B2114Excadrill,
    B2115Ferroseed,
    B2116Ferrothorn,
    B2117GalarianStunfisk,
    B2118Honedge,
    B2119Doublade,
    B2120Aegislash,
    B2121Bagon,
    B2122Shelgon,
    B2123Salamence,
    B2124Meowth,
    B2125Persian,
    B2126Kangaskhan,
    B2127MegaKangaskhanEx,
    B2128Sentret,
    B2129Furret,
    B2130Smeargle,
    B2131Lugia,
    B2132Taillow,
    B2133Swellow,
    B2134Slakoth,
    B2135Vigoroth,
    B2136Slaking,
    B2137Spinda,
    B2138Tornadus,
    B2139Bunnelby,
    B2140Diggersby,
    B2141Furfrou,
    B2142Tandemaus,
    B2143Maushold,
    B2144JawFossil,
    B2145LuckyIcePop,
    B2146SailFossil,
    B2147ProtectivePoncho,
    B2148MetalCoreBarrier,
    B2149Diantha,
    B2150Sightseer,
    B2151Juggler,
    B2152Piers,
    B2153TrainingArea,
    B2154StartingPlains,
    B2155PeculiarPlaza,
    B2156Cacnea,
    B2157Roserade,
    B2158Vivillon,
    B2159Buzzwole,
    B2160Reshiram,
    B2161Oricorio,
    B2162Scorbunny,
    B2163Aurorus,
    B2164Cramorant,
    B2165Minun,
    B2166Toxel,
    B2167GalarianPonyta,
    B2168Snubbull,
    B2169Indeedee,
    B2170Sandshrew,
    B2171Tyrantrum,
    B2172Falinks,
    B2173AlolanMuk,
    B2174Purrloin,
    B2175Yveltal,
    B2176GalarianObstagoon,
    B2177GalarianPerrserker,
    B2178Salamence,
    B2179Slakoth,
    B2180TealMaskOgerponEx,
    B2181BlacephalonEx,
    B2182AlolanNinetalesEx,
    B2183MegaSwampertEx,
    B2184ToxtricityEx,
    B2185MegaGardevoirEx,
    B2186MimikyuEx,
    B2187GigalithEx,
    B2188MegaMawileEx,
    B2189MegaKangaskhanEx,
    B2190Diantha,
    B2191Sightseer,
    B2192Juggler,
    B2193Piers,
    B2194TealMaskOgerponEx,
    B2195BlacephalonEx,
    B2196AlolanNinetalesEx,
    B2197MegaSwampertEx,
    B2198ToxtricityEx,
    B2199MimikyuEx,
    B2200GigalithEx,
    B2201MegaMawileEx,
    B2202MegaKangaskhanEx,
    B2203MegaGardevoirEx,
    B2204Meowth,
    B2205Tangela,
    B2206Magby,
    B2207Magmar,
    B2208Horsea,
    B2209Seadra,
    B2210Mantyke,
    B2211Omanyte,
    B2212Omastar,
    B2213Pichu,
    B2214Clefairy,
    B2215Clefable,
    B2216Latias,
    B2217Latios,
    B2218Hitmonlee,
    B2219Hitmonchan,
    B2220Kabuto,
    B2221Kabutops,
    B2222Phanpy,
    B2223Tyrogue,
    B2224Tauros,
    B2225FlareonEx,
    B2226HoOhEx,
    B2227KingdraEx,
    B2228EspeonEx,
    B2229SylveonEx,
    B2230DonphanEx,
    B2231UmbreonEx,
    B2232LugiaEx,
    B2233Meloetta,
    B2234ProtectivePoncho,
    PA001Potion,
    PA002XSpeed,
    PA003HandScope,
    PA004PokedEx,
    PA005PokeBall,
    PA006RedCard,
    PA007ProfessorsResearch,
    PA008PokedEx,
    PA009Pikachu,
    PA010Mewtwo,
    PA011Chansey,
    PA012Meowth,
    PA013Butterfree,
    PA014LaprasEx,
    PA015Pikachu,
    PA016Clefairy,
    PA017Mankey,
    PA018Venusaur,
    PA019Greninja,
    PA020Haunter,
    PA021Onix,
    PA022Jigglypuff,
    PA023Bulbasaur,
    PA024Magnemite,
    PA025MoltresEx,
    PA026Pikachu,
    PA027Snivy,
    PA028Volcarona,
    PA029Blastoise,
    PA030Eevee,
    PA031Cinccino,
    PA032Charmander,
    PA033Squirtle,
    PA034Piplup,
    PA035Turtwig,
    PA036Electivire,
    PA037CresseliaEx,
    PA038Misdreavus,
    PA039Skarmory,
    PA040Chimchar,
    PA041Togepi,
    PA042DarkraiEx,
    PA043Cherrim,
    PA044Raichu,
    PA045Nosepass,
    PA046Gible,
    PA047Staraptor,
    PA048Manaphy,
    PA049Snorlax,
    PA050MewtwoEx,
    PA051Cyclizar,
    PA052Sprigatito,
    PA053Floatzel,
    PA054Pawmot,
    PA055Machamp,
    PA056Ekans,
    PA057Bidoof,
    PA058Pachirisu,
    PA059Riolu,
    PA060Exeggcute,
    PA061Froakie,
    PA062Farfetchd,
    PA063Rayquaza,
    PA064RayquazaEx,
    PA065RayquazaEx,
    PA066Mimikyu,
    PA067Cosmog,
    PA068Lycanroc,
    PA069AlolanExeggutor,
    PA070AlolanNinetales,
    PA071Crabrawler,
    PA072AlolanGrimer,
    PA073Toucannon,
    PA074Zeraora,
    PA075Kartana,
    PA076Blacephalon,
    PA077Xurkitree,
    PA078DawnWingsNecrozma,
    PA079DuskManeNecrozma,
    PA080Stakataka,
    PA081UltraNecrozmaEx,
    PA082Poipole,
    PA083Stufful,
    PA084TapuKokoEx,
    PA085Vanillite,
    PA086Jolteon,
    PA087Alcremie,
    PA088Dragonair,
    PA089Audino,
    PA090Togedemaru,
    PA091Greedent,
    PA092Eevee,
    PA093Cleffa,
    PA094Horsea,
    PA095Chinchou,
    PA096Houndoom,
    PA097Kangaskhan,
    PA098BlisseyEx,
    PA099Marill,
    PA100Weavile,
    PA101Latias,
    PA102Tropius,
    PA103Poliwag,
    PA104Milotic,
    PA105Zorua,
    PA106Zoroark,
    PA107Miltank,
    PA108Phanpy,
    PA109EeveeEx,
    PA110EnteiEx,
    PA111Pikachu,
    PA112RaichuEx,
    PA113Mimikyu,
    PA114Machamp,
    PA115Regigigas,
    PA116Shaymin,
    PA117Absol,
    PB001Pikachu,
    PB002Petilil,
    PB003Froakie,
    PB004Luxray,
    PB005Pidgey,
    PB006MegaPidgeotEx,
    PB007Torchic,
    PB008Psyduck,
    PB009MegaAbsolEx,
    PB010Drifblim,
    PB011Eevee,
    PB012Ditto,
    PB013Arcanine,
    PB014Magikarp,
    PB015Mareep,
    PB016Krookodile,
    PB017Swablu,
    PB018Heliolisk,
    PB019Buneary,
    PB020Charmeleon,
    PB021Onix,
    PB022Hawlucha,
    PB023Genesect,
    PB024MegaLatiosEx,
}

static CARD_ID_MAP: LazyLock<HashMap<&'static str, CardId>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("A1 001", CardId::A1001Bulbasaur);
    map.insert("A1 002", CardId::A1002Ivysaur);
    map.insert("A1 003", CardId::A1003Venusaur);
    map.insert("A1 004", CardId::A1004VenusaurEx);
    map.insert("A1 005", CardId::A1005Caterpie);
    map.insert("A1 006", CardId::A1006Metapod);
    map.insert("A1 007", CardId::A1007Butterfree);
    map.insert("A1 008", CardId::A1008Weedle);
    map.insert("A1 009", CardId::A1009Kakuna);
    map.insert("A1 010", CardId::A1010Beedrill);
    map.insert("A1 011", CardId::A1011Oddish);
    map.insert("A1 012", CardId::A1012Gloom);
    map.insert("A1 013", CardId::A1013Vileplume);
    map.insert("A1 014", CardId::A1014Paras);
    map.insert("A1 015", CardId::A1015Parasect);
    map.insert("A1 016", CardId::A1016Venonat);
    map.insert("A1 017", CardId::A1017Venomoth);
    map.insert("A1 018", CardId::A1018Bellsprout);
    map.insert("A1 019", CardId::A1019Weepinbell);
    map.insert("A1 020", CardId::A1020Victreebel);
    map.insert("A1 021", CardId::A1021Exeggcute);
    map.insert("A1 022", CardId::A1022Exeggutor);
    map.insert("A1 023", CardId::A1023ExeggutorEx);
    map.insert("A1 024", CardId::A1024Tangela);
    map.insert("A1 025", CardId::A1025Scyther);
    map.insert("A1 026", CardId::A1026Pinsir);
    map.insert("A1 027", CardId::A1027Cottonee);
    map.insert("A1 028", CardId::A1028Whimsicott);
    map.insert("A1 029", CardId::A1029Petilil);
    map.insert("A1 030", CardId::A1030Lilligant);
    map.insert("A1 031", CardId::A1031Skiddo);
    map.insert("A1 032", CardId::A1032Gogoat);
    map.insert("A1 033", CardId::A1033Charmander);
    map.insert("A1 034", CardId::A1034Charmeleon);
    map.insert("A1 035", CardId::A1035Charizard);
    map.insert("A1 036", CardId::A1036CharizardEx);
    map.insert("A1 037", CardId::A1037Vulpix);
    map.insert("A1 038", CardId::A1038Ninetales);
    map.insert("A1 039", CardId::A1039Growlithe);
    map.insert("A1 040", CardId::A1040Arcanine);
    map.insert("A1 041", CardId::A1041ArcanineEx);
    map.insert("A1 042", CardId::A1042Ponyta);
    map.insert("A1 043", CardId::A1043Rapidash);
    map.insert("A1 044", CardId::A1044Magmar);
    map.insert("A1 045", CardId::A1045Flareon);
    map.insert("A1 046", CardId::A1046Moltres);
    map.insert("A1 047", CardId::A1047MoltresEx);
    map.insert("A1 048", CardId::A1048Heatmor);
    map.insert("A1 049", CardId::A1049Salandit);
    map.insert("A1 050", CardId::A1050Salazzle);
    map.insert("A1 051", CardId::A1051Sizzlipede);
    map.insert("A1 052", CardId::A1052Centiskorch);
    map.insert("A1 053", CardId::A1053Squirtle);
    map.insert("A1 054", CardId::A1054Wartortle);
    map.insert("A1 055", CardId::A1055Blastoise);
    map.insert("A1 056", CardId::A1056BlastoiseEx);
    map.insert("A1 057", CardId::A1057Psyduck);
    map.insert("A1 058", CardId::A1058Golduck);
    map.insert("A1 059", CardId::A1059Poliwag);
    map.insert("A1 060", CardId::A1060Poliwhirl);
    map.insert("A1 061", CardId::A1061Poliwrath);
    map.insert("A1 062", CardId::A1062Tentacool);
    map.insert("A1 063", CardId::A1063Tentacruel);
    map.insert("A1 064", CardId::A1064Seel);
    map.insert("A1 065", CardId::A1065Dewgong);
    map.insert("A1 066", CardId::A1066Shellder);
    map.insert("A1 067", CardId::A1067Cloyster);
    map.insert("A1 068", CardId::A1068Krabby);
    map.insert("A1 069", CardId::A1069Kingler);
    map.insert("A1 070", CardId::A1070Horsea);
    map.insert("A1 071", CardId::A1071Seadra);
    map.insert("A1 072", CardId::A1072Goldeen);
    map.insert("A1 073", CardId::A1073Seaking);
    map.insert("A1 074", CardId::A1074Staryu);
    map.insert("A1 075", CardId::A1075Starmie);
    map.insert("A1 076", CardId::A1076StarmieEx);
    map.insert("A1 077", CardId::A1077Magikarp);
    map.insert("A1 078", CardId::A1078Gyarados);
    map.insert("A1 079", CardId::A1079Lapras);
    map.insert("A1 080", CardId::A1080Vaporeon);
    map.insert("A1 081", CardId::A1081Omanyte);
    map.insert("A1 082", CardId::A1082Omastar);
    map.insert("A1 083", CardId::A1083Articuno);
    map.insert("A1 084", CardId::A1084ArticunoEx);
    map.insert("A1 085", CardId::A1085Ducklett);
    map.insert("A1 086", CardId::A1086Swanna);
    map.insert("A1 087", CardId::A1087Froakie);
    map.insert("A1 088", CardId::A1088Frogadier);
    map.insert("A1 089", CardId::A1089Greninja);
    map.insert("A1 090", CardId::A1090Pyukumuku);
    map.insert("A1 091", CardId::A1091Bruxish);
    map.insert("A1 092", CardId::A1092Snom);
    map.insert("A1 093", CardId::A1093Frosmoth);
    map.insert("A1 094", CardId::A1094Pikachu);
    map.insert("A1 095", CardId::A1095Raichu);
    map.insert("A1 096", CardId::A1096PikachuEx);
    map.insert("A1 097", CardId::A1097Magnemite);
    map.insert("A1 098", CardId::A1098Magneton);
    map.insert("A1 099", CardId::A1099Voltorb);
    map.insert("A1 100", CardId::A1100Electrode);
    map.insert("A1 101", CardId::A1101Electabuzz);
    map.insert("A1 102", CardId::A1102Jolteon);
    map.insert("A1 103", CardId::A1103Zapdos);
    map.insert("A1 104", CardId::A1104ZapdosEx);
    map.insert("A1 105", CardId::A1105Blitzle);
    map.insert("A1 106", CardId::A1106Zebstrika);
    map.insert("A1 107", CardId::A1107Tynamo);
    map.insert("A1 108", CardId::A1108Eelektrik);
    map.insert("A1 109", CardId::A1109Eelektross);
    map.insert("A1 110", CardId::A1110Helioptile);
    map.insert("A1 111", CardId::A1111Heliolisk);
    map.insert("A1 112", CardId::A1112Pincurchin);
    map.insert("A1 113", CardId::A1113Clefairy);
    map.insert("A1 114", CardId::A1114Clefable);
    map.insert("A1 115", CardId::A1115Abra);
    map.insert("A1 116", CardId::A1116Kadabra);
    map.insert("A1 117", CardId::A1117Alakazam);
    map.insert("A1 118", CardId::A1118Slowpoke);
    map.insert("A1 119", CardId::A1119Slowbro);
    map.insert("A1 120", CardId::A1120Gastly);
    map.insert("A1 121", CardId::A1121Haunter);
    map.insert("A1 122", CardId::A1122Gengar);
    map.insert("A1 123", CardId::A1123GengarEx);
    map.insert("A1 124", CardId::A1124Drowzee);
    map.insert("A1 125", CardId::A1125Hypno);
    map.insert("A1 126", CardId::A1126MrMime);
    map.insert("A1 127", CardId::A1127Jynx);
    map.insert("A1 128", CardId::A1128Mewtwo);
    map.insert("A1 129", CardId::A1129MewtwoEx);
    map.insert("A1 130", CardId::A1130Ralts);
    map.insert("A1 131", CardId::A1131Kirlia);
    map.insert("A1 132", CardId::A1132Gardevoir);
    map.insert("A1 133", CardId::A1133Woobat);
    map.insert("A1 134", CardId::A1134Swoobat);
    map.insert("A1 135", CardId::A1135Golett);
    map.insert("A1 136", CardId::A1136Golurk);
    map.insert("A1 137", CardId::A1137Sandshrew);
    map.insert("A1 138", CardId::A1138Sandslash);
    map.insert("A1 139", CardId::A1139Diglett);
    map.insert("A1 140", CardId::A1140Dugtrio);
    map.insert("A1 141", CardId::A1141Mankey);
    map.insert("A1 142", CardId::A1142Primeape);
    map.insert("A1 143", CardId::A1143Machop);
    map.insert("A1 144", CardId::A1144Machoke);
    map.insert("A1 145", CardId::A1145Machamp);
    map.insert("A1 146", CardId::A1146MachampEx);
    map.insert("A1 147", CardId::A1147Geodude);
    map.insert("A1 148", CardId::A1148Graveler);
    map.insert("A1 149", CardId::A1149Golem);
    map.insert("A1 150", CardId::A1150Onix);
    map.insert("A1 151", CardId::A1151Cubone);
    map.insert("A1 152", CardId::A1152Marowak);
    map.insert("A1 153", CardId::A1153MarowakEx);
    map.insert("A1 154", CardId::A1154Hitmonlee);
    map.insert("A1 155", CardId::A1155Hitmonchan);
    map.insert("A1 156", CardId::A1156Rhyhorn);
    map.insert("A1 157", CardId::A1157Rhydon);
    map.insert("A1 158", CardId::A1158Kabuto);
    map.insert("A1 159", CardId::A1159Kabutops);
    map.insert("A1 160", CardId::A1160Mienfoo);
    map.insert("A1 161", CardId::A1161Mienshao);
    map.insert("A1 162", CardId::A1162Clobbopus);
    map.insert("A1 163", CardId::A1163Grapploct);
    map.insert("A1 164", CardId::A1164Ekans);
    map.insert("A1 165", CardId::A1165Arbok);
    map.insert("A1 166", CardId::A1166NidoranF);
    map.insert("A1 167", CardId::A1167Nidorina);
    map.insert("A1 168", CardId::A1168Nidoqueen);
    map.insert("A1 169", CardId::A1169NidoranM);
    map.insert("A1 170", CardId::A1170Nidorino);
    map.insert("A1 171", CardId::A1171Nidoking);
    map.insert("A1 172", CardId::A1172Zubat);
    map.insert("A1 173", CardId::A1173Golbat);
    map.insert("A1 174", CardId::A1174Grimer);
    map.insert("A1 175", CardId::A1175Muk);
    map.insert("A1 176", CardId::A1176Koffing);
    map.insert("A1 177", CardId::A1177Weezing);
    map.insert("A1 178", CardId::A1178Mawile);
    map.insert("A1 179", CardId::A1179Pawniard);
    map.insert("A1 180", CardId::A1180Bisharp);
    map.insert("A1 181", CardId::A1181Meltan);
    map.insert("A1 182", CardId::A1182Melmetal);
    map.insert("A1 183", CardId::A1183Dratini);
    map.insert("A1 184", CardId::A1184Dragonair);
    map.insert("A1 185", CardId::A1185Dragonite);
    map.insert("A1 186", CardId::A1186Pidgey);
    map.insert("A1 187", CardId::A1187Pidgeotto);
    map.insert("A1 188", CardId::A1188Pidgeot);
    map.insert("A1 189", CardId::A1189Rattata);
    map.insert("A1 190", CardId::A1190Raticate);
    map.insert("A1 191", CardId::A1191Spearow);
    map.insert("A1 192", CardId::A1192Fearow);
    map.insert("A1 193", CardId::A1193Jigglypuff);
    map.insert("A1 194", CardId::A1194Wigglytuff);
    map.insert("A1 195", CardId::A1195WigglytuffEx);
    map.insert("A1 196", CardId::A1196Meowth);
    map.insert("A1 197", CardId::A1197Persian);
    map.insert("A1 198", CardId::A1198Farfetchd);
    map.insert("A1 199", CardId::A1199Doduo);
    map.insert("A1 200", CardId::A1200Dodrio);
    map.insert("A1 201", CardId::A1201Lickitung);
    map.insert("A1 202", CardId::A1202Chansey);
    map.insert("A1 203", CardId::A1203Kangaskhan);
    map.insert("A1 204", CardId::A1204Tauros);
    map.insert("A1 205", CardId::A1205Ditto);
    map.insert("A1 206", CardId::A1206Eevee);
    map.insert("A1 207", CardId::A1207Eevee);
    map.insert("A1 208", CardId::A1208Eevee);
    map.insert("A1 209", CardId::A1209Porygon);
    map.insert("A1 210", CardId::A1210Aerodactyl);
    map.insert("A1 211", CardId::A1211Snorlax);
    map.insert("A1 212", CardId::A1212Minccino);
    map.insert("A1 213", CardId::A1213Cinccino);
    map.insert("A1 214", CardId::A1214Wooloo);
    map.insert("A1 215", CardId::A1215Dubwool);
    map.insert("A1 216", CardId::A1216HelixFossil);
    map.insert("A1 217", CardId::A1217DomeFossil);
    map.insert("A1 218", CardId::A1218OldAmber);
    map.insert("A1 219", CardId::A1219Erika);
    map.insert("A1 220", CardId::A1220Misty);
    map.insert("A1 221", CardId::A1221Blaine);
    map.insert("A1 222", CardId::A1222Koga);
    map.insert("A1 223", CardId::A1223Giovanni);
    map.insert("A1 224", CardId::A1224Brock);
    map.insert("A1 225", CardId::A1225Sabrina);
    map.insert("A1 226", CardId::A1226LtSurge);
    map.insert("A1 227", CardId::A1227Bulbasaur);
    map.insert("A1 228", CardId::A1228Gloom);
    map.insert("A1 229", CardId::A1229Pinsir);
    map.insert("A1 230", CardId::A1230Charmander);
    map.insert("A1 231", CardId::A1231Rapidash);
    map.insert("A1 232", CardId::A1232Squirtle);
    map.insert("A1 233", CardId::A1233Gyarados);
    map.insert("A1 234", CardId::A1234Lapras);
    map.insert("A1 235", CardId::A1235Electrode);
    map.insert("A1 236", CardId::A1236Alakazam);
    map.insert("A1 237", CardId::A1237Slowpoke);
    map.insert("A1 238", CardId::A1238Diglett);
    map.insert("A1 239", CardId::A1239Cubone);
    map.insert("A1 240", CardId::A1240Nidoqueen);
    map.insert("A1 241", CardId::A1241Nidoking);
    map.insert("A1 242", CardId::A1242Golbat);
    map.insert("A1 243", CardId::A1243Weezing);
    map.insert("A1 244", CardId::A1244Dragonite);
    map.insert("A1 245", CardId::A1245Pidgeot);
    map.insert("A1 246", CardId::A1246Meowth);
    map.insert("A1 247", CardId::A1247Ditto);
    map.insert("A1 248", CardId::A1248Eevee);
    map.insert("A1 249", CardId::A1249Porygon);
    map.insert("A1 250", CardId::A1250Snorlax);
    map.insert("A1 251", CardId::A1251VenusaurEx);
    map.insert("A1 252", CardId::A1252ExeggutorEx);
    map.insert("A1 253", CardId::A1253CharizardEx);
    map.insert("A1 254", CardId::A1254ArcanineEx);
    map.insert("A1 255", CardId::A1255MoltresEx);
    map.insert("A1 256", CardId::A1256BlastoiseEx);
    map.insert("A1 257", CardId::A1257StarmieEx);
    map.insert("A1 258", CardId::A1258ArticunoEx);
    map.insert("A1 259", CardId::A1259PikachuEx);
    map.insert("A1 260", CardId::A1260ZapdosEx);
    map.insert("A1 261", CardId::A1261GengarEx);
    map.insert("A1 262", CardId::A1262MewtwoEx);
    map.insert("A1 263", CardId::A1263MachampEx);
    map.insert("A1 264", CardId::A1264MarowakEx);
    map.insert("A1 265", CardId::A1265WigglytuffEx);
    map.insert("A1 266", CardId::A1266Erika);
    map.insert("A1 267", CardId::A1267Misty);
    map.insert("A1 268", CardId::A1268Blaine);
    map.insert("A1 269", CardId::A1269Koga);
    map.insert("A1 270", CardId::A1270Giovanni);
    map.insert("A1 271", CardId::A1271Brock);
    map.insert("A1 272", CardId::A1272Sabrina);
    map.insert("A1 273", CardId::A1273LtSurge);
    map.insert("A1 274", CardId::A1274MoltresEx);
    map.insert("A1 275", CardId::A1275ArticunoEx);
    map.insert("A1 276", CardId::A1276ZapdosEx);
    map.insert("A1 277", CardId::A1277GengarEx);
    map.insert("A1 278", CardId::A1278MachampEx);
    map.insert("A1 279", CardId::A1279WigglytuffEx);
    map.insert("A1 280", CardId::A1280CharizardEx);
    map.insert("A1 281", CardId::A1281PikachuEx);
    map.insert("A1 282", CardId::A1282MewtwoEx);
    map.insert("A1 283", CardId::A1283Mew);
    map.insert("A1 284", CardId::A1284CharizardEx);
    map.insert("A1 285", CardId::A1285PikachuEx);
    map.insert("A1 286", CardId::A1286MewtwoEx);
    map.insert("A1a 001", CardId::A1a001Exeggcute);
    map.insert("A1a 002", CardId::A1a002Exeggutor);
    map.insert("A1a 003", CardId::A1a003CelebiEx);
    map.insert("A1a 004", CardId::A1a004Snivy);
    map.insert("A1a 005", CardId::A1a005Servine);
    map.insert("A1a 006", CardId::A1a006Serperior);
    map.insert("A1a 007", CardId::A1a007Morelull);
    map.insert("A1a 008", CardId::A1a008Shiinotic);
    map.insert("A1a 009", CardId::A1a009Dhelmise);
    map.insert("A1a 010", CardId::A1a010Ponyta);
    map.insert("A1a 011", CardId::A1a011Rapidash);
    map.insert("A1a 012", CardId::A1a012Magmar);
    map.insert("A1a 013", CardId::A1a013Larvesta);
    map.insert("A1a 014", CardId::A1a014Volcarona);
    map.insert("A1a 015", CardId::A1a015Salandit);
    map.insert("A1a 016", CardId::A1a016Salazzle);
    map.insert("A1a 017", CardId::A1a017Magikarp);
    map.insert("A1a 018", CardId::A1a018GyaradosEx);
    map.insert("A1a 019", CardId::A1a019Vaporeon);
    map.insert("A1a 020", CardId::A1a020Finneon);
    map.insert("A1a 021", CardId::A1a021Lumineon);
    map.insert("A1a 022", CardId::A1a022Chewtle);
    map.insert("A1a 023", CardId::A1a023Drednaw);
    map.insert("A1a 024", CardId::A1a024Cramorant);
    map.insert("A1a 025", CardId::A1a025Pikachu);
    map.insert("A1a 026", CardId::A1a026Raichu);
    map.insert("A1a 027", CardId::A1a027Electabuzz);
    map.insert("A1a 028", CardId::A1a028Joltik);
    map.insert("A1a 029", CardId::A1a029Galvantula);
    map.insert("A1a 030", CardId::A1a030Dedenne);
    map.insert("A1a 031", CardId::A1a031Mew);
    map.insert("A1a 032", CardId::A1a032MewEx);
    map.insert("A1a 033", CardId::A1a033Sigilyph);
    map.insert("A1a 034", CardId::A1a034Elgyem);
    map.insert("A1a 035", CardId::A1a035Beheeyem);
    map.insert("A1a 036", CardId::A1a036Flabebe);
    map.insert("A1a 037", CardId::A1a037Floette);
    map.insert("A1a 038", CardId::A1a038Florges);
    map.insert("A1a 039", CardId::A1a039Swirlix);
    map.insert("A1a 040", CardId::A1a040Slurpuff);
    map.insert("A1a 041", CardId::A1a041Mankey);
    map.insert("A1a 042", CardId::A1a042Primeape);
    map.insert("A1a 043", CardId::A1a043Geodude);
    map.insert("A1a 044", CardId::A1a044Graveler);
    map.insert("A1a 045", CardId::A1a045Golem);
    map.insert("A1a 046", CardId::A1a046AerodactylEx);
    map.insert("A1a 047", CardId::A1a047Marshadow);
    map.insert("A1a 048", CardId::A1a048Stonjourner);
    map.insert("A1a 049", CardId::A1a049Koffing);
    map.insert("A1a 050", CardId::A1a050Weezing);
    map.insert("A1a 051", CardId::A1a051Purrloin);
    map.insert("A1a 052", CardId::A1a052Liepard);
    map.insert("A1a 053", CardId::A1a053Venipede);
    map.insert("A1a 054", CardId::A1a054Whirlipede);
    map.insert("A1a 055", CardId::A1a055Scolipede);
    map.insert("A1a 056", CardId::A1a056Druddigon);
    map.insert("A1a 057", CardId::A1a057Pidgey);
    map.insert("A1a 058", CardId::A1a058Pidgeotto);
    map.insert("A1a 059", CardId::A1a059PidgeotEx);
    map.insert("A1a 060", CardId::A1a060Tauros);
    map.insert("A1a 061", CardId::A1a061Eevee);
    map.insert("A1a 062", CardId::A1a062Chatot);
    map.insert("A1a 063", CardId::A1a063OldAmber);
    map.insert("A1a 064", CardId::A1a064PokemonFlute);
    map.insert("A1a 065", CardId::A1a065MythicalSlab);
    map.insert("A1a 066", CardId::A1a066BuddingExpeditioner);
    map.insert("A1a 067", CardId::A1a067Blue);
    map.insert("A1a 068", CardId::A1a068Leaf);
    map.insert("A1a 069", CardId::A1a069Exeggutor);
    map.insert("A1a 070", CardId::A1a070Serperior);
    map.insert("A1a 071", CardId::A1a071Salandit);
    map.insert("A1a 072", CardId::A1a072Vaporeon);
    map.insert("A1a 073", CardId::A1a073Dedenne);
    map.insert("A1a 074", CardId::A1a074Marshadow);
    map.insert("A1a 075", CardId::A1a075CelebiEx);
    map.insert("A1a 076", CardId::A1a076GyaradosEx);
    map.insert("A1a 077", CardId::A1a077MewEx);
    map.insert("A1a 078", CardId::A1a078AerodactylEx);
    map.insert("A1a 079", CardId::A1a079PidgeotEx);
    map.insert("A1a 080", CardId::A1a080BuddingExpeditioner);
    map.insert("A1a 081", CardId::A1a081Blue);
    map.insert("A1a 082", CardId::A1a082Leaf);
    map.insert("A1a 083", CardId::A1a083MewEx);
    map.insert("A1a 084", CardId::A1a084AerodactylEx);
    map.insert("A1a 085", CardId::A1a085CelebiEx);
    map.insert("A1a 086", CardId::A1a086MewEx);
    map.insert("A2 001", CardId::A2001Oddish);
    map.insert("A2 002", CardId::A2002Gloom);
    map.insert("A2 003", CardId::A2003Bellossom);
    map.insert("A2 004", CardId::A2004Tangela);
    map.insert("A2 005", CardId::A2005Tangrowth);
    map.insert("A2 006", CardId::A2006Yanma);
    map.insert("A2 007", CardId::A2007YanmegaEx);
    map.insert("A2 008", CardId::A2008Roselia);
    map.insert("A2 009", CardId::A2009Roserade);
    map.insert("A2 010", CardId::A2010Turtwig);
    map.insert("A2 011", CardId::A2011Grotle);
    map.insert("A2 012", CardId::A2012Torterra);
    map.insert("A2 013", CardId::A2013Kricketot);
    map.insert("A2 014", CardId::A2014Kricketune);
    map.insert("A2 015", CardId::A2015Burmy);
    map.insert("A2 016", CardId::A2016Wormadam);
    map.insert("A2 017", CardId::A2017Combee);
    map.insert("A2 018", CardId::A2018Vespiquen);
    map.insert("A2 019", CardId::A2019Carnivine);
    map.insert("A2 020", CardId::A2020Leafeon);
    map.insert("A2 021", CardId::A2021MowRotom);
    map.insert("A2 022", CardId::A2022Shaymin);
    map.insert("A2 023", CardId::A2023Magmar);
    map.insert("A2 024", CardId::A2024Magmortar);
    map.insert("A2 025", CardId::A2025Slugma);
    map.insert("A2 026", CardId::A2026Magcargo);
    map.insert("A2 027", CardId::A2027Chimchar);
    map.insert("A2 028", CardId::A2028Monferno);
    map.insert("A2 029", CardId::A2029InfernapeEx);
    map.insert("A2 030", CardId::A2030HeatRotom);
    map.insert("A2 031", CardId::A2031Swinub);
    map.insert("A2 032", CardId::A2032Piloswine);
    map.insert("A2 033", CardId::A2033Mamoswine);
    map.insert("A2 034", CardId::A2034Regice);
    map.insert("A2 035", CardId::A2035Piplup);
    map.insert("A2 036", CardId::A2036Prinplup);
    map.insert("A2 037", CardId::A2037Empoleon);
    map.insert("A2 038", CardId::A2038Buizel);
    map.insert("A2 039", CardId::A2039Floatzel);
    map.insert("A2 040", CardId::A2040Shellos);
    map.insert("A2 041", CardId::A2041Gastrodon);
    map.insert("A2 042", CardId::A2042Finneon);
    map.insert("A2 043", CardId::A2043Lumineon);
    map.insert("A2 044", CardId::A2044Snover);
    map.insert("A2 045", CardId::A2045Abomasnow);
    map.insert("A2 046", CardId::A2046Glaceon);
    map.insert("A2 047", CardId::A2047WashRotom);
    map.insert("A2 048", CardId::A2048FrostRotom);
    map.insert("A2 049", CardId::A2049PalkiaEx);
    map.insert("A2 050", CardId::A2050Manaphy);
    map.insert("A2 051", CardId::A2051Magnemite);
    map.insert("A2 052", CardId::A2052Magneton);
    map.insert("A2 053", CardId::A2053Magnezone);
    map.insert("A2 054", CardId::A2054Voltorb);
    map.insert("A2 055", CardId::A2055Electrode);
    map.insert("A2 056", CardId::A2056Electabuzz);
    map.insert("A2 057", CardId::A2057Electivire);
    map.insert("A2 058", CardId::A2058Shinx);
    map.insert("A2 059", CardId::A2059Luxio);
    map.insert("A2 060", CardId::A2060Luxray);
    map.insert("A2 061", CardId::A2061PachirisuEx);
    map.insert("A2 062", CardId::A2062Rotom);
    map.insert("A2 063", CardId::A2063Togepi);
    map.insert("A2 064", CardId::A2064Togetic);
    map.insert("A2 065", CardId::A2065Togekiss);
    map.insert("A2 066", CardId::A2066Misdreavus);
    map.insert("A2 067", CardId::A2067MismagiusEx);
    map.insert("A2 068", CardId::A2068Ralts);
    map.insert("A2 069", CardId::A2069Kirlia);
    map.insert("A2 070", CardId::A2070Duskull);
    map.insert("A2 071", CardId::A2071Dusclops);
    map.insert("A2 072", CardId::A2072Dusknoir);
    map.insert("A2 073", CardId::A2073Drifloon);
    map.insert("A2 074", CardId::A2074Drifblim);
    map.insert("A2 075", CardId::A2075Uxie);
    map.insert("A2 076", CardId::A2076Mesprit);
    map.insert("A2 077", CardId::A2077Azelf);
    map.insert("A2 078", CardId::A2078Giratina);
    map.insert("A2 079", CardId::A2079Cresselia);
    map.insert("A2 080", CardId::A2080Rhyhorn);
    map.insert("A2 081", CardId::A2081Rhydon);
    map.insert("A2 082", CardId::A2082Rhyperior);
    map.insert("A2 083", CardId::A2083Gligar);
    map.insert("A2 084", CardId::A2084Gliscor);
    map.insert("A2 085", CardId::A2085Hitmontop);
    map.insert("A2 086", CardId::A2086Nosepass);
    map.insert("A2 087", CardId::A2087Regirock);
    map.insert("A2 088", CardId::A2088Cranidos);
    map.insert("A2 089", CardId::A2089Rampardos);
    map.insert("A2 090", CardId::A2090Wormadam);
    map.insert("A2 091", CardId::A2091Riolu);
    map.insert("A2 092", CardId::A2092Lucario);
    map.insert("A2 093", CardId::A2093Hippopotas);
    map.insert("A2 094", CardId::A2094Hippowdon);
    map.insert("A2 095", CardId::A2095GalladeEx);
    map.insert("A2 096", CardId::A2096Murkrow);
    map.insert("A2 097", CardId::A2097Honchkrow);
    map.insert("A2 098", CardId::A2098Sneasel);
    map.insert("A2 099", CardId::A2099WeavileEx);
    map.insert("A2 100", CardId::A2100Poochyena);
    map.insert("A2 101", CardId::A2101Mightyena);
    map.insert("A2 102", CardId::A2102Stunky);
    map.insert("A2 103", CardId::A2103Skuntank);
    map.insert("A2 104", CardId::A2104Spiritomb);
    map.insert("A2 105", CardId::A2105Skorupi);
    map.insert("A2 106", CardId::A2106Drapion);
    map.insert("A2 107", CardId::A2107Croagunk);
    map.insert("A2 108", CardId::A2108Toxicroak);
    map.insert("A2 109", CardId::A2109Darkrai);
    map.insert("A2 110", CardId::A2110DarkraiEx);
    map.insert("A2 111", CardId::A2111Skarmory);
    map.insert("A2 112", CardId::A2112Registeel);
    map.insert("A2 113", CardId::A2113Shieldon);
    map.insert("A2 114", CardId::A2114Bastiodon);
    map.insert("A2 115", CardId::A2115Wormadam);
    map.insert("A2 116", CardId::A2116Bronzor);
    map.insert("A2 117", CardId::A2117Bronzong);
    map.insert("A2 118", CardId::A2118Probopass);
    map.insert("A2 119", CardId::A2119DialgaEx);
    map.insert("A2 120", CardId::A2120Heatran);
    map.insert("A2 121", CardId::A2121Gible);
    map.insert("A2 122", CardId::A2122Gabite);
    map.insert("A2 123", CardId::A2123Garchomp);
    map.insert("A2 124", CardId::A2124Lickitung);
    map.insert("A2 125", CardId::A2125LickilickyEx);
    map.insert("A2 126", CardId::A2126Eevee);
    map.insert("A2 127", CardId::A2127Porygon);
    map.insert("A2 128", CardId::A2128Porygon2);
    map.insert("A2 129", CardId::A2129PorygonZ);
    map.insert("A2 130", CardId::A2130Aipom);
    map.insert("A2 131", CardId::A2131Ambipom);
    map.insert("A2 132", CardId::A2132Starly);
    map.insert("A2 133", CardId::A2133Staravia);
    map.insert("A2 134", CardId::A2134Staraptor);
    map.insert("A2 135", CardId::A2135Bidoof);
    map.insert("A2 136", CardId::A2136Bibarel);
    map.insert("A2 137", CardId::A2137Buneary);
    map.insert("A2 138", CardId::A2138Lopunny);
    map.insert("A2 139", CardId::A2139Glameow);
    map.insert("A2 140", CardId::A2140Purugly);
    map.insert("A2 141", CardId::A2141Chatot);
    map.insert("A2 142", CardId::A2142FanRotom);
    map.insert("A2 143", CardId::A2143Regigigas);
    map.insert("A2 144", CardId::A2144SkullFossil);
    map.insert("A2 145", CardId::A2145ArmorFossil);
    map.insert("A2 146", CardId::A2146PokemonCommunication);
    map.insert("A2 147", CardId::A2147GiantCape);
    map.insert("A2 148", CardId::A2148RockyHelmet);
    map.insert("A2 149", CardId::A2149LumBerry);
    map.insert("A2 150", CardId::A2150Cyrus);
    map.insert("A2 151", CardId::A2151TeamGalacticGrunt);
    map.insert("A2 152", CardId::A2152Cynthia);
    map.insert("A2 153", CardId::A2153Volkner);
    map.insert("A2 154", CardId::A2154Dawn);
    map.insert("A2 155", CardId::A2155Mars);
    map.insert("A2 156", CardId::A2156Tangrowth);
    map.insert("A2 157", CardId::A2157Combee);
    map.insert("A2 158", CardId::A2158Carnivine);
    map.insert("A2 159", CardId::A2159Shaymin);
    map.insert("A2 160", CardId::A2160Mamoswine);
    map.insert("A2 161", CardId::A2161Gastrodon);
    map.insert("A2 162", CardId::A2162Manaphy);
    map.insert("A2 163", CardId::A2163Shinx);
    map.insert("A2 164", CardId::A2164Rotom);
    map.insert("A2 165", CardId::A2165Drifloon);
    map.insert("A2 166", CardId::A2166Mesprit);
    map.insert("A2 167", CardId::A2167Giratina);
    map.insert("A2 168", CardId::A2168Cresselia);
    map.insert("A2 169", CardId::A2169Rhyperior);
    map.insert("A2 170", CardId::A2170Lucario);
    map.insert("A2 171", CardId::A2171Hippopotas);
    map.insert("A2 172", CardId::A2172Spiritomb);
    map.insert("A2 173", CardId::A2173Croagunk);
    map.insert("A2 174", CardId::A2174Heatran);
    map.insert("A2 175", CardId::A2175Garchomp);
    map.insert("A2 176", CardId::A2176Staraptor);
    map.insert("A2 177", CardId::A2177Bidoof);
    map.insert("A2 178", CardId::A2178Glameow);
    map.insert("A2 179", CardId::A2179Regigigas);
    map.insert("A2 180", CardId::A2180YanmegaEx);
    map.insert("A2 181", CardId::A2181InfernapeEx);
    map.insert("A2 182", CardId::A2182PalkiaEx);
    map.insert("A2 183", CardId::A2183PachirisuEx);
    map.insert("A2 184", CardId::A2184MismagiusEx);
    map.insert("A2 185", CardId::A2185GalladeEx);
    map.insert("A2 186", CardId::A2186WeavileEx);
    map.insert("A2 187", CardId::A2187DarkraiEx);
    map.insert("A2 188", CardId::A2188DialgaEx);
    map.insert("A2 189", CardId::A2189LickilickyEx);
    map.insert("A2 190", CardId::A2190Cyrus);
    map.insert("A2 191", CardId::A2191TeamGalacticGrunt);
    map.insert("A2 192", CardId::A2192Cynthia);
    map.insert("A2 193", CardId::A2193Volkner);
    map.insert("A2 194", CardId::A2194Dawn);
    map.insert("A2 195", CardId::A2195Mars);
    map.insert("A2 196", CardId::A2196YanmegaEx);
    map.insert("A2 197", CardId::A2197InfernapeEx);
    map.insert("A2 198", CardId::A2198PachirisuEx);
    map.insert("A2 199", CardId::A2199MismagiusEx);
    map.insert("A2 200", CardId::A2200GalladeEx);
    map.insert("A2 201", CardId::A2201WeavileEx);
    map.insert("A2 202", CardId::A2202DarkraiEx);
    map.insert("A2 203", CardId::A2203LickilickyEx);
    map.insert("A2 204", CardId::A2204PalkiaEx);
    map.insert("A2 205", CardId::A2205DialgaEx);
    map.insert("A2 206", CardId::A2206PalkiaEx);
    map.insert("A2 207", CardId::A2207DialgaEx);
    map.insert("A2a 001", CardId::A2a001Heracross);
    map.insert("A2a 002", CardId::A2a002Burmy);
    map.insert("A2a 003", CardId::A2a003Mothim);
    map.insert("A2a 004", CardId::A2a004Combee);
    map.insert("A2a 005", CardId::A2a005Vespiquen);
    map.insert("A2a 006", CardId::A2a006Cherubi);
    map.insert("A2a 007", CardId::A2a007Cherrim);
    map.insert("A2a 008", CardId::A2a008Cherrim);
    map.insert("A2a 009", CardId::A2a009Carnivine);
    map.insert("A2a 010", CardId::A2a010LeafeonEx);
    map.insert("A2a 011", CardId::A2a011Houndour);
    map.insert("A2a 012", CardId::A2a012Houndoom);
    map.insert("A2a 013", CardId::A2a013Heatran);
    map.insert("A2a 014", CardId::A2a014Marill);
    map.insert("A2a 015", CardId::A2a015Azumarill);
    map.insert("A2a 016", CardId::A2a016Barboach);
    map.insert("A2a 017", CardId::A2a017Whiscash);
    map.insert("A2a 018", CardId::A2a018Snorunt);
    map.insert("A2a 019", CardId::A2a019Froslass);
    map.insert("A2a 020", CardId::A2a020Snover);
    map.insert("A2a 021", CardId::A2a021Abomasnow);
    map.insert("A2a 022", CardId::A2a022GlaceonEx);
    map.insert("A2a 023", CardId::A2a023OriginFormePalkia);
    map.insert("A2a 024", CardId::A2a024Phione);
    map.insert("A2a 025", CardId::A2a025Pikachu);
    map.insert("A2a 026", CardId::A2a026Raichu);
    map.insert("A2a 027", CardId::A2a027Electrike);
    map.insert("A2a 028", CardId::A2a028Manectric);
    map.insert("A2a 029", CardId::A2a029Clefairy);
    map.insert("A2a 030", CardId::A2a030Clefable);
    map.insert("A2a 031", CardId::A2a031Gastly);
    map.insert("A2a 032", CardId::A2a032Haunter);
    map.insert("A2a 033", CardId::A2a033Gengar);
    map.insert("A2a 034", CardId::A2a034Unown);
    map.insert("A2a 035", CardId::A2a035Rotom);
    map.insert("A2a 036", CardId::A2a036Sudowoodo);
    map.insert("A2a 037", CardId::A2a037Phanpy);
    map.insert("A2a 038", CardId::A2a038Donphan);
    map.insert("A2a 039", CardId::A2a039Larvitar);
    map.insert("A2a 040", CardId::A2a040Pupitar);
    map.insert("A2a 041", CardId::A2a041Tyranitar);
    map.insert("A2a 042", CardId::A2a042Nosepass);
    map.insert("A2a 043", CardId::A2a043Meditite);
    map.insert("A2a 044", CardId::A2a044Medicham);
    map.insert("A2a 045", CardId::A2a045Gible);
    map.insert("A2a 046", CardId::A2a046Gabite);
    map.insert("A2a 047", CardId::A2a047GarchompEx);
    map.insert("A2a 048", CardId::A2a048Zubat);
    map.insert("A2a 049", CardId::A2a049Golbat);
    map.insert("A2a 050", CardId::A2a050Crobat);
    map.insert("A2a 051", CardId::A2a051Croagunk);
    map.insert("A2a 052", CardId::A2a052Toxicroak);
    map.insert("A2a 053", CardId::A2a053Magnemite);
    map.insert("A2a 054", CardId::A2a054Magneton);
    map.insert("A2a 055", CardId::A2a055Magnezone);
    map.insert("A2a 056", CardId::A2a056Mawile);
    map.insert("A2a 057", CardId::A2a057ProbopassEx);
    map.insert("A2a 058", CardId::A2a058Bronzor);
    map.insert("A2a 059", CardId::A2a059Bronzong);
    map.insert("A2a 060", CardId::A2a060OriginFormeDialga);
    map.insert("A2a 061", CardId::A2a061Giratina);
    map.insert("A2a 062", CardId::A2a062Eevee);
    map.insert("A2a 063", CardId::A2a063Snorlax);
    map.insert("A2a 064", CardId::A2a064Hoothoot);
    map.insert("A2a 065", CardId::A2a065Noctowl);
    map.insert("A2a 066", CardId::A2a066Starly);
    map.insert("A2a 067", CardId::A2a067Staravia);
    map.insert("A2a 068", CardId::A2a068Staraptor);
    map.insert("A2a 069", CardId::A2a069Shaymin);
    map.insert("A2a 070", CardId::A2a070Arceus);
    map.insert("A2a 071", CardId::A2a071ArceusEx);
    map.insert("A2a 072", CardId::A2a072Irida);
    map.insert("A2a 073", CardId::A2a073CelesticTownElder);
    map.insert("A2a 074", CardId::A2a074Barry);
    map.insert("A2a 075", CardId::A2a075Adaman);
    map.insert("A2a 076", CardId::A2a076Houndoom);
    map.insert("A2a 077", CardId::A2a077Marill);
    map.insert("A2a 078", CardId::A2a078Unown);
    map.insert("A2a 079", CardId::A2a079Sudowoodo);
    map.insert("A2a 080", CardId::A2a080Magnemite);
    map.insert("A2a 081", CardId::A2a081Shaymin);
    map.insert("A2a 082", CardId::A2a082LeafeonEx);
    map.insert("A2a 083", CardId::A2a083GlaceonEx);
    map.insert("A2a 084", CardId::A2a084GarchompEx);
    map.insert("A2a 085", CardId::A2a085ProbopassEx);
    map.insert("A2a 086", CardId::A2a086ArceusEx);
    map.insert("A2a 087", CardId::A2a087Irida);
    map.insert("A2a 088", CardId::A2a088CelesticTownElder);
    map.insert("A2a 089", CardId::A2a089Barry);
    map.insert("A2a 090", CardId::A2a090Adaman);
    map.insert("A2a 091", CardId::A2a091LeafeonEx);
    map.insert("A2a 092", CardId::A2a092GlaceonEx);
    map.insert("A2a 093", CardId::A2a093GarchompEx);
    map.insert("A2a 094", CardId::A2a094ProbopassEx);
    map.insert("A2a 095", CardId::A2a095ArceusEx);
    map.insert("A2a 096", CardId::A2a096ArceusEx);
    map.insert("A2b 001", CardId::A2b001Weedle);
    map.insert("A2b 002", CardId::A2b002Kakuna);
    map.insert("A2b 003", CardId::A2b003BeedrillEx);
    map.insert("A2b 004", CardId::A2b004Pinsir);
    map.insert("A2b 005", CardId::A2b005Sprigatito);
    map.insert("A2b 006", CardId::A2b006Floragato);
    map.insert("A2b 007", CardId::A2b007Meowscarada);
    map.insert("A2b 008", CardId::A2b008Charmander);
    map.insert("A2b 009", CardId::A2b009Charmeleon);
    map.insert("A2b 010", CardId::A2b010CharizardEx);
    map.insert("A2b 011", CardId::A2b011Magmar);
    map.insert("A2b 012", CardId::A2b012Magmortar);
    map.insert("A2b 013", CardId::A2b013PaldeanTauros);
    map.insert("A2b 014", CardId::A2b014Tentacool);
    map.insert("A2b 015", CardId::A2b015Tentacruel);
    map.insert("A2b 016", CardId::A2b016Buizel);
    map.insert("A2b 017", CardId::A2b017Floatzel);
    map.insert("A2b 018", CardId::A2b018Wiglett);
    map.insert("A2b 019", CardId::A2b019WugtrioEx);
    map.insert("A2b 020", CardId::A2b020Dondozo);
    map.insert("A2b 021", CardId::A2b021Tatsugiri);
    map.insert("A2b 022", CardId::A2b022PikachuEx);
    map.insert("A2b 023", CardId::A2b023Voltorb);
    map.insert("A2b 024", CardId::A2b024Electrode);
    map.insert("A2b 025", CardId::A2b025Pachirisu);
    map.insert("A2b 026", CardId::A2b026Pawmi);
    map.insert("A2b 027", CardId::A2b027Pawmo);
    map.insert("A2b 028", CardId::A2b028Pawmot);
    map.insert("A2b 029", CardId::A2b029Abra);
    map.insert("A2b 030", CardId::A2b030Kadabra);
    map.insert("A2b 031", CardId::A2b031Alakazam);
    map.insert("A2b 032", CardId::A2b032MrMime);
    map.insert("A2b 033", CardId::A2b033Drifloon);
    map.insert("A2b 034", CardId::A2b034Drifblim);
    map.insert("A2b 035", CardId::A2b035GiratinaEx);
    map.insert("A2b 036", CardId::A2b036Gimmighoul);
    map.insert("A2b 037", CardId::A2b037Machop);
    map.insert("A2b 038", CardId::A2b038Machoke);
    map.insert("A2b 039", CardId::A2b039Machamp);
    map.insert("A2b 040", CardId::A2b040Hitmonlee);
    map.insert("A2b 041", CardId::A2b041Hitmonchan);
    map.insert("A2b 042", CardId::A2b042Riolu);
    map.insert("A2b 043", CardId::A2b043LucarioEx);
    map.insert("A2b 044", CardId::A2b044Flamigo);
    map.insert("A2b 045", CardId::A2b045Ekans);
    map.insert("A2b 046", CardId::A2b046Arbok);
    map.insert("A2b 047", CardId::A2b047PaldeanWooper);
    map.insert("A2b 048", CardId::A2b048PaldeanClodsireEx);
    map.insert("A2b 049", CardId::A2b049Spiritomb);
    map.insert("A2b 050", CardId::A2b050Shroodle);
    map.insert("A2b 051", CardId::A2b051Grafaiai);
    map.insert("A2b 052", CardId::A2b052Tinkatink);
    map.insert("A2b 053", CardId::A2b053Tinkatuff);
    map.insert("A2b 054", CardId::A2b054TinkatonEx);
    map.insert("A2b 055", CardId::A2b055Varoom);
    map.insert("A2b 056", CardId::A2b056Revavroom);
    map.insert("A2b 057", CardId::A2b057Gholdengo);
    map.insert("A2b 058", CardId::A2b058Rattata);
    map.insert("A2b 059", CardId::A2b059Raticate);
    map.insert("A2b 060", CardId::A2b060Jigglypuff);
    map.insert("A2b 061", CardId::A2b061Wigglytuff);
    map.insert("A2b 062", CardId::A2b062Lickitung);
    map.insert("A2b 063", CardId::A2b063Lickilicky);
    map.insert("A2b 064", CardId::A2b064Bidoof);
    map.insert("A2b 065", CardId::A2b065BibarelEx);
    map.insert("A2b 066", CardId::A2b066Buneary);
    map.insert("A2b 067", CardId::A2b067Lopunny);
    map.insert("A2b 068", CardId::A2b068Cyclizar);
    map.insert("A2b 069", CardId::A2b069Iono);
    map.insert("A2b 070", CardId::A2b070PokemonCenterLady);
    map.insert("A2b 071", CardId::A2b071Red);
    map.insert("A2b 072", CardId::A2b072TeamRocketGrunt);
    map.insert("A2b 073", CardId::A2b073Meowscarada);
    map.insert("A2b 074", CardId::A2b074Buizel);
    map.insert("A2b 075", CardId::A2b075Tatsugiri);
    map.insert("A2b 076", CardId::A2b076Grafaiai);
    map.insert("A2b 077", CardId::A2b077Gholdengo);
    map.insert("A2b 078", CardId::A2b078Wigglytuff);
    map.insert("A2b 079", CardId::A2b079BeedrillEx);
    map.insert("A2b 080", CardId::A2b080CharizardEx);
    map.insert("A2b 081", CardId::A2b081WugtrioEx);
    map.insert("A2b 082", CardId::A2b082PikachuEx);
    map.insert("A2b 083", CardId::A2b083GiratinaEx);
    map.insert("A2b 084", CardId::A2b084LucarioEx);
    map.insert("A2b 085", CardId::A2b085PaldeanClodsireEx);
    map.insert("A2b 086", CardId::A2b086TinkatonEx);
    map.insert("A2b 087", CardId::A2b087BibarelEx);
    map.insert("A2b 088", CardId::A2b088Iono);
    map.insert("A2b 089", CardId::A2b089PokemonCenterLady);
    map.insert("A2b 090", CardId::A2b090Red);
    map.insert("A2b 091", CardId::A2b091TeamRocketGrunt);
    map.insert("A2b 092", CardId::A2b092PikachuEx);
    map.insert("A2b 093", CardId::A2b093PaldeanClodsireEx);
    map.insert("A2b 094", CardId::A2b094TinkatonEx);
    map.insert("A2b 095", CardId::A2b095BibarelEx);
    map.insert("A2b 096", CardId::A2b096GiratinaEx);
    map.insert("A2b 097", CardId::A2b097Weedle);
    map.insert("A2b 098", CardId::A2b098Kakuna);
    map.insert("A2b 099", CardId::A2b099Charmander);
    map.insert("A2b 100", CardId::A2b100Charmeleon);
    map.insert("A2b 101", CardId::A2b101Wiglett);
    map.insert("A2b 102", CardId::A2b102Dondozo);
    map.insert("A2b 103", CardId::A2b103Pachirisu);
    map.insert("A2b 104", CardId::A2b104Riolu);
    map.insert("A2b 105", CardId::A2b105Varoom);
    map.insert("A2b 106", CardId::A2b106Revavroom);
    map.insert("A2b 107", CardId::A2b107BeedrillEx);
    map.insert("A2b 108", CardId::A2b108CharizardEx);
    map.insert("A2b 109", CardId::A2b109WugtrioEx);
    map.insert("A2b 110", CardId::A2b110LucarioEx);
    map.insert("A2b 111", CardId::A2b111PokeBall);
    map.insert("A3 001", CardId::A3001Exeggcute);
    map.insert("A3 002", CardId::A3002AlolanExeggutor);
    map.insert("A3 003", CardId::A3003Surskit);
    map.insert("A3 004", CardId::A3004Masquerain);
    map.insert("A3 005", CardId::A3005Maractus);
    map.insert("A3 006", CardId::A3006Karrablast);
    map.insert("A3 007", CardId::A3007Phantump);
    map.insert("A3 008", CardId::A3008Trevenant);
    map.insert("A3 009", CardId::A3009Rowlet);
    map.insert("A3 010", CardId::A3010Rowlet);
    map.insert("A3 011", CardId::A3011Dartrix);
    map.insert("A3 012", CardId::A3012DecidueyeEx);
    map.insert("A3 013", CardId::A3013Grubbin);
    map.insert("A3 014", CardId::A3014Fomantis);
    map.insert("A3 015", CardId::A3015Lurantis);
    map.insert("A3 016", CardId::A3016Morelull);
    map.insert("A3 017", CardId::A3017Shiinotic);
    map.insert("A3 018", CardId::A3018Bounsweet);
    map.insert("A3 019", CardId::A3019Steenee);
    map.insert("A3 020", CardId::A3020Tsareena);
    map.insert("A3 021", CardId::A3021Wimpod);
    map.insert("A3 022", CardId::A3022Golisopod);
    map.insert("A3 023", CardId::A3023DhelmiseEx);
    map.insert("A3 024", CardId::A3024TapuBulu);
    map.insert("A3 025", CardId::A3025Growlithe);
    map.insert("A3 026", CardId::A3026Arcanine);
    map.insert("A3 027", CardId::A3027AlolanMarowak);
    map.insert("A3 028", CardId::A3028Fletchinder);
    map.insert("A3 029", CardId::A3029Talonflame);
    map.insert("A3 030", CardId::A3030Litten);
    map.insert("A3 031", CardId::A3031Litten);
    map.insert("A3 032", CardId::A3032Torracat);
    map.insert("A3 033", CardId::A3033IncineroarEx);
    map.insert("A3 034", CardId::A3034Oricorio);
    map.insert("A3 035", CardId::A3035Salandit);
    map.insert("A3 036", CardId::A3036Salazzle);
    map.insert("A3 037", CardId::A3037Turtonator);
    map.insert("A3 038", CardId::A3038AlolanSandshrew);
    map.insert("A3 039", CardId::A3039AlolanSandslash);
    map.insert("A3 040", CardId::A3040AlolanVulpix);
    map.insert("A3 041", CardId::A3041AlolanNinetales);
    map.insert("A3 042", CardId::A3042Shellder);
    map.insert("A3 043", CardId::A3043Cloyster);
    map.insert("A3 044", CardId::A3044Lapras);
    map.insert("A3 045", CardId::A3045Popplio);
    map.insert("A3 046", CardId::A3046Popplio);
    map.insert("A3 047", CardId::A3047Brionne);
    map.insert("A3 048", CardId::A3048Primarina);
    map.insert("A3 049", CardId::A3049CrabominableEx);
    map.insert("A3 050", CardId::A3050Wishiwashi);
    map.insert("A3 051", CardId::A3051WishiwashiEx);
    map.insert("A3 052", CardId::A3052Dewpider);
    map.insert("A3 053", CardId::A3053Araquanid);
    map.insert("A3 054", CardId::A3054Pyukumuku);
    map.insert("A3 055", CardId::A3055Bruxish);
    map.insert("A3 056", CardId::A3056TapuFini);
    map.insert("A3 057", CardId::A3057Pikachu);
    map.insert("A3 058", CardId::A3058AlolanRaichuEx);
    map.insert("A3 059", CardId::A3059AlolanGeodude);
    map.insert("A3 060", CardId::A3060AlolanGraveler);
    map.insert("A3 061", CardId::A3061AlolanGolem);
    map.insert("A3 062", CardId::A3062Helioptile);
    map.insert("A3 063", CardId::A3063Heliolisk);
    map.insert("A3 064", CardId::A3064Charjabug);
    map.insert("A3 065", CardId::A3065Vikavolt);
    map.insert("A3 066", CardId::A3066Oricorio);
    map.insert("A3 067", CardId::A3067Togedemaru);
    map.insert("A3 068", CardId::A3068TapuKoko);
    map.insert("A3 069", CardId::A3069MrMime);
    map.insert("A3 070", CardId::A3070Sableye);
    map.insert("A3 071", CardId::A3071Spoink);
    map.insert("A3 072", CardId::A3072Grumpig);
    map.insert("A3 073", CardId::A3073Lunatone);
    map.insert("A3 074", CardId::A3074Shuppet);
    map.insert("A3 075", CardId::A3075Banette);
    map.insert("A3 076", CardId::A3076Oricorio);
    map.insert("A3 077", CardId::A3077Oricorio);
    map.insert("A3 078", CardId::A3078Cutiefly);
    map.insert("A3 079", CardId::A3079Ribombee);
    map.insert("A3 080", CardId::A3080Comfey);
    map.insert("A3 081", CardId::A3081Sandygast);
    map.insert("A3 082", CardId::A3082Palossand);
    map.insert("A3 083", CardId::A3083Mimikyu);
    map.insert("A3 084", CardId::A3084TapuLele);
    map.insert("A3 085", CardId::A3085Cosmog);
    map.insert("A3 086", CardId::A3086Cosmoem);
    map.insert("A3 087", CardId::A3087LunalaEx);
    map.insert("A3 088", CardId::A3088Necrozma);
    map.insert("A3 089", CardId::A3089Cubone);
    map.insert("A3 090", CardId::A3090Makuhita);
    map.insert("A3 091", CardId::A3091Hariyama);
    map.insert("A3 092", CardId::A3092Solrock);
    map.insert("A3 093", CardId::A3093Drilbur);
    map.insert("A3 094", CardId::A3094Timburr);
    map.insert("A3 095", CardId::A3095Gurdurr);
    map.insert("A3 096", CardId::A3096Conkeldurr);
    map.insert("A3 097", CardId::A3097Crabrawler);
    map.insert("A3 098", CardId::A3098Rockruff);
    map.insert("A3 099", CardId::A3099Rockruff);
    map.insert("A3 100", CardId::A3100Lycanroc);
    map.insert("A3 101", CardId::A3101Lycanroc);
    map.insert("A3 102", CardId::A3102Mudbray);
    map.insert("A3 103", CardId::A3103Mudsdale);
    map.insert("A3 104", CardId::A3104PassimianEx);
    map.insert("A3 105", CardId::A3105Minior);
    map.insert("A3 106", CardId::A3106AlolanRattata);
    map.insert("A3 107", CardId::A3107AlolanRaticate);
    map.insert("A3 108", CardId::A3108AlolanMeowth);
    map.insert("A3 109", CardId::A3109AlolanPersian);
    map.insert("A3 110", CardId::A3110AlolanGrimer);
    map.insert("A3 111", CardId::A3111AlolanMukEx);
    map.insert("A3 112", CardId::A3112Absol);
    map.insert("A3 113", CardId::A3113Trubbish);
    map.insert("A3 114", CardId::A3114Garbodor);
    map.insert("A3 115", CardId::A3115Mareanie);
    map.insert("A3 116", CardId::A3116ToxapEx);
    map.insert("A3 117", CardId::A3117AlolanDiglett);
    map.insert("A3 118", CardId::A3118AlolanDugtrio);
    map.insert("A3 119", CardId::A3119Excadrill);
    map.insert("A3 120", CardId::A3120Escavalier);
    map.insert("A3 121", CardId::A3121Klefki);
    map.insert("A3 122", CardId::A3122SolgaleoEx);
    map.insert("A3 123", CardId::A3123Magearna);
    map.insert("A3 124", CardId::A3124Drampa);
    map.insert("A3 125", CardId::A3125Jangmoo);
    map.insert("A3 126", CardId::A3126Hakamoo);
    map.insert("A3 127", CardId::A3127Kommoo);
    map.insert("A3 128", CardId::A3128Tauros);
    map.insert("A3 129", CardId::A3129Skitty);
    map.insert("A3 130", CardId::A3130Delcatty);
    map.insert("A3 131", CardId::A3131Fletchling);
    map.insert("A3 132", CardId::A3132Hawlucha);
    map.insert("A3 133", CardId::A3133Pikipek);
    map.insert("A3 134", CardId::A3134Trumbeak);
    map.insert("A3 135", CardId::A3135Toucannon);
    map.insert("A3 136", CardId::A3136Yungoos);
    map.insert("A3 137", CardId::A3137Gumshoos);
    map.insert("A3 138", CardId::A3138Stufful);
    map.insert("A3 139", CardId::A3139Bewear);
    map.insert("A3 140", CardId::A3140Oranguru);
    map.insert("A3 141", CardId::A3141Komala);
    map.insert("A3 142", CardId::A3142BigMalasada);
    map.insert("A3 143", CardId::A3143FishingNet);
    map.insert("A3 144", CardId::A3144RareCandy);
    map.insert("A3 145", CardId::A3145RotomDEx);
    map.insert("A3 146", CardId::A3146PoisonBarb);
    map.insert("A3 147", CardId::A3147LeafCape);
    map.insert("A3 148", CardId::A3148Acerola);
    map.insert("A3 149", CardId::A3149Ilima);
    map.insert("A3 150", CardId::A3150Kiawe);
    map.insert("A3 151", CardId::A3151Guzma);
    map.insert("A3 152", CardId::A3152Lana);
    map.insert("A3 153", CardId::A3153Sophocles);
    map.insert("A3 154", CardId::A3154Mallow);
    map.insert("A3 155", CardId::A3155Lillie);
    map.insert("A3 156", CardId::A3156AlolanExeggutor);
    map.insert("A3 157", CardId::A3157Morelull);
    map.insert("A3 158", CardId::A3158Tsareena);
    map.insert("A3 159", CardId::A3159TapuBulu);
    map.insert("A3 160", CardId::A3160AlolanMarowak);
    map.insert("A3 161", CardId::A3161Turtonator);
    map.insert("A3 162", CardId::A3162AlolanVulpix);
    map.insert("A3 163", CardId::A3163Pyukumuku);
    map.insert("A3 164", CardId::A3164TapuFini);
    map.insert("A3 165", CardId::A3165Oricorio);
    map.insert("A3 166", CardId::A3166TapuKoko);
    map.insert("A3 167", CardId::A3167Cutiefly);
    map.insert("A3 168", CardId::A3168Comfey);
    map.insert("A3 169", CardId::A3169Sandygast);
    map.insert("A3 170", CardId::A3170TapuLele);
    map.insert("A3 171", CardId::A3171Cosmog);
    map.insert("A3 172", CardId::A3172Rockruff);
    map.insert("A3 173", CardId::A3173Mudsdale);
    map.insert("A3 174", CardId::A3174Minior);
    map.insert("A3 175", CardId::A3175Magearna);
    map.insert("A3 176", CardId::A3176Drampa);
    map.insert("A3 177", CardId::A3177Pikipek);
    map.insert("A3 178", CardId::A3178Bewear);
    map.insert("A3 179", CardId::A3179Komala);
    map.insert("A3 180", CardId::A3180DecidueyeEx);
    map.insert("A3 181", CardId::A3181DhelmiseEx);
    map.insert("A3 182", CardId::A3182IncineroarEx);
    map.insert("A3 183", CardId::A3183CrabominableEx);
    map.insert("A3 184", CardId::A3184WishiwashiEx);
    map.insert("A3 185", CardId::A3185AlolanRaichuEx);
    map.insert("A3 186", CardId::A3186LunalaEx);
    map.insert("A3 187", CardId::A3187PassimianEx);
    map.insert("A3 188", CardId::A3188AlolanMukEx);
    map.insert("A3 189", CardId::A3189SolgaleoEx);
    map.insert("A3 190", CardId::A3190Acerola);
    map.insert("A3 191", CardId::A3191Ilima);
    map.insert("A3 192", CardId::A3192Kiawe);
    map.insert("A3 193", CardId::A3193Guzma);
    map.insert("A3 194", CardId::A3194Lana);
    map.insert("A3 195", CardId::A3195Sophocles);
    map.insert("A3 196", CardId::A3196Mallow);
    map.insert("A3 197", CardId::A3197Lillie);
    map.insert("A3 198", CardId::A3198DecidueyeEx);
    map.insert("A3 199", CardId::A3199DhelmiseEx);
    map.insert("A3 200", CardId::A3200IncineroarEx);
    map.insert("A3 201", CardId::A3201CrabominableEx);
    map.insert("A3 202", CardId::A3202WishiwashiEx);
    map.insert("A3 203", CardId::A3203AlolanRaichuEx);
    map.insert("A3 204", CardId::A3204LunalaEx);
    map.insert("A3 205", CardId::A3205PassimianEx);
    map.insert("A3 206", CardId::A3206AlolanMukEx);
    map.insert("A3 207", CardId::A3207SolgaleoEx);
    map.insert("A3 208", CardId::A3208Guzma);
    map.insert("A3 209", CardId::A3209Lillie);
    map.insert("A3 210", CardId::A3210Bulbasaur);
    map.insert("A3 211", CardId::A3211Ivysaur);
    map.insert("A3 212", CardId::A3212Venusaur);
    map.insert("A3 213", CardId::A3213Exeggcute);
    map.insert("A3 214", CardId::A3214Exeggutor);
    map.insert("A3 215", CardId::A3215Squirtle);
    map.insert("A3 216", CardId::A3216Wartortle);
    map.insert("A3 217", CardId::A3217Blastoise);
    map.insert("A3 218", CardId::A3218Staryu);
    map.insert("A3 219", CardId::A3219Starmie);
    map.insert("A3 220", CardId::A3220Gastly);
    map.insert("A3 221", CardId::A3221Haunter);
    map.insert("A3 222", CardId::A3222Gengar);
    map.insert("A3 223", CardId::A3223Machop);
    map.insert("A3 224", CardId::A3224Machoke);
    map.insert("A3 225", CardId::A3225Machamp);
    map.insert("A3 226", CardId::A3226Cubone);
    map.insert("A3 227", CardId::A3227Marowak);
    map.insert("A3 228", CardId::A3228Jigglypuff);
    map.insert("A3 229", CardId::A3229Wigglytuff);
    map.insert("A3 230", CardId::A3230VenusaurEx);
    map.insert("A3 231", CardId::A3231ExeggutorEx);
    map.insert("A3 232", CardId::A3232BlastoiseEx);
    map.insert("A3 233", CardId::A3233StarmieEx);
    map.insert("A3 234", CardId::A3234GengarEx);
    map.insert("A3 235", CardId::A3235MachampEx);
    map.insert("A3 236", CardId::A3236MarowakEx);
    map.insert("A3 237", CardId::A3237WigglytuffEx);
    map.insert("A3 238", CardId::A3238LunalaEx);
    map.insert("A3 239", CardId::A3239SolgaleoEx);
    map.insert("A3a 001", CardId::A3a001Petilil);
    map.insert("A3a 002", CardId::A3a002Lilligant);
    map.insert("A3a 003", CardId::A3a003Rowlet);
    map.insert("A3a 004", CardId::A3a004Dartrix);
    map.insert("A3a 005", CardId::A3a005Decidueye);
    map.insert("A3a 006", CardId::A3a006BuzzwoleEx);
    map.insert("A3a 007", CardId::A3a007Pheromosa);
    map.insert("A3a 008", CardId::A3a008Kartana);
    map.insert("A3a 009", CardId::A3a009Blacephalon);
    map.insert("A3a 010", CardId::A3a010Mantine);
    map.insert("A3a 011", CardId::A3a011Carvanha);
    map.insert("A3a 012", CardId::A3a012Sharpedo);
    map.insert("A3a 013", CardId::A3a013Shinx);
    map.insert("A3a 014", CardId::A3a014Luxio);
    map.insert("A3a 015", CardId::A3a015Luxray);
    map.insert("A3a 016", CardId::A3a016Blitzle);
    map.insert("A3a 017", CardId::A3a017Zebstrika);
    map.insert("A3a 018", CardId::A3a018Emolga);
    map.insert("A3a 019", CardId::A3a019TapuKokoEx);
    map.insert("A3a 020", CardId::A3a020Xurkitree);
    map.insert("A3a 021", CardId::A3a021Zeraora);
    map.insert("A3a 022", CardId::A3a022Clefairy);
    map.insert("A3a 023", CardId::A3a023Clefable);
    map.insert("A3a 024", CardId::A3a024Phantump);
    map.insert("A3a 025", CardId::A3a025Trevenant);
    map.insert("A3a 026", CardId::A3a026Morelull);
    map.insert("A3a 027", CardId::A3a027Shiinotic);
    map.insert("A3a 028", CardId::A3a028Meditite);
    map.insert("A3a 029", CardId::A3a029Medicham);
    map.insert("A3a 030", CardId::A3a030Baltoy);
    map.insert("A3a 031", CardId::A3a031Claydol);
    map.insert("A3a 032", CardId::A3a032Rockruff);
    map.insert("A3a 033", CardId::A3a033LycanrocEx);
    map.insert("A3a 034", CardId::A3a034Passimian);
    map.insert("A3a 035", CardId::A3a035Sandygast);
    map.insert("A3a 036", CardId::A3a036Palossand);
    map.insert("A3a 037", CardId::A3a037AlolanMeowth);
    map.insert("A3a 038", CardId::A3a038AlolanPersian);
    map.insert("A3a 039", CardId::A3a039Sandile);
    map.insert("A3a 040", CardId::A3a040Krokorok);
    map.insert("A3a 041", CardId::A3a041Krookodile);
    map.insert("A3a 042", CardId::A3a042Nihilego);
    map.insert("A3a 043", CardId::A3a043GuzzlordEx);
    map.insert("A3a 044", CardId::A3a044Poipole);
    map.insert("A3a 045", CardId::A3a045Naganadel);
    map.insert("A3a 046", CardId::A3a046AlolanDiglett);
    map.insert("A3a 047", CardId::A3a047AlolanDugtrioEx);
    map.insert("A3a 048", CardId::A3a048Aron);
    map.insert("A3a 049", CardId::A3a049Lairon);
    map.insert("A3a 050", CardId::A3a050Aggron);
    map.insert("A3a 051", CardId::A3a051Ferroseed);
    map.insert("A3a 052", CardId::A3a052Ferrothorn);
    map.insert("A3a 053", CardId::A3a053Stakataka);
    map.insert("A3a 054", CardId::A3a054Lillipup);
    map.insert("A3a 055", CardId::A3a055Herdier);
    map.insert("A3a 056", CardId::A3a056Stoutland);
    map.insert("A3a 057", CardId::A3a057Stufful);
    map.insert("A3a 058", CardId::A3a058Bewear);
    map.insert("A3a 059", CardId::A3a059Oranguru);
    map.insert("A3a 060", CardId::A3a060TypeNull);
    map.insert("A3a 061", CardId::A3a061Silvally);
    map.insert("A3a 062", CardId::A3a062Celesteela);
    map.insert("A3a 063", CardId::A3a063BeastWall);
    map.insert("A3a 064", CardId::A3a064Repel);
    map.insert("A3a 065", CardId::A3a065ElectricalCord);
    map.insert("A3a 066", CardId::A3a066Beastite);
    map.insert("A3a 067", CardId::A3a067Gladion);
    map.insert("A3a 068", CardId::A3a068Looker);
    map.insert("A3a 069", CardId::A3a069Lusamine);
    map.insert("A3a 070", CardId::A3a070Rowlet);
    map.insert("A3a 071", CardId::A3a071Pheromosa);
    map.insert("A3a 072", CardId::A3a072Blacephalon);
    map.insert("A3a 073", CardId::A3a073AlolanMeowth);
    map.insert("A3a 074", CardId::A3a074Silvally);
    map.insert("A3a 075", CardId::A3a075Celesteela);
    map.insert("A3a 076", CardId::A3a076BuzzwoleEx);
    map.insert("A3a 077", CardId::A3a077TapuKokoEx);
    map.insert("A3a 078", CardId::A3a078LycanrocEx);
    map.insert("A3a 079", CardId::A3a079GuzzlordEx);
    map.insert("A3a 080", CardId::A3a080AlolanDugtrioEx);
    map.insert("A3a 081", CardId::A3a081Gladion);
    map.insert("A3a 082", CardId::A3a082Looker);
    map.insert("A3a 083", CardId::A3a083Lusamine);
    map.insert("A3a 084", CardId::A3a084TapuKokoEx);
    map.insert("A3a 085", CardId::A3a085LycanrocEx);
    map.insert("A3a 086", CardId::A3a086GuzzlordEx);
    map.insert("A3a 087", CardId::A3a087AlolanDugtrioEx);
    map.insert("A3a 088", CardId::A3a088BuzzwoleEx);
    map.insert("A3a 089", CardId::A3a089Growlithe);
    map.insert("A3a 090", CardId::A3a090Arcanine);
    map.insert("A3a 091", CardId::A3a091Froakie);
    map.insert("A3a 092", CardId::A3a092Frogadier);
    map.insert("A3a 093", CardId::A3a093Greninja);
    map.insert("A3a 094", CardId::A3a094Jynx);
    map.insert("A3a 095", CardId::A3a095Pidgey);
    map.insert("A3a 096", CardId::A3a096Pidgeotto);
    map.insert("A3a 097", CardId::A3a097Pidgeot);
    map.insert("A3a 098", CardId::A3a098Aerodactyl);
    map.insert("A3a 099", CardId::A3a099CelebiEx);
    map.insert("A3a 100", CardId::A3a100ArcanineEx);
    map.insert("A3a 101", CardId::A3a101AerodactylEx);
    map.insert("A3a 102", CardId::A3a102PidgeotEx);
    map.insert("A3a 103", CardId::A3a103Nihilego);
    map.insert("A3b 001", CardId::A3b001Tropius);
    map.insert("A3b 002", CardId::A3b002Leafeon);
    map.insert("A3b 003", CardId::A3b003Bounsweet);
    map.insert("A3b 004", CardId::A3b004Steenee);
    map.insert("A3b 005", CardId::A3b005Tsareena);
    map.insert("A3b 006", CardId::A3b006Applin);
    map.insert("A3b 007", CardId::A3b007Appletun);
    map.insert("A3b 008", CardId::A3b008Flareon);
    map.insert("A3b 009", CardId::A3b009FlareonEx);
    map.insert("A3b 010", CardId::A3b010Torkoal);
    map.insert("A3b 011", CardId::A3b011Litten);
    map.insert("A3b 012", CardId::A3b012Torracat);
    map.insert("A3b 013", CardId::A3b013Incineroar);
    map.insert("A3b 014", CardId::A3b014Salandit);
    map.insert("A3b 015", CardId::A3b015Salazzle);
    map.insert("A3b 016", CardId::A3b016Vaporeon);
    map.insert("A3b 017", CardId::A3b017Glaceon);
    map.insert("A3b 018", CardId::A3b018Vanillite);
    map.insert("A3b 019", CardId::A3b019Vanillish);
    map.insert("A3b 020", CardId::A3b020Vanilluxe);
    map.insert("A3b 021", CardId::A3b021Alomomola);
    map.insert("A3b 022", CardId::A3b022Popplio);
    map.insert("A3b 023", CardId::A3b023Brionne);
    map.insert("A3b 024", CardId::A3b024PrimarinaEx);
    map.insert("A3b 025", CardId::A3b025Jolteon);
    map.insert("A3b 026", CardId::A3b026Joltik);
    map.insert("A3b 027", CardId::A3b027Galvantula);
    map.insert("A3b 028", CardId::A3b028Espeon);
    map.insert("A3b 029", CardId::A3b029Woobat);
    map.insert("A3b 030", CardId::A3b030Swoobat);
    map.insert("A3b 031", CardId::A3b031Swirlix);
    map.insert("A3b 032", CardId::A3b032Slurpuff);
    map.insert("A3b 033", CardId::A3b033Sylveon);
    map.insert("A3b 034", CardId::A3b034SylveonEx);
    map.insert("A3b 035", CardId::A3b035Mimikyu);
    map.insert("A3b 036", CardId::A3b036Milcery);
    map.insert("A3b 037", CardId::A3b037Alcremie);
    map.insert("A3b 038", CardId::A3b038Barboach);
    map.insert("A3b 039", CardId::A3b039Whiscash);
    map.insert("A3b 040", CardId::A3b040Mienfoo);
    map.insert("A3b 041", CardId::A3b041Mienshao);
    map.insert("A3b 042", CardId::A3b042Carbink);
    map.insert("A3b 043", CardId::A3b043Umbreon);
    map.insert("A3b 044", CardId::A3b044Sableye);
    map.insert("A3b 045", CardId::A3b045Purrloin);
    map.insert("A3b 046", CardId::A3b046Liepard);
    map.insert("A3b 047", CardId::A3b047Mawile);
    map.insert("A3b 048", CardId::A3b048Togedemaru);
    map.insert("A3b 049", CardId::A3b049Meltan);
    map.insert("A3b 050", CardId::A3b050Melmetal);
    map.insert("A3b 051", CardId::A3b051Dratini);
    map.insert("A3b 052", CardId::A3b052Dragonair);
    map.insert("A3b 053", CardId::A3b053DragoniteEx);
    map.insert("A3b 054", CardId::A3b054Drampa);
    map.insert("A3b 055", CardId::A3b055Eevee);
    map.insert("A3b 056", CardId::A3b056EeveeEx);
    map.insert("A3b 057", CardId::A3b057SnorlaxEx);
    map.insert("A3b 058", CardId::A3b058Aipom);
    map.insert("A3b 059", CardId::A3b059Ambipom);
    map.insert("A3b 060", CardId::A3b060Chatot);
    map.insert("A3b 061", CardId::A3b061Audino);
    map.insert("A3b 062", CardId::A3b062Minccino);
    map.insert("A3b 063", CardId::A3b063Cinccino);
    map.insert("A3b 064", CardId::A3b064Skwovet);
    map.insert("A3b 065", CardId::A3b065Greedent);
    map.insert("A3b 066", CardId::A3b066EeveeBag);
    map.insert("A3b 067", CardId::A3b067Leftovers);
    map.insert("A3b 068", CardId::A3b068Hau);
    map.insert("A3b 069", CardId::A3b069Penny);
    map.insert("A3b 070", CardId::A3b070Leafeon);
    map.insert("A3b 071", CardId::A3b071Flareon);
    map.insert("A3b 072", CardId::A3b072Vaporeon);
    map.insert("A3b 073", CardId::A3b073Glaceon);
    map.insert("A3b 074", CardId::A3b074Jolteon);
    map.insert("A3b 075", CardId::A3b075Espeon);
    map.insert("A3b 076", CardId::A3b076Sylveon);
    map.insert("A3b 077", CardId::A3b077Umbreon);
    map.insert("A3b 078", CardId::A3b078Eevee);
    map.insert("A3b 079", CardId::A3b079FlareonEx);
    map.insert("A3b 080", CardId::A3b080PrimarinaEx);
    map.insert("A3b 081", CardId::A3b081SylveonEx);
    map.insert("A3b 082", CardId::A3b082DragoniteEx);
    map.insert("A3b 083", CardId::A3b083EeveeEx);
    map.insert("A3b 084", CardId::A3b084SnorlaxEx);
    map.insert("A3b 085", CardId::A3b085Hau);
    map.insert("A3b 086", CardId::A3b086Penny);
    map.insert("A3b 087", CardId::A3b087FlareonEx);
    map.insert("A3b 088", CardId::A3b088PrimarinaEx);
    map.insert("A3b 089", CardId::A3b089SylveonEx);
    map.insert("A3b 090", CardId::A3b090DragoniteEx);
    map.insert("A3b 091", CardId::A3b091SnorlaxEx);
    map.insert("A3b 092", CardId::A3b092EeveeEx);
    map.insert("A3b 093", CardId::A3b093Pinsir);
    map.insert("A3b 094", CardId::A3b094Lapras);
    map.insert("A3b 095", CardId::A3b095Voltorb);
    map.insert("A3b 096", CardId::A3b096Electrode);
    map.insert("A3b 097", CardId::A3b097Ralts);
    map.insert("A3b 098", CardId::A3b098Kirlia);
    map.insert("A3b 099", CardId::A3b099Gardevoir);
    map.insert("A3b 100", CardId::A3b100Ekans);
    map.insert("A3b 101", CardId::A3b101Arbok);
    map.insert("A3b 102", CardId::A3b102Farfetchd);
    map.insert("A3b 103", CardId::A3b103MoltresEx);
    map.insert("A3b 104", CardId::A3b104ArticunoEx);
    map.insert("A3b 105", CardId::A3b105ZapdosEx);
    map.insert("A3b 106", CardId::A3b106GalladeEx);
    map.insert("A3b 107", CardId::A3b107EeveeBag);
    map.insert("A4 001", CardId::A4001Oddish);
    map.insert("A4 002", CardId::A4002Gloom);
    map.insert("A4 003", CardId::A4003Bellossom);
    map.insert("A4 004", CardId::A4004Tangela);
    map.insert("A4 005", CardId::A4005Tangrowth);
    map.insert("A4 006", CardId::A4006Scyther);
    map.insert("A4 007", CardId::A4007Pinsir);
    map.insert("A4 008", CardId::A4008Chikorita);
    map.insert("A4 009", CardId::A4009Bayleef);
    map.insert("A4 010", CardId::A4010Meganium);
    map.insert("A4 011", CardId::A4011Ledyba);
    map.insert("A4 012", CardId::A4012Ledian);
    map.insert("A4 013", CardId::A4013Hoppip);
    map.insert("A4 014", CardId::A4014Skiploom);
    map.insert("A4 015", CardId::A4015Jumpluff);
    map.insert("A4 016", CardId::A4016Sunkern);
    map.insert("A4 017", CardId::A4017Sunflora);
    map.insert("A4 018", CardId::A4018Yanma);
    map.insert("A4 019", CardId::A4019Yanmega);
    map.insert("A4 020", CardId::A4020Pineco);
    map.insert("A4 021", CardId::A4021ShuckleEx);
    map.insert("A4 022", CardId::A4022Heracross);
    map.insert("A4 023", CardId::A4023Cherubi);
    map.insert("A4 024", CardId::A4024Cherrim);
    map.insert("A4 025", CardId::A4025Vulpix);
    map.insert("A4 026", CardId::A4026Ninetales);
    map.insert("A4 027", CardId::A4027Cyndaquil);
    map.insert("A4 028", CardId::A4028Quilava);
    map.insert("A4 029", CardId::A4029Typhlosion);
    map.insert("A4 030", CardId::A4030Slugma);
    map.insert("A4 031", CardId::A4031Magcargo);
    map.insert("A4 032", CardId::A4032Magby);
    map.insert("A4 033", CardId::A4033Entei);
    map.insert("A4 034", CardId::A4034HoOhEx);
    map.insert("A4 035", CardId::A4035Darumaka);
    map.insert("A4 036", CardId::A4036Darmanitan);
    map.insert("A4 037", CardId::A4037Heatmor);
    map.insert("A4 038", CardId::A4038Poliwag);
    map.insert("A4 039", CardId::A4039Poliwhirl);
    map.insert("A4 040", CardId::A4040Politoed);
    map.insert("A4 041", CardId::A4041Horsea);
    map.insert("A4 042", CardId::A4042Seadra);
    map.insert("A4 043", CardId::A4043KingdraEx);
    map.insert("A4 044", CardId::A4044Magikarp);
    map.insert("A4 045", CardId::A4045Gyarados);
    map.insert("A4 046", CardId::A4046Totodile);
    map.insert("A4 047", CardId::A4047Croconaw);
    map.insert("A4 048", CardId::A4048Feraligatr);
    map.insert("A4 049", CardId::A4049Marill);
    map.insert("A4 050", CardId::A4050Azumarill);
    map.insert("A4 051", CardId::A4051Wooper);
    map.insert("A4 052", CardId::A4052Quagsire);
    map.insert("A4 053", CardId::A4053Qwilfish);
    map.insert("A4 054", CardId::A4054Corsola);
    map.insert("A4 055", CardId::A4055Remoraid);
    map.insert("A4 056", CardId::A4056Octillery);
    map.insert("A4 057", CardId::A4057Delibird);
    map.insert("A4 058", CardId::A4058Mantine);
    map.insert("A4 059", CardId::A4059Suicune);
    map.insert("A4 060", CardId::A4060Corphish);
    map.insert("A4 061", CardId::A4061Crawdaunt);
    map.insert("A4 062", CardId::A4062Ducklett);
    map.insert("A4 063", CardId::A4063Swanna);
    map.insert("A4 064", CardId::A4064Chinchou);
    map.insert("A4 065", CardId::A4065LanturnEx);
    map.insert("A4 066", CardId::A4066Pichu);
    map.insert("A4 067", CardId::A4067Mareep);
    map.insert("A4 068", CardId::A4068Flaaffy);
    map.insert("A4 069", CardId::A4069Ampharos);
    map.insert("A4 070", CardId::A4070Elekid);
    map.insert("A4 071", CardId::A4071Raikou);
    map.insert("A4 072", CardId::A4072Emolga);
    map.insert("A4 073", CardId::A4073Slowpoke);
    map.insert("A4 074", CardId::A4074Slowking);
    map.insert("A4 075", CardId::A4075Smoochum);
    map.insert("A4 076", CardId::A4076Jynx);
    map.insert("A4 077", CardId::A4077Cleffa);
    map.insert("A4 078", CardId::A4078Togepi);
    map.insert("A4 079", CardId::A4079Togetic);
    map.insert("A4 080", CardId::A4080Togekiss);
    map.insert("A4 081", CardId::A4081Natu);
    map.insert("A4 082", CardId::A4082Xatu);
    map.insert("A4 083", CardId::A4083EspeonEx);
    map.insert("A4 084", CardId::A4084Unown);
    map.insert("A4 085", CardId::A4085Unown);
    map.insert("A4 086", CardId::A4086Wobbuffet);
    map.insert("A4 087", CardId::A4087Girafarig);
    map.insert("A4 088", CardId::A4088Snubbull);
    map.insert("A4 089", CardId::A4089Granbull);
    map.insert("A4 090", CardId::A4090Munna);
    map.insert("A4 091", CardId::A4091Musharna);
    map.insert("A4 092", CardId::A4092Onix);
    map.insert("A4 093", CardId::A4093Sudowoodo);
    map.insert("A4 094", CardId::A4094Gligar);
    map.insert("A4 095", CardId::A4095Gliscor);
    map.insert("A4 096", CardId::A4096Swinub);
    map.insert("A4 097", CardId::A4097Piloswine);
    map.insert("A4 098", CardId::A4098Mamoswine);
    map.insert("A4 099", CardId::A4099Phanpy);
    map.insert("A4 100", CardId::A4100DonphanEx);
    map.insert("A4 101", CardId::A4101Tyrogue);
    map.insert("A4 102", CardId::A4102Hitmontop);
    map.insert("A4 103", CardId::A4103Larvitar);
    map.insert("A4 104", CardId::A4104Pupitar);
    map.insert("A4 105", CardId::A4105Binacle);
    map.insert("A4 106", CardId::A4106Barbaracle);
    map.insert("A4 107", CardId::A4107Zubat);
    map.insert("A4 108", CardId::A4108Golbat);
    map.insert("A4 109", CardId::A4109CrobatEx);
    map.insert("A4 110", CardId::A4110Spinarak);
    map.insert("A4 111", CardId::A4111Ariados);
    map.insert("A4 112", CardId::A4112UmbreonEx);
    map.insert("A4 113", CardId::A4113Murkrow);
    map.insert("A4 114", CardId::A4114Honchkrow);
    map.insert("A4 115", CardId::A4115Sneasel);
    map.insert("A4 116", CardId::A4116Weavile);
    map.insert("A4 117", CardId::A4117Houndour);
    map.insert("A4 118", CardId::A4118Houndoom);
    map.insert("A4 119", CardId::A4119Tyranitar);
    map.insert("A4 120", CardId::A4120Absol);
    map.insert("A4 121", CardId::A4121Forretress);
    map.insert("A4 122", CardId::A4122Steelix);
    map.insert("A4 123", CardId::A4123Scizor);
    map.insert("A4 124", CardId::A4124SkarmoryEx);
    map.insert("A4 125", CardId::A4125Mawile);
    map.insert("A4 126", CardId::A4126Klink);
    map.insert("A4 127", CardId::A4127Klang);
    map.insert("A4 128", CardId::A4128Klinklang);
    map.insert("A4 129", CardId::A4129Spearow);
    map.insert("A4 130", CardId::A4130Fearow);
    map.insert("A4 131", CardId::A4131Chansey);
    map.insert("A4 132", CardId::A4132Blissey);
    map.insert("A4 133", CardId::A4133Kangaskhan);
    map.insert("A4 134", CardId::A4134Eevee);
    map.insert("A4 135", CardId::A4135Porygon);
    map.insert("A4 136", CardId::A4136Porygon2);
    map.insert("A4 137", CardId::A4137PorygonZ);
    map.insert("A4 138", CardId::A4138Sentret);
    map.insert("A4 139", CardId::A4139Furret);
    map.insert("A4 140", CardId::A4140Hoothoot);
    map.insert("A4 141", CardId::A4141Noctowl);
    map.insert("A4 142", CardId::A4142Aipom);
    map.insert("A4 143", CardId::A4143Ambipom);
    map.insert("A4 144", CardId::A4144Dunsparce);
    map.insert("A4 145", CardId::A4145Teddiursa);
    map.insert("A4 146", CardId::A4146Ursaring);
    map.insert("A4 147", CardId::A4147Stantler);
    map.insert("A4 148", CardId::A4148Smeargle);
    map.insert("A4 149", CardId::A4149LugiaEx);
    map.insert("A4 150", CardId::A4150Bouffalant);
    map.insert("A4 151", CardId::A4151ElementalSwitch);
    map.insert("A4 152", CardId::A4152SquirtBottle);
    map.insert("A4 153", CardId::A4153SteelApron);
    map.insert("A4 154", CardId::A4154DarkPendant);
    map.insert("A4 155", CardId::A4155RescueScarf);
    map.insert("A4 156", CardId::A4156Will);
    map.insert("A4 157", CardId::A4157Lyra);
    map.insert("A4 158", CardId::A4158Silver);
    map.insert("A4 159", CardId::A4159Fisher);
    map.insert("A4 160", CardId::A4160Jasmine);
    map.insert("A4 161", CardId::A4161Hiker);
    map.insert("A4 162", CardId::A4162Chikorita);
    map.insert("A4 163", CardId::A4163Bellossom);
    map.insert("A4 164", CardId::A4164Heracross);
    map.insert("A4 165", CardId::A4165Cyndaquil);
    map.insert("A4 166", CardId::A4166Magby);
    map.insert("A4 167", CardId::A4167Totodile);
    map.insert("A4 168", CardId::A4168Qwilfish);
    map.insert("A4 169", CardId::A4169Octillery);
    map.insert("A4 170", CardId::A4170Delibird);
    map.insert("A4 171", CardId::A4171Pichu);
    map.insert("A4 172", CardId::A4172Ampharos);
    map.insert("A4 173", CardId::A4173Togepi);
    map.insert("A4 174", CardId::A4174Xatu);
    map.insert("A4 175", CardId::A4175Wobbuffet);
    map.insert("A4 176", CardId::A4176Gligar);
    map.insert("A4 177", CardId::A4177Spinarak);
    map.insert("A4 178", CardId::A4178Murkrow);
    map.insert("A4 179", CardId::A4179Tyranitar);
    map.insert("A4 180", CardId::A4180Scizor);
    map.insert("A4 181", CardId::A4181Sentret);
    map.insert("A4 182", CardId::A4182Hoothoot);
    map.insert("A4 183", CardId::A4183Stantler);
    map.insert("A4 184", CardId::A4184Smeargle);
    map.insert("A4 185", CardId::A4185Blissey);
    map.insert("A4 186", CardId::A4186ShuckleEx);
    map.insert("A4 187", CardId::A4187HoOhEx);
    map.insert("A4 188", CardId::A4188KingdraEx);
    map.insert("A4 189", CardId::A4189LanturnEx);
    map.insert("A4 190", CardId::A4190EspeonEx);
    map.insert("A4 191", CardId::A4191DonphanEx);
    map.insert("A4 192", CardId::A4192CrobatEx);
    map.insert("A4 193", CardId::A4193UmbreonEx);
    map.insert("A4 194", CardId::A4194SkarmoryEx);
    map.insert("A4 195", CardId::A4195LugiaEx);
    map.insert("A4 196", CardId::A4196Will);
    map.insert("A4 197", CardId::A4197Lyra);
    map.insert("A4 198", CardId::A4198Silver);
    map.insert("A4 199", CardId::A4199Fisher);
    map.insert("A4 200", CardId::A4200Jasmine);
    map.insert("A4 201", CardId::A4201Hiker);
    map.insert("A4 202", CardId::A4202ShuckleEx);
    map.insert("A4 203", CardId::A4203KingdraEx);
    map.insert("A4 204", CardId::A4204LanturnEx);
    map.insert("A4 205", CardId::A4205EspeonEx);
    map.insert("A4 206", CardId::A4206DonphanEx);
    map.insert("A4 207", CardId::A4207CrobatEx);
    map.insert("A4 208", CardId::A4208UmbreonEx);
    map.insert("A4 209", CardId::A4209SkarmoryEx);
    map.insert("A4 210", CardId::A4210HoOhEx);
    map.insert("A4 211", CardId::A4211LugiaEx);
    map.insert("A4 212", CardId::A4212Yanma);
    map.insert("A4 213", CardId::A4213Flareon);
    map.insert("A4 214", CardId::A4214Magikarp);
    map.insert("A4 215", CardId::A4215Gyarados);
    map.insert("A4 216", CardId::A4216Vaporeon);
    map.insert("A4 217", CardId::A4217Magnemite);
    map.insert("A4 218", CardId::A4218Magneton);
    map.insert("A4 219", CardId::A4219Jolteon);
    map.insert("A4 220", CardId::A4220Misdreavus);
    map.insert("A4 221", CardId::A4221Mankey);
    map.insert("A4 222", CardId::A4222Primeape);
    map.insert("A4 223", CardId::A4223NidoranF);
    map.insert("A4 224", CardId::A4224Nidorina);
    map.insert("A4 225", CardId::A4225Nidoqueen);
    map.insert("A4 226", CardId::A4226NidoranM);
    map.insert("A4 227", CardId::A4227Nidorino);
    map.insert("A4 228", CardId::A4228Nidoking);
    map.insert("A4 229", CardId::A4229Sneasel);
    map.insert("A4 230", CardId::A4230Lickitung);
    map.insert("A4 231", CardId::A4231Eevee);
    map.insert("A4 232", CardId::A4232YanmegaEx);
    map.insert("A4 233", CardId::A4233LeafeonEx);
    map.insert("A4 234", CardId::A4234GyaradosEx);
    map.insert("A4 235", CardId::A4235GlaceonEx);
    map.insert("A4 236", CardId::A4236PachirisuEx);
    map.insert("A4 237", CardId::A4237MismagiusEx);
    map.insert("A4 238", CardId::A4238WeavileEx);
    map.insert("A4 239", CardId::A4239LickilickyEx);
    map.insert("A4 240", CardId::A4240HoOhEx);
    map.insert("A4 241", CardId::A4241LugiaEx);
    map.insert("A4a 001", CardId::A4a001Hoppip);
    map.insert("A4a 002", CardId::A4a002Skiploom);
    map.insert("A4a 003", CardId::A4a003JumpluffEx);
    map.insert("A4a 004", CardId::A4a004Sunkern);
    map.insert("A4a 005", CardId::A4a005Sunflora);
    map.insert("A4a 006", CardId::A4a006Celebi);
    map.insert("A4a 007", CardId::A4a007Durant);
    map.insert("A4a 008", CardId::A4a008Slugma);
    map.insert("A4a 009", CardId::A4a009Magcargo);
    map.insert("A4a 010", CardId::A4a010EnteiEx);
    map.insert("A4a 011", CardId::A4a011Fletchinder);
    map.insert("A4a 012", CardId::A4a012Talonflame);
    map.insert("A4a 013", CardId::A4a013Poliwag);
    map.insert("A4a 014", CardId::A4a014Poliwhirl);
    map.insert("A4a 015", CardId::A4a015Tentacool);
    map.insert("A4a 016", CardId::A4a016Tentacruel);
    map.insert("A4a 017", CardId::A4a017Slowpoke);
    map.insert("A4a 018", CardId::A4a018Slowking);
    map.insert("A4a 019", CardId::A4a019Jynx);
    map.insert("A4a 020", CardId::A4a020SuicuneEx);
    map.insert("A4a 021", CardId::A4a021Feebas);
    map.insert("A4a 022", CardId::A4a022Milotic);
    map.insert("A4a 023", CardId::A4a023Mantyke);
    map.insert("A4a 024", CardId::A4a024Cryogonal);
    map.insert("A4a 025", CardId::A4a025RaikouEx);
    map.insert("A4a 026", CardId::A4a026Tynamo);
    map.insert("A4a 027", CardId::A4a027Eelektrik);
    map.insert("A4a 028", CardId::A4a028Eelektross);
    map.insert("A4a 029", CardId::A4a029Stunfisk);
    map.insert("A4a 030", CardId::A4a030Yamper);
    map.insert("A4a 031", CardId::A4a031Boltund);
    map.insert("A4a 032", CardId::A4a032Misdreavus);
    map.insert("A4a 033", CardId::A4a033Mismagius);
    map.insert("A4a 034", CardId::A4a034GalarianCorsola);
    map.insert("A4a 035", CardId::A4a035GalarianCursola);
    map.insert("A4a 036", CardId::A4a036Latias);
    map.insert("A4a 037", CardId::A4a037Latios);
    map.insert("A4a 038", CardId::A4a038Frillish);
    map.insert("A4a 039", CardId::A4a039Jellicent);
    map.insert("A4a 040", CardId::A4a040Diglett);
    map.insert("A4a 041", CardId::A4a041Dugtrio);
    map.insert("A4a 042", CardId::A4a042PoliwrathEx);
    map.insert("A4a 043", CardId::A4a043Phanpy);
    map.insert("A4a 044", CardId::A4a044Donphan);
    map.insert("A4a 045", CardId::A4a045Relicanth);
    map.insert("A4a 046", CardId::A4a046Dwebble);
    map.insert("A4a 047", CardId::A4a047Crustle);
    map.insert("A4a 048", CardId::A4a048Seviper);
    map.insert("A4a 049", CardId::A4a049Zorua);
    map.insert("A4a 050", CardId::A4a050Zoroark);
    map.insert("A4a 051", CardId::A4a051Inkay);
    map.insert("A4a 052", CardId::A4a052Malamar);
    map.insert("A4a 053", CardId::A4a053Skrelp);
    map.insert("A4a 054", CardId::A4a054Dragalge);
    map.insert("A4a 055", CardId::A4a055Altaria);
    map.insert("A4a 056", CardId::A4a056Farfetchd);
    map.insert("A4a 057", CardId::A4a057Lickitung);
    map.insert("A4a 058", CardId::A4a058Lickilicky);
    map.insert("A4a 059", CardId::A4a059Igglybuff);
    map.insert("A4a 060", CardId::A4a060Teddiursa);
    map.insert("A4a 061", CardId::A4a061Ursaring);
    map.insert("A4a 062", CardId::A4a062Miltank);
    map.insert("A4a 063", CardId::A4a063Azurill);
    map.insert("A4a 064", CardId::A4a064Swablu);
    map.insert("A4a 065", CardId::A4a065Zangoose);
    map.insert("A4a 066", CardId::A4a066Fletchling);
    map.insert("A4a 067", CardId::A4a067InflatableBoat);
    map.insert("A4a 068", CardId::A4a068MemoryLight);
    map.insert("A4a 069", CardId::A4a069Whitney);
    map.insert("A4a 070", CardId::A4a070TravelingMerchant);
    map.insert("A4a 071", CardId::A4a071Morty);
    map.insert("A4a 072", CardId::A4a072Milotic);
    map.insert("A4a 073", CardId::A4a073Stunfisk);
    map.insert("A4a 074", CardId::A4a074Yamper);
    map.insert("A4a 075", CardId::A4a075Latios);
    map.insert("A4a 076", CardId::A4a076Phanpy);
    map.insert("A4a 077", CardId::A4a077Azurill);
    map.insert("A4a 078", CardId::A4a078JumpluffEx);
    map.insert("A4a 079", CardId::A4a079EnteiEx);
    map.insert("A4a 080", CardId::A4a080SuicuneEx);
    map.insert("A4a 081", CardId::A4a081RaikouEx);
    map.insert("A4a 082", CardId::A4a082PoliwrathEx);
    map.insert("A4a 083", CardId::A4a083Whitney);
    map.insert("A4a 084", CardId::A4a084TravelingMerchant);
    map.insert("A4a 085", CardId::A4a085Morty);
    map.insert("A4a 086", CardId::A4a086JumpluffEx);
    map.insert("A4a 087", CardId::A4a087EnteiEx);
    map.insert("A4a 088", CardId::A4a088RaikouEx);
    map.insert("A4a 089", CardId::A4a089PoliwrathEx);
    map.insert("A4a 090", CardId::A4a090SuicuneEx);
    map.insert("A4a 091", CardId::A4a091Chimchar);
    map.insert("A4a 092", CardId::A4a092Monferno);
    map.insert("A4a 093", CardId::A4a093Psyduck);
    map.insert("A4a 094", CardId::A4a094Golduck);
    map.insert("A4a 095", CardId::A4a095Krabby);
    map.insert("A4a 096", CardId::A4a096Kingler);
    map.insert("A4a 097", CardId::A4a097Pyukumuku);
    map.insert("A4a 098", CardId::A4a098Gible);
    map.insert("A4a 099", CardId::A4a099Gabite);
    map.insert("A4a 100", CardId::A4a100PaldeanWooper);
    map.insert("A4a 101", CardId::A4a101InfernapeEx);
    map.insert("A4a 102", CardId::A4a102MewEx);
    map.insert("A4a 103", CardId::A4a103GarchompEx);
    map.insert("A4a 104", CardId::A4a104PaldeanClodsireEx);
    map.insert("A4a 105", CardId::A4a105Mantyke);
    map.insert("A4b 001", CardId::A4b001Bulbasaur);
    map.insert("A4b 002", CardId::A4b002Bulbasaur);
    map.insert("A4b 003", CardId::A4b003Ivysaur);
    map.insert("A4b 004", CardId::A4b004Ivysaur);
    map.insert("A4b 005", CardId::A4b005VenusaurEx);
    map.insert("A4b 006", CardId::A4b006Weedle);
    map.insert("A4b 007", CardId::A4b007Weedle);
    map.insert("A4b 008", CardId::A4b008Kakuna);
    map.insert("A4b 009", CardId::A4b009Kakuna);
    map.insert("A4b 010", CardId::A4b010BeedrillEx);
    map.insert("A4b 011", CardId::A4b011Exeggcute);
    map.insert("A4b 012", CardId::A4b012Exeggcute);
    map.insert("A4b 013", CardId::A4b013ExeggutorEx);
    map.insert("A4b 014", CardId::A4b014Hoppip);
    map.insert("A4b 015", CardId::A4b015Hoppip);
    map.insert("A4b 016", CardId::A4b016Skiploom);
    map.insert("A4b 017", CardId::A4b017Skiploom);
    map.insert("A4b 018", CardId::A4b018Jumpluff);
    map.insert("A4b 019", CardId::A4b019Jumpluff);
    map.insert("A4b 020", CardId::A4b020Yanma);
    map.insert("A4b 021", CardId::A4b021Yanma);
    map.insert("A4b 022", CardId::A4b022YanmegaEx);
    map.insert("A4b 023", CardId::A4b023ShuckleEx);
    map.insert("A4b 024", CardId::A4b024CelebiEx);
    map.insert("A4b 025", CardId::A4b025Cherubi);
    map.insert("A4b 026", CardId::A4b026Cherubi);
    map.insert("A4b 027", CardId::A4b027Cherrim);
    map.insert("A4b 028", CardId::A4b028Cherrim);
    map.insert("A4b 029", CardId::A4b029LeafeonEx);
    map.insert("A4b 030", CardId::A4b030Shaymin);
    map.insert("A4b 031", CardId::A4b031Shaymin);
    map.insert("A4b 032", CardId::A4b032Snivy);
    map.insert("A4b 033", CardId::A4b033Snivy);
    map.insert("A4b 034", CardId::A4b034Servine);
    map.insert("A4b 035", CardId::A4b035Servine);
    map.insert("A4b 036", CardId::A4b036Serperior);
    map.insert("A4b 037", CardId::A4b037Serperior);
    map.insert("A4b 038", CardId::A4b038Rowlet);
    map.insert("A4b 039", CardId::A4b039Rowlet);
    map.insert("A4b 040", CardId::A4b040Dartrix);
    map.insert("A4b 041", CardId::A4b041Dartrix);
    map.insert("A4b 042", CardId::A4b042DecidueyeEx);
    map.insert("A4b 043", CardId::A4b043DhelmiseEx);
    map.insert("A4b 044", CardId::A4b044BuzzwoleEx);
    map.insert("A4b 045", CardId::A4b045Pheromosa);
    map.insert("A4b 046", CardId::A4b046Pheromosa);
    map.insert("A4b 047", CardId::A4b047Kartana);
    map.insert("A4b 048", CardId::A4b048Kartana);
    map.insert("A4b 049", CardId::A4b049Sprigatito);
    map.insert("A4b 050", CardId::A4b050Sprigatito);
    map.insert("A4b 051", CardId::A4b051Floragato);
    map.insert("A4b 052", CardId::A4b052Floragato);
    map.insert("A4b 053", CardId::A4b053Meowscarada);
    map.insert("A4b 054", CardId::A4b054Meowscarada);
    map.insert("A4b 055", CardId::A4b055Charmander);
    map.insert("A4b 056", CardId::A4b056Charmander);
    map.insert("A4b 057", CardId::A4b057Charmeleon);
    map.insert("A4b 058", CardId::A4b058Charmeleon);
    map.insert("A4b 059", CardId::A4b059CharizardEx);
    map.insert("A4b 060", CardId::A4b060CharizardEx);
    map.insert("A4b 061", CardId::A4b061Growlithe);
    map.insert("A4b 062", CardId::A4b062Growlithe);
    map.insert("A4b 063", CardId::A4b063ArcanineEx);
    map.insert("A4b 064", CardId::A4b064Flareon);
    map.insert("A4b 065", CardId::A4b065Flareon);
    map.insert("A4b 066", CardId::A4b066FlareonEx);
    map.insert("A4b 067", CardId::A4b067MoltresEx);
    map.insert("A4b 068", CardId::A4b068HoOhEx);
    map.insert("A4b 069", CardId::A4b069Torkoal);
    map.insert("A4b 070", CardId::A4b070Torkoal);
    map.insert("A4b 071", CardId::A4b071Chimchar);
    map.insert("A4b 072", CardId::A4b072Chimchar);
    map.insert("A4b 073", CardId::A4b073Monferno);
    map.insert("A4b 074", CardId::A4b074Monferno);
    map.insert("A4b 075", CardId::A4b075InfernapeEx);
    map.insert("A4b 076", CardId::A4b076Heatran);
    map.insert("A4b 077", CardId::A4b077Heatran);
    map.insert("A4b 078", CardId::A4b078Litten);
    map.insert("A4b 079", CardId::A4b079Litten);
    map.insert("A4b 080", CardId::A4b080Torracat);
    map.insert("A4b 081", CardId::A4b081Torracat);
    map.insert("A4b 082", CardId::A4b082IncineroarEx);
    map.insert("A4b 083", CardId::A4b083Squirtle);
    map.insert("A4b 084", CardId::A4b084Squirtle);
    map.insert("A4b 085", CardId::A4b085Wartortle);
    map.insert("A4b 086", CardId::A4b086Wartortle);
    map.insert("A4b 087", CardId::A4b087BlastoiseEx);
    map.insert("A4b 088", CardId::A4b088Horsea);
    map.insert("A4b 089", CardId::A4b089Horsea);
    map.insert("A4b 090", CardId::A4b090Seadra);
    map.insert("A4b 091", CardId::A4b091Seadra);
    map.insert("A4b 092", CardId::A4b092KingdraEx);
    map.insert("A4b 093", CardId::A4b093Staryu);
    map.insert("A4b 094", CardId::A4b094Staryu);
    map.insert("A4b 095", CardId::A4b095StarmieEx);
    map.insert("A4b 096", CardId::A4b096Magikarp);
    map.insert("A4b 097", CardId::A4b097Magikarp);
    map.insert("A4b 098", CardId::A4b098GyaradosEx);
    map.insert("A4b 099", CardId::A4b099Vaporeon);
    map.insert("A4b 100", CardId::A4b100Vaporeon);
    map.insert("A4b 101", CardId::A4b101ArticunoEx);
    map.insert("A4b 102", CardId::A4b102Corphish);
    map.insert("A4b 103", CardId::A4b103Corphish);
    map.insert("A4b 104", CardId::A4b104Crawdaunt);
    map.insert("A4b 105", CardId::A4b105Crawdaunt);
    map.insert("A4b 106", CardId::A4b106GlaceonEx);
    map.insert("A4b 107", CardId::A4b107PalkiaEx);
    map.insert("A4b 108", CardId::A4b108Manaphy);
    map.insert("A4b 109", CardId::A4b109Manaphy);
    map.insert("A4b 110", CardId::A4b110Froakie);
    map.insert("A4b 111", CardId::A4b111Froakie);
    map.insert("A4b 112", CardId::A4b112Frogadier);
    map.insert("A4b 113", CardId::A4b113Frogadier);
    map.insert("A4b 114", CardId::A4b114Greninja);
    map.insert("A4b 115", CardId::A4b115Greninja);
    map.insert("A4b 116", CardId::A4b116Popplio);
    map.insert("A4b 117", CardId::A4b117Popplio);
    map.insert("A4b 118", CardId::A4b118Brionne);
    map.insert("A4b 119", CardId::A4b119Brionne);
    map.insert("A4b 120", CardId::A4b120PrimarinaEx);
    map.insert("A4b 121", CardId::A4b121CrabominableEx);
    map.insert("A4b 122", CardId::A4b122Wishiwashi);
    map.insert("A4b 123", CardId::A4b123Wishiwashi);
    map.insert("A4b 124", CardId::A4b124WishiwashiEx);
    map.insert("A4b 125", CardId::A4b125Wiglett);
    map.insert("A4b 126", CardId::A4b126Wiglett);
    map.insert("A4b 127", CardId::A4b127WugtrioEx);
    map.insert("A4b 128", CardId::A4b128Pikachu);
    map.insert("A4b 129", CardId::A4b129Pikachu);
    map.insert("A4b 130", CardId::A4b130AlolanRaichuEx);
    map.insert("A4b 131", CardId::A4b131PikachuEx);
    map.insert("A4b 132", CardId::A4b132PikachuEx);
    map.insert("A4b 133", CardId::A4b133Magnemite);
    map.insert("A4b 134", CardId::A4b134Magnemite);
    map.insert("A4b 135", CardId::A4b135Magneton);
    map.insert("A4b 136", CardId::A4b136Magneton);
    map.insert("A4b 137", CardId::A4b137Magnezone);
    map.insert("A4b 138", CardId::A4b138Magnezone);
    map.insert("A4b 139", CardId::A4b139ZapdosEx);
    map.insert("A4b 140", CardId::A4b140Chinchou);
    map.insert("A4b 141", CardId::A4b141Chinchou);
    map.insert("A4b 142", CardId::A4b142LanturnEx);
    map.insert("A4b 143", CardId::A4b143Pachirisu);
    map.insert("A4b 144", CardId::A4b144Pachirisu);
    map.insert("A4b 145", CardId::A4b145PachirisuEx);
    map.insert("A4b 146", CardId::A4b146Oricorio);
    map.insert("A4b 147", CardId::A4b147Oricorio);
    map.insert("A4b 148", CardId::A4b148TapuKokoEx);
    map.insert("A4b 149", CardId::A4b149Zeraora);
    map.insert("A4b 150", CardId::A4b150Zeraora);
    map.insert("A4b 151", CardId::A4b151Gastly);
    map.insert("A4b 152", CardId::A4b152Gastly);
    map.insert("A4b 153", CardId::A4b153Haunter);
    map.insert("A4b 154", CardId::A4b154Haunter);
    map.insert("A4b 155", CardId::A4b155GengarEx);
    map.insert("A4b 156", CardId::A4b156Jynx);
    map.insert("A4b 157", CardId::A4b157Jynx);
    map.insert("A4b 158", CardId::A4b158MewtwoEx);
    map.insert("A4b 159", CardId::A4b159MewEx);
    map.insert("A4b 160", CardId::A4b160EspeonEx);
    map.insert("A4b 161", CardId::A4b161Misdreavus);
    map.insert("A4b 162", CardId::A4b162Misdreavus);
    map.insert("A4b 163", CardId::A4b163MismagiusEx);
    map.insert("A4b 164", CardId::A4b164Ralts);
    map.insert("A4b 165", CardId::A4b165Ralts);
    map.insert("A4b 166", CardId::A4b166Kirlia);
    map.insert("A4b 167", CardId::A4b167Kirlia);
    map.insert("A4b 168", CardId::A4b168Gardevoir);
    map.insert("A4b 169", CardId::A4b169Gardevoir);
    map.insert("A4b 170", CardId::A4b170Giratina);
    map.insert("A4b 171", CardId::A4b171Giratina);
    map.insert("A4b 172", CardId::A4b172GiratinaEx);
    map.insert("A4b 173", CardId::A4b173Swirlix);
    map.insert("A4b 174", CardId::A4b174Swirlix);
    map.insert("A4b 175", CardId::A4b175Slurpuff);
    map.insert("A4b 176", CardId::A4b176Slurpuff);
    map.insert("A4b 177", CardId::A4b177SylveonEx);
    map.insert("A4b 178", CardId::A4b178Oricorio);
    map.insert("A4b 179", CardId::A4b179Oricorio);
    map.insert("A4b 180", CardId::A4b180Cosmog);
    map.insert("A4b 181", CardId::A4b181Cosmog);
    map.insert("A4b 182", CardId::A4b182Cosmoem);
    map.insert("A4b 183", CardId::A4b183Cosmoem);
    map.insert("A4b 184", CardId::A4b184LunalaEx);
    map.insert("A4b 185", CardId::A4b185Milcery);
    map.insert("A4b 186", CardId::A4b186Milcery);
    map.insert("A4b 187", CardId::A4b187Alcremie);
    map.insert("A4b 188", CardId::A4b188Alcremie);
    map.insert("A4b 189", CardId::A4b189Machop);
    map.insert("A4b 190", CardId::A4b190Machop);
    map.insert("A4b 191", CardId::A4b191Machoke);
    map.insert("A4b 192", CardId::A4b192Machoke);
    map.insert("A4b 193", CardId::A4b193MachampEx);
    map.insert("A4b 194", CardId::A4b194Cubone);
    map.insert("A4b 195", CardId::A4b195Cubone);
    map.insert("A4b 196", CardId::A4b196MarowakEx);
    map.insert("A4b 197", CardId::A4b197AerodactylEx);
    map.insert("A4b 198", CardId::A4b198Sudowoodo);
    map.insert("A4b 199", CardId::A4b199Sudowoodo);
    map.insert("A4b 200", CardId::A4b200Phanpy);
    map.insert("A4b 201", CardId::A4b201Phanpy);
    map.insert("A4b 202", CardId::A4b202DonphanEx);
    map.insert("A4b 203", CardId::A4b203Nosepass);
    map.insert("A4b 204", CardId::A4b204Nosepass);
    map.insert("A4b 205", CardId::A4b205Gible);
    map.insert("A4b 206", CardId::A4b206Gible);
    map.insert("A4b 207", CardId::A4b207Gabite);
    map.insert("A4b 208", CardId::A4b208Gabite);
    map.insert("A4b 209", CardId::A4b209GarchompEx);
    map.insert("A4b 210", CardId::A4b210Riolu);
    map.insert("A4b 211", CardId::A4b211Riolu);
    map.insert("A4b 212", CardId::A4b212Lucario);
    map.insert("A4b 213", CardId::A4b213Lucario);
    map.insert("A4b 214", CardId::A4b214LucarioEx);
    map.insert("A4b 215", CardId::A4b215GalladeEx);
    map.insert("A4b 216", CardId::A4b216Drilbur);
    map.insert("A4b 217", CardId::A4b217Drilbur);
    map.insert("A4b 218", CardId::A4b218Crabrawler);
    map.insert("A4b 219", CardId::A4b219Crabrawler);
    map.insert("A4b 220", CardId::A4b220Rockruff);
    map.insert("A4b 221", CardId::A4b221Rockruff);
    map.insert("A4b 222", CardId::A4b222LycanrocEx);
    map.insert("A4b 223", CardId::A4b223PassimianEx);
    map.insert("A4b 224", CardId::A4b224Marshadow);
    map.insert("A4b 225", CardId::A4b225Marshadow);
    map.insert("A4b 226", CardId::A4b226Zubat);
    map.insert("A4b 227", CardId::A4b227Zubat);
    map.insert("A4b 228", CardId::A4b228Golbat);
    map.insert("A4b 229", CardId::A4b229Golbat);
    map.insert("A4b 230", CardId::A4b230Crobat);
    map.insert("A4b 231", CardId::A4b231Crobat);
    map.insert("A4b 232", CardId::A4b232CrobatEx);
    map.insert("A4b 233", CardId::A4b233AlolanGrimer);
    map.insert("A4b 234", CardId::A4b234AlolanGrimer);
    map.insert("A4b 235", CardId::A4b235AlolanMukEx);
    map.insert("A4b 236", CardId::A4b236PaldeanWooper);
    map.insert("A4b 237", CardId::A4b237PaldeanWooper);
    map.insert("A4b 238", CardId::A4b238PaldeanClodsireEx);
    map.insert("A4b 239", CardId::A4b239Umbreon);
    map.insert("A4b 240", CardId::A4b240Umbreon);
    map.insert("A4b 241", CardId::A4b241UmbreonEx);
    map.insert("A4b 242", CardId::A4b242Sneasel);
    map.insert("A4b 243", CardId::A4b243Sneasel);
    map.insert("A4b 244", CardId::A4b244WeavileEx);
    map.insert("A4b 245", CardId::A4b245DarkraiEx);
    map.insert("A4b 246", CardId::A4b246Nihilego);
    map.insert("A4b 247", CardId::A4b247Nihilego);
    map.insert("A4b 248", CardId::A4b248GuzzlordEx);
    map.insert("A4b 249", CardId::A4b249AlolanDiglett);
    map.insert("A4b 250", CardId::A4b250AlolanDiglett);
    map.insert("A4b 251", CardId::A4b251AlolanDugtrioEx);
    map.insert("A4b 252", CardId::A4b252SkarmoryEx);
    map.insert("A4b 253", CardId::A4b253ProbopassEx);
    map.insert("A4b 254", CardId::A4b254DialgaEx);
    map.insert("A4b 255", CardId::A4b255Excadrill);
    map.insert("A4b 256", CardId::A4b256Excadrill);
    map.insert("A4b 257", CardId::A4b257Klefki);
    map.insert("A4b 258", CardId::A4b258Klefki);
    map.insert("A4b 259", CardId::A4b259SolgaleoEx);
    map.insert("A4b 260", CardId::A4b260Magearna);
    map.insert("A4b 261", CardId::A4b261Magearna);
    map.insert("A4b 262", CardId::A4b262Tinkatink);
    map.insert("A4b 263", CardId::A4b263Tinkatink);
    map.insert("A4b 264", CardId::A4b264Tinkatuff);
    map.insert("A4b 265", CardId::A4b265Tinkatuff);
    map.insert("A4b 266", CardId::A4b266TinkatonEx);
    map.insert("A4b 267", CardId::A4b267Dratini);
    map.insert("A4b 268", CardId::A4b268Dratini);
    map.insert("A4b 269", CardId::A4b269Dragonair);
    map.insert("A4b 270", CardId::A4b270Dragonair);
    map.insert("A4b 271", CardId::A4b271DragoniteEx);
    map.insert("A4b 272", CardId::A4b272Pidgey);
    map.insert("A4b 273", CardId::A4b273Pidgey);
    map.insert("A4b 274", CardId::A4b274Pidgeotto);
    map.insert("A4b 275", CardId::A4b275Pidgeotto);
    map.insert("A4b 276", CardId::A4b276PidgeotEx);
    map.insert("A4b 277", CardId::A4b277Jigglypuff);
    map.insert("A4b 278", CardId::A4b278Jigglypuff);
    map.insert("A4b 279", CardId::A4b279WigglytuffEx);
    map.insert("A4b 280", CardId::A4b280Farfetchd);
    map.insert("A4b 281", CardId::A4b281Farfetchd);
    map.insert("A4b 282", CardId::A4b282Lickitung);
    map.insert("A4b 283", CardId::A4b283Lickitung);
    map.insert("A4b 284", CardId::A4b284LickilickyEx);
    map.insert("A4b 285", CardId::A4b285Eevee);
    map.insert("A4b 286", CardId::A4b286Eevee);
    map.insert("A4b 287", CardId::A4b287EeveeEx);
    map.insert("A4b 288", CardId::A4b288SnorlaxEx);
    map.insert("A4b 289", CardId::A4b289LugiaEx);
    map.insert("A4b 290", CardId::A4b290Skitty);
    map.insert("A4b 291", CardId::A4b291Skitty);
    map.insert("A4b 292", CardId::A4b292Delcatty);
    map.insert("A4b 293", CardId::A4b293Delcatty);
    map.insert("A4b 294", CardId::A4b294Bidoof);
    map.insert("A4b 295", CardId::A4b295Bidoof);
    map.insert("A4b 296", CardId::A4b296BibarelEx);
    map.insert("A4b 297", CardId::A4b297Shaymin);
    map.insert("A4b 298", CardId::A4b298Shaymin);
    map.insert("A4b 299", CardId::A4b299ArceusEx);
    map.insert("A4b 300", CardId::A4b300TypeNull);
    map.insert("A4b 301", CardId::A4b301TypeNull);
    map.insert("A4b 302", CardId::A4b302Silvally);
    map.insert("A4b 303", CardId::A4b303Silvally);
    map.insert("A4b 304", CardId::A4b304Celesteela);
    map.insert("A4b 305", CardId::A4b305Celesteela);
    map.insert("A4b 306", CardId::A4b306Cyclizar);
    map.insert("A4b 307", CardId::A4b307Cyclizar);
    map.insert("A4b 308", CardId::A4b308EeveeBag);
    map.insert("A4b 309", CardId::A4b309EeveeBag);
    map.insert("A4b 310", CardId::A4b310ElementalSwitch);
    map.insert("A4b 311", CardId::A4b311ElementalSwitch);
    map.insert("A4b 312", CardId::A4b312OldAmber);
    map.insert("A4b 313", CardId::A4b313OldAmber);
    map.insert("A4b 314", CardId::A4b314RareCandy);
    map.insert("A4b 315", CardId::A4b315RareCandy);
    map.insert("A4b 316", CardId::A4b316PokemonCommunication);
    map.insert("A4b 317", CardId::A4b317PokemonCommunication);
    map.insert("A4b 318", CardId::A4b318ElectricalCord);
    map.insert("A4b 319", CardId::A4b319ElectricalCord);
    map.insert("A4b 320", CardId::A4b320GiantCape);
    map.insert("A4b 321", CardId::A4b321GiantCape);
    map.insert("A4b 322", CardId::A4b322RockyHelmet);
    map.insert("A4b 323", CardId::A4b323RockyHelmet);
    map.insert("A4b 324", CardId::A4b324LeafCape);
    map.insert("A4b 325", CardId::A4b325LeafCape);
    map.insert("A4b 326", CardId::A4b326Cyrus);
    map.insert("A4b 327", CardId::A4b327Cyrus);
    map.insert("A4b 328", CardId::A4b328Erika);
    map.insert("A4b 329", CardId::A4b329Erika);
    map.insert("A4b 330", CardId::A4b330Irida);
    map.insert("A4b 331", CardId::A4b331Irida);
    map.insert("A4b 332", CardId::A4b332Lyra);
    map.insert("A4b 333", CardId::A4b333Lyra);
    map.insert("A4b 334", CardId::A4b334Giovanni);
    map.insert("A4b 335", CardId::A4b335Giovanni);
    map.insert("A4b 336", CardId::A4b336Silver);
    map.insert("A4b 337", CardId::A4b337Silver);
    map.insert("A4b 338", CardId::A4b338Sabrina);
    map.insert("A4b 339", CardId::A4b339Sabrina);
    map.insert("A4b 340", CardId::A4b340Iono);
    map.insert("A4b 341", CardId::A4b341Iono);
    map.insert("A4b 342", CardId::A4b342Dawn);
    map.insert("A4b 343", CardId::A4b343Dawn);
    map.insert("A4b 344", CardId::A4b344Mars);
    map.insert("A4b 345", CardId::A4b345Mars);
    map.insert("A4b 346", CardId::A4b346Leaf);
    map.insert("A4b 347", CardId::A4b347Leaf);
    map.insert("A4b 348", CardId::A4b348Lillie);
    map.insert("A4b 349", CardId::A4b349Lillie);
    map.insert("A4b 350", CardId::A4b350Lusamine);
    map.insert("A4b 351", CardId::A4b351Lusamine);
    map.insert("A4b 352", CardId::A4b352Red);
    map.insert("A4b 353", CardId::A4b353Red);
    map.insert("A4b 354", CardId::A4b354Floragato);
    map.insert("A4b 355", CardId::A4b355Crawdaunt);
    map.insert("A4b 356", CardId::A4b356Greninja);
    map.insert("A4b 357", CardId::A4b357Gardevoir);
    map.insert("A4b 358", CardId::A4b358Slurpuff);
    map.insert("A4b 359", CardId::A4b359Farfetchd);
    map.insert("A4b 360", CardId::A4b360BuzzwoleEx);
    map.insert("A4b 361", CardId::A4b361CharizardEx);
    map.insert("A4b 362", CardId::A4b362HoOhEx);
    map.insert("A4b 363", CardId::A4b363PalkiaEx);
    map.insert("A4b 364", CardId::A4b364PikachuEx);
    map.insert("A4b 365", CardId::A4b365MewtwoEx);
    map.insert("A4b 366", CardId::A4b366MewEx);
    map.insert("A4b 367", CardId::A4b367LunalaEx);
    map.insert("A4b 368", CardId::A4b368DialgaEx);
    map.insert("A4b 369", CardId::A4b369SolgaleoEx);
    map.insert("A4b 370", CardId::A4b370EeveeEx);
    map.insert("A4b 371", CardId::A4b371LugiaEx);
    map.insert("A4b 372", CardId::A4b372ArceusEx);
    map.insert("A4b 373", CardId::A4b373ProfessorsResearch);
    map.insert("A4b 374", CardId::A4b374Lillie);
    map.insert("A4b 375", CardId::A4b375Lusamine);
    map.insert("A4b 376", CardId::A4b376PikachuEx);
    map.insert("A4b 377", CardId::A4b377GiratinaEx);
    map.insert("A4b 378", CardId::A4b378DarkraiEx);
    map.insert("A4b 379", CardId::A4b379RareCandy);
    map.insert("B1 001", CardId::B1001Pinsir);
    map.insert("B1 002", CardId::B1002MegaPinsirEx);
    map.insert("B1 003", CardId::B1003Wurmple);
    map.insert("B1 004", CardId::B1004Silcoon);
    map.insert("B1 005", CardId::B1005Beautifly);
    map.insert("B1 006", CardId::B1006Cascoon);
    map.insert("B1 007", CardId::B1007Dustox);
    map.insert("B1 008", CardId::B1008Seedot);
    map.insert("B1 009", CardId::B1009Nuzleaf);
    map.insert("B1 010", CardId::B1010Shiftry);
    map.insert("B1 011", CardId::B1011Shroomish);
    map.insert("B1 012", CardId::B1012Breloom);
    map.insert("B1 013", CardId::B1013Pansage);
    map.insert("B1 014", CardId::B1014Simisage);
    map.insert("B1 015", CardId::B1015Cottonee);
    map.insert("B1 016", CardId::B1016WhimsicottEx);
    map.insert("B1 017", CardId::B1017Petilil);
    map.insert("B1 018", CardId::B1018Lilligant);
    map.insert("B1 019", CardId::B1019Maractus);
    map.insert("B1 020", CardId::B1020Virizion);
    map.insert("B1 021", CardId::B1021Skiddo);
    map.insert("B1 022", CardId::B1022Gogoat);
    map.insert("B1 023", CardId::B1023Phantump);
    map.insert("B1 024", CardId::B1024Trevenant);
    map.insert("B1 025", CardId::B1025Grookey);
    map.insert("B1 026", CardId::B1026Thwackey);
    map.insert("B1 027", CardId::B1027Rillaboom);
    map.insert("B1 028", CardId::B1028Growlithe);
    map.insert("B1 029", CardId::B1029Arcanine);
    map.insert("B1 030", CardId::B1030Ponyta);
    map.insert("B1 031", CardId::B1031RapidashEx);
    map.insert("B1 032", CardId::B1032HoOh);
    map.insert("B1 033", CardId::B1033Torchic);
    map.insert("B1 034", CardId::B1034Combusken);
    map.insert("B1 035", CardId::B1035Blaziken);
    map.insert("B1 036", CardId::B1036MegaBlazikenEx);
    map.insert("B1 037", CardId::B1037Pansear);
    map.insert("B1 038", CardId::B1038Simisear);
    map.insert("B1 039", CardId::B1039Darumaka);
    map.insert("B1 040", CardId::B1040Darmanitan);
    map.insert("B1 041", CardId::B1041Litwick);
    map.insert("B1 042", CardId::B1042Lampent);
    map.insert("B1 043", CardId::B1043Chandelure);
    map.insert("B1 044", CardId::B1044Heatmor);
    map.insert("B1 045", CardId::B1045Litleo);
    map.insert("B1 046", CardId::B1046Pyroar);
    map.insert("B1 047", CardId::B1047Turtonator);
    map.insert("B1 048", CardId::B1048Psyduck);
    map.insert("B1 049", CardId::B1049Golduck);
    map.insert("B1 050", CardId::B1050Magikarp);
    map.insert("B1 051", CardId::B1051Gyarados);
    map.insert("B1 052", CardId::B1052MegaGyaradosEx);
    map.insert("B1 053", CardId::B1053Lotad);
    map.insert("B1 054", CardId::B1054Lombre);
    map.insert("B1 055", CardId::B1055Ludicolo);
    map.insert("B1 056", CardId::B1056Wailmer);
    map.insert("B1 057", CardId::B1057Wailord);
    map.insert("B1 058", CardId::B1058Corphish);
    map.insert("B1 059", CardId::B1059Crawdaunt);
    map.insert("B1 060", CardId::B1060Luvdisc);
    map.insert("B1 061", CardId::B1061Panpour);
    map.insert("B1 062", CardId::B1062Simipour);
    map.insert("B1 063", CardId::B1063Tympole);
    map.insert("B1 064", CardId::B1064Palpitoad);
    map.insert("B1 065", CardId::B1065Seismitoad);
    map.insert("B1 066", CardId::B1066Tirtouga);
    map.insert("B1 067", CardId::B1067Carracosta);
    map.insert("B1 068", CardId::B1068Frillish);
    map.insert("B1 069", CardId::B1069Jellicent);
    map.insert("B1 070", CardId::B1070Keldeo);
    map.insert("B1 071", CardId::B1071Froakie);
    map.insert("B1 072", CardId::B1072Frogadier);
    map.insert("B1 073", CardId::B1073GreninjaEx);
    map.insert("B1 074", CardId::B1074Bergmite);
    map.insert("B1 075", CardId::B1075Avalugg);
    map.insert("B1 076", CardId::B1076Chewtle);
    map.insert("B1 077", CardId::B1077Drednaw);
    map.insert("B1 078", CardId::B1078Arrokuda);
    map.insert("B1 079", CardId::B1079Barraskewda);
    map.insert("B1 080", CardId::B1080Eiscue);
    map.insert("B1 081", CardId::B1081JolteonEx);
    map.insert("B1 082", CardId::B1082Mareep);
    map.insert("B1 083", CardId::B1083Flaaffy);
    map.insert("B1 084", CardId::B1084Ampharos);
    map.insert("B1 085", CardId::B1085MegaAmpharosEx);
    map.insert("B1 086", CardId::B1086Shinx);
    map.insert("B1 087", CardId::B1087Luxio);
    map.insert("B1 088", CardId::B1088Luxray);
    map.insert("B1 089", CardId::B1089Pachirisu);
    map.insert("B1 090", CardId::B1090Blitzle);
    map.insert("B1 091", CardId::B1091Zebstrika);
    map.insert("B1 092", CardId::B1092Joltik);
    map.insert("B1 093", CardId::B1093Galvantula);
    map.insert("B1 094", CardId::B1094Dedenne);
    map.insert("B1 095", CardId::B1095Yamper);
    map.insert("B1 096", CardId::B1096Boltund);
    map.insert("B1 097", CardId::B1097Natu);
    map.insert("B1 098", CardId::B1098Xatu);
    map.insert("B1 099", CardId::B1099Misdreavus);
    map.insert("B1 100", CardId::B1100Mismagius);
    map.insert("B1 101", CardId::B1101Sableye);
    map.insert("B1 102", CardId::B1102MegaAltariaEx);
    map.insert("B1 103", CardId::B1103Duskull);
    map.insert("B1 104", CardId::B1104Dusclops);
    map.insert("B1 105", CardId::B1105Dusknoir);
    map.insert("B1 106", CardId::B1106Jirachi);
    map.insert("B1 107", CardId::B1107Drifloon);
    map.insert("B1 108", CardId::B1108Drifblim);
    map.insert("B1 109", CardId::B1109Chingling);
    map.insert("B1 110", CardId::B1110Yamask);
    map.insert("B1 111", CardId::B1111Cofagrigus);
    map.insert("B1 112", CardId::B1112Gothita);
    map.insert("B1 113", CardId::B1113Gothorita);
    map.insert("B1 114", CardId::B1114Gothitelle);
    map.insert("B1 115", CardId::B1115Spritzee);
    map.insert("B1 116", CardId::B1116Aromatisse);
    map.insert("B1 117", CardId::B1117Swirlix);
    map.insert("B1 118", CardId::B1118Slurpuff);
    map.insert("B1 119", CardId::B1119Carbink);
    map.insert("B1 120", CardId::B1120Klefki);
    map.insert("B1 121", CardId::B1121IndeedeeEx);
    map.insert("B1 122", CardId::B1122Sandshrew);
    map.insert("B1 123", CardId::B1123Sandslash);
    map.insert("B1 124", CardId::B1124HitmonchanEx);
    map.insert("B1 125", CardId::B1125Sudowoodo);
    map.insert("B1 126", CardId::B1126Makuhita);
    map.insert("B1 127", CardId::B1127Hariyama);
    map.insert("B1 128", CardId::B1128Hippopotas);
    map.insert("B1 129", CardId::B1129Hippowdon);
    map.insert("B1 130", CardId::B1130Sandile);
    map.insert("B1 131", CardId::B1131Krokorok);
    map.insert("B1 132", CardId::B1132Krookodile);
    map.insert("B1 133", CardId::B1133Archen);
    map.insert("B1 134", CardId::B1134Archeops);
    map.insert("B1 135", CardId::B1135Golett);
    map.insert("B1 136", CardId::B1136Golurk);
    map.insert("B1 137", CardId::B1137Terrakion);
    map.insert("B1 138", CardId::B1138Pancham);
    map.insert("B1 139", CardId::B1139Crabrawler);
    map.insert("B1 140", CardId::B1140Crabominable);
    map.insert("B1 141", CardId::B1141Stufful);
    map.insert("B1 142", CardId::B1142Bewear);
    map.insert("B1 143", CardId::B1143Sandygast);
    map.insert("B1 144", CardId::B1144Palossand);
    map.insert("B1 145", CardId::B1145Rolycoly);
    map.insert("B1 146", CardId::B1146Carkol);
    map.insert("B1 147", CardId::B1147Coalossal);
    map.insert("B1 148", CardId::B1148Murkrow);
    map.insert("B1 149", CardId::B1149Honchkrow);
    map.insert("B1 150", CardId::B1150Absol);
    map.insert("B1 151", CardId::B1151MegaAbsolEx);
    map.insert("B1 152", CardId::B1152Skorupi);
    map.insert("B1 153", CardId::B1153Drapion);
    map.insert("B1 154", CardId::B1154Darkrai);
    map.insert("B1 155", CardId::B1155Deino);
    map.insert("B1 156", CardId::B1156Zweilous);
    map.insert("B1 157", CardId::B1157Hydreigon);
    map.insert("B1 158", CardId::B1158Pangoro);
    map.insert("B1 159", CardId::B1159Skrelp);
    map.insert("B1 160", CardId::B1160DragalgeEx);
    map.insert("B1 161", CardId::B1161Mareanie);
    map.insert("B1 162", CardId::B1162ToxapEx);
    map.insert("B1 163", CardId::B1163Impidimp);
    map.insert("B1 164", CardId::B1164Morgrem);
    map.insert("B1 165", CardId::B1165Grimmsnarl);
    map.insert("B1 166", CardId::B1166Ferroseed);
    map.insert("B1 167", CardId::B1167Ferrothorn);
    map.insert("B1 168", CardId::B1168Durant);
    map.insert("B1 169", CardId::B1169Cobalion);
    map.insert("B1 170", CardId::B1170Honedge);
    map.insert("B1 171", CardId::B1171Doublade);
    map.insert("B1 172", CardId::B1172Aegislash);
    map.insert("B1 173", CardId::B1173Meltan);
    map.insert("B1 174", CardId::B1174MelmetalEx);
    map.insert("B1 175", CardId::B1175Corviknight);
    map.insert("B1 176", CardId::B1176Druddigon);
    map.insert("B1 177", CardId::B1177Goomy);
    map.insert("B1 178", CardId::B1178Sliggoo);
    map.insert("B1 179", CardId::B1179Goodra);
    map.insert("B1 180", CardId::B1180Pidgey);
    map.insert("B1 181", CardId::B1181Pidgeotto);
    map.insert("B1 182", CardId::B1182Pidgeot);
    map.insert("B1 183", CardId::B1183TaurosEx);
    map.insert("B1 184", CardId::B1184Eevee);
    map.insert("B1 185", CardId::B1185Aipom);
    map.insert("B1 186", CardId::B1186Ambipom);
    map.insert("B1 187", CardId::B1187Miltank);
    map.insert("B1 188", CardId::B1188Zigzagoon);
    map.insert("B1 189", CardId::B1189Linoone);
    map.insert("B1 190", CardId::B1190Whismur);
    map.insert("B1 191", CardId::B1191Loudred);
    map.insert("B1 192", CardId::B1192Exploud);
    map.insert("B1 193", CardId::B1193Skitty);
    map.insert("B1 194", CardId::B1194Delcatty);
    map.insert("B1 195", CardId::B1195Spinda);
    map.insert("B1 196", CardId::B1196Swablu);
    map.insert("B1 197", CardId::B1197Altaria);
    map.insert("B1 198", CardId::B1198Chatot);
    map.insert("B1 199", CardId::B1199Patrat);
    map.insert("B1 200", CardId::B1200Watchog);
    map.insert("B1 201", CardId::B1201Lillipup);
    map.insert("B1 202", CardId::B1202Herdier);
    map.insert("B1 203", CardId::B1203Stoutland);
    map.insert("B1 204", CardId::B1204Rufflet);
    map.insert("B1 205", CardId::B1205Braviary);
    map.insert("B1 206", CardId::B1206Furfrou);
    map.insert("B1 207", CardId::B1207Furfrou);
    map.insert("B1 208", CardId::B1208Furfrou);
    map.insert("B1 209", CardId::B1209Rookidee);
    map.insert("B1 210", CardId::B1210Corvisquire);
    map.insert("B1 211", CardId::B1211Wooloo);
    map.insert("B1 212", CardId::B1212Dubwool);
    map.insert("B1 213", CardId::B1213PrankSpinner);
    map.insert("B1 214", CardId::B1214PlumeFossil);
    map.insert("B1 215", CardId::B1215HittingHammer);
    map.insert("B1 216", CardId::B1216CoverFossil);
    map.insert("B1 217", CardId::B1217FlamePatch);
    map.insert("B1 218", CardId::B1218SitrusBerry);
    map.insert("B1 219", CardId::B1219HeavyHelmet);
    map.insert("B1 220", CardId::B1220LuckyMittens);
    map.insert("B1 221", CardId::B1221Marlon);
    map.insert("B1 222", CardId::B1222Hala);
    map.insert("B1 223", CardId::B1223May);
    map.insert("B1 224", CardId::B1224Fantina);
    map.insert("B1 225", CardId::B1225Copycat);
    map.insert("B1 226", CardId::B1226Lisia);
    map.insert("B1 227", CardId::B1227Beautifly);
    map.insert("B1 228", CardId::B1228Skiddo);
    map.insert("B1 229", CardId::B1229Rillaboom);
    map.insert("B1 230", CardId::B1230Growlithe);
    map.insert("B1 231", CardId::B1231Chandelure);
    map.insert("B1 232", CardId::B1232Magikarp);
    map.insert("B1 233", CardId::B1233Ludicolo);
    map.insert("B1 234", CardId::B1234Jellicent);
    map.insert("B1 235", CardId::B1235Keldeo);
    map.insert("B1 236", CardId::B1236Eiscue);
    map.insert("B1 237", CardId::B1237Luxray);
    map.insert("B1 238", CardId::B1238Cofagrigus);
    map.insert("B1 239", CardId::B1239Gothita);
    map.insert("B1 240", CardId::B1240Makuhita);
    map.insert("B1 241", CardId::B1241Hippowdon);
    map.insert("B1 242", CardId::B1242Archeops);
    map.insert("B1 243", CardId::B1243Pancham);
    map.insert("B1 244", CardId::B1244Honchkrow);
    map.insert("B1 245", CardId::B1245Hydreigon);
    map.insert("B1 246", CardId::B1246Corviknight);
    map.insert("B1 247", CardId::B1247Goomy);
    map.insert("B1 248", CardId::B1248Delcatty);
    map.insert("B1 249", CardId::B1249Stoutland);
    map.insert("B1 250", CardId::B1250Rufflet);
    map.insert("B1 251", CardId::B1251MegaPinsirEx);
    map.insert("B1 252", CardId::B1252WhimsicottEx);
    map.insert("B1 253", CardId::B1253RapidashEx);
    map.insert("B1 254", CardId::B1254MegaBlazikenEx);
    map.insert("B1 255", CardId::B1255MegaGyaradosEx);
    map.insert("B1 256", CardId::B1256GreninjaEx);
    map.insert("B1 257", CardId::B1257JolteonEx);
    map.insert("B1 258", CardId::B1258MegaAmpharosEx);
    map.insert("B1 259", CardId::B1259MegaAltariaEx);
    map.insert("B1 260", CardId::B1260IndeedeeEx);
    map.insert("B1 261", CardId::B1261HitmonchanEx);
    map.insert("B1 262", CardId::B1262MegaAbsolEx);
    map.insert("B1 263", CardId::B1263DragalgeEx);
    map.insert("B1 264", CardId::B1264MelmetalEx);
    map.insert("B1 265", CardId::B1265TaurosEx);
    map.insert("B1 266", CardId::B1266Marlon);
    map.insert("B1 267", CardId::B1267Hala);
    map.insert("B1 268", CardId::B1268May);
    map.insert("B1 269", CardId::B1269Fantina);
    map.insert("B1 270", CardId::B1270Copycat);
    map.insert("B1 271", CardId::B1271Lisia);
    map.insert("B1 272", CardId::B1272MegaPinsirEx);
    map.insert("B1 273", CardId::B1273WhimsicottEx);
    map.insert("B1 274", CardId::B1274RapidashEx);
    map.insert("B1 275", CardId::B1275GreninjaEx);
    map.insert("B1 276", CardId::B1276JolteonEx);
    map.insert("B1 277", CardId::B1277MegaAmpharosEx);
    map.insert("B1 278", CardId::B1278IndeedeeEx);
    map.insert("B1 279", CardId::B1279HitmonchanEx);
    map.insert("B1 280", CardId::B1280MegaAbsolEx);
    map.insert("B1 281", CardId::B1281DragalgeEx);
    map.insert("B1 282", CardId::B1282MelmetalEx);
    map.insert("B1 283", CardId::B1283TaurosEx);
    map.insert("B1 284", CardId::B1284MegaBlazikenEx);
    map.insert("B1 285", CardId::B1285MegaGyaradosEx);
    map.insert("B1 286", CardId::B1286MegaAltariaEx);
    map.insert("B1 287", CardId::B1287Bellsprout);
    map.insert("B1 288", CardId::B1288Weepinbell);
    map.insert("B1 289", CardId::B1289Victreebel);
    map.insert("B1 290", CardId::B1290Rowlet);
    map.insert("B1 291", CardId::B1291Dartrix);
    map.insert("B1 292", CardId::B1292Moltres);
    map.insert("B1 293", CardId::B1293Litten);
    map.insert("B1 294", CardId::B1294Torracat);
    map.insert("B1 295", CardId::B1295Poliwag);
    map.insert("B1 296", CardId::B1296Poliwhirl);
    map.insert("B1 297", CardId::B1297Poliwrath);
    map.insert("B1 298", CardId::B1298Articuno);
    map.insert("B1 299", CardId::B1299Manaphy);
    map.insert("B1 300", CardId::B1300Popplio);
    map.insert("B1 301", CardId::B1301Brionne);
    map.insert("B1 302", CardId::B1302Zapdos);
    map.insert("B1 303", CardId::B1303Oricorio);
    map.insert("B1 304", CardId::B1304Zeraora);
    map.insert("B1 305", CardId::B1305Drowzee);
    map.insert("B1 306", CardId::B1306Hypno);
    map.insert("B1 307", CardId::B1307Geodude);
    map.insert("B1 308", CardId::B1308Graveler);
    map.insert("B1 309", CardId::B1309Golem);
    map.insert("B1 310", CardId::B1310Rockruff);
    map.insert("B1 311", CardId::B1311AlolanDiglett);
    map.insert("B1 312", CardId::B1312Meowth);
    map.insert("B1 313", CardId::B1313Persian);
    map.insert("B1 314", CardId::B1314Doduo);
    map.insert("B1 315", CardId::B1315Dodrio);
    map.insert("B1 316", CardId::B1316Bidoof);
    map.insert("B1 317", CardId::B1317DecidueyeEx);
    map.insert("B1 318", CardId::B1318IncineroarEx);
    map.insert("B1 319", CardId::B1319PalkiaEx);
    map.insert("B1 320", CardId::B1320PrimarinaEx);
    map.insert("B1 321", CardId::B1321PikachuEx);
    map.insert("B1 322", CardId::B1322TapuKokoEx);
    map.insert("B1 323", CardId::B1323LycanrocEx);
    map.insert("B1 324", CardId::B1324PassimianEx);
    map.insert("B1 325", CardId::B1325AlolanDugtrioEx);
    map.insert("B1 326", CardId::B1326DialgaEx);
    map.insert("B1 327", CardId::B1327BibarelEx);
    map.insert("B1 328", CardId::B1328ArceusEx);
    map.insert("B1 329", CardId::B1329Lilligant);
    map.insert("B1 330", CardId::B1330Klefki);
    map.insert("B1 331", CardId::B1331FlamePatch);
    map.insert("B1a 001", CardId::B1a001Bulbasaur);
    map.insert("B1a 002", CardId::B1a002Ivysaur);
    map.insert("B1a 003", CardId::B1a003Venusaur);
    map.insert("B1a 004", CardId::B1a004MegaVenusaurEx);
    map.insert("B1a 005", CardId::B1a005Spinarak);
    map.insert("B1a 006", CardId::B1a006Ariados);
    map.insert("B1a 007", CardId::B1a007Sunkern);
    map.insert("B1a 008", CardId::B1a008Sunflora);
    map.insert("B1a 009", CardId::B1a009Burmy);
    map.insert("B1a 010", CardId::B1a010Mothim);
    map.insert("B1a 011", CardId::B1a011Charmander);
    map.insert("B1a 012", CardId::B1a012Charmeleon);
    map.insert("B1a 013", CardId::B1a013Charizard);
    map.insert("B1a 014", CardId::B1a014MegaCharizardYEx);
    map.insert("B1a 015", CardId::B1a015Houndour);
    map.insert("B1a 016", CardId::B1a016Houndoom);
    map.insert("B1a 017", CardId::B1a017Squirtle);
    map.insert("B1a 018", CardId::B1a018Wartortle);
    map.insert("B1a 019", CardId::B1a019Blastoise);
    map.insert("B1a 020", CardId::B1a020MegaBlastoiseEx);
    map.insert("B1a 021", CardId::B1a021Basculin);
    map.insert("B1a 022", CardId::B1a022Clauncher);
    map.insert("B1a 023", CardId::B1a023Clawitzer);
    map.insert("B1a 024", CardId::B1a024Magnemite);
    map.insert("B1a 025", CardId::B1a025Magneton);
    map.insert("B1a 026", CardId::B1a026Magnezone);
    map.insert("B1a 027", CardId::B1a027Emolga);
    map.insert("B1a 028", CardId::B1a028Helioptile);
    map.insert("B1a 029", CardId::B1a029Heliolisk);
    map.insert("B1a 030", CardId::B1a030Misdreavus);
    map.insert("B1a 031", CardId::B1a031Mismagius);
    map.insert("B1a 032", CardId::B1a032Solosis);
    map.insert("B1a 033", CardId::B1a033Duosion);
    map.insert("B1a 034", CardId::B1a034Reuniclus);
    map.insert("B1a 035", CardId::B1a035Spritzee);
    map.insert("B1a 036", CardId::B1a036Aromatisse);
    map.insert("B1a 037", CardId::B1a037Xerneas);
    map.insert("B1a 038", CardId::B1a038Onix);
    map.insert("B1a 039", CardId::B1a039Makuhita);
    map.insert("B1a 040", CardId::B1a040Hariyama);
    map.insert("B1a 041", CardId::B1a041Nosepass);
    map.insert("B1a 042", CardId::B1a042MegaLopunnyEx);
    map.insert("B1a 043", CardId::B1a043Mienfoo);
    map.insert("B1a 044", CardId::B1a044Mienshao);
    map.insert("B1a 045", CardId::B1a045Grimer);
    map.insert("B1a 046", CardId::B1a046Muk);
    map.insert("B1a 047", CardId::B1a047Purrloin);
    map.insert("B1a 048", CardId::B1a048Liepard);
    map.insert("B1a 049", CardId::B1a049Trubbish);
    map.insert("B1a 050", CardId::B1a050Garbodor);
    map.insert("B1a 051", CardId::B1a051Steelix);
    map.insert("B1a 052", CardId::B1a052MegaSteelixEx);
    map.insert("B1a 053", CardId::B1a053Probopass);
    map.insert("B1a 054", CardId::B1a054Genesect);
    map.insert("B1a 055", CardId::B1a055Ditto);
    map.insert("B1a 056", CardId::B1a056Porygon);
    map.insert("B1a 057", CardId::B1a057Porygon2);
    map.insert("B1a 058", CardId::B1a058PorygonZ);
    map.insert("B1a 059", CardId::B1a059Starly);
    map.insert("B1a 060", CardId::B1a060Staravia);
    map.insert("B1a 061", CardId::B1a061Staraptor);
    map.insert("B1a 062", CardId::B1a062Buneary);
    map.insert("B1a 063", CardId::B1a063Lopunny);
    map.insert("B1a 064", CardId::B1a064Bouffalant);
    map.insert("B1a 065", CardId::B1a065Furfrou);
    map.insert("B1a 066", CardId::B1a066ClemontsBackpack);
    map.insert("B1a 067", CardId::B1a067QuickGrowExtract);
    map.insert("B1a 068", CardId::B1a068Clemont);
    map.insert("B1a 069", CardId::B1a069Serena);
    map.insert("B1a 070", CardId::B1a070Ariados);
    map.insert("B1a 071", CardId::B1a071Sunflora);
    map.insert("B1a 072", CardId::B1a072Reuniclus);
    map.insert("B1a 073", CardId::B1a073Xerneas);
    map.insert("B1a 074", CardId::B1a074Trubbish);
    map.insert("B1a 075", CardId::B1a075Buneary);
    map.insert("B1a 076", CardId::B1a076MegaVenusaurEx);
    map.insert("B1a 077", CardId::B1a077MegaCharizardYEx);
    map.insert("B1a 078", CardId::B1a078MegaBlastoiseEx);
    map.insert("B1a 079", CardId::B1a079MegaLopunnyEx);
    map.insert("B1a 080", CardId::B1a080MegaSteelixEx);
    map.insert("B1a 081", CardId::B1a081Clemont);
    map.insert("B1a 082", CardId::B1a082Serena);
    map.insert("B1a 083", CardId::B1a083MegaVenusaurEx);
    map.insert("B1a 084", CardId::B1a084MegaBlastoiseEx);
    map.insert("B1a 085", CardId::B1a085MegaLopunnyEx);
    map.insert("B1a 086", CardId::B1a086MegaSteelixEx);
    map.insert("B1a 087", CardId::B1a087MegaCharizardYEx);
    map.insert("B1a 088", CardId::B1a088Oddish);
    map.insert("B1a 089", CardId::B1a089Gloom);
    map.insert("B1a 090", CardId::B1a090Vileplume);
    map.insert("B1a 091", CardId::B1a091Charizard);
    map.insert("B1a 092", CardId::B1a092Shellder);
    map.insert("B1a 093", CardId::B1a093Cloyster);
    map.insert("B1a 094", CardId::B1a094Sandshrew);
    map.insert("B1a 095", CardId::B1a095Sandslash);
    map.insert("B1a 096", CardId::B1a096TypeNull);
    map.insert("B1a 097", CardId::B1a097Silvally);
    map.insert("B1a 098", CardId::B1a098BuzzwoleEx);
    map.insert("B1a 099", CardId::B1a099LunalaEx);
    map.insert("B1a 100", CardId::B1a100GuzzlordEx);
    map.insert("B1a 101", CardId::B1a101SolgaleoEx);
    map.insert("B1a 102", CardId::B1a102Aegislash);
    map.insert("B1a 103", CardId::B1a103QuickGrowExtract);
    map.insert("B2 001", CardId::B2001Ledyba);
    map.insert("B2 002", CardId::B2002Ledian);
    map.insert("B2 003", CardId::B2003Shuckle);
    map.insert("B2 004", CardId::B2004Roselia);
    map.insert("B2 005", CardId::B2005Roserade);
    map.insert("B2 006", CardId::B2006Cacnea);
    map.insert("B2 007", CardId::B2007Cacturne);
    map.insert("B2 008", CardId::B2008Chespin);
    map.insert("B2 009", CardId::B2009Quilladin);
    map.insert("B2 010", CardId::B2010Chesnaught);
    map.insert("B2 011", CardId::B2011Scatterbug);
    map.insert("B2 012", CardId::B2012Spewpa);
    map.insert("B2 013", CardId::B2013Vivillon);
    map.insert("B2 014", CardId::B2014Buzzwole);
    map.insert("B2 015", CardId::B2015Gossifleur);
    map.insert("B2 016", CardId::B2016Eldegoss);
    map.insert("B2 017", CardId::B2017TealMaskOgerponEx);
    map.insert("B2 018", CardId::B2018AlolanMarowak);
    map.insert("B2 019", CardId::B2019Reshiram);
    map.insert("B2 020", CardId::B2020Litleo);
    map.insert("B2 021", CardId::B2021Pyroar);
    map.insert("B2 022", CardId::B2022Oricorio);
    map.insert("B2 023", CardId::B2023BlacephalonEx);
    map.insert("B2 024", CardId::B2024Scorbunny);
    map.insert("B2 025", CardId::B2025Raboot);
    map.insert("B2 026", CardId::B2026Cinderace);
    map.insert("B2 027", CardId::B2027HearthflameMaskOgerpon);
    map.insert("B2 028", CardId::B2028AlolanVulpix);
    map.insert("B2 029", CardId::B2029AlolanNinetalesEx);
    map.insert("B2 030", CardId::B2030GalarianMrMime);
    map.insert("B2 031", CardId::B2031GalarianMrRime);
    map.insert("B2 032", CardId::B2032Delibird);
    map.insert("B2 033", CardId::B2033Mudkip);
    map.insert("B2 034", CardId::B2034Marshtomp);
    map.insert("B2 035", CardId::B2035Swampert);
    map.insert("B2 036", CardId::B2036MegaSwampertEx);
    map.insert("B2 037", CardId::B2037Vanillite);
    map.insert("B2 038", CardId::B2038Vanillish);
    map.insert("B2 039", CardId::B2039Vanilluxe);
    map.insert("B2 040", CardId::B2040Cryogonal);
    map.insert("B2 041", CardId::B2041Amaura);
    map.insert("B2 042", CardId::B2042Aurorus);
    map.insert("B2 043", CardId::B2043Chewtle);
    map.insert("B2 044", CardId::B2044Drednaw);
    map.insert("B2 045", CardId::B2045Cramorant);
    map.insert("B2 046", CardId::B2046Arrokuda);
    map.insert("B2 047", CardId::B2047Barraskewda);
    map.insert("B2 048", CardId::B2048WellspringMaskOgerpon);
    map.insert("B2 049", CardId::B2049Pikachu);
    map.insert("B2 050", CardId::B2050AlolanRaichu);
    map.insert("B2 051", CardId::B2051Zapdos);
    map.insert("B2 052", CardId::B2052Plusle);
    map.insert("B2 053", CardId::B2053Minun);
    map.insert("B2 054", CardId::B2054Toxel);
    map.insert("B2 055", CardId::B2055ToxtricityEx);
    map.insert("B2 056", CardId::B2056Tadbulb);
    map.insert("B2 057", CardId::B2057Bellibolt);
    map.insert("B2 058", CardId::B2058GalarianPonyta);
    map.insert("B2 059", CardId::B2059GalarianRapidash);
    map.insert("B2 060", CardId::B2060Wobbuffet);
    map.insert("B2 061", CardId::B2061Snubbull);
    map.insert("B2 062", CardId::B2062Granbull);
    map.insert("B2 063", CardId::B2063Ralts);
    map.insert("B2 064", CardId::B2064Kirlia);
    map.insert("B2 065", CardId::B2065Gardevoir);
    map.insert("B2 066", CardId::B2066MegaGardevoirEx);
    map.insert("B2 067", CardId::B2067Litwick);
    map.insert("B2 068", CardId::B2068Lampent);
    map.insert("B2 069", CardId::B2069Chandelure);
    map.insert("B2 070", CardId::B2070Meloetta);
    map.insert("B2 071", CardId::B2071Pumpkaboo);
    map.insert("B2 072", CardId::B2072Gourgeist);
    map.insert("B2 073", CardId::B2073MimikyuEx);
    map.insert("B2 074", CardId::B2074Sinistea);
    map.insert("B2 075", CardId::B2075Polteageist);
    map.insert("B2 076", CardId::B2076Indeedee);
    map.insert("B2 077", CardId::B2077Sandshrew);
    map.insert("B2 078", CardId::B2078Sandslash);
    map.insert("B2 079", CardId::B2079Machop);
    map.insert("B2 080", CardId::B2080Machoke);
    map.insert("B2 081", CardId::B2081Machamp);
    map.insert("B2 082", CardId::B2082Cubone);
    map.insert("B2 083", CardId::B2083Meditite);
    map.insert("B2 084", CardId::B2084Medicham);
    map.insert("B2 085", CardId::B2085Roggenrola);
    map.insert("B2 086", CardId::B2086Boldore);
    map.insert("B2 087", CardId::B2087GigalithEx);
    map.insert("B2 088", CardId::B2088Drilbur);
    map.insert("B2 089", CardId::B2089Tyrunt);
    map.insert("B2 090", CardId::B2090Tyrantrum);
    map.insert("B2 091", CardId::B2091Passimian);
    map.insert("B2 093", CardId::B2093CornerstoneMaskOgerpon);
    map.insert("B2 094", CardId::B2094AlolanMeowth);
    map.insert("B2 095", CardId::B2095AlolanPersian);
    map.insert("B2 096", CardId::B2096AlolanGrimer);
    map.insert("B2 097", CardId::B2097AlolanMuk);
    map.insert("B2 098", CardId::B2098GalarianZigzagoon);
    map.insert("B2 099", CardId::B2099GalarianLinoone);
    map.insert("B2 100", CardId::B2100GalarianObstagoon);
    map.insert("B2 101", CardId::B2101Stunky);
    map.insert("B2 102", CardId::B2102Skuntank);
    map.insert("B2 103", CardId::B2103Spiritomb);
    map.insert("B2 104", CardId::B2104Purrloin);
    map.insert("B2 105", CardId::B2105Liepard);
    map.insert("B2 106", CardId::B2106Scraggy);
    map.insert("B2 107", CardId::B2107Scrafty);
    map.insert("B2 108", CardId::B2108Yveltal);
    map.insert("B2 109", CardId::B2109Guzzlord);
    map.insert("B2 110", CardId::B2110GalarianMeowth);
    map.insert("B2 111", CardId::B2111GalarianPerrserker);
    map.insert("B2 112", CardId::B2112Mawile);
    map.insert("B2 113", CardId::B2113MegaMawileEx);
    map.insert("B2 114", CardId::B2114Excadrill);
    map.insert("B2 115", CardId::B2115Ferroseed);
    map.insert("B2 116", CardId::B2116Ferrothorn);
    map.insert("B2 117", CardId::B2117GalarianStunfisk);
    map.insert("B2 118", CardId::B2118Honedge);
    map.insert("B2 119", CardId::B2119Doublade);
    map.insert("B2 120", CardId::B2120Aegislash);
    map.insert("B2 121", CardId::B2121Bagon);
    map.insert("B2 122", CardId::B2122Shelgon);
    map.insert("B2 123", CardId::B2123Salamence);
    map.insert("B2 124", CardId::B2124Meowth);
    map.insert("B2 125", CardId::B2125Persian);
    map.insert("B2 126", CardId::B2126Kangaskhan);
    map.insert("B2 127", CardId::B2127MegaKangaskhanEx);
    map.insert("B2 128", CardId::B2128Sentret);
    map.insert("B2 129", CardId::B2129Furret);
    map.insert("B2 130", CardId::B2130Smeargle);
    map.insert("B2 131", CardId::B2131Lugia);
    map.insert("B2 132", CardId::B2132Taillow);
    map.insert("B2 133", CardId::B2133Swellow);
    map.insert("B2 134", CardId::B2134Slakoth);
    map.insert("B2 135", CardId::B2135Vigoroth);
    map.insert("B2 136", CardId::B2136Slaking);
    map.insert("B2 137", CardId::B2137Spinda);
    map.insert("B2 138", CardId::B2138Tornadus);
    map.insert("B2 139", CardId::B2139Bunnelby);
    map.insert("B2 140", CardId::B2140Diggersby);
    map.insert("B2 141", CardId::B2141Furfrou);
    map.insert("B2 142", CardId::B2142Tandemaus);
    map.insert("B2 143", CardId::B2143Maushold);
    map.insert("B2 144", CardId::B2144JawFossil);
    map.insert("B2 145", CardId::B2145LuckyIcePop);
    map.insert("B2 146", CardId::B2146SailFossil);
    map.insert("B2 147", CardId::B2147ProtectivePoncho);
    map.insert("B2 148", CardId::B2148MetalCoreBarrier);
    map.insert("B2 149", CardId::B2149Diantha);
    map.insert("B2 150", CardId::B2150Sightseer);
    map.insert("B2 151", CardId::B2151Juggler);
    map.insert("B2 152", CardId::B2152Piers);
    map.insert("B2 153", CardId::B2153TrainingArea);
    map.insert("B2 154", CardId::B2154StartingPlains);
    map.insert("B2 155", CardId::B2155PeculiarPlaza);
    map.insert("B2 156", CardId::B2156Cacnea);
    map.insert("B2 157", CardId::B2157Roserade);
    map.insert("B2 158", CardId::B2158Vivillon);
    map.insert("B2 159", CardId::B2159Buzzwole);
    map.insert("B2 160", CardId::B2160Reshiram);
    map.insert("B2 161", CardId::B2161Oricorio);
    map.insert("B2 162", CardId::B2162Scorbunny);
    map.insert("B2 163", CardId::B2163Aurorus);
    map.insert("B2 164", CardId::B2164Cramorant);
    map.insert("B2 165", CardId::B2165Minun);
    map.insert("B2 166", CardId::B2166Toxel);
    map.insert("B2 167", CardId::B2167GalarianPonyta);
    map.insert("B2 168", CardId::B2168Snubbull);
    map.insert("B2 169", CardId::B2169Indeedee);
    map.insert("B2 170", CardId::B2170Sandshrew);
    map.insert("B2 171", CardId::B2171Tyrantrum);
    map.insert("B2 172", CardId::B2172Falinks);
    map.insert("B2 173", CardId::B2173AlolanMuk);
    map.insert("B2 174", CardId::B2174Purrloin);
    map.insert("B2 175", CardId::B2175Yveltal);
    map.insert("B2 176", CardId::B2176GalarianObstagoon);
    map.insert("B2 177", CardId::B2177GalarianPerrserker);
    map.insert("B2 178", CardId::B2178Salamence);
    map.insert("B2 179", CardId::B2179Slakoth);
    map.insert("B2 180", CardId::B2180TealMaskOgerponEx);
    map.insert("B2 181", CardId::B2181BlacephalonEx);
    map.insert("B2 182", CardId::B2182AlolanNinetalesEx);
    map.insert("B2 183", CardId::B2183MegaSwampertEx);
    map.insert("B2 184", CardId::B2184ToxtricityEx);
    map.insert("B2 185", CardId::B2185MegaGardevoirEx);
    map.insert("B2 186", CardId::B2186MimikyuEx);
    map.insert("B2 187", CardId::B2187GigalithEx);
    map.insert("B2 188", CardId::B2188MegaMawileEx);
    map.insert("B2 189", CardId::B2189MegaKangaskhanEx);
    map.insert("B2 190", CardId::B2190Diantha);
    map.insert("B2 191", CardId::B2191Sightseer);
    map.insert("B2 192", CardId::B2192Juggler);
    map.insert("B2 193", CardId::B2193Piers);
    map.insert("B2 194", CardId::B2194TealMaskOgerponEx);
    map.insert("B2 195", CardId::B2195BlacephalonEx);
    map.insert("B2 196", CardId::B2196AlolanNinetalesEx);
    map.insert("B2 197", CardId::B2197MegaSwampertEx);
    map.insert("B2 198", CardId::B2198ToxtricityEx);
    map.insert("B2 199", CardId::B2199MimikyuEx);
    map.insert("B2 200", CardId::B2200GigalithEx);
    map.insert("B2 201", CardId::B2201MegaMawileEx);
    map.insert("B2 202", CardId::B2202MegaKangaskhanEx);
    map.insert("B2 203", CardId::B2203MegaGardevoirEx);
    map.insert("B2 204", CardId::B2204Meowth);
    map.insert("B2 205", CardId::B2205Tangela);
    map.insert("B2 206", CardId::B2206Magby);
    map.insert("B2 207", CardId::B2207Magmar);
    map.insert("B2 208", CardId::B2208Horsea);
    map.insert("B2 209", CardId::B2209Seadra);
    map.insert("B2 210", CardId::B2210Mantyke);
    map.insert("B2 211", CardId::B2211Omanyte);
    map.insert("B2 212", CardId::B2212Omastar);
    map.insert("B2 213", CardId::B2213Pichu);
    map.insert("B2 214", CardId::B2214Clefairy);
    map.insert("B2 215", CardId::B2215Clefable);
    map.insert("B2 216", CardId::B2216Latias);
    map.insert("B2 217", CardId::B2217Latios);
    map.insert("B2 218", CardId::B2218Hitmonlee);
    map.insert("B2 219", CardId::B2219Hitmonchan);
    map.insert("B2 220", CardId::B2220Kabuto);
    map.insert("B2 221", CardId::B2221Kabutops);
    map.insert("B2 222", CardId::B2222Phanpy);
    map.insert("B2 223", CardId::B2223Tyrogue);
    map.insert("B2 224", CardId::B2224Tauros);
    map.insert("B2 225", CardId::B2225FlareonEx);
    map.insert("B2 226", CardId::B2226HoOhEx);
    map.insert("B2 227", CardId::B2227KingdraEx);
    map.insert("B2 228", CardId::B2228EspeonEx);
    map.insert("B2 229", CardId::B2229SylveonEx);
    map.insert("B2 230", CardId::B2230DonphanEx);
    map.insert("B2 231", CardId::B2231UmbreonEx);
    map.insert("B2 232", CardId::B2232LugiaEx);
    map.insert("B2 233", CardId::B2233Meloetta);
    map.insert("B2 234", CardId::B2234ProtectivePoncho);
    map.insert("P-A 001", CardId::PA001Potion);
    map.insert("P-A 002", CardId::PA002XSpeed);
    map.insert("P-A 003", CardId::PA003HandScope);
    map.insert("P-A 004", CardId::PA004PokedEx);
    map.insert("P-A 005", CardId::PA005PokeBall);
    map.insert("P-A 006", CardId::PA006RedCard);
    map.insert("P-A 007", CardId::PA007ProfessorsResearch);
    map.insert("P-A 008", CardId::PA008PokedEx);
    map.insert("P-A 009", CardId::PA009Pikachu);
    map.insert("P-A 010", CardId::PA010Mewtwo);
    map.insert("P-A 011", CardId::PA011Chansey);
    map.insert("P-A 012", CardId::PA012Meowth);
    map.insert("P-A 013", CardId::PA013Butterfree);
    map.insert("P-A 014", CardId::PA014LaprasEx);
    map.insert("P-A 015", CardId::PA015Pikachu);
    map.insert("P-A 016", CardId::PA016Clefairy);
    map.insert("P-A 017", CardId::PA017Mankey);
    map.insert("P-A 018", CardId::PA018Venusaur);
    map.insert("P-A 019", CardId::PA019Greninja);
    map.insert("P-A 020", CardId::PA020Haunter);
    map.insert("P-A 021", CardId::PA021Onix);
    map.insert("P-A 022", CardId::PA022Jigglypuff);
    map.insert("P-A 023", CardId::PA023Bulbasaur);
    map.insert("P-A 024", CardId::PA024Magnemite);
    map.insert("P-A 025", CardId::PA025MoltresEx);
    map.insert("P-A 026", CardId::PA026Pikachu);
    map.insert("P-A 027", CardId::PA027Snivy);
    map.insert("P-A 028", CardId::PA028Volcarona);
    map.insert("P-A 029", CardId::PA029Blastoise);
    map.insert("P-A 030", CardId::PA030Eevee);
    map.insert("P-A 031", CardId::PA031Cinccino);
    map.insert("P-A 032", CardId::PA032Charmander);
    map.insert("P-A 033", CardId::PA033Squirtle);
    map.insert("P-A 034", CardId::PA034Piplup);
    map.insert("P-A 035", CardId::PA035Turtwig);
    map.insert("P-A 036", CardId::PA036Electivire);
    map.insert("P-A 037", CardId::PA037CresseliaEx);
    map.insert("P-A 038", CardId::PA038Misdreavus);
    map.insert("P-A 039", CardId::PA039Skarmory);
    map.insert("P-A 040", CardId::PA040Chimchar);
    map.insert("P-A 041", CardId::PA041Togepi);
    map.insert("P-A 042", CardId::PA042DarkraiEx);
    map.insert("P-A 043", CardId::PA043Cherrim);
    map.insert("P-A 044", CardId::PA044Raichu);
    map.insert("P-A 045", CardId::PA045Nosepass);
    map.insert("P-A 046", CardId::PA046Gible);
    map.insert("P-A 047", CardId::PA047Staraptor);
    map.insert("P-A 048", CardId::PA048Manaphy);
    map.insert("P-A 049", CardId::PA049Snorlax);
    map.insert("P-A 050", CardId::PA050MewtwoEx);
    map.insert("P-A 051", CardId::PA051Cyclizar);
    map.insert("P-A 052", CardId::PA052Sprigatito);
    map.insert("P-A 053", CardId::PA053Floatzel);
    map.insert("P-A 054", CardId::PA054Pawmot);
    map.insert("P-A 055", CardId::PA055Machamp);
    map.insert("P-A 056", CardId::PA056Ekans);
    map.insert("P-A 057", CardId::PA057Bidoof);
    map.insert("P-A 058", CardId::PA058Pachirisu);
    map.insert("P-A 059", CardId::PA059Riolu);
    map.insert("P-A 060", CardId::PA060Exeggcute);
    map.insert("P-A 061", CardId::PA061Froakie);
    map.insert("P-A 062", CardId::PA062Farfetchd);
    map.insert("P-A 063", CardId::PA063Rayquaza);
    map.insert("P-A 064", CardId::PA064RayquazaEx);
    map.insert("P-A 065", CardId::PA065RayquazaEx);
    map.insert("P-A 066", CardId::PA066Mimikyu);
    map.insert("P-A 067", CardId::PA067Cosmog);
    map.insert("P-A 068", CardId::PA068Lycanroc);
    map.insert("P-A 069", CardId::PA069AlolanExeggutor);
    map.insert("P-A 070", CardId::PA070AlolanNinetales);
    map.insert("P-A 071", CardId::PA071Crabrawler);
    map.insert("P-A 072", CardId::PA072AlolanGrimer);
    map.insert("P-A 073", CardId::PA073Toucannon);
    map.insert("P-A 074", CardId::PA074Zeraora);
    map.insert("P-A 075", CardId::PA075Kartana);
    map.insert("P-A 076", CardId::PA076Blacephalon);
    map.insert("P-A 077", CardId::PA077Xurkitree);
    map.insert("P-A 078", CardId::PA078DawnWingsNecrozma);
    map.insert("P-A 079", CardId::PA079DuskManeNecrozma);
    map.insert("P-A 080", CardId::PA080Stakataka);
    map.insert("P-A 081", CardId::PA081UltraNecrozmaEx);
    map.insert("P-A 082", CardId::PA082Poipole);
    map.insert("P-A 083", CardId::PA083Stufful);
    map.insert("P-A 084", CardId::PA084TapuKokoEx);
    map.insert("P-A 085", CardId::PA085Vanillite);
    map.insert("P-A 086", CardId::PA086Jolteon);
    map.insert("P-A 087", CardId::PA087Alcremie);
    map.insert("P-A 088", CardId::PA088Dragonair);
    map.insert("P-A 089", CardId::PA089Audino);
    map.insert("P-A 090", CardId::PA090Togedemaru);
    map.insert("P-A 091", CardId::PA091Greedent);
    map.insert("P-A 092", CardId::PA092Eevee);
    map.insert("P-A 093", CardId::PA093Cleffa);
    map.insert("P-A 094", CardId::PA094Horsea);
    map.insert("P-A 095", CardId::PA095Chinchou);
    map.insert("P-A 096", CardId::PA096Houndoom);
    map.insert("P-A 097", CardId::PA097Kangaskhan);
    map.insert("P-A 098", CardId::PA098BlisseyEx);
    map.insert("P-A 099", CardId::PA099Marill);
    map.insert("P-A 100", CardId::PA100Weavile);
    map.insert("P-A 101", CardId::PA101Latias);
    map.insert("P-A 102", CardId::PA102Tropius);
    map.insert("P-A 103", CardId::PA103Poliwag);
    map.insert("P-A 104", CardId::PA104Milotic);
    map.insert("P-A 105", CardId::PA105Zorua);
    map.insert("P-A 106", CardId::PA106Zoroark);
    map.insert("P-A 107", CardId::PA107Miltank);
    map.insert("P-A 108", CardId::PA108Phanpy);
    map.insert("P-A 109", CardId::PA109EeveeEx);
    map.insert("P-A 110", CardId::PA110EnteiEx);
    map.insert("P-A 111", CardId::PA111Pikachu);
    map.insert("P-A 112", CardId::PA112RaichuEx);
    map.insert("P-A 113", CardId::PA113Mimikyu);
    map.insert("P-A 114", CardId::PA114Machamp);
    map.insert("P-A 115", CardId::PA115Regigigas);
    map.insert("P-A 116", CardId::PA116Shaymin);
    map.insert("P-A 117", CardId::PA117Absol);
    map.insert("P-B 001", CardId::PB001Pikachu);
    map.insert("P-B 002", CardId::PB002Petilil);
    map.insert("P-B 003", CardId::PB003Froakie);
    map.insert("P-B 004", CardId::PB004Luxray);
    map.insert("P-B 005", CardId::PB005Pidgey);
    map.insert("P-B 006", CardId::PB006MegaPidgeotEx);
    map.insert("P-B 007", CardId::PB007Torchic);
    map.insert("P-B 008", CardId::PB008Psyduck);
    map.insert("P-B 009", CardId::PB009MegaAbsolEx);
    map.insert("P-B 010", CardId::PB010Drifblim);
    map.insert("P-B 011", CardId::PB011Eevee);
    map.insert("P-B 012", CardId::PB012Ditto);
    map.insert("P-B 013", CardId::PB013Arcanine);
    map.insert("P-B 014", CardId::PB014Magikarp);
    map.insert("P-B 015", CardId::PB015Mareep);
    map.insert("P-B 016", CardId::PB016Krookodile);
    map.insert("P-B 017", CardId::PB017Swablu);
    map.insert("P-B 018", CardId::PB018Heliolisk);
    map.insert("P-B 019", CardId::PB019Buneary);
    map.insert("P-B 020", CardId::PB020Charmeleon);
    map.insert("P-B 021", CardId::PB021Onix);
    map.insert("P-B 022", CardId::PB022Hawlucha);
    map.insert("P-B 023", CardId::PB023Genesect);
    map.insert("P-B 024", CardId::PB024MegaLatiosEx);
    map
});

impl CardId {
    pub fn from_card_id(id: &str) -> Option<Self> {
        CARD_ID_MAP.get(id).copied()
    }
}
