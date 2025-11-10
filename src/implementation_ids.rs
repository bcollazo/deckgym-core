// This is code generated from the database.json by card_enum_generator.rs. Do not edit manually.

use crate::attack_ids::AttackId;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EffectId {
    NoEffect,
    Effect0,
    Effect1,
    Effect2,
    Effect3,
    Effect4,
    Effect5,
    Effect6,
    Effect7,
    Effect8,
    Effect9,
    Effect10,
    Effect11,
    Effect12,
    Effect13,
    Effect14,
    Effect15,
    Effect16,
    Effect17,
    Effect18,
    Effect19,
    Effect20,
    Effect21,
    Effect22,
    Effect23,
    Effect24,
    Effect25,
    Effect26,
    Effect27,
    Effect28,
    Effect29,
    Effect30,
    Effect31,
    Effect32,
    Effect33,
    Effect34,
    Effect35,
    Effect36,
    Effect37,
    Effect38,
    Effect39,
    Effect40,
    Effect41,
    Effect42,
    Effect43,
    Effect44,
    Effect45,
    Effect46,
    Effect47,
    Effect48,
    Effect49,
    Effect50,
    Effect51,
    Effect52,
    Effect53,
    Effect54,
    Effect55,
    Effect56,
    Effect57,
    Effect58,
    Effect59,
    Effect60,
    Effect61,
    Effect62,
    Effect63,
    Effect64,
    Effect65,
    Effect66,
    Effect67,
    Effect68,
    Effect69,
    Effect70,
    Effect71,
    Effect72,
    Effect73,
    Effect74,
    Effect75,
    Effect76,
    Effect77,
    Effect78,
    Effect79,
    Effect80,
    Effect81,
    Effect82,
    Effect83,
    Effect84,
    Effect85,
    Effect86,
    Effect87,
    Effect88,
    Effect89,
    Effect90,
    Effect91,
    Effect92,
    Effect93,
    Effect94,
    Effect95,
    Effect96,
    Effect97,
    Effect98,
    Effect99,
    Effect100,
    Effect101,
    Effect102,
    Effect103,
    Effect104,
    Effect105,
    Effect106,
    Effect107,
    Effect108,
    Effect109,
    Effect110,
    Effect111,
    Effect112,
    Effect113,
    Effect114,
    Effect115,
    Effect116,
    Effect117,
    Effect118,
    Effect119,
    Effect120,
    Effect121,
    Effect122,
    Effect123,
    Effect124,
    Effect125,
    Effect126,
    Effect127,
    Effect128,
    Effect129,
    Effect130,
    Effect131,
    Effect132,
    Effect133,
    Effect134,
    Effect135,
    Effect136,
    Effect137,
    Effect138,
    Effect139,
    Effect140,
    Effect141,
    Effect142,
    Effect143,
    Effect144,
    Effect145,
    Effect146,
    Effect147,
    Effect148,
    Effect149,
    Effect150,
    Effect151,
    Effect152,
    Effect153,
    Effect154,
    Effect155,
    Effect156,
    Effect157,
    Effect158,
    Effect159,
    Effect160,
    Effect161,
    Effect162,
    Effect163,
    Effect164,
    Effect165,
    Effect166,
    Effect167,
    Effect168,
    Effect169,
    Effect170,
    Effect171,
    Effect172,
    Effect173,
    Effect174,
    Effect175,
    Effect176,
    Effect177,
    Effect178,
    Effect179,
    Effect180,
    Effect181,
    Effect182,
    Effect183,
    Effect184,
    Effect185,
    Effect186,
    Effect187,
    Effect188,
    Effect189,
    Effect190,
    Effect191,
    Effect192,
    Effect193,
    Effect194,
    Effect195,
    Effect196,
    Effect197,
    Effect198,
    Effect199,
    Effect200,
    Effect201,
    Effect202,
    Effect203,
    Effect204,
    Effect205,
    Effect206,
    Effect207,
    Effect208,
    Effect209,
    Effect210,
    Effect211,
    Effect212,
    Effect213,
    Effect214,
    Effect215,
    Effect216,
    Effect217,
    Effect218,
    Effect219,
    Effect220,
    Effect221,
    Effect222,
    Effect223,
    Effect224,
    Effect225,
    Effect226,
    Effect227,
    Effect228,
    Effect229,
    Effect230,
    Effect231,
    Effect232,
    Effect233,
    Effect234,
    Effect235,
    Effect236,
    Effect237,
    Effect238,
    Effect239,
    Effect240,
    Effect241,
    Effect242,
    Effect243,
    Effect244,
    Effect245,
    Effect246,
    Effect247,
    Effect248,
    Effect249,
    Effect250,
    Effect251,
    Effect252,
    Effect253,
    Effect254,
    Effect255,
    Effect256,
    Effect257,
    Effect258,
    Effect259,
    Effect260,
    Effect261,
    Effect262,
    Effect263,
    Effect264,
    Effect265,
    Effect266,
    Effect267,
    Effect268,
    Effect269,
    Effect270,
    Effect271,
    Effect272,
    Effect273,
    Effect274,
    Effect275,
    Effect276,
    Effect277,
    Effect278,
    Effect279,
    Effect280,
    Effect281,
    Effect282,
    Effect283,
    Effect284,
    Effect285,
    Effect286,
    Effect287,
    Effect288,
    Effect289,
    Effect290,
    Effect291,
    Effect292,
    Effect293,
    Effect294,
    Effect295,
    Effect296,
    Effect297,
    Effect298,
    Effect299,
    Effect300,
    Effect301,
    Effect302,
    Effect303,
    Effect304,
    Effect305,
    Effect306,
    Effect307,
    Effect308,
    Effect309,
    Effect310,
    Effect311,
    Effect312,
    Effect313,
    Effect314,
    Effect315,
    Effect316,
    Effect317,
    Effect318,
    Effect319,
    Effect320,
    Effect321,
    Effect322,
    Effect323,
    Effect324,
    Effect325,
    Effect326,
    Effect327,
    Effect328,
    Effect329,
    Effect330,
    Effect331,
    Effect332,
    Effect333,
    Effect334,
    Effect335,
    Effect336,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplementationStatus {
    Implemented,
    NotImplemented,
}

static ATTACK_EFFECT_MAP: LazyLock<HashMap<AttackId, EffectId>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map
});

static EFFECT_DESCRIPTIONS: LazyLock<HashMap<EffectId, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(EffectId::NoEffect, "None");
    map.insert(EffectId::Effect0, "Heal 30 damage from this Pokémon.");
    map.insert(EffectId::Effect1, "Put 1 random [G] Pokémon from your deck into your hand.");
    map.insert(EffectId::Effect2, "Your opponent's Active Pokémon is now Asleep.");
    map.insert(EffectId::Effect3, "Your opponent's Active Pokémon is now Poisoned.");
    map.insert(EffectId::Effect4, "Flip a coin. If heads, this attack does 30 more damage.");
    map.insert(EffectId::Effect5, "Flip a coin. If heads, this attack does 40 more damage.");
    map.insert(EffectId::Effect6, "Heal 10 damage from this Pokémon.");
    map.insert(EffectId::Effect7, "Flip 2 coins. This attack does 50 damage for each heads.");
    map.insert(EffectId::Effect8, "Take a [G] Energy from your Energy Zone and attach it to 1 of your Benched [G] Pokémon.");
    map.insert(EffectId::Effect9, "Flip a coin. If tails, this attack does nothing.");
    map.insert(EffectId::Effect10, "Discard a [R] Energy from this Pokémon.");
    map.insert(EffectId::Effect11, "Discard 2 [R] Energy from this Pokémon.");
    map.insert(EffectId::Effect12, "Flip a coin. If heads, the Defending Pokémon can't attack during your opponent's next turn.");
    map.insert(EffectId::Effect13, "This Pokémon also does 20 damage to itself.");
    map.insert(EffectId::Effect14, "Flip 3 coins. Take an amount of [R] Energy from your Energy Zone equal to the number of heads and attach it to your Benched [R] Pokémon in any way you like.");
    map.insert(EffectId::Effect15, "If this Pokémon has at least 2 extra [W] Energy attached, this attack does 60 more damage.");
    map.insert(EffectId::Effect16, "Your opponent can't use any Supporter cards from their hand during their next turn.");
    map.insert(EffectId::Effect17, "Flip 2 coins. If both of them are heads, this attack does 80 more damage.");
    map.insert(EffectId::Effect18, "This attack does 50 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect19, "Discard a random Energy from your opponent's Active Pokémon.");
    map.insert(EffectId::Effect20, "If this Pokémon has at least 3 extra [W] Energy attached, this attack does 70 more damage.");
    map.insert(EffectId::Effect21, "During your opponent's next turn, the Defending Pokémon can't attack.");
    map.insert(EffectId::Effect22, "Flip a coin. If heads, your opponent's Active Pokémon is now Paralyzed.");
    map.insert(EffectId::Effect23, "This attack also does 10 damage to each of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect24, "If your opponent's Active Pokémon has damage on it, this attack does 60 more damage.");
    map.insert(EffectId::Effect25, "Discard all Energy from this Pokémon.");
    map.insert(EffectId::Effect26, "This attack does 30 damage for each of your Benched [L] Pokémon.");
    map.insert(EffectId::Effect27, "Flip a coin. If heads, this attack does 40 more damage. If tails, this Pokémon also does 20 damage to itself.");
    map.insert(EffectId::Effect28, "Flip 4 coins. This attack does 40 damage for each heads.");
    map.insert(EffectId::Effect29, "This attack also does 30 damage to 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect30, "Flip 4 coins. This attack does 50 damage for each heads.");
    map.insert(EffectId::Effect31, "This attack does 30 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect32, "Switch this Pokémon with 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect33, "This attack does 30 more damage for each Energy attached to your opponent's Active Pokémon.");
    map.insert(EffectId::Effect34, "During your opponent's next turn, this Pokémon takes -20 damage from attacks.");
    map.insert(EffectId::Effect35, "This attack does 20 more damage for each Energy attached to your opponent's Active Pokémon.");
    map.insert(EffectId::Effect36, "Discard 2 [P] Energy from this Pokémon.");
    map.insert(EffectId::Effect37, "Flip 2 coins. This attack does 100 damage for each heads.");
    map.insert(EffectId::Effect38, "Flip a coin. If heads, during your opponent's next turn, prevent all damage from—and effects of—attacks done to this Pokémon.");
    map.insert(EffectId::Effect39, "If this Pokémon has damage on it, this attack does 60 more damage.");
    map.insert(EffectId::Effect40, "This Pokémon also does 50 damage to itself.");
    map.insert(EffectId::Effect41, "During your opponent's next turn, attacks used by the Defending Pokémon do -20 damage.");
    map.insert(EffectId::Effect42, "Flip 2 coins. This attack does 80 damage for each heads.");
    map.insert(EffectId::Effect43, "This attack does 30 damage to 1 of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect44, "Heal from this Pokémon the same amount of damage you did to your opponent's Active Pokémon.");
    map.insert(EffectId::Effect45, "Switch out your opponent's Active Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)");
    map.insert(EffectId::Effect46, "During your opponent's next turn, the Defending Pokémon can't retreat.");
    map.insert(EffectId::Effect47, "Put 1 random Nidoran♂ from your deck onto your Bench.");
    map.insert(EffectId::Effect48, "This attack does 50 more damage for each of your Benched Nidoking.");
    map.insert(EffectId::Effect49, "If your opponent's Active Pokémon is Poisoned, this attack does 50 more damage.");
    map.insert(EffectId::Effect50, "Flip a coin. If heads, discard a random Energy from your opponent's Active Pokémon.");
    map.insert(EffectId::Effect51, "Take a [M] Energy from your Energy Zone and attach it to this Pokémon.");
    map.insert(EffectId::Effect52, "1 of your opponent's Pokémon is chosen at random 4 times. For each time a Pokémon was chosen, do 50 damage to it.");
    map.insert(EffectId::Effect53, "Draw a card.");
    map.insert(EffectId::Effect54, "Flip a coin. If heads, discard a random card from your opponent's hand.");
    map.insert(EffectId::Effect55, "Flip a coin until you get tails. This attack does 60 damage for each heads.");
    map.insert(EffectId::Effect56, "Flip 2 coins. This attack does 30 damage for each heads.");
    map.insert(EffectId::Effect57, "Choose 1 of your opponent's Pokémon's attacks and use it as this attack. If this Pokémon doesn't have the necessary Energy to use that attack, this attack does nothing.");
    map.insert(EffectId::Effect58, "Flip a coin. If heads, your opponent shuffles their Active Pokémon into their deck.");
    map.insert(EffectId::Effect59, "This attack does 30 damage for each of your Benched Pokémon.");
    map.insert(EffectId::Effect60, "Your opponent reveals their hand.");
    map.insert(EffectId::Effect61, "Take a [G] Energy from your Energy Zone and attach it to this Pokémon.");
    map.insert(EffectId::Effect62, "Flip a coin for each Energy attached to this Pokémon. This attack does 50 damage for each heads.");
    map.insert(EffectId::Effect63, "If this Pokémon has at least 3 extra [G] Energy attached, this attack does 70 more damage.");
    map.insert(EffectId::Effect64, "Flip a coin. If heads, this attack does 60 more damage.");
    map.insert(EffectId::Effect65, "Discard 2 [R] Energy from this Pokémon. This attack does 80 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect66, "If your opponent's Active Pokémon is Poisoned, this attack does 40 more damage.");
    map.insert(EffectId::Effect67, "Discard a random Energy from among the Energy attached to all Pokémon (both yours and your opponent's).");
    map.insert(EffectId::Effect68, "This attack does 50 damage to 1 of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect69, "This attack does 10 damage for each of your Benched [L] Pokémon.");
    map.insert(EffectId::Effect70, "This attack also does 20 damage to each of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect71, "This attack does 40 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect72, "Choose 1 of your opponent's Active Pokémon's attacks and use it as this attack.");
    map.insert(EffectId::Effect73, "This attack does 20 more damage for each of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect74, "Heal 20 damage from each of your Pokémon.");
    map.insert(EffectId::Effect75, "During your opponent's next turn, this Pokémon takes -30 damage from attacks.");
    map.insert(EffectId::Effect76, "If any of your Pokémon were Knocked Out by damage from an attack during your opponent's last turn, this attack does 60 more damage.");
    map.insert(EffectId::Effect77, "Put 1 random Koffing from your deck onto your Bench.");
    map.insert(EffectId::Effect78, "During your opponent's next turn, if the Defending Pokémon tries to use an attack, your opponent flips a coin. If tails, that attack doesn't happen.");
    map.insert(EffectId::Effect79, "If your opponent's Active Pokémon is a Pokémon ex, this attack does 80 more damage.");
    map.insert(EffectId::Effect80, "Flip a coin until you get tails. This attack does 20 damage for each heads.");
    map.insert(EffectId::Effect81, "Shuffle your hand into your deck. Draw a card for each card in your opponent's hand.");
    map.insert(EffectId::Effect82, "Discard a random Energy from this Pokémon.");
    map.insert(EffectId::Effect83, "During your next turn, this Pokémon can't use Frenzy Plant.");
    map.insert(EffectId::Effect84, "Your opponent's Active Pokémon is now Confused.");
    map.insert(EffectId::Effect85, "Put 1 random Basic Pokémon from your deck onto your Bench.");
    map.insert(EffectId::Effect86, "Flip a coin. If heads, this attack does 50 more damage.");
    map.insert(EffectId::Effect87, "During your next turn, this Pokémon can't attack.");
    map.insert(EffectId::Effect88, "Take a [R] Energy from your Energy Zone and attach it to this Pokémon.");
    map.insert(EffectId::Effect89, "Your opponent's Active Pokémon is now Burned.");
    map.insert(EffectId::Effect90, "Discard all [R] Energy from this Pokémon.");
    map.insert(EffectId::Effect91, "Heal 20 damage from this Pokémon.");
    map.insert(EffectId::Effect92, "This attack also does 30 damage to 1 of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect93, "This attack also does 20 damage to 1 of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect94, "If your opponent's Active Pokémon is a [F] Pokémon, this attack does 30 more damage.");
    map.insert(EffectId::Effect95, "Discard 3 [W] Energy from this Pokémon. This attack also does 20 damage to each of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect96, "Choose 2 of your Benched Pokémon. For each of those Pokémon, take a [W] Energy from your Energy Zone and attach it to that Pokémon.");
    map.insert(EffectId::Effect97, "Discard a [L] Energy from this Pokémon.");
    map.insert(EffectId::Effect98, "This Pokémon also does 10 damage to itself.");
    map.insert(EffectId::Effect99, "Take a [L] Energy from your Energy Zone and attach it to this Pokémon.");
    map.insert(EffectId::Effect100, "If this Pokémon has at least 2 extra [L] Energy attached, this attack does 80 more damage.");
    map.insert(EffectId::Effect101, "Discard all [L] Energy from this Pokémon. This attack does 120 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect102, "If this Pokémon has a Pokémon Tool attached, this attack does 40 more damage.");
    map.insert(EffectId::Effect103, "If your opponent's Active Pokémon has a Pokémon Tool attached, this attack does 30 more damage.");
    map.insert(EffectId::Effect104, "During your next turn, this Pokémon's Overdrive Smash attack does +60 damage.");
    map.insert(EffectId::Effect105, "Take a [P] Energy from your Energy Zone and attach it to Mesprit or Azelf.");
    map.insert(EffectId::Effect106, "You can use this attack only if you have Uxie and Azelf on your Bench. Discard all Energy from this Pokémon.");
    map.insert(EffectId::Effect107, "This attack does 20 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect108, "Discard the top 3 cards of your deck.");
    map.insert(EffectId::Effect109, "Flip 2 coins. This attack does 20 more damage for each heads.");
    map.insert(EffectId::Effect110, "If your opponent's Pokémon is Knocked Out by damage from this attack, this Pokémon also does 50 damage to itself.");
    map.insert(EffectId::Effect111, "Flip 2 coins. This attack does 20 damage for each heads.");
    map.insert(EffectId::Effect112, "If your opponent's Active Pokémon has damage on it, this attack does 40 more damage.");
    map.insert(EffectId::Effect113, "This attack does 10 damage to each of your opponent's Pokémon.");
    map.insert(EffectId::Effect114, "Flip 4 coins. This attack does 40 damage for each heads. If at least 2 of them are heads, your opponent's Active Pokémon is now Poisoned.");
    map.insert(EffectId::Effect115, "Flip a coin for each Pokémon you have in play. This attack does 20 damage for each heads.");
    map.insert(EffectId::Effect116, "Flip a coin for each Pokémon you have in play. This attack does 40 damage for each heads.");
    map.insert(EffectId::Effect117, "If this Pokémon has a Pokémon Tool attached, this attack does 30 more damage.");
    map.insert(EffectId::Effect118, "Flip a coin until you get tails. This attack does 30 more damage for each heads.");
    map.insert(EffectId::Effect119, "Flip 3 coins. This attack does 50 more damage for each heads.");
    map.insert(EffectId::Effect120, "Take 2 [M] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect121, "Flip a coin until you get tails. This attack does 40 more damage for each heads.");
    map.insert(EffectId::Effect122, "Flip a coin. If heads, this attack does 20 more damage.");
    map.insert(EffectId::Effect123, "Change the type of the next Energy that will be generated for your opponent to 1 of the following at random: [G], [R], [W], [L], [P], [F], [D], or [M].");
    map.insert(EffectId::Effect124, "Flip 2 coins. This attack does 40 damage for each heads.");
    map.insert(EffectId::Effect125, "Before doing damage, discard all Pokémon Tools from your opponent's Active Pokémon.");
    map.insert(EffectId::Effect126, "Halve your opponent's Active Pokémon's remaining HP, rounded down.");
    map.insert(EffectId::Effect127, "Your opponent reveals their hand. Choose a card you find there and shuffle it into your opponent's deck.");
    map.insert(EffectId::Effect128, "Flip 3 coins. This attack does 20 damage for each heads.");
    map.insert(EffectId::Effect129, "Flip a coin. If heads, put your opponent's Active Pokémon into their hand.");
    map.insert(EffectId::Effect130, "This attack does more damage equal to the damage this Pokémon has on it.");
    map.insert(EffectId::Effect131, "Flip 2 coins. If both of them are heads, this attack does 70 more damage.");
    map.insert(EffectId::Effect132, "This Pokémon also does 30 damage to itself.");
    map.insert(EffectId::Effect133, "If this Pokémon has damage on it, this attack does 40 more damage.");
    map.insert(EffectId::Effect134, "Flip a coin. If heads, this attack does 60 more damage. If tails, this Pokémon also does 20 damage to itself.");
    map.insert(EffectId::Effect135, "This attack also does 10 damage to 1 of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect136, "During your opponent's next turn, attacks used by the Defending Pokémon do -30 damage.");
    map.insert(EffectId::Effect137, "Flip a coin. If heads, your opponent reveals a random card from their hand and shuffles it into their deck.");
    map.insert(EffectId::Effect138, "This attack does 20 damage to 1 of your opponent's Benched Pokémon.");
    map.insert(EffectId::Effect139, "If your opponent's Active Pokémon is a Pokémon ex, this attack does 30 more damage.");
    map.insert(EffectId::Effect140, "During your next turn, this Pokémon's Rolling Spin attack does +60 damage.");
    map.insert(EffectId::Effect141, "Your opponent's Active Pokémon is now Poisoned. Do 20 damage to this Pokémon instead of the usual amount for this Special Condition.");
    map.insert(EffectId::Effect142, "If your opponent's Active Pokémon is a [M] Pokémon, this attack does 30 more damage.");
    map.insert(EffectId::Effect143, "Flip a coin. If tails, during your next turn, this Pokémon can't attack.");
    map.insert(EffectId::Effect144, "Discard 2 random Energy from this Pokémon.");
    map.insert(EffectId::Effect145, "This Pokémon is now Asleep.");
    map.insert(EffectId::Effect146, "This attack does 20 more damage for each of your Benched Pokémon.");
    map.insert(EffectId::Effect147, "Put 1 random Weedle from your deck onto your Bench.");
    map.insert(EffectId::Effect148, "If your opponent's Active Pokémon is a Pokémon ex, this attack does 70 more damage.");
    map.insert(EffectId::Effect149, "Take 3 [R] Energy from your Energy Zone and attach it to this Pokémon.");
    map.insert(EffectId::Effect150, "1 of your opponent's Pokémon is chosen at random. Do 30 damage to it.");
    map.insert(EffectId::Effect151, "1 of your opponent's Pokémon is chosen at random 3 times. For each time a Pokémon was chosen, do 50 damage to it.");
    map.insert(EffectId::Effect152, "Take a [L] Energy from your Energy Zone and attach it to 1 of your Benched [L] Pokémon.");
    map.insert(EffectId::Effect153, "This attack also does 20 damage to each of your opponent's Benched Pokémon that has any Energy attached.");
    map.insert(EffectId::Effect154, "Flip 4 coins. This attack does 20 damage for each heads.");
    map.insert(EffectId::Effect155, "If this Pokémon has at least 2 extra [F] Energy attached, this attack does 50 more damage.");
    map.insert(EffectId::Effect156, "If your opponent's Active Pokémon is Poisoned, this attack does 60 more damage.");
    map.insert(EffectId::Effect157, "Flip a coin. If heads, your opponent's Active Pokémon is now Confused.");
    map.insert(EffectId::Effect158, "Flip a coin. If heads, this attack does 80 more damage.");
    map.insert(EffectId::Effect159, "Flip a coin for each [M] Energy attached to this Pokémon. This attack does 50 damage for each heads.");
    map.insert(EffectId::Effect160, "During your next turn, this Pokémon's Overacceleration attack does +20 damage.");
    map.insert(EffectId::Effect161, "This attack does 10 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect162, "This attack does 100 damage to 1 of your opponent's Pokémon that have damage on them.");
    map.insert(EffectId::Effect163, "This attack does 20 damage to each of your opponent's Pokémon.");
    map.insert(EffectId::Effect164, "Flip 3 coins. This attack does 50 damage for each heads.");
    map.insert(EffectId::Effect165, "If this Pokémon moved from your Bench to the Active Spot this turn, this attack does 60 more damage.");
    map.insert(EffectId::Effect166, "Flip a coin. If tails, this Pokémon also does 20 damage to itself.");
    map.insert(EffectId::Effect167, "Flip 2 coins. This attack does 70 damage for each heads. If at least 1 of them is heads, your opponent's Active Pokémon is now Burned.");
    map.insert(EffectId::Effect168, "Discard a random Energy from both Active Pokémon.");
    map.insert(EffectId::Effect169, "Your opponent's Active Pokémon is now Poisoned and Burned.");
    map.insert(EffectId::Effect170, "During your opponent's next turn, if this Pokémon is damaged by an attack, do 40 damage to the Attacking Pokémon.");
    map.insert(EffectId::Effect171, "Take a [W] Energy from your Energy Zone and attach it to this Pokémon.");
    map.insert(EffectId::Effect172, "During your next turn, this Pokémon's Insatiable Striking attack does +40 damage.");
    map.insert(EffectId::Effect173, "Put 1 random Wishiwashi or Wishiwashi ex from your deck onto your Bench.");
    map.insert(EffectId::Effect174, "This attack does 40 more damage for each of your Benched Wishiwashi and Wishiwashi ex.");
    map.insert(EffectId::Effect175, "If your opponent's Active Pokémon is a Basic Pokémon, this attack does 60 more damage.");
    map.insert(EffectId::Effect176, "Discard 2 [L] Energy from this Pokémon.");
    map.insert(EffectId::Effect177, "During your opponent's next turn, they can't play any Item cards from their hand.");
    map.insert(EffectId::Effect178, "Switch this Pokémon with 1 of your Benched [L] Pokémon.");
    map.insert(EffectId::Effect179, "Take a [P] Energy from your Energy Zone and attach it to this Pokémon.");
    map.insert(EffectId::Effect180, "During your opponent's next turn, they can't take any Energy from their Energy Zone to attach to their Active Pokémon.");
    map.insert(EffectId::Effect181, "This attack also does 20 damage to 1 of your Pokémon.");
    map.insert(EffectId::Effect182, "This attack does 20 damage to 1 of your opponent's Pokémon for each Energy attached to that Pokémon.");
    map.insert(EffectId::Effect183, "During your opponent's next turn, this Pokémon takes -50 damage from attacks.");
    map.insert(EffectId::Effect184, "Put a random card that evolves from Rockruff from your deck into your hand.");
    map.insert(EffectId::Effect185, "If your opponent's Active Pokémon has more remaining HP than this Pokémon, this attack does 50 more damage.");
    map.insert(EffectId::Effect186, "This Pokémon also does 40 damage to itself.");
    map.insert(EffectId::Effect187, "Discard a random Item card from your opponent's hand.");
    map.insert(EffectId::Effect188, "1 Special Condition from among Asleep, Burned, Confused, Paralyzed, and Poisoned is chosen at random, and your opponent's Active Pokémon is now affected by that Special Condition. Any Special Conditions already affecting that Pokémon will not be chosen.");
    map.insert(EffectId::Effect189, "If your opponent's Active Pokémon is affected by a Special Condition, this attack does 60 more damage.");
    map.insert(EffectId::Effect190, "Flip a coin until you get tails. This attack does 70 damage for each heads.");
    map.insert(EffectId::Effect191, "If your opponent's Active Pokémon has an Ability, this attack does 40 more damage.");
    map.insert(EffectId::Effect192, "If any of your Benched Pokémon have damage on them, this attack does 50 more damage.");
    map.insert(EffectId::Effect193, "During your opponent's next turn, this Pokémon takes +30 damage from attacks.");
    map.insert(EffectId::Effect194, "Take a [C] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect195, "If your opponent's Active Pokémon is a [D] Pokémon, this attack does 30 more damage.");
    map.insert(EffectId::Effect196, "This Pokémon is now Confused.");
    map.insert(EffectId::Effect197, "During your opponent's next turn, attacks used by the Defending Pokémon cost 1 [C] more, and its Retreat Cost is 1 [C] more.");
    map.insert(EffectId::Effect198, "Flip 3 coins. This attack does 10 damage for each heads.");
    map.insert(EffectId::Effect199, "This attack does 70 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect200, "During your next turn, this Pokémon can't use Big Beat.");
    map.insert(EffectId::Effect201, "This Pokémon also does 70 damage to itself.");
    map.insert(EffectId::Effect202, "Discard a [F] Energy from this Pokémon.");
    map.insert(EffectId::Effect203, "If Passimian is on your Bench, this attack does 40 more damage.");
    map.insert(EffectId::Effect204, "Discard a random Pokémon Tool card from your opponent's hand.");
    map.insert(EffectId::Effect205, "Flip 3 coins. For each heads, a card is chosen at random from your opponent's hand. Your opponent reveals that card and shuffles it into their deck.");
    map.insert(EffectId::Effect206, "Flip a coin until you get tails. For each heads, discard a random Energy from your opponent's Active Pokémon.");
    map.insert(EffectId::Effect207, "Flip 3 coins. This attack does 60 damage for each heads.");
    map.insert(EffectId::Effect208, "Flip 2 coins. If both of them are heads, your opponent's Active Pokémon is Knocked Out.");
    map.insert(EffectId::Effect209, "If you played a Supporter card from your hand during this turn, this attack does 50 more damage.");
    map.insert(EffectId::Effect210, "This attack does 20 more damage for each [G] Energy attached to this Pokémon.");
    map.insert(EffectId::Effect211, "Your opponent reveals a random card from their hand and shuffles it into their deck.");
    map.insert(EffectId::Effect212, "If 1 of your Pokémon used Sweets Relay during your last turn, this attack does 30 more damage.");
    map.insert(EffectId::Effect213, "Take a [R] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect214, "If 1 of your Pokémon used Sweets Relay during your last turn, this attack does 20 more damage.");
    map.insert(EffectId::Effect215, "If this Pokémon has at least 1 extra [W] Energy attached, this attack does 40 more damage.");
    map.insert(EffectId::Effect216, "If this Pokémon evolved during this turn, this attack does 20 more damage.");
    map.insert(EffectId::Effect217, "Discard all Energy attached to this Pokémon. Your opponent's Active Pokémon is now Paralyzed.");
    map.insert(EffectId::Effect218, "This attack does 20 damage for each Energy attached to all of your opponent's Pokémon.");
    map.insert(EffectId::Effect219, "If 1 of your Pokémon used Sweets Relay during your last turn, this attack does 60 more damage.");
    map.insert(EffectId::Effect220, "This attack does 30 more damage for each Evolution Pokémon on your Bench.");
    map.insert(EffectId::Effect221, "Flip a coin. If heads, choose 1 of your opponent's Active Pokémon's attacks and use it as this attack.");
    map.insert(EffectId::Effect222, "This attack does 40 damage for each time your Pokémon used Sweets Relay during this game.");
    map.insert(EffectId::Effect223, "This attack also does 10 damage to each of your Benched Pokémon.");
    map.insert(EffectId::Effect224, "If the Defending Pokémon is a Basic Pokémon, it can't attack during your opponent's next turn.");
    map.insert(EffectId::Effect225, "Discard all Pokémon Tools from your opponent's Active Pokémon.");
    map.insert(EffectId::Effect226, "During your opponent's next turn, if this Pokémon is damaged by an attack, do 30 damage to the Attacking Pokémon.");
    map.insert(EffectId::Effect227, "Flip a coin. If tails, this attack does nothing. If heads, your opponent's Active Pokémon is now Paralyzed.");
    map.insert(EffectId::Effect228, "This attack does 20 damage for each of your Benched Pokémon.");
    map.insert(EffectId::Effect229, "If this Pokémon has a Pokémon Tool attached, this attack does 50 more damage.");
    map.insert(EffectId::Effect230, "Flip 3 coins. This attack does 60 damage for each heads. This Pokémon is now Confused.");
    map.insert(EffectId::Effect231, "This attack does 40 more damage for each Energy in your opponent's Active Pokémon's Retreat Cost.");
    map.insert(EffectId::Effect232, "If this Pokémon has no damage on it, this attack does 40 more damage.");
    map.insert(EffectId::Effect233, "1 other Pokémon (either yours or your opponent's) is chosen at random 3 times. For each time a Pokémon was chosen, do 50 damage to it.");
    map.insert(EffectId::Effect234, "Take a [R] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon.");
    map.insert(EffectId::Effect235, "Flip a coin. If tails, discard 2 random Energy from this Pokémon.");
    map.insert(EffectId::Effect236, "Take a [R], [W], and [L] Energy from your Energy Zone and attach them to your Benched Basic Pokémon in any way you like.");
    map.insert(EffectId::Effect237, "If your opponent's Active Pokémon is Burned, this attack does 60 more damage.");
    map.insert(EffectId::Effect238, "You may discard any number of your Benched [W] Pokémon. This attack does 40 more damage for each Benched Pokémon you discarded in this way.");
    map.insert(EffectId::Effect239, "Put a random Pokémon from your deck into your hand.");
    map.insert(EffectId::Effect240, "If the Defending Pokémon tries to use an attack, your opponent flips a coin. If tails, that attack doesn't happen. This effect lasts until the Defending Pokémon leaves the Active Spot, and it doesn't stack.");
    map.insert(EffectId::Effect241, "Move all Energy from this Pokémon to 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect242, "Flip a coin. If heads, your opponent's Active Pokémon is now Paralyzed. If tails, your opponent's Active Pokémon is now Confused.");
    map.insert(EffectId::Effect243, "Take a [L] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon.");
    map.insert(EffectId::Effect244, "This attack also does 10 damage to 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect245, "This Pokémon is now Asleep. Heal 30 damage from it.");
    map.insert(EffectId::Effect246, "This attack does 20 damage for each Energy attached to your opponent's Active Pokémon.");
    map.insert(EffectId::Effect247, "Flip a coin. If heads, your opponent's Active Pokémon's remaining HP is now 10.");
    map.insert(EffectId::Effect248, "If this Pokémon was damaged by an attack during your opponent's last turn while it was in the Active Spot, this attack does 50 more damage.");
    map.insert(EffectId::Effect249, "Both Active Pokémon are now Asleep.");
    map.insert(EffectId::Effect250, "This attack also does 20 damage to each of your Benched Pokémon.");
    map.insert(EffectId::Effect251, "If this Pokémon has at least 2 extra [F] Energy attached, this attack does 60 more damage.");
    map.insert(EffectId::Effect252, "Discard a random card from your opponent's hand.");
    map.insert(EffectId::Effect253, "If this Pokémon moved from your Bench to the Active Spot this turn, this attack does 50 more damage.");
    map.insert(EffectId::Effect254, "During your next turn, this Pokémon's Gear Spinner attack does +70 damage.");
    map.insert(EffectId::Effect255, "If your opponent's Active Pokémon is a [G] Pokémon, this attack does 40 more damage.");
    map.insert(EffectId::Effect256, "Flip a coin. If heads, during your opponent's next turn, prevent all damage done to this Pokémon by attacks.");
    map.insert(EffectId::Effect257, "This attack does 20 more damage for each Energy attached to this Pokémon.");
    map.insert(EffectId::Effect258, "If your opponent's Active Pokémon is an Evolution Pokémon, this attack does 40 more damage.");
    map.insert(EffectId::Effect259, "During your opponent's next turn, attacks used by the Defending Pokémon cost 1 [C] more.");
    map.insert(EffectId::Effect260, "Flip a coin. If tails, this attack does nothing. If heads, during your opponent's next turn, prevent all damage from—and effects of—attacks done to this Pokémon.");
    map.insert(EffectId::Effect261, "Draw cards until you have the same number of cards in your hand as your opponent.");
    map.insert(EffectId::Effect262, "Change the type of a random Energy attached to your opponent's Active Pokémon to 1 of the following at random: [G], [R], [W], [L], [P], [F], [D], or [M].");
    map.insert(EffectId::Effect263, "Discard a [R], [W], and [L] Energy from this Pokémon.");
    map.insert(EffectId::Effect264, "You may switch this Pokémon with 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect265, "If your opponent's Active Pokémon is an evolved Pokémon, devolve it by putting the highest Stage Evolution card on it into your opponent's hand.");
    map.insert(EffectId::Effect266, "Discard the top card of your opponent's deck.");
    map.insert(EffectId::Effect267, "If this Pokémon has at least 2 extra [R] Energy attached, this attack does 60 more damage.");
    map.insert(EffectId::Effect268, "Put 1 random Poliwag from your deck onto your Bench.");
    map.insert(EffectId::Effect269, "Flip a coin. If heads, your opponent's Active Pokémon is now Poisoned and Paralyzed.");
    map.insert(EffectId::Effect270, "Discard up to 2 Pokémon Tool cards from your hand. This attack does 50 damage for each card you discarded in this way.");
    map.insert(EffectId::Effect271, "This attack does 20 damage for each Benched Pokémon (both yours and your opponent's).");
    map.insert(EffectId::Effect272, "Take a [W] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon.");
    map.insert(EffectId::Effect273, "If this Pokémon has damage on it, this attack can be used for 1 [L] Energy.");
    map.insert(EffectId::Effect274, "At the end of your opponent's next turn, do 90 damage to the Defending Pokémon.");
    map.insert(EffectId::Effect275, "If Latios is on your Bench, this attack does 20 more damage.");
    map.insert(EffectId::Effect276, "Discard the top card of your deck. If that card is a [F] Pokémon, this attack does 60 more damage.");
    map.insert(EffectId::Effect277, "If this Pokémon has any [W] Energy attached, this attack does 40 more damage.");
    map.insert(EffectId::Effect278, "This attack does damage to your opponent's Active Pokémon equal to the damage this Pokémon has on it.");
    map.insert(EffectId::Effect279, "If your opponent's Active Pokémon is Zangoose, this attack does 40 more damage.");
    map.insert(EffectId::Effect280, "If this Pokémon has 2 or more different types of Energy attached, this attack does 60 more damage.");
    map.insert(EffectId::Effect281, "This attack does 60 damage to 1 of your opponent's Pokémon.");
    map.insert(EffectId::Effect282, "Until this Pokémon leaves the Active Spot, this Pokémon's Rolling Frenzy attack does +30 damage. This effect stacks.");
    map.insert(EffectId::Effect283, "Heal 50 damage from 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect284, "Flip a coin. If heads, this attack does 70 more damage.");
    map.insert(EffectId::Effect285, "During your opponent's next turn, prevent all damage done to this Pokémon by attacks if that damage is 40 or less.");
    map.insert(EffectId::Effect286, "Choose either Poisoned or Confused. Your opponent's Active Pokémon is now affected by that Special Condition.");
    map.insert(EffectId::Effect287, "Flip 3 coins. This attack does 40 damage for each heads.");
    map.insert(EffectId::Effect288, "This attack does 30 more damage for each Energy in your opponent's Active Pokémon's Retreat Cost.");
    map.insert(EffectId::Effect289, "During your next turn, this Pokémon can't use Sacred Sword.");
    map.insert(EffectId::Effect290, "Flip a coin. If heads, your opponent's Active Pokémon is now Burned.");
    map.insert(EffectId::Effect291, "Heal 30 damage from each of your Benched Basic Pokémon.");
    map.insert(EffectId::Effect292, "Discard Fire[R] Energy from this Pokémon. Your opponent's Active Pokémon is now Burned.");
    map.insert(EffectId::Effect293, "Flip 2 coins. This attack does 30 more damage for each heads.");
    map.insert(EffectId::Effect294, "During your opponent's next turn, if this Pokémon is damaged by an attack, do 20 damage to the Attacking Pokémon.");
    map.insert(EffectId::Effect295, "Put a random card from your deck that evolves from this Pokémon onto this Pokémon to evolve it.");
    map.insert(EffectId::Effect296, "Discard the top 3 cards of your opponent's deck.");
    map.insert(EffectId::Effect297, "If you have exactly 1, 3, or 5 cards in your hand, this attack does 60 more damage.");
    map.insert(EffectId::Effect298, "This attack does 10 more damage for each [W] Energy attached to this Pokémon.");
    map.insert(EffectId::Effect299, "If you have exactly 2, 4, or 6 cards in your hand, this attack does 30 more damage.");
    map.insert(EffectId::Effect300, "Prevent all damage done to this Pokémon by attacks from Basic Pokémon during your opponent's next turn.");
    map.insert(EffectId::Effect301, "1 of your opponent's Benched Pokémon is chosen at random. This attack also does 20 damage to it.");
    map.insert(EffectId::Effect302, "1 of your opponent's Benched Pokémon is chosen at random 3 times. For each time a Pokémon was chosen, also do 20 damage to it.");
    map.insert(EffectId::Effect303, "This attack also does 20 damage to 1 of your Benched Pokémon.");
    map.insert(EffectId::Effect304, "If your opponent's Active Pokémon has damage on it, this attack does 30 more damage.");
    map.insert(EffectId::Effect305, "Discard a [L] Energy from your opponent's Active Pokémon.");
    map.insert(EffectId::Effect306, "Discard a card from your hand. If you can't, this attack does nothing.");
    map.insert(EffectId::Effect307, "This attack does 30 more damage for each of your Benched Pokémon.");
    map.insert(EffectId::Effect308, "Discard 2 cards from your hand. If you can't discard 2 cards, this attack does nothing.");
    map.insert(EffectId::Effect309, "During your opponent's next turn, if they attach Energy from their Energy Zone to the Defending Pokémon, that Pokémon will be Asleep.");
    map.insert(EffectId::Effect310, "This attack's damage isn't affected by Weakness.");
    map.insert(EffectId::Effect311, "If this Pokémon has damage on it, this attack does 50 more damage.");
    map.insert(EffectId::Effect312, "This attack does 20 damage to each of your opponent's Pokémon. During your next turn, this Pokémon's Wild Spin attack does +20 damage to each of your opponent's Pokémon.");
    map.insert(EffectId::Effect313, "Reveal the top 3 cards of your deck. This attack does 60 damage for each Pokémon with a Retreat Cost of 3 or more you find there. Shuffle the revealed cards back into your deck.");
    map.insert(EffectId::Effect314, "If this Pokémon's remaining HP is 30 or less, this attack does 60 more damage.");
    map.insert(EffectId::Effect315, "If your opponent's Active Pokémon is a [G] Pokémon, this attack does 50 more damage.");
    map.insert(EffectId::Effect316, "This attack does 40 more damage for each of your opponent's Pokémon in play that has an Ability.");
    map.insert(EffectId::Effect317, "Flip a coin. If heads, your opponent reveals their hand. Choose a Supporter card you find there and discard it.");
    map.insert(EffectId::Effect318, "Your opponent reveals their hand. Choose a Supporter card you find there and discard it.");
    map.insert(EffectId::Effect319, "During your next turn, this Pokémon's Overdrive Smash attack does +30 damage.");
    map.insert(EffectId::Effect320, "If your opponent's Active Pokémon is Poisoned, this attack does 70 more damage.");
    map.insert(EffectId::Effect321, "This attack's damage isn't affected by any effects on your opponent's Active Pokémon.");
    map.insert(EffectId::Effect322, "If Durant is on your Bench, this attack does 40 more damage.");
    map.insert(EffectId::Effect323, "Discard 2 [M] Energy from this Pokémon. During your opponent's next turn, this Pokémon takes -50 damage from attacks.");
    map.insert(EffectId::Effect324, "Flip 2 coins. If both of them are tails, this attack does nothing.");
    map.insert(EffectId::Effect325, "Heal 40 damage from this Pokémon.");
    map.insert(EffectId::Effect326, "Flip 2 coins. For each heads, discard a random Energy from your opponent's Active Pokémon. If both of them are tails, this attack does nothing.");
    map.insert(EffectId::Effect327, "Flip a coin. If tails, this Pokémon also does 30 damage to itself.");
    map.insert(EffectId::Effect328, "Flip 2 coins. This attack does 30 damage for each heads. If this Pokémon has Lucky Mittens attached, flip 4 coins instead.");
    map.insert(EffectId::Effect329, "Both Active Pokémon are now Confused.");
    map.insert(EffectId::Effect330, "If your opponent's Active Pokémon is a Basic Pokémon, this attack does 70 more damage.");
    map.insert(EffectId::Effect331, "Flip a coin until you get tails. This attack does 40 damage for each heads.");
    map.insert(EffectId::Effect332, "1 of your opponent's Pokémon is chosen at random 4 times. For each time a Pokémon was chosen, do 40 damage to it.");
    map.insert(EffectId::Effect333, "Discard a [M] Energy from this Pokémon.");
    map.insert(EffectId::Effect334, "Discard the top 5 cards of each player's deck.");
    map.insert(EffectId::Effect335, "Flip a coin. If heads, switch in 1 of your opponent's Benched Pokémon to the Active Spot.");
    map.insert(EffectId::Effect336, "Flip a coin. If heads, heal 60 damage from this Pokémon.");
    map
});

impl EffectId {
    /// Get the effect ID for a given attack
    pub fn from_attack(attack_id: AttackId) -> Option<Self> {
        ATTACK_EFFECT_MAP.get(&attack_id).copied()
    }

    /// Get the description text for this effect
    pub fn description(&self) -> &'static str {
        EFFECT_DESCRIPTIONS.get(self).copied().unwrap_or("Unknown effect")
    }

    /// Check if this effect has an implementation
    pub fn implementation_status(&self) -> ImplementationStatus {
        // TODO: This should be generated based on actual implementations in apply_attack_action.rs
        // For now, returning NotImplemented for all
        ImplementationStatus::NotImplemented
    }
}

// Statistics:
// Total unique effects: 338
// Total attacks: 2301

// Effect groupings (attacks with same effect text):
// NoEffect - 1089 attacks:
//   - AttackId::A1001Bulbasaur (Vine Whip)
//   - AttackId::A1002Ivysaur (Razor Leaf)
//   - AttackId::A1004VenusaurEx (Razor Leaf)
//   - AttackId::A1006Metapod (Bug Bite)
//   - AttackId::A1007Butterfree (Gust)
//   - AttackId::A1008Weedle (Sting)
//   - AttackId::A1009Kakuna (Bug Bite)
//   - AttackId::A1010Beedrill (Sharp Sting)
//   - AttackId::A1011Oddish (Ram)
//   - AttackId::A1012Gloom (Drool)
//   - AttackId::A1014Paras (Scratch)
//   - AttackId::A1015Parasect (Slash)
//   - AttackId::A1016Venonat (Tackle)
//   - AttackId::A1018Bellsprout (Vine Whip)
//   - AttackId::A1019Weepinbell (Razor Leaf)
//   - AttackId::A1020Victreebel (Vine Whip)
//   - AttackId::A1021Exeggcute (Seed Bomb)
//   - AttackId::A1025Scyther (Sharp Scythe)
//   - AttackId::A1027Cottonee (Attach)
//   - AttackId::A1028Whimsicott (Rolling Tackle)
//   - AttackId::A1032Gogoat (Razor Leaf)
//   - AttackId::A1034Charmeleon (Fire Claws)
//   - AttackId::A1036CharizardEx (Slash)
//   - AttackId::A1039Growlithe (Bite)
//   - AttackId::A1042Ponyta (Flare)
//   - AttackId::A1043Rapidash (Fire Mane)
//   - AttackId::A1044Magmar (Magma Punch)
//   - AttackId::A1047MoltresEx (Heat Blast)
//   - AttackId::A1048Heatmor (Combustion)
//   - AttackId::A1049Salandit (Scratch)
//   - AttackId::A1050Salazzle (Fire Claws)
//   - AttackId::A1051Sizzlipede (Gnaw)
//   - AttackId::A1053Squirtle (Water Gun)
//   - AttackId::A1054Wartortle (Wave Splash)
//   - AttackId::A1056BlastoiseEx (Surf)
//   - AttackId::A1058Golduck (Aqua Edge)
//   - AttackId::A1059Poliwag (Razor Fin)
//   - AttackId::A1060Poliwhirl (Knuckle Punch)
//   - AttackId::A1061Poliwrath (Mega Punch)
//   - AttackId::A1062Tentacool (Gentle Slap)
//   - AttackId::A1064Seel (Headbutt)
//   - AttackId::A1065Dewgong (Surf)
//   - AttackId::A1066Shellder (Tongue Slap)
//   - AttackId::A1067Cloyster (Surf)
//   - AttackId::A1068Krabby (Vise Grip)
//   - AttackId::A1070Horsea (Water Gun)
//   - AttackId::A1072Goldeen (Flop)
//   - AttackId::A1074Staryu (Smack)
//   - AttackId::A1075Starmie (Wave Splash)
//   - AttackId::A1076StarmieEx (Hydro Splash)
//   - AttackId::A1077Magikarp (Splash)
//   - AttackId::A1081Omanyte (Water Gun)
//   - AttackId::A1084ArticunoEx (Ice Wing)
//   - AttackId::A1085Ducklett (Flap)
//   - AttackId::A1086Swanna (Wing Attack)
//   - AttackId::A1087Froakie (Flop)
//   - AttackId::A1088Frogadier (Water Drip)
//   - AttackId::A1089Greninja (Mist Slash)
//   - AttackId::A1090Pyukumuku (Rain Splash)
//   - AttackId::A1092Snom (Ram)
//   - AttackId::A1094Pikachu (Gnaw)
//   - AttackId::A1097Magnemite (Lightning Ball)
//   - AttackId::A1098Magneton (Spinning Attack)
//   - AttackId::A1099Voltorb (Tackle)
//   - AttackId::A1100Electrode (Electro Ball)
//   - AttackId::A1104ZapdosEx (Peck)
//   - AttackId::A1105Blitzle (Zap Kick)
//   - AttackId::A1107Tynamo (Tiny Charge)
//   - AttackId::A1108Eelektrik (Head Bolt)
//   - AttackId::A1110Helioptile (Tail Whap)
//   - AttackId::A1113Clefairy (Slap)
//   - AttackId::A1114Clefable (Magical Shot)
//   - AttackId::A1116Kadabra (Super Psy Bolt)
//   - AttackId::A1118Slowpoke (Tail Whap)
//   - AttackId::A1119Slowbro (Super Psy Bolt)
//   - AttackId::A1120Gastly (Suffocating Gas)
//   - AttackId::A1121Haunter (Will-O-Wisp)
//   - AttackId::A1123GengarEx (Spooky Shot)
//   - AttackId::A1124Drowzee (Mumble)
//   - AttackId::A1125Hypno (Psypunch)
//   - AttackId::A1129MewtwoEx (Psychic Sphere)
//   - AttackId::A1130Ralts (Ram)
//   - AttackId::A1131Kirlia (Smack)
//   - AttackId::A1132Gardevoir (Psyshot)
//   - AttackId::A1133Woobat (Gnaw)
//   - AttackId::A1134Swoobat (Heart Stamp)
//   - AttackId::A1135Golett (Mega Punch)
//   - AttackId::A1137Sandshrew (Scratch)
//   - AttackId::A1138Sandslash (Slash)
//   - AttackId::A1139Diglett (Mud-Slap)
//   - AttackId::A1141Mankey (Low Kick)
//   - AttackId::A1143Machop (Knuckle Punch)
//   - AttackId::A1144Machoke (Strength)
//   - AttackId::A1145Machamp (Seismic Toss)
//   - AttackId::A1146MachampEx (Mega Punch)
//   - AttackId::A1147Geodude (Tackle)
//   - AttackId::A1148Graveler (Rollout)
//   - AttackId::A1150Onix (Land Crush)
//   - AttackId::A1152Marowak (Bone Beatdown)
//   - AttackId::A1155Hitmonchan (Jab)
//   - AttackId::A1156Rhyhorn (Tackle)
//   - AttackId::A1157Rhydon (Horn Drill)
//   - AttackId::A1158Kabuto (Shell Attack)
//   - AttackId::A1160Mienfoo (Pound)
//   - AttackId::A1161Mienshao (Spiral Kick)
//   - AttackId::A1162Clobbopus (Knuckle Punch)
//   - AttackId::A1164Ekans (Bite)
//   - AttackId::A1167Nidorina (Bite)
//   - AttackId::A1169NidoranM (Peck)
//   - AttackId::A1170Nidorino (Horn Attack)
//   - AttackId::A1172Zubat (Glide)
//   - AttackId::A1173Golbat (Wing Attack)
//   - AttackId::A1176Koffing (Suffocating Gas)
//   - AttackId::A1177Weezing (Tackle)
//   - AttackId::A1179Pawniard (Pierce)
//   - AttackId::A1180Bisharp (Metal Claw)
//   - AttackId::A1182Melmetal (Heavy Impact)
//   - AttackId::A1183Dratini (Ram)
//   - AttackId::A1184Dragonair (Tail Smack)
//   - AttackId::A1186Pidgey (Gust)
//   - AttackId::A1187Pidgeotto (Gust)
//   - AttackId::A1188Pidgeot (Wing Attack)
//   - AttackId::A1189Rattata (Gnaw)
//   - AttackId::A1190Raticate (Bite)
//   - AttackId::A1191Spearow (Peck)
//   - AttackId::A1193Jigglypuff (Pound)
//   - AttackId::A1194Wigglytuff (Hyper Voice)
//   - AttackId::A1198Farfetchd (Leek Slap)
//   - AttackId::A1199Doduo (Peck)
//   - AttackId::A1200Dodrio (Drill Peck)
//   - AttackId::A1202Chansey (Gentle Slap)
//   - AttackId::A1204Tauros (Horn Attack)
//   - AttackId::A1206Eevee (Tackle)
//   - AttackId::A1207Eevee (Tackle)
//   - AttackId::A1208Eevee (Tackle)
//   - AttackId::A1209Porygon (Sharpen)
//   - AttackId::A1211Snorlax (Rollout)
//   - AttackId::A1212Minccino (Tail Smack)
//   - AttackId::A1214Wooloo (Tackle)
//   - AttackId::A1215Dubwool (Rolling Tackle)
//   - AttackId::A1227Bulbasaur (Vine Whip)
//   - AttackId::A1228Gloom (Drool)
//   - AttackId::A1231Rapidash (Fire Mane)
//   - AttackId::A1232Squirtle (Water Gun)
//   - AttackId::A1235Electrode (Electro Ball)
//   - AttackId::A1237Slowpoke (Tail Whap)
//   - AttackId::A1238Diglett (Mud-Slap)
//   - AttackId::A1242Golbat (Wing Attack)
//   - AttackId::A1243Weezing (Tackle)
//   - AttackId::A1245Pidgeot (Wing Attack)
//   - AttackId::A1248Eevee (Tackle)
//   - AttackId::A1249Porygon (Sharpen)
//   - AttackId::A1250Snorlax (Rollout)
//   - AttackId::A1251VenusaurEx (Razor Leaf)
//   - AttackId::A1253CharizardEx (Slash)
//   - AttackId::A1255MoltresEx (Heat Blast)
//   - AttackId::A1256BlastoiseEx (Surf)
//   - AttackId::A1257StarmieEx (Hydro Splash)
//   - AttackId::A1258ArticunoEx (Ice Wing)
//   - AttackId::A1260ZapdosEx (Peck)
//   - AttackId::A1261GengarEx (Spooky Shot)
//   - AttackId::A1262MewtwoEx (Psychic Sphere)
//   - AttackId::A1263MachampEx (Mega Punch)
//   - AttackId::A1274MoltresEx (Heat Blast)
//   - AttackId::A1275ArticunoEx (Ice Wing)
//   - AttackId::A1276ZapdosEx (Peck)
//   - AttackId::A1277GengarEx (Spooky Shot)
//   - AttackId::A1278MachampEx (Mega Punch)
//   - AttackId::A1280CharizardEx (Slash)
//   - AttackId::A1282MewtwoEx (Psychic Sphere)
//   - AttackId::A1284CharizardEx (Slash)
//   - AttackId::A1286MewtwoEx (Psychic Sphere)
//   - AttackId::A1a004Snivy (Vine Whip)
//   - AttackId::A1a005Servine (Vine Whip)
//   - AttackId::A1a006Serperior (Solar Beam)
//   - AttackId::A1a007Morelull (Ram)
//   - AttackId::A1a013Larvesta (Combustion)
//   - AttackId::A1a019Vaporeon (Wave Splash)
//   - AttackId::A1a020Finneon (Water Gun)
//   - AttackId::A1a022Chewtle (Bite)
//   - AttackId::A1a028Joltik (Bug Bite)
//   - AttackId::A1a032MewEx (Psyshot)
//   - AttackId::A1a034Elgyem (Headbutt)
//   - AttackId::A1a039Swirlix (Fairy Wind)
//   - AttackId::A1a040Slurpuff (Magical Shot)
//   - AttackId::A1a042Primeape (Punch)
//   - AttackId::A1a043Geodude (Light Punch)
//   - AttackId::A1a044Graveler (Lunge Out)
//   - AttackId::A1a046AerodactylEx (Land Crush)
//   - AttackId::A1a048Stonjourner (Mega Kick)
//   - AttackId::A1a051Purrloin (Scratch)
//   - AttackId::A1a052Liepard (Slash)
//   - AttackId::A1a053Venipede (Ram)
//   - AttackId::A1a056Druddigon (Dragon Claw)
//   - AttackId::A1a057Pidgey (Flap)
//   - AttackId::A1a058Pidgeotto (Wing Attack)
//   - AttackId::A1a070Serperior (Solar Beam)
//   - AttackId::A1a072Vaporeon (Wave Splash)
//   - AttackId::A1a077MewEx (Psyshot)
//   - AttackId::A1a078AerodactylEx (Land Crush)
//   - AttackId::A1a083MewEx (Psyshot)
//   - AttackId::A1a084AerodactylEx (Land Crush)
//   - AttackId::A1a086MewEx (Psyshot)
//   - AttackId::A2002Gloom (Razor Leaf)
//   - AttackId::A2003Bellossom (Leaf Step)
//   - AttackId::A2004Tangela (Vine Whip)
//   - AttackId::A2006Yanma (Flap)
//   - AttackId::A2008Roselia (Sting)
//   - AttackId::A2010Turtwig (Bite)
//   - AttackId::A2011Grotle (Razor Leaf)
//   - AttackId::A2013Kricketot (Bug Bite)
//   - AttackId::A2015Burmy (Tackle)
//   - AttackId::A2018Vespiquen (Pierce)
//   - AttackId::A2022Shaymin (Flop)
//   - AttackId::A2025Slugma (Flare)
//   - AttackId::A2027Chimchar (Scratch)
//   - AttackId::A2028Monferno (Fiery Punch)
//   - AttackId::A2031Swinub (Headbutt)
//   - AttackId::A2032Piloswine (Hammer In)
//   - AttackId::A2033Mamoswine (Frosty Flattening)
//   - AttackId::A2034Regice (Frost Smash)
//   - AttackId::A2036Prinplup (Surf)
//   - AttackId::A2038Buizel (Water Gun)
//   - AttackId::A2040Shellos (Mud-Slap)
//   - AttackId::A2043Lumineon (Waterfall)
//   - AttackId::A2045Abomasnow (Frost Breath)
//   - AttackId::A2047WashRotom (Wave Splash)
//   - AttackId::A2049PalkiaEx (Slash)
//   - AttackId::A2051Magnemite (Ram)
//   - AttackId::A2052Magneton (Lightning Ball)
//   - AttackId::A2055Electrode (Rolling Attack)
//   - AttackId::A2059Luxio (Electric Claws)
//   - AttackId::A2063Togepi (Pound)
//   - AttackId::A2064Togetic (Fairy Wind)
//   - AttackId::A2066Misdreavus (Mumble)
//   - AttackId::A2069Kirlia (Slap)
//   - AttackId::A2070Duskull (Will-O-Wisp)
//   - AttackId::A2071Dusclops (Psypunch)
//   - AttackId::A2072Dusknoir (Devour Soul)
//   - AttackId::A2074Drifblim (Balloon Strike)
//   - AttackId::A2078Giratina (Spooky Shot)
//   - AttackId::A2080Rhyhorn (Horn Attack)
//   - AttackId::A2081Rhydon (Wrack Down)
//   - AttackId::A2083Gligar (Pierce)
//   - AttackId::A2085Hitmontop (Spinning Attack)
//   - AttackId::A2086Nosepass (Ram)
//   - AttackId::A2087Regirock (Boulder Crush)
//   - AttackId::A2088Cranidos (Headbutt)
//   - AttackId::A2090Wormadam (Land Crush)
//   - AttackId::A2091Riolu (Jab)
//   - AttackId::A2092Lucario (Submarine Blow)
//   - AttackId::A2093Hippopotas (Rolling Tackle)
//   - AttackId::A2094Hippowdon (Earthen Press)
//   - AttackId::A2096Murkrow (Peck)
//   - AttackId::A2100Poochyena (Bite)
//   - AttackId::A2101Mightyena (Darkness Fang)
//   - AttackId::A2102Stunky (Scratch)
//   - AttackId::A2105Skorupi (Pierce)
//   - AttackId::A2110DarkraiEx (Dark Prism)
//   - AttackId::A2112Registeel (Metal Claw)
//   - AttackId::A2113Shieldon (Headbutt)
//   - AttackId::A2114Bastiodon (Headbang)
//   - AttackId::A2116Bronzor (Tackle)
//   - AttackId::A2119DialgaEx (Heavy Impact)
//   - AttackId::A2121Gible (Gnaw)
//   - AttackId::A2122Gabite (Slash)
//   - AttackId::A2123Garchomp (Dragon Claw)
//   - AttackId::A2124Lickitung (Tongue Slap)
//   - AttackId::A2127Porygon (Beam)
//   - AttackId::A2128Porygon2 (Sharpen)
//   - AttackId::A2130Aipom (Tail Jab)
//   - AttackId::A2133Staravia (Wing Attack)
//   - AttackId::A2136Bibarel (Rolling Tackle)
//   - AttackId::A2137Buneary (Splash)
//   - AttackId::A2159Shaymin (Flop)
//   - AttackId::A2160Mamoswine (Frosty Flattening)
//   - AttackId::A2167Giratina (Spooky Shot)
//   - AttackId::A2170Lucario (Submarine Blow)
//   - AttackId::A2171Hippopotas (Rolling Tackle)
//   - AttackId::A2175Garchomp (Dragon Claw)
//   - AttackId::A2182PalkiaEx (Slash)
//   - AttackId::A2187DarkraiEx (Dark Prism)
//   - AttackId::A2188DialgaEx (Heavy Impact)
//   - AttackId::A2202DarkraiEx (Dark Prism)
//   - AttackId::A2204PalkiaEx (Slash)
//   - AttackId::A2205DialgaEx (Heavy Impact)
//   - AttackId::A2206PalkiaEx (Slash)
//   - AttackId::A2207DialgaEx (Heavy Impact)
//   - AttackId::A2a002Burmy (Ram)
//   - AttackId::A2a003Mothim (Gust)
//   - AttackId::A2a004Combee (Bug Bite)
//   - AttackId::A2a006Cherubi (Leafage)
//   - AttackId::A2a009Carnivine (Vine Whip)
//   - AttackId::A2a010LeafeonEx (Solar Beam)
//   - AttackId::A2a011Houndour (Flare)
//   - AttackId::A2a014Marill (Water Gun)
//   - AttackId::A2a015Azumarill (Waterfall)
//   - AttackId::A2a016Barboach (Mud-Slap)
//   - AttackId::A2a018Snorunt (Icicle)
//   - AttackId::A2a020Snover (Corkscrew Punch)
//   - AttackId::A2a021Abomasnow (Mega Punch)
//   - AttackId::A2a022GlaceonEx (Freezing Wind)
//   - AttackId::A2a027Electrike (Zap Kick)
//   - AttackId::A2a029Clefairy (Smack)
//   - AttackId::A2a032Haunter (Mumble)
//   - AttackId::A2a034Unown (Hidden Power)
//   - AttackId::A2a037Phanpy (Rollout)
//   - AttackId::A2a039Larvitar (Corkscrew Punch)
//   - AttackId::A2a040Pupitar (Speed Attack)
//   - AttackId::A2a041Tyranitar (Land Crush)
//   - AttackId::A2a042Nosepass (Tackle)
//   - AttackId::A2a043Meditite (Kick)
//   - AttackId::A2a045Gible (Tackle)
//   - AttackId::A2a046Gabite (Sharp Scythe)
//   - AttackId::A2a047GarchompEx (Dragon Claw)
//   - AttackId::A2a048Zubat (Bite)
//   - AttackId::A2a049Golbat (Bite)
//   - AttackId::A2a050Crobat (Darkness Fang)
//   - AttackId::A2a051Croagunk (Beat)
//   - AttackId::A2a053Magnemite (Tackle)
//   - AttackId::A2a054Magneton (Rolling Attack)
//   - AttackId::A2a055Magnezone (Power Beam)
//   - AttackId::A2a058Bronzor (Ram)
//   - AttackId::A2a062Eevee (Tail Whap)
//   - AttackId::A2a064Hoothoot (Peck)
//   - AttackId::A2a066Starly (Glide)
//   - AttackId::A2a067Staravia (Wing Attack)
//   - AttackId::A2a069Shaymin (Flap)
//   - AttackId::A2a077Marill (Water Gun)
//   - AttackId::A2a078Unown (Hidden Power)
//   - AttackId::A2a080Magnemite (Tackle)
//   - AttackId::A2a081Shaymin (Flap)
//   - AttackId::A2a082LeafeonEx (Solar Beam)
//   - AttackId::A2a083GlaceonEx (Freezing Wind)
//   - AttackId::A2a084GarchompEx (Dragon Claw)
//   - AttackId::A2a091LeafeonEx (Solar Beam)
//   - AttackId::A2a092GlaceonEx (Freezing Wind)
//   - AttackId::A2a093GarchompEx (Dragon Claw)
//   - AttackId::A2b006Floragato (Slash)
//   - AttackId::A2b008Charmander (Combustion)
//   - AttackId::A2b009Charmeleon (Combustion)
//   - AttackId::A2b010CharizardEx (Steam Artillery)
//   - AttackId::A2b011Magmar (Flare)
//   - AttackId::A2b014Tentacool (Rain Splash)
//   - AttackId::A2b015Tentacruel (Wave Splash)
//   - AttackId::A2b016Buizel (Water Gun)
//   - AttackId::A2b021Tatsugiri (Rain Splash)
//   - AttackId::A2b023Voltorb (Lightning Ball)
//   - AttackId::A2b026Pawmi (Punch)
//   - AttackId::A2b027Pawmo (Punch)
//   - AttackId::A2b028Pawmot (Electric Punch)
//   - AttackId::A2b029Abra (Psyshot)
//   - AttackId::A2b030Kadabra (Psyshot)
//   - AttackId::A2b033Drifloon (Ram)
//   - AttackId::A2b034Drifblim (Gust)
//   - AttackId::A2b037Machop (Low Kick)
//   - AttackId::A2b040Hitmonlee (Kick)
//   - AttackId::A2b041Hitmonchan (Magnum Punch)
//   - AttackId::A2b042Riolu (Punch)
//   - AttackId::A2b045Ekans (Ram)
//   - AttackId::A2b050Shroodle (Gnaw)
//   - AttackId::A2b051Grafaiai (Bite)
//   - AttackId::A2b052Tinkatink (Corkscrew Punch)
//   - AttackId::A2b055Varoom (Headbutt)
//   - AttackId::A2b058Rattata (Tackle)
//   - AttackId::A2b059Raticate (Tackle)
//   - AttackId::A2b060Jigglypuff (Rollout)
//   - AttackId::A2b061Wigglytuff (Gentle Slap)
//   - AttackId::A2b062Lickitung (Rollout)
//   - AttackId::A2b063Lickilicky (Rollout)
//   - AttackId::A2b064Bidoof (Headbutt)
//   - AttackId::A2b066Buneary (Pound)
//   - AttackId::A2b067Lopunny (Hopping Shot)
//   - AttackId::A2b074Buizel (Water Gun)
//   - AttackId::A2b075Tatsugiri (Rain Splash)
//   - AttackId::A2b076Grafaiai (Bite)
//   - AttackId::A2b078Wigglytuff (Gentle Slap)
//   - AttackId::A2b080CharizardEx (Steam Artillery)
//   - AttackId::A2b099Charmander (Combustion)
//   - AttackId::A2b100Charmeleon (Combustion)
//   - AttackId::A2b104Riolu (Punch)
//   - AttackId::A2b105Varoom (Headbutt)
//   - AttackId::A2b108CharizardEx (Steam Artillery)
//   - AttackId::A3001Exeggcute (Rolling Tackle)
//   - AttackId::A3004Masquerain (Bug Buzz)
//   - AttackId::A3005Maractus (Sting)
//   - AttackId::A3006Karrablast (Headbutt)
//   - AttackId::A3007Phantump (Hook)
//   - AttackId::A3008Trevenant (Claw Slash)
//   - AttackId::A3009Rowlet (Leafage)
//   - AttackId::A3011Dartrix (Razor Wing)
//   - AttackId::A3012DecidueyeEx (Razor Leaf)
//   - AttackId::A3013Grubbin (Gnaw)
//   - AttackId::A3014Fomantis (Leafage)
//   - AttackId::A3016Morelull (Stampede)
//   - AttackId::A3018Bounsweet (Stampede)
//   - AttackId::A3021Wimpod (Gnaw)
//   - AttackId::A3025Growlithe (Combustion)
//   - AttackId::A3026Arcanine (Fire Mane)
//   - AttackId::A3028Fletchinder (Steady Firebreathing)
//   - AttackId::A3029Talonflame (Fire Wing)
//   - AttackId::A3031Litten (Scratch)
//   - AttackId::A3035Salandit (Gnaw)
//   - AttackId::A3038AlolanSandshrew (Scratch)
//   - AttackId::A3042Shellder (Tackle)
//   - AttackId::A3044Lapras (Surf)
//   - AttackId::A3046Popplio (Water Gun)
//   - AttackId::A3047Brionne (Wave Splash)
//   - AttackId::A3048Primarina (Surf)
//   - AttackId::A3052Dewpider (Hook)
//   - AttackId::A3054Pyukumuku (Sprinkle Water)
//   - AttackId::A3055Bruxish (Wave Splash)
//   - AttackId::A3057Pikachu (Tail Smack)
//   - AttackId::A3059AlolanGeodude (Knuckle Punch)
//   - AttackId::A3060AlolanGraveler (Heavy Impact)
//   - AttackId::A3062Helioptile (Smash Kick)
//   - AttackId::A3063Heliolisk (Rear Kick)
//   - AttackId::A3064Charjabug (Vise Grip)
//   - AttackId::A3066Oricorio (Zzzap)
//   - AttackId::A3072Grumpig (Zen Headbutt)
//   - AttackId::A3073Lunatone (Moon Press)
//   - AttackId::A3074Shuppet (Will-O-Wisp)
//   - AttackId::A3078Cutiefly (Flap)
//   - AttackId::A3079Ribombee (Fairy Wind)
//   - AttackId::A3080Comfey (Spinning Attack)
//   - AttackId::A3081Sandygast (Vibration)
//   - AttackId::A3082Palossand (Spooky Shot)
//   - AttackId::A3087LunalaEx (Lunar Blast)
//   - AttackId::A3089Cubone (Beat)
//   - AttackId::A3090Makuhita (Slap Push)
//   - AttackId::A3092Solrock (Solar Beam)
//   - AttackId::A3093Drilbur (Scratch)
//   - AttackId::A3094Timburr (Corkscrew Punch)
//   - AttackId::A3095Gurdurr (Strength)
//   - AttackId::A3096Conkeldurr (Mega Punch)
//   - AttackId::A3097Crabrawler (Punch)
//   - AttackId::A3099Rockruff (Tackle)
//   - AttackId::A3102Mudbray (Rear Kick)
//   - AttackId::A3104PassimianEx (Seismic Toss)
//   - AttackId::A3106AlolanRattata (Bite)
//   - AttackId::A3108AlolanMeowth (Scratch)
//   - AttackId::A3109AlolanPersian (Claw Slash)
//   - AttackId::A3110AlolanGrimer (Sludge Toss)
//   - AttackId::A3113Trubbish (Pound)
//   - AttackId::A3115Mareanie (Pierce)
//   - AttackId::A3117AlolanDiglett (Headbutt)
//   - AttackId::A3119Excadrill (Slash)
//   - AttackId::A3125Jangmoo (Headbutt)
//   - AttackId::A3126Hakamoo (Dragon Claw)
//   - AttackId::A3129Skitty (Tackle)
//   - AttackId::A3131Fletchling (Flap)
//   - AttackId::A3133Pikipek (Peck)
//   - AttackId::A3134Trumbeak (Glide)
//   - AttackId::A3135Toucannon (Drill Peck)
//   - AttackId::A3136Yungoos (Tackle)
//   - AttackId::A3137Gumshoos (Headbang)
//   - AttackId::A3138Stufful (Hammer In)
//   - AttackId::A3141Komala (Rolling Tackle)
//   - AttackId::A3157Morelull (Stampede)
//   - AttackId::A3163Pyukumuku (Sprinkle Water)
//   - AttackId::A3165Oricorio (Zzzap)
//   - AttackId::A3167Cutiefly (Flap)
//   - AttackId::A3168Comfey (Spinning Attack)
//   - AttackId::A3169Sandygast (Vibration)
//   - AttackId::A3177Pikipek (Peck)
//   - AttackId::A3179Komala (Rolling Tackle)
//   - AttackId::A3180DecidueyeEx (Razor Leaf)
//   - AttackId::A3186LunalaEx (Lunar Blast)
//   - AttackId::A3187PassimianEx (Seismic Toss)
//   - AttackId::A3198DecidueyeEx (Razor Leaf)
//   - AttackId::A3204LunalaEx (Lunar Blast)
//   - AttackId::A3205PassimianEx (Seismic Toss)
//   - AttackId::A3210Bulbasaur (Vine Whip)
//   - AttackId::A3211Ivysaur (Razor Leaf)
//   - AttackId::A3213Exeggcute (Rolling Tackle)
//   - AttackId::A3215Squirtle (Water Gun)
//   - AttackId::A3216Wartortle (Wave Splash)
//   - AttackId::A3218Staryu (Smack)
//   - AttackId::A3219Starmie (Wave Splash)
//   - AttackId::A3221Haunter (Will-O-Wisp)
//   - AttackId::A3223Machop (Knuckle Punch)
//   - AttackId::A3224Machoke (Strength)
//   - AttackId::A3227Marowak (Bone Beatdown)
//   - AttackId::A3228Jigglypuff (Rollout)
//   - AttackId::A3229Wigglytuff (Gentle Slap)
//   - AttackId::A3230VenusaurEx (Razor Leaf)
//   - AttackId::A3232BlastoiseEx (Surf)
//   - AttackId::A3233StarmieEx (Hydro Splash)
//   - AttackId::A3234GengarEx (Spooky Shot)
//   - AttackId::A3235MachampEx (Mega Punch)
//   - AttackId::A3238LunalaEx (Lunar Blast)
//   - AttackId::A3a001Petilil (Seed Bomb)
//   - AttackId::A3a002Lilligant (Cut)
//   - AttackId::A3a006BuzzwoleEx (Punch)
//   - AttackId::A3a008Kartana (Thrash Metal)
//   - AttackId::A3a010Mantine (Surf)
//   - AttackId::A3a011Carvanha (Bite)
//   - AttackId::A3a013Shinx (Bite)
//   - AttackId::A3a014Luxio (Head Bolt)
//   - AttackId::A3a015Luxray (Electric Ball)
//   - AttackId::A3a016Blitzle (Rear Kick)
//   - AttackId::A3a017Zebstrika (Head Bolt)
//   - AttackId::A3a018Emolga (Static Shock)
//   - AttackId::A3a019TapuKokoEx (Mach Bolt)
//   - AttackId::A3a021Zeraora (Lightning Claw)
//   - AttackId::A3a022Clefairy (Pound)
//   - AttackId::A3a023Clefable (Moon Kick)
//   - AttackId::A3a024Phantump (Spooky Shot)
//   - AttackId::A3a025Trevenant (Wrack Down)
//   - AttackId::A3a026Morelull (Hook)
//   - AttackId::A3a027Shiinotic (Gentle Slap)
//   - AttackId::A3a029Medicham (High Jump Kick)
//   - AttackId::A3a030Baltoy (Spinning Attack)
//   - AttackId::A3a031Claydol (Power Beam)
//   - AttackId::A3a032Rockruff (Rock Throw)
//   - AttackId::A3a035Sandygast (Sandy Shot)
//   - AttackId::A3a039Sandile (Bite)
//   - AttackId::A3a040Krokorok (Light Punch)
//   - AttackId::A3a043GuzzlordEx (Tyrannical Hole)
//   - AttackId::A3a048Aron (Headbutt)
//   - AttackId::A3a049Lairon (Lunge Out)
//   - AttackId::A3a051Ferroseed (Spike Sting)
//   - AttackId::A3a052Ferrothorn (Spinning Attack)
//   - AttackId::A3a054Lillipup (Tackle)
//   - AttackId::A3a055Herdier (Bite)
//   - AttackId::A3a056Stoutland (Sharp Fang)
//   - AttackId::A3a057Stufful (Ram)
//   - AttackId::A3a059Oranguru (Hammer In)
//   - AttackId::A3a076BuzzwoleEx (Punch)
//   - AttackId::A3a077TapuKokoEx (Mach Bolt)
//   - AttackId::A3a079GuzzlordEx (Tyrannical Hole)
//   - AttackId::A3a084TapuKokoEx (Mach Bolt)
//   - AttackId::A3a086GuzzlordEx (Tyrannical Hole)
//   - AttackId::A3a088BuzzwoleEx (Punch)
//   - AttackId::A3a089Growlithe (Bite)
//   - AttackId::A3a091Froakie (Flop)
//   - AttackId::A3a092Frogadier (Water Drip)
//   - AttackId::A3a093Greninja (Mist Slash)
//   - AttackId::A3a095Pidgey (Gust)
//   - AttackId::A3a096Pidgeotto (Gust)
//   - AttackId::A3a097Pidgeot (Wing Attack)
//   - AttackId::A3a101AerodactylEx (Land Crush)
//   - AttackId::A3b001Tropius (Cutting Wind)
//   - AttackId::A3b003Bounsweet (Flop)
//   - AttackId::A3b004Steenee (Leaf Step)
//   - AttackId::A3b006Applin (Ram)
//   - AttackId::A3b011Litten (Flare)
//   - AttackId::A3b012Torracat (Claw Slash)
//   - AttackId::A3b015Salazzle (Combustion)
//   - AttackId::A3b022Popplio (Watering)
//   - AttackId::A3b023Brionne (Water Gun)
//   - AttackId::A3b029Woobat (Gust)
//   - AttackId::A3b030Swoobat (Psyshot)
//   - AttackId::A3b034SylveonEx (Fairy Wind)
//   - AttackId::A3b038Barboach (Mud Shot)
//   - AttackId::A3b041Mienshao (Low Kick)
//   - AttackId::A3b044Sableye (Scratch)
//   - AttackId::A3b046Liepard (Slashing Claw)
//   - AttackId::A3b049Meltan (Headbutt)
//   - AttackId::A3b050Melmetal (Mega Punch)
//   - AttackId::A3b051Dratini (Beat)
//   - AttackId::A3b052Dragonair (Waterfall)
//   - AttackId::A3b056EeveeEx (Bite)
//   - AttackId::A3b059Ambipom (Corkscrew Punch)
//   - AttackId::A3b062Minccino (Pound)
//   - AttackId::A3b063Cinccino (Slap)
//   - AttackId::A3b064Skwovet (Bite)
//   - AttackId::A3b081SylveonEx (Fairy Wind)
//   - AttackId::A3b083EeveeEx (Bite)
//   - AttackId::A3b089SylveonEx (Fairy Wind)
//   - AttackId::A3b092EeveeEx (Bite)
//   - AttackId::A3b096Electrode (Rolling Attack)
//   - AttackId::A3b098Kirlia (Slap)
//   - AttackId::A3b099Gardevoir (Psyshot)
//   - AttackId::A3b100Ekans (Bite)
//   - AttackId::A3b102Farfetchd (Leek Slap)
//   - AttackId::A3b103MoltresEx (Heat Blast)
//   - AttackId::A3b104ArticunoEx (Ice Wing)
//   - AttackId::A3b105ZapdosEx (Peck)
//   - AttackId::A4002Gloom (Leaf Step)
//   - AttackId::A4006Scyther (Slash)
//   - AttackId::A4007Pinsir (Guillotine)
//   - AttackId::A4008Chikorita (Razor Leaf)
//   - AttackId::A4009Bayleef (Vine Whip)
//   - AttackId::A4011Ledyba (Bug Bite)
//   - AttackId::A4012Ledian (Punch)
//   - AttackId::A4013Hoppip (Tackle)
//   - AttackId::A4014Skiploom (Seed Bomb)
//   - AttackId::A4015Jumpluff (Spinning Attack)
//   - AttackId::A4017Sunflora (Solar Beam)
//   - AttackId::A4019Yanmega (Bug Buzz)
//   - AttackId::A4020Pineco (Ram)
//   - AttackId::A4024Cherrim (Seed Bomb)
//   - AttackId::A4025Vulpix (Live Coal)
//   - AttackId::A4027Cyndaquil (Flare)
//   - AttackId::A4028Quilava (Combustion)
//   - AttackId::A4029Typhlosion (Magma Punch)
//   - AttackId::A4030Slugma (Stampede)
//   - AttackId::A4035Darumaka (Flop)
//   - AttackId::A4036Darmanitan (Heat Blast)
//   - AttackId::A4038Poliwag (Water Gun)
//   - AttackId::A4039Poliwhirl (Wave Splash)
//   - AttackId::A4040Politoed (Hyper Voice)
//   - AttackId::A4041Horsea (Hook)
//   - AttackId::A4042Seadra (Razor Fin)
//   - AttackId::A4046Totodile (Wave Splash)
//   - AttackId::A4047Croconaw (Bite)
//   - AttackId::A4049Marill (Tackle)
//   - AttackId::A4050Azumarill (Tail Smack)
//   - AttackId::A4051Wooper (Rain Splash)
//   - AttackId::A4052Quagsire (Surf)
//   - AttackId::A4055Remoraid (Water Gun)
//   - AttackId::A4060Corphish (Vise Grip)
//   - AttackId::A4061Crawdaunt (Crabhammer)
//   - AttackId::A4062Ducklett (Rain Splash)
//   - AttackId::A4064Chinchou (Lightning Ball)
//   - AttackId::A4067Mareep (Static Shock)
//   - AttackId::A4068Flaaffy (Zap Kick)
//   - AttackId::A4079Togetic (Speed Dive)
//   - AttackId::A4080Togekiss (Magical Shot)
//   - AttackId::A4081Natu (Peck)
//   - AttackId::A4083EspeonEx (Super Psy Bolt)
//   - AttackId::A4084Unown (Hidden Power)
//   - AttackId::A4085Unown (Hidden Power)
//   - AttackId::A4088Snubbull (Ram)
//   - AttackId::A4090Munna (Psyshot)
//   - AttackId::A4092Onix (Headbutt)
//   - AttackId::A4093Sudowoodo (Rock Throw)
//   - AttackId::A4094Gligar (Glide)
//   - AttackId::A4095Gliscor (Sharp Fang)
//   - AttackId::A4096Swinub (Mud-Slap)
//   - AttackId::A4097Piloswine (Headbutt Bounce)
//   - AttackId::A4101Tyrogue (Slappy Knuckle)
//   - AttackId::A4103Larvitar (Tackle)
//   - AttackId::A4106Barbaracle (Dynamic Chop)
//   - AttackId::A4110Spinarak (Pierce)
//   - AttackId::A4112UmbreonEx (Darkness Fang)
//   - AttackId::A4114Honchkrow (Dark Cutter)
//   - AttackId::A4115Sneasel (Scratch)
//   - AttackId::A4116Weavile (Slash)
//   - AttackId::A4117Houndour (Bite)
//   - AttackId::A4119Tyranitar (Buster Tail)
//   - AttackId::A4121Forretress (Shell Crash)
//   - AttackId::A4122Steelix (Heavy Impact)
//   - AttackId::A4125Mawile (Bite)
//   - AttackId::A4126Klink (Vise Grip)
//   - AttackId::A4127Klang (Gear Cutter)
//   - AttackId::A4129Spearow (Glide)
//   - AttackId::A4135Porygon (Ram)
//   - AttackId::A4136Porygon2 (Spinning Attack)
//   - AttackId::A4138Sentret (Scratch)
//   - AttackId::A4140Hoothoot (Wing Attack)
//   - AttackId::A4143Ambipom (Tail Jab)
//   - AttackId::A4162Chikorita (Razor Leaf)
//   - AttackId::A4165Cyndaquil (Flare)
//   - AttackId::A4167Totodile (Wave Splash)
//   - AttackId::A4176Gligar (Glide)
//   - AttackId::A4177Spinarak (Pierce)
//   - AttackId::A4179Tyranitar (Buster Tail)
//   - AttackId::A4181Sentret (Scratch)
//   - AttackId::A4182Hoothoot (Wing Attack)
//   - AttackId::A4190EspeonEx (Super Psy Bolt)
//   - AttackId::A4193UmbreonEx (Darkness Fang)
//   - AttackId::A4205EspeonEx (Super Psy Bolt)
//   - AttackId::A4208UmbreonEx (Darkness Fang)
//   - AttackId::A4217Magnemite (Lightning Ball)
//   - AttackId::A4218Magneton (Spinning Attack)
//   - AttackId::A4220Misdreavus (Mumble)
//   - AttackId::A4224Nidorina (Bite)
//   - AttackId::A4226NidoranM (Peck)
//   - AttackId::A4227Nidorino (Horn Attack)
//   - AttackId::A4229Sneasel (Scratch)
//   - AttackId::A4230Lickitung (Tongue Slap)
//   - AttackId::A4233LeafeonEx (Solar Beam)
//   - AttackId::A4235GlaceonEx (Freezing Wind)
//   - AttackId::A4a001Hoppip (Splash)
//   - AttackId::A4a004Sunkern (Tackle)
//   - AttackId::A4a008Slugma (Combustion)
//   - AttackId::A4a009Magcargo (Heat Blast)
//   - AttackId::A4a011Fletchinder (Fire Wing)
//   - AttackId::A4a014Poliwhirl (Ram)
//   - AttackId::A4a017Slowpoke (Water Gun)
//   - AttackId::A4a022Milotic (Aqua Edge)
//   - AttackId::A4a026Tynamo (Tiny Bolt)
//   - AttackId::A4a027Eelektrik (Electro Ball)
//   - AttackId::A4a029Stunfisk (Static Shock)
//   - AttackId::A4a030Yamper (Zap Kick)
//   - AttackId::A4a032Misdreavus (Will-O-Wisp)
//   - AttackId::A4a034GalarianCorsola (Tackle)
//   - AttackId::A4a035GalarianCursola (Spooky Shot)
//   - AttackId::A4a039Jellicent (Devour Soul)
//   - AttackId::A4a040Diglett (Headbutt)
//   - AttackId::A4a044Donphan (Land Crush)
//   - AttackId::A4a046Dwebble (Sand Spray)
//   - AttackId::A4a049Zorua (Ram)
//   - AttackId::A4a050Zoroark (Night Daze)
//   - AttackId::A4a051Inkay (Tackle)
//   - AttackId::A4a053Skrelp (Melt)
//   - AttackId::A4a054Dragalge (Sludge Bomb)
//   - AttackId::A4a056Farfetchd (Leek Slam)
//   - AttackId::A4a061Ursaring (Claw Slash)
//   - AttackId::A4a064Swablu (Peck)
//   - AttackId::A4a065Zangoose (Slash)
//   - AttackId::A4a072Milotic (Aqua Edge)
//   - AttackId::A4a073Stunfisk (Static Shock)
//   - AttackId::A4a074Yamper (Zap Kick)
//   - AttackId::A4a091Chimchar (Scratch)
//   - AttackId::A4a092Monferno (Fiery Punch)
//   - AttackId::A4a094Golduck (Aqua Edge)
//   - AttackId::A4a095Krabby (Vise Grip)
//   - AttackId::A4a097Pyukumuku (Sprinkle Water)
//   - AttackId::A4a098Gible (Tackle)
//   - AttackId::A4a099Gabite (Sharp Scythe)
//   - AttackId::A4a102MewEx (Psyshot)
//   - AttackId::A4a103GarchompEx (Dragon Claw)
//   - AttackId::A4b001Bulbasaur (Vine Whip)
//   - AttackId::A4b002Bulbasaur (Vine Whip)
//   - AttackId::A4b003Ivysaur (Razor Leaf)
//   - AttackId::A4b004Ivysaur (Razor Leaf)
//   - AttackId::A4b005VenusaurEx (Razor Leaf)
//   - AttackId::A4b011Exeggcute (Seed Bomb)
//   - AttackId::A4b012Exeggcute (Seed Bomb)
//   - AttackId::A4b014Hoppip (Tackle)
//   - AttackId::A4b015Hoppip (Tackle)
//   - AttackId::A4b016Skiploom (Seed Bomb)
//   - AttackId::A4b017Skiploom (Seed Bomb)
//   - AttackId::A4b018Jumpluff (Spinning Attack)
//   - AttackId::A4b019Jumpluff (Spinning Attack)
//   - AttackId::A4b020Yanma (Flap)
//   - AttackId::A4b021Yanma (Flap)
//   - AttackId::A4b027Cherrim (Seed Bomb)
//   - AttackId::A4b028Cherrim (Seed Bomb)
//   - AttackId::A4b029LeafeonEx (Solar Beam)
//   - AttackId::A4b030Shaymin (Flop)
//   - AttackId::A4b031Shaymin (Flop)
//   - AttackId::A4b032Snivy (Vine Whip)
//   - AttackId::A4b033Snivy (Vine Whip)
//   - AttackId::A4b034Servine (Vine Whip)
//   - AttackId::A4b035Servine (Vine Whip)
//   - AttackId::A4b036Serperior (Solar Beam)
//   - AttackId::A4b037Serperior (Solar Beam)
//   - AttackId::A4b040Dartrix (Razor Wing)
//   - AttackId::A4b041Dartrix (Razor Wing)
//   - AttackId::A4b042DecidueyeEx (Razor Leaf)
//   - AttackId::A4b044BuzzwoleEx (Punch)
//   - AttackId::A4b047Kartana (Thrash Metal)
//   - AttackId::A4b048Kartana (Thrash Metal)
//   - AttackId::A4b051Floragato (Slash)
//   - AttackId::A4b052Floragato (Slash)
//   - AttackId::A4b057Charmeleon (Fire Claws)
//   - AttackId::A4b058Charmeleon (Fire Claws)
//   - AttackId::A4b059CharizardEx (Slash)
//   - AttackId::A4b060CharizardEx (Steam Artillery)
//   - AttackId::A4b061Growlithe (Bite)
//   - AttackId::A4b062Growlithe (Bite)
//   - AttackId::A4b067MoltresEx (Heat Blast)
//   - AttackId::A4b071Chimchar (Scratch)
//   - AttackId::A4b072Chimchar (Scratch)
//   - AttackId::A4b073Monferno (Fiery Punch)
//   - AttackId::A4b074Monferno (Fiery Punch)
//   - AttackId::A4b083Squirtle (Water Gun)
//   - AttackId::A4b084Squirtle (Water Gun)
//   - AttackId::A4b085Wartortle (Wave Splash)
//   - AttackId::A4b086Wartortle (Wave Splash)
//   - AttackId::A4b087BlastoiseEx (Surf)
//   - AttackId::A4b088Horsea (Hook)
//   - AttackId::A4b089Horsea (Hook)
//   - AttackId::A4b090Seadra (Razor Fin)
//   - AttackId::A4b091Seadra (Razor Fin)
//   - AttackId::A4b093Staryu (Smack)
//   - AttackId::A4b094Staryu (Smack)
//   - AttackId::A4b095StarmieEx (Hydro Splash)
//   - AttackId::A4b099Vaporeon (Wave Splash)
//   - AttackId::A4b100Vaporeon (Wave Splash)
//   - AttackId::A4b101ArticunoEx (Ice Wing)
//   - AttackId::A4b102Corphish (Vise Grip)
//   - AttackId::A4b103Corphish (Vise Grip)
//   - AttackId::A4b104Crawdaunt (Crabhammer)
//   - AttackId::A4b105Crawdaunt (Crabhammer)
//   - AttackId::A4b106GlaceonEx (Freezing Wind)
//   - AttackId::A4b107PalkiaEx (Slash)
//   - AttackId::A4b110Froakie (Flop)
//   - AttackId::A4b111Froakie (Flop)
//   - AttackId::A4b112Frogadier (Water Drip)
//   - AttackId::A4b113Frogadier (Water Drip)
//   - AttackId::A4b114Greninja (Mist Slash)
//   - AttackId::A4b115Greninja (Mist Slash)
//   - AttackId::A4b116Popplio (Watering)
//   - AttackId::A4b117Popplio (Watering)
//   - AttackId::A4b118Brionne (Water Gun)
//   - AttackId::A4b119Brionne (Water Gun)
//   - AttackId::A4b128Pikachu (Gnaw)
//   - AttackId::A4b129Pikachu (Gnaw)
//   - AttackId::A4b133Magnemite (Lightning Ball)
//   - AttackId::A4b134Magnemite (Lightning Ball)
//   - AttackId::A4b135Magneton (Spinning Attack)
//   - AttackId::A4b136Magneton (Spinning Attack)
//   - AttackId::A4b139ZapdosEx (Peck)
//   - AttackId::A4b140Chinchou (Lightning Ball)
//   - AttackId::A4b141Chinchou (Lightning Ball)
//   - AttackId::A4b146Oricorio (Zzzap)
//   - AttackId::A4b147Oricorio (Zzzap)
//   - AttackId::A4b148TapuKokoEx (Mach Bolt)
//   - AttackId::A4b149Zeraora (Lightning Claw)
//   - AttackId::A4b150Zeraora (Lightning Claw)
//   - AttackId::A4b151Gastly (Suffocating Gas)
//   - AttackId::A4b152Gastly (Suffocating Gas)
//   - AttackId::A4b153Haunter (Will-O-Wisp)
//   - AttackId::A4b154Haunter (Will-O-Wisp)
//   - AttackId::A4b155GengarEx (Spooky Shot)
//   - AttackId::A4b158MewtwoEx (Psychic Sphere)
//   - AttackId::A4b159MewEx (Psyshot)
//   - AttackId::A4b160EspeonEx (Super Psy Bolt)
//   - AttackId::A4b161Misdreavus (Mumble)
//   - AttackId::A4b162Misdreavus (Mumble)
//   - AttackId::A4b166Kirlia (Slap)
//   - AttackId::A4b167Kirlia (Slap)
//   - AttackId::A4b168Gardevoir (Psyshot)
//   - AttackId::A4b169Gardevoir (Psyshot)
//   - AttackId::A4b170Giratina (Spooky Shot)
//   - AttackId::A4b171Giratina (Spooky Shot)
//   - AttackId::A4b177SylveonEx (Fairy Wind)
//   - AttackId::A4b184LunalaEx (Lunar Blast)
//   - AttackId::A4b189Machop (Knuckle Punch)
//   - AttackId::A4b190Machop (Knuckle Punch)
//   - AttackId::A4b191Machoke (Strength)
//   - AttackId::A4b192Machoke (Strength)
//   - AttackId::A4b193MachampEx (Mega Punch)
//   - AttackId::A4b197AerodactylEx (Land Crush)
//   - AttackId::A4b203Nosepass (Tackle)
//   - AttackId::A4b204Nosepass (Tackle)
//   - AttackId::A4b205Gible (Tackle)
//   - AttackId::A4b206Gible (Tackle)
//   - AttackId::A4b207Gabite (Sharp Scythe)
//   - AttackId::A4b208Gabite (Sharp Scythe)
//   - AttackId::A4b209GarchompEx (Dragon Claw)
//   - AttackId::A4b210Riolu (Punch)
//   - AttackId::A4b211Riolu (Punch)
//   - AttackId::A4b212Lucario (Submarine Blow)
//   - AttackId::A4b213Lucario (Submarine Blow)
//   - AttackId::A4b216Drilbur (Scratch)
//   - AttackId::A4b217Drilbur (Scratch)
//   - AttackId::A4b218Crabrawler (Punch)
//   - AttackId::A4b219Crabrawler (Punch)
//   - AttackId::A4b220Rockruff (Rock Throw)
//   - AttackId::A4b221Rockruff (Rock Throw)
//   - AttackId::A4b223PassimianEx (Seismic Toss)
//   - AttackId::A4b226Zubat (Bite)
//   - AttackId::A4b227Zubat (Bite)
//   - AttackId::A4b228Golbat (Bite)
//   - AttackId::A4b229Golbat (Bite)
//   - AttackId::A4b230Crobat (Darkness Fang)
//   - AttackId::A4b231Crobat (Darkness Fang)
//   - AttackId::A4b233AlolanGrimer (Sludge Toss)
//   - AttackId::A4b234AlolanGrimer (Sludge Toss)
//   - AttackId::A4b241UmbreonEx (Darkness Fang)
//   - AttackId::A4b245DarkraiEx (Dark Prism)
//   - AttackId::A4b248GuzzlordEx (Tyrannical Hole)
//   - AttackId::A4b254DialgaEx (Heavy Impact)
//   - AttackId::A4b255Excadrill (Slash)
//   - AttackId::A4b256Excadrill (Slash)
//   - AttackId::A4b262Tinkatink (Corkscrew Punch)
//   - AttackId::A4b263Tinkatink (Corkscrew Punch)
//   - AttackId::A4b267Dratini (Beat)
//   - AttackId::A4b268Dratini (Beat)
//   - AttackId::A4b269Dragonair (Waterfall)
//   - AttackId::A4b270Dragonair (Waterfall)
//   - AttackId::A4b272Pidgey (Flap)
//   - AttackId::A4b273Pidgey (Flap)
//   - AttackId::A4b274Pidgeotto (Wing Attack)
//   - AttackId::A4b275Pidgeotto (Wing Attack)
//   - AttackId::A4b277Jigglypuff (Pound)
//   - AttackId::A4b278Jigglypuff (Pound)
//   - AttackId::A4b280Farfetchd (Leek Slap)
//   - AttackId::A4b281Farfetchd (Leek Slap)
//   - AttackId::A4b282Lickitung (Tongue Slap)
//   - AttackId::A4b283Lickitung (Tongue Slap)
//   - AttackId::A4b287EeveeEx (Bite)
//   - AttackId::A4b290Skitty (Tackle)
//   - AttackId::A4b291Skitty (Tackle)
//   - AttackId::A4b294Bidoof (Headbutt)
//   - AttackId::A4b295Bidoof (Headbutt)
//   - AttackId::A4b297Shaymin (Flap)
//   - AttackId::A4b298Shaymin (Flap)
//   - AttackId::A4b354Floragato (Slash)
//   - AttackId::A4b355Crawdaunt (Crabhammer)
//   - AttackId::A4b356Greninja (Mist Slash)
//   - AttackId::A4b357Gardevoir (Psyshot)
//   - AttackId::A4b359Farfetchd (Leek Slap)
//   - AttackId::A4b360BuzzwoleEx (Punch)
//   - AttackId::A4b361CharizardEx (Slash)
//   - AttackId::A4b363PalkiaEx (Slash)
//   - AttackId::A4b365MewtwoEx (Psychic Sphere)
//   - AttackId::A4b366MewEx (Psyshot)
//   - AttackId::A4b367LunalaEx (Lunar Blast)
//   - AttackId::A4b368DialgaEx (Heavy Impact)
//   - AttackId::A4b370EeveeEx (Bite)
//   - AttackId::A4b378DarkraiEx (Dark Prism)
//   - AttackId::B1003Wurmple (Ram)
//   - AttackId::B1008Seedot (Rollout)
//   - AttackId::B1009Nuzleaf (Gentle Slap)
//   - AttackId::B1011Shroomish (Tackle)
//   - AttackId::B1012Breloom (Seed Bomb)
//   - AttackId::B1013Pansage (Beat)
//   - AttackId::B1015Cottonee (Razor Leaf)
//   - AttackId::B1017Petilil (Flop)
//   - AttackId::B1018Lilligant (Smack)
//   - AttackId::B1021Skiddo (Razor Leaf)
//   - AttackId::B1023Phantump (Branch Poke)
//   - AttackId::B1025Grookey (Beat)
//   - AttackId::B1026Thwackey (Hammer In)
//   - AttackId::B1027Rillaboom (Drum Rush)
//   - AttackId::B1028Growlithe (Dig Claws)
//   - AttackId::B1030Ponyta (Combustion)
//   - AttackId::B1033Torchic (Peck)
//   - AttackId::B1034Combusken (High Jump Kick)
//   - AttackId::B1037Pansear (Beat)
//   - AttackId::B1041Litwick (Flare)
//   - AttackId::B1042Lampent (Will-O-Wisp)
//   - AttackId::B1043Chandelure (Heat Blast)
//   - AttackId::B1045Litleo (Live Coal)
//   - AttackId::B1048Psyduck (Rain Splash)
//   - AttackId::B1049Golduck (Wave Splash)
//   - AttackId::B1053Lotad (Water Gun)
//   - AttackId::B1054Lombre (Gentle Slap)
//   - AttackId::B1056Wailmer (Surf)
//   - AttackId::B1058Corphish (Water Gun)
//   - AttackId::B1059Crawdaunt (Guillotine)
//   - AttackId::B1061Panpour (Beat)
//   - AttackId::B1063Tympole (Mud-Slap)
//   - AttackId::B1064Palpitoad (Hyper Voice)
//   - AttackId::B1066Tirtouga (Shell Attack)
//   - AttackId::B1068Frillish (Water Gun)
//   - AttackId::B1069Jellicent (Surf)
//   - AttackId::B1071Froakie (Water Drip)
//   - AttackId::B1073GreninjaEx (Aqua Edge)
//   - AttackId::B1074Bergmite (Icicle)
//   - AttackId::B1076Chewtle (Wave Splash)
//   - AttackId::B1078Arrokuda (Peck)
//   - AttackId::B1080Eiscue (Ram)
//   - AttackId::B1081JolteonEx (Mach Bolt)
//   - AttackId::B1082Mareep (Rear Kick)
//   - AttackId::B1083Flaaffy (Electric Punch)
//   - AttackId::B1086Shinx (Flop)
//   - AttackId::B1087Luxio (Bite)
//   - AttackId::B1090Blitzle (Tiny Charge)
//   - AttackId::B1091Zebstrika (Electric Ball)
//   - AttackId::B1092Joltik (Attach)
//   - AttackId::B1093Galvantula (Electro Ball)
//   - AttackId::B1096Boltund (Zap Kick)
//   - AttackId::B1097Natu (Peck)
//   - AttackId::B1098Xatu (Zen Headbutt)
//   - AttackId::B1100Mismagius (Spooky Shot)
//   - AttackId::B1104Dusclops (Will-O-Wisp)
//   - AttackId::B1105Dusknoir (Hammer In)
//   - AttackId::B1110Yamask (Mumble)
//   - AttackId::B1112Gothita (Stampede)
//   - AttackId::B1113Gothorita (Slap)
//   - AttackId::B1115Spritzee (Fairy Wind)
//   - AttackId::B1120Klefki (Hook)
//   - AttackId::B1122Sandshrew (Sand Spray)
//   - AttackId::B1123Sandslash (Cut)
//   - AttackId::B1126Makuhita (Lunge Out)
//   - AttackId::B1128Hippopotas (Mud Shot)
//   - AttackId::B1130Sandile (Gnaw)
//   - AttackId::B1131Krokorok (Bite)
//   - AttackId::B1133Archen (Rock Throw)
//   - AttackId::B1135Golett (Stampede)
//   - AttackId::B1138Pancham (Punch)
//   - AttackId::B1139Crabrawler (Knuckle Punch)
//   - AttackId::B1141Stufful (Magnum Punch)
//   - AttackId::B1142Bewear (Heavy Impact)
//   - AttackId::B1145Rolycoly (Ram)
//   - AttackId::B1146Carkol (Heat Crash)
//   - AttackId::B1148Murkrow (Glide)
//   - AttackId::B1152Skorupi (Bug Bite)
//   - AttackId::B1155Deino (Headbutt)
//   - AttackId::B1156Zweilous (Darkness Fang)
//   - AttackId::B1159Skrelp (Razor Fin)
//   - AttackId::B1160DragalgeEx (Draconic Whip)
//   - AttackId::B1163Impidimp (Bite)
//   - AttackId::B1165Grimmsnarl (Wrack Down)
//   - AttackId::B1166Ferroseed (Rolling Tackle)
//   - AttackId::B1170Honedge (Pierce)
//   - AttackId::B1171Doublade (Slash)
//   - AttackId::B1172Aegislash (Slicing Blade)
//   - AttackId::B1173Meltan (Beam)
//   - AttackId::B1174MelmetalEx (Headbutt)
//   - AttackId::B1177Goomy (Ram)
//   - AttackId::B1178Sliggoo (Gentle Slap)
//   - AttackId::B1180Pidgey (Peck)
//   - AttackId::B1181Pidgeotto (Speed Wing)
//   - AttackId::B1184Eevee (Stampede)
//   - AttackId::B1188Zigzagoon (Tackle)
//   - AttackId::B1189Linoone (Jet Headbutt)
//   - AttackId::B1190Whismur (Pound)
//   - AttackId::B1191Loudred (Hyper Voice)
//   - AttackId::B1194Delcatty (Cat Kick)
//   - AttackId::B1199Patrat (Bite)
//   - AttackId::B1202Herdier (Headbutt Bounce)
//   - AttackId::B1204Rufflet (Wing Attack)
//   - AttackId::B1205Braviary (Slash)
//   - AttackId::B1206Furfrou (Sharp Fang)
//   - AttackId::B1207Furfrou (Sharp Fang)
//   - AttackId::B1208Furfrou (Sharp Fang)
//   - AttackId::B1209Rookidee (Flap)
//   - AttackId::B1210Corvisquire (Drill Peck)
//   - AttackId::B1228Skiddo (Razor Leaf)
//   - AttackId::B1229Rillaboom (Drum Rush)
//   - AttackId::B1230Growlithe (Dig Claws)
//   - AttackId::B1231Chandelure (Heat Blast)
//   - AttackId::B1234Jellicent (Surf)
//   - AttackId::B1236Eiscue (Ram)
//   - AttackId::B1239Gothita (Stampede)
//   - AttackId::B1240Makuhita (Lunge Out)
//   - AttackId::B1243Pancham (Punch)
//   - AttackId::B1247Goomy (Ram)
//   - AttackId::B1248Delcatty (Cat Kick)
//   - AttackId::B1250Rufflet (Wing Attack)
//   - AttackId::B1256GreninjaEx (Aqua Edge)
//   - AttackId::B1257JolteonEx (Mach Bolt)
//   - AttackId::B1263DragalgeEx (Draconic Whip)
//   - AttackId::B1264MelmetalEx (Headbutt)
//   - AttackId::B1275GreninjaEx (Aqua Edge)
//   - AttackId::B1276JolteonEx (Mach Bolt)
//   - AttackId::B1281DragalgeEx (Draconic Whip)
//   - AttackId::B1282MelmetalEx (Headbutt)
//   - AttackId::B1287Bellsprout (Vine Whip)
//   - AttackId::B1288Weepinbell (Razor Leaf)
//   - AttackId::B1289Victreebel (Vine Whip)
//   - AttackId::B1296Poliwhirl (Knuckle Punch)
//   - AttackId::B1297Poliwrath (Mega Punch)
//   - AttackId::B1301Brionne (Water Gun)
//   - AttackId::B1303Oricorio (Zzzap)
//   - AttackId::B1304Zeraora (Lightning Claw)
//   - AttackId::B1305Drowzee (Mumble)
//   - AttackId::B1306Hypno (Psypunch)
//   - AttackId::B1307Geodude (Tackle)
//   - AttackId::B1308Graveler (Lunge Out)
//   - AttackId::B1311AlolanDiglett (Headbutt)
//   - AttackId::B1314Doduo (Peck)
//   - AttackId::B1315Dodrio (Drill Peck)
//   - AttackId::B1317DecidueyeEx (Razor Leaf)
//   - AttackId::B1319PalkiaEx (Slash)
//   - AttackId::B1322TapuKokoEx (Mach Bolt)
//   - AttackId::B1324PassimianEx (Seismic Toss)
//   - AttackId::B1326DialgaEx (Heavy Impact)
//   - AttackId::B1329Lilligant (Smack)
//   - AttackId::B1330Klefki (Hook)
//   - AttackId::PA009Pikachu (Gnaw)
//   - AttackId::PA011Chansey (Gentle Slap)
//   - AttackId::PA013Butterfree (Gust)
//   - AttackId::PA015Pikachu (Gnaw)
//   - AttackId::PA016Clefairy (Slap)
//   - AttackId::PA019Greninja (Mist Slash)
//   - AttackId::PA021Onix (Land Crush)
//   - AttackId::PA023Bulbasaur (Vine Whip)
//   - AttackId::PA024Magnemite (Lightning Ball)
//   - AttackId::PA025MoltresEx (Heat Blast)
//   - AttackId::PA026Pikachu (Gnaw)
//   - AttackId::PA027Snivy (Tackle)
//   - AttackId::PA033Squirtle (Water Gun)
//   - AttackId::PA035Turtwig (Bite)
//   - AttackId::PA037CresseliaEx (Psychic Flash)
//   - AttackId::PA040Chimchar (Scratch)
//   - AttackId::PA041Togepi (Pound)
//   - AttackId::PA042DarkraiEx (Dark Prism)
//   - AttackId::PA046Gible (Tackle)
//   - AttackId::PA047Staraptor (Wing Attack)
//   - AttackId::PA050MewtwoEx (Psychic Sphere)
//   - AttackId::PA054Pawmot (Electric Punch)
//   - AttackId::PA057Bidoof (Headbutt)
//   - AttackId::PA059Riolu (Punch)
//   - AttackId::PA061Froakie (Flop)
//   - AttackId::PA062Farfetchd (Leek Slap)
//   - AttackId::PA071Crabrawler (Punch)
//   - AttackId::PA074Zeraora (Lightning Claw)
//   - AttackId::PA075Kartana (Thrash Metal)
//   - AttackId::PA081UltraNecrozmaEx (Photon Claw)
//   - AttackId::PA083Stufful (Ram)
//   - AttackId::PA084TapuKokoEx (Mach Bolt)
//   - AttackId::PA092Eevee (Tackle)
//   - AttackId::PA099Marill (Tackle)
//   - AttackId::PA100Weavile (Slash)
//   - AttackId::PA104Milotic (Aqua Edge)
//   - AttackId::PA105Zorua (Ram)
//   - AttackId::PA106Zoroark (Night Daze)
//   - AttackId::PA109EeveeEx (Bite)
//   - AttackId::PA116Shaymin (Flop)
//
// Effect0 - 18 attacks:
//   - AttackId::A1003Venusaur (Mega Drain)
//   - AttackId::A1004VenusaurEx (Giant Bloom)
//   - AttackId::A1080Vaporeon (Bubble Drain)
//   - AttackId::A1251VenusaurEx (Giant Bloom)
//   - AttackId::A2005Tangrowth (Mega Drain)
//   - AttackId::A2156Tangrowth (Mega Drain)
//   - AttackId::A2b065BibarelEx (Carefree Press)
//   - AttackId::A2b087BibarelEx (Carefree Press)
//   - AttackId::A2b095BibarelEx (Carefree Press)
//   - AttackId::A3212Venusaur (Mega Drain)
//   - AttackId::A3230VenusaurEx (Giant Bloom)
//   - AttackId::A4216Vaporeon (Bubble Drain)
//   - AttackId::A4b005VenusaurEx (Giant Bloom)
//   - AttackId::A4b296BibarelEx (Carefree Press)
//   - AttackId::B1118Slurpuff (Draining Kiss)
//   - AttackId::B1327BibarelEx (Carefree Press)
//   - AttackId::PA018Venusaur (Mega Drain)
//   - AttackId::PA088Dragonair (Shed Skin)
//
// Effect1 - 5 attacks:
//   - AttackId::A1005Caterpie (Find a Friend)
//   - AttackId::A2b005Sprigatito (Cry for Help)
//   - AttackId::A4b049Sprigatito (Cry for Help)
//   - AttackId::A4b050Sprigatito (Cry for Help)
//   - AttackId::PA052Sprigatito (Cry for Help)
//
// Effect2 - 20 attacks:
//   - AttackId::A1013Vileplume (Soothing Scent)
//   - AttackId::A1093Frosmoth (Powder Snow)
//   - AttackId::A1195WigglytuffEx (Sleepy Song)
//   - AttackId::A1265WigglytuffEx (Sleepy Song)
//   - AttackId::A1279WigglytuffEx (Sleepy Song)
//   - AttackId::A1a008Shiinotic (Flickering Spores)
//   - AttackId::A1a036Flabebe (Hypnotic Gaze)
//   - AttackId::A2109Darkrai (Dark Void)
//   - AttackId::A2a024Phione (Water Pulse)
//   - AttackId::A2a033Gengar (Hypnoblast)
//   - AttackId::A3045Popplio (Sing)
//   - AttackId::A3237WigglytuffEx (Sleepy Song)
//   - AttackId::A3b021Alomomola (Water Pulse)
//   - AttackId::A4004Tangela (Sleep Powder)
//   - AttackId::A4a059Igglybuff (Sleepy Lullaby)
//   - AttackId::A4b279WigglytuffEx (Sleepy Song)
//   - AttackId::B1196Swablu (Sing)
//   - AttackId::B1198Chatot (Sleepy Song)
//   - AttackId::B1300Popplio (Sing)
//   - AttackId::PA022Jigglypuff (Sing)
//
// Effect3 - 34 attacks:
//   - AttackId::A1017Venomoth (Poison Powder)
//   - AttackId::A1063Tentacruel (Poison Tentacles)
//   - AttackId::A1171Nidoking (Poison Horn)
//   - AttackId::A1174Grimer (Poison Gas)
//   - AttackId::A1241Nidoking (Poison Horn)
//   - AttackId::A1a016Salazzle (Poison Claws)
//   - AttackId::A1a054Whirlipede (Poison Sting)
//   - AttackId::A2009Roserade (Poisonous Whip)
//   - AttackId::A2103Skuntank (Poison Gas)
//   - AttackId::A2b046Arbok (Venomous Fang)
//   - AttackId::A2b047PaldeanWooper (Poison Jab)
//   - AttackId::A3114Garbodor (Super Poison Breath)
//   - AttackId::A3a042Nihilego (New Wave)
//   - AttackId::A3a045Naganadel (Electro House)
//   - AttackId::A3a103Nihilego (New Wave)
//   - AttackId::A4001Oddish (Poison Powder)
//   - AttackId::A4053Qwilfish (Poison Sting)
//   - AttackId::A4107Zubat (Venomous Fang)
//   - AttackId::A4108Golbat (Venomous Fang)
//   - AttackId::A4109CrobatEx (Venomous Slash)
//   - AttackId::A4168Qwilfish (Poison Sting)
//   - AttackId::A4192CrobatEx (Venomous Slash)
//   - AttackId::A4207CrobatEx (Venomous Slash)
//   - AttackId::A4228Nidoking (Poison Horn)
//   - AttackId::A4a015Tentacool (Poison Sting)
//   - AttackId::A4a100PaldeanWooper (Poison Jab)
//   - AttackId::A4b232CrobatEx (Venomous Slash)
//   - AttackId::A4b236PaldeanWooper (Poison Jab)
//   - AttackId::A4b237PaldeanWooper (Poison Jab)
//   - AttackId::A4b246Nihilego (New Wave)
//   - AttackId::A4b247Nihilego (New Wave)
//   - AttackId::B1161Mareanie (Poison Sting)
//   - AttackId::PA056Ekans (Poison Sting)
//   - AttackId::PA072AlolanGrimer (Poison Gas)
//
// Effect4 - 16 attacks:
//   - AttackId::A1022Exeggutor (Stomp)
//   - AttackId::A1a010Ponyta (Stomp)
//   - AttackId::A2016Wormadam (Leaf Cutter)
//   - AttackId::A2030HeatRotom (Heat Breath)
//   - AttackId::A2039Floatzel (Jet Screw)
//   - AttackId::A2b024Electrode (Tumbling Attack)
//   - AttackId::A2b038Machoke (Pummel)
//   - AttackId::A2b053Tinkatuff (Tenacious Hammer)
//   - AttackId::A3067Togedemaru (Electrosmash)
//   - AttackId::A4a005Sunflora (Trip Over)
//   - AttackId::A4a066Fletchling (Quick Attack)
//   - AttackId::A4b264Tinkatuff (Tenacious Hammer)
//   - AttackId::A4b265Tinkatuff (Tenacious Hammer)
//   - AttackId::B1103Duskull (Ambush)
//   - AttackId::B1193Skitty (Play Rough)
//   - AttackId::PA111Pikachu (Quick Attack)
//
// Effect5 - 6 attacks:
//   - AttackId::A1023ExeggutorEx (Tropical Swing)
//   - AttackId::A1111Heliolisk (Quick Attack)
//   - AttackId::A1252ExeggutorEx (Tropical Swing)
//   - AttackId::A3231ExeggutorEx (Tropical Swing)
//   - AttackId::A4b013ExeggutorEx (Tropical Swing)
//   - AttackId::B1200Watchog (Biting Fang)
//
// Effect6 - 7 attacks:
//   - AttackId::A1024Tangela (Absorb)
//   - AttackId::A1029Petilil (Blot)
//   - AttackId::A1a037Floette (Leaf Drain)
//   - AttackId::A2001Oddish (Blot)
//   - AttackId::A4016Sunkern (Blot)
//   - AttackId::A4a038Frillish (Absorb)
//   - AttackId::PA089Audino (Drain Slap)
//
// Effect7 - 3 attacks:
//   - AttackId::A1026Pinsir (Double Horn)
//   - AttackId::A1229Pinsir (Double Horn)
//   - AttackId::A2b044Flamigo (Double Kick)
//
// Effect9 - 25 attacks:
//   - AttackId::A1031Skiddo (Surprise Attack)
//   - AttackId::A1046Moltres (Sky Attack)
//   - AttackId::A1073Seaking (Horn Hazard)
//   - AttackId::A1a041Mankey (Focus Fist)
//   - AttackId::A2139Glameow (Pose)
//   - AttackId::A2178Glameow (Pose)
//   - AttackId::A2a044Medicham (Kick Shot)
//   - AttackId::A3002AlolanExeggutor (Tropical Hammer)
//   - AttackId::A3156AlolanExeggutor (Tropical Hammer)
//   - AttackId::A3a046AlolanDiglett (Happened to Headbutt)
//   - AttackId::A3a062Celesteela (Moombahton)
//   - AttackId::A3a075Celesteela (Moombahton)
//   - AttackId::A3b040Mienfoo (Kick Shot)
//   - AttackId::A4139Furret (Tail Smash)
//   - AttackId::A4221Mankey (Focus Fist)
//   - AttackId::A4a060Teddiursa (Surprise Attack)
//   - AttackId::A4b249AlolanDiglett (Happened to Headbutt)
//   - AttackId::A4b250AlolanDiglett (Happened to Headbutt)
//   - AttackId::A4b304Celesteela (Moombahton)
//   - AttackId::A4b305Celesteela (Moombahton)
//   - AttackId::B1185Aipom (Tail Smash)
//   - AttackId::B1292Moltres (Sky Attack)
//   - AttackId::PA020Haunter (Surprise Attack)
//   - AttackId::PA069AlolanExeggutor (Tropical Hammer)
//   - AttackId::PA103Poliwag (Surprise Attack)
//
// Effect10 - 15 attacks:
//   - AttackId::A1033Charmander (Ember)
//   - AttackId::A1038Ninetales (Flamethrower)
//   - AttackId::A1045Flareon (Flamethrower)
//   - AttackId::A1052Centiskorch (Fire Blast)
//   - AttackId::A1230Charmander (Ember)
//   - AttackId::A3037Turtonator (Fire Spin)
//   - AttackId::A3161Turtonator (Fire Spin)
//   - AttackId::A3b010Torkoal (Flamethrower)
//   - AttackId::A3b014Salandit (Ember)
//   - AttackId::A4b055Charmander (Ember)
//   - AttackId::A4b056Charmander (Ember)
//   - AttackId::A4b069Torkoal (Flamethrower)
//   - AttackId::A4b070Torkoal (Flamethrower)
//   - AttackId::B1035Blaziken (Blaze Kick)
//   - AttackId::PA032Charmander (Ember)
//
// Effect11 - 13 attacks:
//   - AttackId::A1035Charizard (Fire Spin)
//   - AttackId::A1036CharizardEx (Crimson Storm)
//   - AttackId::A1253CharizardEx (Crimson Storm)
//   - AttackId::A1280CharizardEx (Crimson Storm)
//   - AttackId::A1284CharizardEx (Crimson Storm)
//   - AttackId::A1a012Magmar (Fire Blast)
//   - AttackId::A3b009FlareonEx (Fire Spin)
//   - AttackId::A3b079FlareonEx (Fire Spin)
//   - AttackId::A3b087FlareonEx (Fire Spin)
//   - AttackId::A4b059CharizardEx (Crimson Storm)
//   - AttackId::A4b066FlareonEx (Fire Spin)
//   - AttackId::A4b361CharizardEx (Crimson Storm)
//   - AttackId::B1046Pyroar (Fire Blast)
//
// Effect12 - 2 attacks:
//   - AttackId::A1037Vulpix (Tail Whip)
//   - AttackId::A3017Shiinotic (Flickering Light)
//
// Effect13 - 19 attacks:
//   - AttackId::A1040Arcanine (Heat Tackle)
//   - AttackId::A1041ArcanineEx (Inferno Onrush)
//   - AttackId::A1254ArcanineEx (Inferno Onrush)
//   - AttackId::A2120Heatran (Steel Tackle)
//   - AttackId::A2174Heatran (Steel Tackle)
//   - AttackId::A2a068Staraptor (Brave Bird)
//   - AttackId::A2b035GiratinaEx (Chaotic Impact)
//   - AttackId::A2b083GiratinaEx (Chaotic Impact)
//   - AttackId::A2b096GiratinaEx (Chaotic Impact)
//   - AttackId::A3a090Arcanine (Heat Tackle)
//   - AttackId::A3a100ArcanineEx (Inferno Onrush)
//   - AttackId::A4071Raikou (Thunder)
//   - AttackId::A4089Granbull (Wild Tackle)
//   - AttackId::A4a028Eelektross (Wild Charge)
//   - AttackId::A4b063ArcanineEx (Inferno Onrush)
//   - AttackId::A4b172GiratinaEx (Chaotic Impact)
//   - AttackId::A4b377GiratinaEx (Chaotic Impact)
//   - AttackId::B1022Gogoat (Double-Edge)
//   - AttackId::B1079Barraskewda (Double-Edge)
//
// Effect14 - 6 attacks:
//   - AttackId::A1047MoltresEx (Inferno Dance)
//   - AttackId::A1255MoltresEx (Inferno Dance)
//   - AttackId::A1274MoltresEx (Inferno Dance)
//   - AttackId::A3b103MoltresEx (Inferno Dance)
//   - AttackId::A4b067MoltresEx (Inferno Dance)
//   - AttackId::PA025MoltresEx (Inferno Dance)
//
// Effect15 - 7 attacks:
//   - AttackId::A1055Blastoise (Hydro Pump)
//   - AttackId::A1056BlastoiseEx (Hydro Bazooka)
//   - AttackId::A1256BlastoiseEx (Hydro Bazooka)
//   - AttackId::A3217Blastoise (Hydro Pump)
//   - AttackId::A3232BlastoiseEx (Hydro Bazooka)
//   - AttackId::A4b087BlastoiseEx (Hydro Bazooka)
//   - AttackId::PA029Blastoise (Hydro Pump)
//
// Effect16 - 4 attacks:
//   - AttackId::A1057Psyduck (Headache)
//   - AttackId::A1122Gengar (Bother)
//   - AttackId::A3222Gengar (Bother)
//   - AttackId::A4a093Psyduck (Headache)
//
// Effect17 - 2 attacks:
//   - AttackId::A1069Kingler (KO Crab)
//   - AttackId::A4a096Kingler (KO Crab)
//
// Effect18 - 11 attacks:
//   - AttackId::A1071Seadra (Water Arrow)
//   - AttackId::A2097Honchkrow (Skill Dive)
//   - AttackId::A2a047GarchompEx (Linear Attack)
//   - AttackId::A2a084GarchompEx (Linear Attack)
//   - AttackId::A2a093GarchompEx (Linear Attack)
//   - AttackId::A3b017Glaceon (Ice Blade)
//   - AttackId::A3b073Glaceon (Ice Blade)
//   - AttackId::A4a103GarchompEx (Linear Attack)
//   - AttackId::A4b209GarchompEx (Linear Attack)
//   - AttackId::B1005Beautifly (Skill Dive)
//   - AttackId::B1227Beautifly (Skill Dive)
//
// Effect19 - 7 attacks:
//   - AttackId::A1078Gyarados (Hyper Beam)
//   - AttackId::A1233Gyarados (Hyper Beam)
//   - AttackId::A2b003BeedrillEx (Crushing Spear)
//   - AttackId::A2b079BeedrillEx (Crushing Spear)
//   - AttackId::A2b107BeedrillEx (Crushing Spear)
//   - AttackId::A4048Feraligatr (Destructive Whirlpool)
//   - AttackId::A4b010BeedrillEx (Crushing Spear)
//
// Effect20 - 3 attacks:
//   - AttackId::A1079Lapras (Hydro Pump)
//   - AttackId::A1234Lapras (Hydro Pump)
//   - AttackId::A3b094Lapras (Hydro Pump)
//
// Effect22 - 17 attacks:
//   - AttackId::A1083Articuno (Ice Beam)
//   - AttackId::A1109Eelektross (Thunder Fang)
//   - AttackId::A1112Pincurchin (Thunder Shock)
//   - AttackId::A1a030Dedenne (Thunder Shock)
//   - AttackId::A1a073Dedenne (Thunder Shock)
//   - AttackId::A2046Glaceon (Ice Beam)
//   - AttackId::A2b002Kakuna (String Shot)
//   - AttackId::A2b098Kakuna (String Shot)
//   - AttackId::A3a038AlolanPersian (Fake Out)
//   - AttackId::A3b026Joltik (Jolt)
//   - AttackId::A3b047Mawile (Powerful Vise)
//   - AttackId::A4076Jynx (Attract Smack)
//   - AttackId::A4144Dunsparce (Sudden Flash)
//   - AttackId::A4b008Kakuna (String Shot)
//   - AttackId::A4b009Kakuna (String Shot)
//   - AttackId::B1187Miltank (Body Slam)
//   - AttackId::B1298Articuno (Ice Beam)
//
// Effect23 - 9 attacks:
//   - AttackId::A1084ArticunoEx (Blizzard)
//   - AttackId::A1258ArticunoEx (Blizzard)
//   - AttackId::A1275ArticunoEx (Blizzard)
//   - AttackId::A2048FrostRotom (Blizzard)
//   - AttackId::A2a019Froslass (Blizzard)
//   - AttackId::A2b020Dondozo (Ocean Cyclone)
//   - AttackId::A2b102Dondozo (Ocean Cyclone)
//   - AttackId::A3b104ArticunoEx (Blizzard)
//   - AttackId::A4b101ArticunoEx (Blizzard)
//
// Effect24 - 2 attacks:
//   - AttackId::A1091Bruxish (Second Strike)
//   - AttackId::PA053Floatzel (Attack the Wound)
//
// Effect25 - 10 attacks:
//   - AttackId::A1095Raichu (Thunderbolt)
//   - AttackId::A2b022PikachuEx (Thunderbolt)
//   - AttackId::A2b082PikachuEx (Thunderbolt)
//   - AttackId::A2b092PikachuEx (Thunderbolt)
//   - AttackId::A4a037Latios (Luster Purge)
//   - AttackId::A4a075Latios (Luster Purge)
//   - AttackId::A4b132PikachuEx (Thunderbolt)
//   - AttackId::B1157Hydreigon (Hyper Ray)
//   - AttackId::B1245Hydreigon (Hyper Ray)
//   - AttackId::B1321PikachuEx (Thunderbolt)
//
// Effect26 - 7 attacks:
//   - AttackId::A1096PikachuEx (Circle Circuit)
//   - AttackId::A1259PikachuEx (Circle Circuit)
//   - AttackId::A1281PikachuEx (Circle Circuit)
//   - AttackId::A1285PikachuEx (Circle Circuit)
//   - AttackId::A4b131PikachuEx (Circle Circuit)
//   - AttackId::A4b364PikachuEx (Circle Circuit)
//   - AttackId::A4b376PikachuEx (Circle Circuit)
//
// Effect27 - 3 attacks:
//   - AttackId::A1101Electabuzz (Thunder Punch)
//   - AttackId::A4069Ampharos (Thunder Punch)
//   - AttackId::A4172Ampharos (Thunder Punch)
//
// Effect29 - 2 attacks:
//   - AttackId::A1103Zapdos (Raging Thunder)
//   - AttackId::B1302Zapdos (Raging Thunder)
//
// Effect30 - 6 attacks:
//   - AttackId::A1104ZapdosEx (Thundering Hurricane)
//   - AttackId::A1260ZapdosEx (Thundering Hurricane)
//   - AttackId::A1276ZapdosEx (Thundering Hurricane)
//   - AttackId::A3b105ZapdosEx (Thundering Hurricane)
//   - AttackId::A4b139ZapdosEx (Thundering Hurricane)
//   - AttackId::PA114Machamp (Hurricane Punch)
//
// Effect31 - 4 attacks:
//   - AttackId::A1106Zebstrika (Thunder Spear)
//   - AttackId::A2b017Floatzel (Water Arrow)
//   - AttackId::A4a057Lickitung (Stretch Tongue)
//   - AttackId::B1106Jirachi (Star Drop)
//
// Effect32 - 19 attacks:
//   - AttackId::A1115Abra (Teleport)
//   - AttackId::A1a017Magikarp (Leap Out)
//   - AttackId::A2068Ralts (Teleport)
//   - AttackId::A3085Cosmog (Teleport)
//   - AttackId::A3171Cosmog (Teleport)
//   - AttackId::A3b097Ralts (Teleport)
//   - AttackId::A4018Yanma (U-turn)
//   - AttackId::A4212Yanma (U-turn)
//   - AttackId::A4214Magikarp (Leap Out)
//   - AttackId::A4a002Skiploom (Bounce)
//   - AttackId::A4a021Feebas (Leap Out)
//   - AttackId::A4b096Magikarp (Leap Out)
//   - AttackId::A4b097Magikarp (Leap Out)
//   - AttackId::A4b164Ralts (Teleport)
//   - AttackId::A4b165Ralts (Teleport)
//   - AttackId::A4b180Cosmog (Teleport)
//   - AttackId::A4b181Cosmog (Teleport)
//   - AttackId::B1072Frogadier (Bounce)
//   - AttackId::PA067Cosmog (Teleport)
//
// Effect33 - 10 attacks:
//   - AttackId::A1117Alakazam (Psychic)
//   - AttackId::A1236Alakazam (Psychic)
//   - AttackId::A3058AlolanRaichuEx (Psychic)
//   - AttackId::A3185AlolanRaichuEx (Psychic)
//   - AttackId::A3203AlolanRaichuEx (Psychic)
//   - AttackId::A4074Slowking (Psychic)
//   - AttackId::A4b130AlolanRaichuEx (Psychic)
//   - AttackId::B1121IndeedeeEx (Psychic)
//   - AttackId::B1260IndeedeeEx (Psychic)
//   - AttackId::B1278IndeedeeEx (Psychic)
//
// Effect34 - 18 attacks:
//   - AttackId::A1126MrMime (Barrier Attack)
//   - AttackId::A2073Drifloon (Expand)
//   - AttackId::A2117Bronzong (Guard Press)
//   - AttackId::A2165Drifloon (Expand)
//   - AttackId::A2a057ProbopassEx (Defensive Unit)
//   - AttackId::A2a085ProbopassEx (Defensive Unit)
//   - AttackId::A2a094ProbopassEx (Defensive Unit)
//   - AttackId::A3043Cloyster (Guard Press)
//   - AttackId::A3a053Stakataka (Brass Rock)
//   - AttackId::A4124SkarmoryEx (Steel Wing)
//   - AttackId::A4194SkarmoryEx (Steel Wing)
//   - AttackId::A4209SkarmoryEx (Steel Wing)
//   - AttackId::A4b252SkarmoryEx (Steel Wing)
//   - AttackId::A4b253ProbopassEx (Defensive Unit)
//   - AttackId::B1075Avalugg (Frost Barrier)
//   - AttackId::B1117Swirlix (Cotton Guard)
//   - AttackId::B1167Ferrothorn (Guard Press)
//   - AttackId::PA080Stakataka (Brass Rock)
//
// Effect35 - 13 attacks:
//   - AttackId::A1127Jynx (Psychic)
//   - AttackId::A1a002Exeggutor (Psychic)
//   - AttackId::A1a069Exeggutor (Psychic)
//   - AttackId::A2095GalladeEx (Energized Blade)
//   - AttackId::A2185GalladeEx (Energized Blade)
//   - AttackId::A2200GalladeEx (Energized Blade)
//   - AttackId::A2a059Bronzong (Psychic)
//   - AttackId::A3214Exeggutor (Psychic)
//   - AttackId::A3a094Jynx (Psychic)
//   - AttackId::A3b106GalladeEx (Energized Blade)
//   - AttackId::A4b156Jynx (Psychic)
//   - AttackId::A4b157Jynx (Psychic)
//   - AttackId::A4b215GalladeEx (Energized Blade)
//
// Effect36 - 9 attacks:
//   - AttackId::A1128Mewtwo (Power Blast)
//   - AttackId::A1129MewtwoEx (Psydrive)
//   - AttackId::A1262MewtwoEx (Psydrive)
//   - AttackId::A1282MewtwoEx (Psydrive)
//   - AttackId::A1286MewtwoEx (Psydrive)
//   - AttackId::A4b158MewtwoEx (Psydrive)
//   - AttackId::A4b365MewtwoEx (Psydrive)
//   - AttackId::PA010Mewtwo (Power Blast)
//   - AttackId::PA050MewtwoEx (Psydrive)
//
// Effect37 - 2 attacks:
//   - AttackId::A1136Golurk (Double Lariat)
//   - AttackId::A3b013Incineroar (Darkest Lariat)
//
// Effect38 - 12 attacks:
//   - AttackId::A1140Dugtrio (Dig)
//   - AttackId::A1a024Cramorant (Dive)
//   - AttackId::A2042Finneon (Elegant Swim)
//   - AttackId::A2058Shinx (Hide)
//   - AttackId::A2163Shinx (Hide)
//   - AttackId::A2b036Gimmighoul (Chest-ouflage)
//   - AttackId::A2b056Revavroom (Spinning Drift)
//   - AttackId::A2b106Revavroom (Spinning Drift)
//   - AttackId::A3069MrMime (Barrier Shove)
//   - AttackId::A3b042Carbink (Hard Roll)
//   - AttackId::A4044Magikarp (Splashing Dodge)
//   - AttackId::A4058Mantine (Dive)
//
// Effect39 - 7 attacks:
//   - AttackId::A1142Primeape (Fight Back)
//   - AttackId::A3033IncineroarEx (Scar-Charged Smash)
//   - AttackId::A3182IncineroarEx (Scar-Charged Smash)
//   - AttackId::A3200IncineroarEx (Scar-Charged Smash)
//   - AttackId::A4222Primeape (Fight Back)
//   - AttackId::A4b082IncineroarEx (Scar-Charged Smash)
//   - AttackId::B1318IncineroarEx (Scar-Charged Smash)
//
// Effect41 - 9 attacks:
//   - AttackId::A1151Cubone (Growl)
//   - AttackId::A1239Cubone (Growl)
//   - AttackId::A3226Cubone (Growl)
//   - AttackId::A4059Suicune (Cure Stream)
//   - AttackId::A4078Togepi (Charm)
//   - AttackId::A4173Togepi (Charm)
//   - AttackId::A4b194Cubone (Growl)
//   - AttackId::A4b195Cubone (Growl)
//   - AttackId::PA030Eevee (Growl)
//
// Effect42 - 5 attacks:
//   - AttackId::A1153MarowakEx (Bonemerang)
//   - AttackId::A1264MarowakEx (Bonemerang)
//   - AttackId::A3236MarowakEx (Bonemerang)
//   - AttackId::A3b020Vanilluxe (Double Spin)
//   - AttackId::A4b196MarowakEx (Bonemerang)
//
// Effect43 - 4 attacks:
//   - AttackId::A1154Hitmonlee (Stretch Kick)
//   - AttackId::A4120Absol (Leap Over)
//   - AttackId::B1044Heatmor (Tongue Whip)
//   - AttackId::PA117Absol (Leap Over)
//
// Effect45 - 3 attacks:
//   - AttackId::A1163Grapploct (Knock Back)
//   - AttackId::A3091Hariyama (Push Out)
//   - AttackId::B1095Yamper (Roar)
//
// Effect46 - 20 attacks:
//   - AttackId::A1165Arbok (Corner)
//   - AttackId::A1a029Galvantula (Electroweb)
//   - AttackId::A2134Staraptor (Clutch)
//   - AttackId::A2176Staraptor (Clutch)
//   - AttackId::A2a012Houndoom (Corner)
//   - AttackId::A2a076Houndoom (Corner)
//   - AttackId::A3023DhelmiseEx (Anchor Shot)
//   - AttackId::A3070Sableye (Corner)
//   - AttackId::A3181DhelmiseEx (Anchor Shot)
//   - AttackId::A3199DhelmiseEx (Anchor Shot)
//   - AttackId::A3a036Palossand (Sand Tomb)
//   - AttackId::A3b101Arbok (Corner)
//   - AttackId::A4043KingdraEx (Stormy Prison)
//   - AttackId::A4111Ariados (Bind Down)
//   - AttackId::A4188KingdraEx (Stormy Prison)
//   - AttackId::A4203KingdraEx (Stormy Prison)
//   - AttackId::A4b043DhelmiseEx (Anchor Shot)
//   - AttackId::A4b092KingdraEx (Stormy Prison)
//   - AttackId::B1077Drednaw (Jaw Lock)
//   - AttackId::B1154Darkrai (Shadow Cage)
//
// Effect47 - 2 attacks:
//   - AttackId::A1166NidoranF (Call for Family)
//   - AttackId::A4223NidoranF (Call for Family)
//
// Effect48 - 3 attacks:
//   - AttackId::A1168Nidoqueen (Lovestrike)
//   - AttackId::A1240Nidoqueen (Lovestrike)
//   - AttackId::A4225Nidoqueen (Lovestrike)
//
// Effect49 - 2 attacks:
//   - AttackId::A1175Muk (Venoshock)
//   - AttackId::A1a055Scolipede (Venoshock)
//
// Effect50 - 7 attacks:
//   - AttackId::A1178Mawile (Crunch)
//   - AttackId::A1192Fearow (Drill Run)
//   - AttackId::A1a023Drednaw (Crunch)
//   - AttackId::A2021MowRotom (Energy Cutoff)
//   - AttackId::A3100Lycanroc (Crunch)
//   - AttackId::A3a012Sharpedo (Crunch)
//   - AttackId::PA068Lycanroc (Crunch)
//
// Effect52 - 2 attacks:
//   - AttackId::A1185Dragonite (Draco Meteor)
//   - AttackId::A1244Dragonite (Draco Meteor)
//
// Effect53 - 15 attacks:
//   - AttackId::A1196Meowth (Pay Day)
//   - AttackId::A1246Meowth (Pay Day)
//   - AttackId::A1a033Sigilyph (Spike Draw)
//   - AttackId::A3121Klefki (Unlock)
//   - AttackId::A3b055Eevee (Collect)
//   - AttackId::A3b078Eevee (Collect)
//   - AttackId::A4057Delibird (Stumbling Draw)
//   - AttackId::A4170Delibird (Stumbling Draw)
//   - AttackId::A4b257Klefki (Unlock)
//   - AttackId::A4b258Klefki (Unlock)
//   - AttackId::A4b285Eevee (Collect)
//   - AttackId::A4b286Eevee (Collect)
//   - AttackId::B1201Lillipup (Collect)
//   - AttackId::B1312Meowth (Pay Day)
//   - AttackId::PA012Meowth (Pay Day)
//
// Effect54 - 2 attacks:
//   - AttackId::A1197Persian (Shadow Claw)
//   - AttackId::B1313Persian (Shadow Claw)
//
// Effect56 - 3 attacks:
//   - AttackId::A1203Kangaskhan (Dizzy Punch)
//   - AttackId::A3019Steenee (Double Spin)
//   - AttackId::A4105Binacle (Dual Chop)
//
// Effect57 - 2 attacks:
//   - AttackId::A1205Ditto (Copy Anything)
//   - AttackId::A1247Ditto (Copy Anything)
//
// Effect58 - 2 attacks:
//   - AttackId::A1210Aerodactyl (Primal Wingbeat)
//   - AttackId::A3a098Aerodactyl (Primal Wingbeat)
//
// Effect59 - 2 attacks:
//   - AttackId::A1213Cinccino (Do the Wave)
//   - AttackId::PA031Cinccino (Do the Wave)
//
// Effect60 - 3 attacks:
//   - AttackId::A1283Mew (Psy Report)
//   - AttackId::A1a031Mew (Psy Report)
//   - AttackId::A2a065Noctowl (Silent Wing)
//
// Effect61 - 2 attacks:
//   - AttackId::A1a001Exeggcute (Growth Spurt)
//   - AttackId::PA060Exeggcute (Growth Spurt)
//
// Effect62 - 5 attacks:
//   - AttackId::A1a003CelebiEx (Powerful Bloom)
//   - AttackId::A1a075CelebiEx (Powerful Bloom)
//   - AttackId::A1a085CelebiEx (Powerful Bloom)
//   - AttackId::A3a099CelebiEx (Powerful Bloom)
//   - AttackId::A4b024CelebiEx (Powerful Bloom)
//
// Effect64 - 2 attacks:
//   - AttackId::A1a011Rapidash (Rising Lunge)
//   - AttackId::A2a023OriginFormePalkia (Zone Smash)
//
// Effect65 - 2 attacks:
//   - AttackId::A1a014Volcarona (Volcanic Ash)
//   - AttackId::PA028Volcarona (Volcanic Ash)
//
// Effect66 - 2 attacks:
//   - AttackId::A1a015Salandit (Venoshock)
//   - AttackId::A1a071Salandit (Venoshock)
//
// Effect67 - 4 attacks:
//   - AttackId::A1a018GyaradosEx (Rampaging Whirlpool)
//   - AttackId::A1a076GyaradosEx (Rampaging Whirlpool)
//   - AttackId::A4234GyaradosEx (Rampaging Whirlpool)
//   - AttackId::A4b098GyaradosEx (Rampaging Whirlpool)
//
// Effect70 - 3 attacks:
//   - AttackId::A1a026Raichu (Gigashock)
//   - AttackId::A3041AlolanNinetales (Blizzard)
//   - AttackId::PA070AlolanNinetales (Blizzard)
//
// Effect71 - 4 attacks:
//   - AttackId::A1a027Electabuzz (Thunder Spear)
//   - AttackId::A3105Minior (Star Drop)
//   - AttackId::A3174Minior (Star Drop)
//   - AttackId::A4a024Cryogonal (Ice Blade)
//
// Effect72 - 7 attacks:
//   - AttackId::A1a032MewEx (Genome Hacking)
//   - AttackId::A1a077MewEx (Genome Hacking)
//   - AttackId::A1a083MewEx (Genome Hacking)
//   - AttackId::A1a086MewEx (Genome Hacking)
//   - AttackId::A4a102MewEx (Genome Hacking)
//   - AttackId::A4b159MewEx (Genome Hacking)
//   - AttackId::A4b366MewEx (Genome Hacking)
//
// Effect73 - 5 attacks:
//   - AttackId::A1a035Beheeyem (Mind Jack)
//   - AttackId::A1a059PidgeotEx (Scattering Cyclone)
//   - AttackId::A1a079PidgeotEx (Scattering Cyclone)
//   - AttackId::A3a102PidgeotEx (Scattering Cyclone)
//   - AttackId::A4b276PidgeotEx (Scattering Cyclone)
//
// Effect74 - 2 attacks:
//   - AttackId::A1a038Florges (Bloomshine)
//   - AttackId::A4010Meganium (Bloomshine)
//
// Effect75 - 4 attacks:
//   - AttackId::A1a045Golem (Guard Press)
//   - AttackId::A4104Pupitar (Guard Press)
//   - AttackId::B1119Carbink (Guard Press)
//   - AttackId::B1309Golem (Guard Press)
//
// Effect76 - 7 attacks:
//   - AttackId::A1a047Marshadow (Revenge)
//   - AttackId::A1a074Marshadow (Revenge)
//   - AttackId::A3077Oricorio (Spiteful Dance)
//   - AttackId::A4b178Oricorio (Spiteful Dance)
//   - AttackId::A4b179Oricorio (Spiteful Dance)
//   - AttackId::A4b224Marshadow (Revenge)
//   - AttackId::A4b225Marshadow (Revenge)
//
// Effect78 - 4 attacks:
//   - AttackId::A1a050Weezing (Smokescreen)
//   - AttackId::A2a028Manectric (Flash)
//   - AttackId::A2b012Magmortar (Smoke Bomb)
//   - AttackId::B1143Sandygast (Sand Attack)
//
// Effect82 - 6 attacks:
//   - AttackId::A2007YanmegaEx (Air Slash)
//   - AttackId::A2180YanmegaEx (Air Slash)
//   - AttackId::A2196YanmegaEx (Air Slash)
//   - AttackId::A2a070Arceus (Power Blast)
//   - AttackId::A4232YanmegaEx (Air Slash)
//   - AttackId::A4b022YanmegaEx (Air Slash)
//
// Effect84 - 17 attacks:
//   - AttackId::A2014Kricketune (Entrancing Melody)
//   - AttackId::A2067MismagiusEx (Magical Delusion)
//   - AttackId::A2184MismagiusEx (Magical Delusion)
//   - AttackId::A2199MismagiusEx (Magical Delusion)
//   - AttackId::A2a007Cherrim (Worry Seed)
//   - AttackId::A3076Oricorio (Dazzle Dance)
//   - AttackId::A3a020Xurkitree (Electronica)
//   - AttackId::A3b060Chatot (Tone-Deaf)
//   - AttackId::A4087Girafarig (Psybeam)
//   - AttackId::A4237MismagiusEx (Magical Delusion)
//   - AttackId::A4a019Jynx (Dazzle Dance)
//   - AttackId::A4b163MismagiusEx (Magical Delusion)
//   - AttackId::B1019Maractus (Dazzle Dance)
//   - AttackId::B1065Seismitoad (Split Spiral Punch)
//   - AttackId::B1116Aromatisse (Confounding Cologne)
//   - AttackId::PA038Misdreavus (Confuse Ray)
//   - AttackId::PA077Xurkitree (Electronica)
//
// Effect85 - 2 attacks:
//   - AttackId::A2017Combee (Call for Family)
//   - AttackId::A2157Combee (Call for Family)
//
// Effect86 - 5 attacks:
//   - AttackId::A2019Carnivine (Flog)
//   - AttackId::A2158Carnivine (Flog)
//   - AttackId::A3128Tauros (Rising Lunge)
//   - AttackId::A4a047Crustle (Stone Edge)
//   - AttackId::B1001Pinsir (X-Scissor)
//
// Effect87 - 9 attacks:
//   - AttackId::A2020Leafeon (Leafy Cyclone)
//   - AttackId::A3088Necrozma (Prismatic Laser)
//   - AttackId::A3a050Aggron (Giga Impact)
//   - AttackId::A3b053DragoniteEx (Giga Impact)
//   - AttackId::A3b082DragoniteEx (Giga Impact)
//   - AttackId::A3b090DragoniteEx (Giga Impact)
//   - AttackId::A4026Ninetales (Scorching Breath)
//   - AttackId::A4b271DragoniteEx (Giga Impact)
//   - AttackId::B1212Dubwool (Boundless Power)
//
// Effect89 - 8 attacks:
//   - AttackId::A2024Magmortar (Bursting Inferno)
//   - AttackId::A2026Magcargo (Searing Flame)
//   - AttackId::A3033IncineroarEx (Fire Fang)
//   - AttackId::A3182IncineroarEx (Fire Fang)
//   - AttackId::A3200IncineroarEx (Fire Fang)
//   - AttackId::A4a012Talonflame (Searing Flame)
//   - AttackId::A4b082IncineroarEx (Fire Fang)
//   - AttackId::B1318IncineroarEx (Fire Fang)
//
// Effect90 - 5 attacks:
//   - AttackId::A2029InfernapeEx (Flare Blitz)
//   - AttackId::A2181InfernapeEx (Flare Blitz)
//   - AttackId::A2197InfernapeEx (Flare Blitz)
//   - AttackId::A4a101InfernapeEx (Flare Blitz)
//   - AttackId::A4b075InfernapeEx (Flare Blitz)
//
// Effect91 - 17 attacks:
//   - AttackId::A2035Piplup (Nap)
//   - AttackId::A2079Cresselia (Moonlight Gain)
//   - AttackId::A2168Cresselia (Moonlight Gain)
//   - AttackId::A2a008Cherrim (Leech Seed)
//   - AttackId::A3056TapuFini (Spiral Drain)
//   - AttackId::A3164TapuFini (Spiral Drain)
//   - AttackId::A3b024PrimarinaEx (Sparkling Aria)
//   - AttackId::A3b080PrimarinaEx (Sparkling Aria)
//   - AttackId::A3b088PrimarinaEx (Sparkling Aria)
//   - AttackId::A4145Teddiursa (Honey Snack)
//   - AttackId::A4b120PrimarinaEx (Sparkling Aria)
//   - AttackId::B1024Trevenant (Horn Leech)
//   - AttackId::B1144Palossand (Life Sucker)
//   - AttackId::B1320PrimarinaEx (Sparkling Aria)
//   - AttackId::PA014LaprasEx (Bubble Drain)
//   - AttackId::PA034Piplup (Nap)
//   - AttackId::PA043Cherrim (Leech Seed)
//
// Effect92 - 5 attacks:
//   - AttackId::A2037Empoleon (Aqua Jet)
//   - AttackId::A2b043LucarioEx (Aura Sphere)
//   - AttackId::A2b084LucarioEx (Aura Sphere)
//   - AttackId::A2b110LucarioEx (Aura Sphere)
//   - AttackId::A4b214LucarioEx (Aura Sphere)
//
// Effect93 - 14 attacks:
//   - AttackId::A2041Gastrodon (Muddy Water)
//   - AttackId::A2138Lopunny (Jump Kick)
//   - AttackId::A2161Gastrodon (Muddy Water)
//   - AttackId::A2a026Raichu (Spark)
//   - AttackId::A3120Escavalier (Pike)
//   - AttackId::A3a007Pheromosa (Jump Blues)
//   - AttackId::A3a071Pheromosa (Jump Blues)
//   - AttackId::A4102Hitmontop (Piercing Spin)
//   - AttackId::A4b045Pheromosa (Jump Blues)
//   - AttackId::A4b046Pheromosa (Jump Blues)
//   - AttackId::B1031RapidashEx (Sprinting Flare)
//   - AttackId::B1253RapidashEx (Sprinting Flare)
//   - AttackId::B1274RapidashEx (Sprinting Flare)
//   - AttackId::PA044Raichu (Spark)
//
// Effect95 - 7 attacks:
//   - AttackId::A2049PalkiaEx (Dimensional Storm)
//   - AttackId::A2182PalkiaEx (Dimensional Storm)
//   - AttackId::A2204PalkiaEx (Dimensional Storm)
//   - AttackId::A2206PalkiaEx (Dimensional Storm)
//   - AttackId::A4b107PalkiaEx (Dimensional Storm)
//   - AttackId::A4b363PalkiaEx (Dimensional Storm)
//   - AttackId::B1319PalkiaEx (Dimensional Storm)
//
// Effect96 - 6 attacks:
//   - AttackId::A2050Manaphy (Oceanic Gift)
//   - AttackId::A2162Manaphy (Oceanic Gift)
//   - AttackId::A4b108Manaphy (Oceanic Gift)
//   - AttackId::A4b109Manaphy (Oceanic Gift)
//   - AttackId::B1299Manaphy (Oceanic Gift)
//   - AttackId::PA048Manaphy (Oceanic Gift)
//
// Effect97 - 3 attacks:
//   - AttackId::A2053Magnezone (Thunder Blast)
//   - AttackId::A4b137Magnezone (Thunder Blast)
//   - AttackId::A4b138Magnezone (Thunder Blast)
//
// Effect98 - 23 attacks:
//   - AttackId::A2054Voltorb (Big Explosion)
//   - AttackId::A2b013PaldeanTauros (Blaze Tackle)
//   - AttackId::A3030Litten (Heat Tackle)
//   - AttackId::A3032Torracat (Heat Tackle)
//   - AttackId::A3122SolgaleoEx (Sol Breaker)
//   - AttackId::A3189SolgaleoEx (Sol Breaker)
//   - AttackId::A3207SolgaleoEx (Sol Breaker)
//   - AttackId::A3239SolgaleoEx (Sol Breaker)
//   - AttackId::A3b095Voltorb (Big Explosion)
//   - AttackId::A4099Phanpy (Reckless Charge)
//   - AttackId::A4a045Relicanth (Take Down)
//   - AttackId::A4b078Litten (Heat Tackle)
//   - AttackId::A4b079Litten (Heat Tackle)
//   - AttackId::A4b080Torracat (Heat Tackle)
//   - AttackId::A4b081Torracat (Heat Tackle)
//   - AttackId::A4b200Phanpy (Reckless Charge)
//   - AttackId::A4b201Phanpy (Reckless Charge)
//   - AttackId::A4b259SolgaleoEx (Sol Breaker)
//   - AttackId::A4b369SolgaleoEx (Sol Breaker)
//   - AttackId::B1039Darumaka (Reckless Charge)
//   - AttackId::B1293Litten (Heat Tackle)
//   - AttackId::B1294Torracat (Heat Tackle)
//   - AttackId::PA017Mankey (Reckless Charge)
//
// Effect99 - 7 attacks:
//   - AttackId::A2056Electabuzz (Charge)
//   - AttackId::A3a019TapuKokoEx (Plasma Hurricane)
//   - AttackId::A3a077TapuKokoEx (Plasma Hurricane)
//   - AttackId::A3a084TapuKokoEx (Plasma Hurricane)
//   - AttackId::A4b148TapuKokoEx (Plasma Hurricane)
//   - AttackId::B1322TapuKokoEx (Plasma Hurricane)
//   - AttackId::PA084TapuKokoEx (Plasma Hurricane)
//
// Effect100 - 2 attacks:
//   - AttackId::A2057Electivire (Exciting Voltage)
//   - AttackId::PA036Electivire (Exciting Voltage)
//
// Effect102 - 5 attacks:
//   - AttackId::A2061PachirisuEx (Sparking Gadget)
//   - AttackId::A2183PachirisuEx (Sparking Gadget)
//   - AttackId::A2198PachirisuEx (Sparking Gadget)
//   - AttackId::A4236PachirisuEx (Sparking Gadget)
//   - AttackId::A4b145PachirisuEx (Sparking Gadget)
//
// Effect103 - 2 attacks:
//   - AttackId::A2062Rotom (Assault Laser)
//   - AttackId::A2164Rotom (Assault Laser)
//
// Effect106 - 2 attacks:
//   - AttackId::A2076Mesprit (Supreme Blast)
//   - AttackId::A2166Mesprit (Supreme Blast)
//
// Effect107 - 3 attacks:
//   - AttackId::A2077Azelf (Psychic Arrow)
//   - AttackId::A3a004Dartrix (Skill Dive)
//   - AttackId::B1291Dartrix (Skill Dive)
//
// Effect108 - 2 attacks:
//   - AttackId::A2082Rhyperior (Mountain Swing)
//   - AttackId::A2169Rhyperior (Mountain Swing)
//
// Effect109 - 2 attacks:
//   - AttackId::A2084Gliscor (Acrobatics)
//   - AttackId::A4146Ursaring (Swing Around)
//
// Effect111 - 7 attacks:
//   - AttackId::A2098Sneasel (Double Scratch)
//   - AttackId::A3a044Poipole (2-Step)
//   - AttackId::A3b058Aipom (Double Hit)
//   - AttackId::A4b242Sneasel (Double Scratch)
//   - AttackId::A4b243Sneasel (Double Scratch)
//   - AttackId::B1099Misdreavus (Double Spin)
//   - AttackId::PA082Poipole (2-Step)
//
// Effect112 - 5 attacks:
//   - AttackId::A2099WeavileEx (Scratching Nails)
//   - AttackId::A2186WeavileEx (Scratching Nails)
//   - AttackId::A2201WeavileEx (Scratching Nails)
//   - AttackId::A4238WeavileEx (Scratching Nails)
//   - AttackId::A4b244WeavileEx (Scratching Nails)
//
// Effect113 - 2 attacks:
//   - AttackId::A2104Spiritomb (Swirling Disaster)
//   - AttackId::A2172Spiritomb (Swirling Disaster)
//
// Effect115 - 2 attacks:
//   - AttackId::A2107Croagunk (Group Beatdown)
//   - AttackId::A2173Croagunk (Group Beatdown)
//
// Effect117 - 2 attacks:
//   - AttackId::A2111Skarmory (Metal Arms)
//   - AttackId::PA039Skarmory (Metal Arms)
//
// Effect118 - 2 attacks:
//   - AttackId::A2115Wormadam (Iron Head)
//   - AttackId::PA063Rayquaza (Spiral Rush)
//
// Effect120 - 7 attacks:
//   - AttackId::A2119DialgaEx (Metallic Turbo)
//   - AttackId::A2188DialgaEx (Metallic Turbo)
//   - AttackId::A2205DialgaEx (Metallic Turbo)
//   - AttackId::A2207DialgaEx (Metallic Turbo)
//   - AttackId::A4b254DialgaEx (Metallic Turbo)
//   - AttackId::A4b368DialgaEx (Metallic Turbo)
//   - AttackId::B1326DialgaEx (Metallic Turbo)
//
// Effect121 - 7 attacks:
//   - AttackId::A2125LickilickyEx (Licking Fury)
//   - AttackId::A2189LickilickyEx (Licking Fury)
//   - AttackId::A2203LickilickyEx (Licking Fury)
//   - AttackId::A2b004Pinsir (Guillotine Rush)
//   - AttackId::A3b093Pinsir (Guillotine Rush)
//   - AttackId::A4239LickilickyEx (Licking Fury)
//   - AttackId::A4b284LickilickyEx (Licking Fury)
//
// Effect122 - 6 attacks:
//   - AttackId::A2126Eevee (Quick Attack)
//   - AttackId::A3003Surskit (Quick Attack)
//   - AttackId::A3a028Meditite (Trip Over)
//   - AttackId::A3a060TypeNull (Quick Blow)
//   - AttackId::A4b300TypeNull (Quick Blow)
//   - AttackId::A4b301TypeNull (Quick Blow)
//
// Effect124 - 2 attacks:
//   - AttackId::A2131Ambipom (Double Hit)
//   - AttackId::PA102Tropius (Double Smash)
//
// Effect125 - 2 attacks:
//   - AttackId::A2132Starly (Pluck)
//   - AttackId::B1127Hariyama (Slapping Knockdown)
//
// Effect126 - 3 attacks:
//   - AttackId::A2135Bidoof (Super Fang)
//   - AttackId::A2177Bidoof (Super Fang)
//   - AttackId::B1316Bidoof (Super Fang)
//
// Effect128 - 5 attacks:
//   - AttackId::A2141Chatot (Fury Attack)
//   - AttackId::A4021ShuckleEx (Triple Slap)
//   - AttackId::A4186ShuckleEx (Triple Slap)
//   - AttackId::A4202ShuckleEx (Triple Slap)
//   - AttackId::A4b023ShuckleEx (Triple Slap)
//
// Effect130 - 3 attacks:
//   - AttackId::A2143Regigigas (Raging Hammer)
//   - AttackId::A2179Regigigas (Raging Hammer)
//   - AttackId::PA115Regigigas (Raging Hammer)
//
// Effect132 - 3 attacks:
//   - AttackId::A2a005Vespiquen (Reckless Charge)
//   - AttackId::PA078DawnWingsNecrozma (Psychobilly)
//   - AttackId::PA112RaichuEx (Volt Tackle)
//
// Effect133 - 3 attacks:
//   - AttackId::A2a013Heatran (Ragin' Mad Strike)
//   - AttackId::A4b076Heatran (Ragin' Mad Strike)
//   - AttackId::A4b077Heatran (Ragin' Mad Strike)
//
// Effect135 - 4 attacks:
//   - AttackId::A2a025Pikachu (Spark)
//   - AttackId::A4a025RaikouEx (Voltaic Bullet)
//   - AttackId::A4a081RaikouEx (Voltaic Bullet)
//   - AttackId::A4a088RaikouEx (Voltaic Bullet)
//
// Effect137 - 4 attacks:
//   - AttackId::A2a031Gastly (Astonish)
//   - AttackId::A3220Gastly (Astonish)
//   - AttackId::A4113Murkrow (Astonish)
//   - AttackId::A4178Murkrow (Astonish)
//
// Effect138 - 3 attacks:
//   - AttackId::A2a035Rotom (Dash Attack)
//   - AttackId::A4070Elekid (Zappy Shot)
//   - AttackId::B1107Drifloon (Wind Blast)
//
// Effect139 - 4 attacks:
//   - AttackId::A2a036Sudowoodo (Fighting Headbutt)
//   - AttackId::A2a079Sudowoodo (Fighting Headbutt)
//   - AttackId::A4b198Sudowoodo (Fighting Headbutt)
//   - AttackId::A4b199Sudowoodo (Fighting Headbutt)
//
// Effect143 - 3 attacks:
//   - AttackId::A2a060OriginFormeDialga (Time Mash)
//   - AttackId::B1129Hippowdon (Crashing Fangs)
//   - AttackId::B1241Hippowdon (Crashing Fangs)
//
// Effect145 - 6 attacks:
//   - AttackId::A2a063Snorlax (Collapse)
//   - AttackId::A3b057SnorlaxEx (Flop-Down Punch)
//   - AttackId::A3b084SnorlaxEx (Flop-Down Punch)
//   - AttackId::A3b091SnorlaxEx (Flop-Down Punch)
//   - AttackId::A4b288SnorlaxEx (Flop-Down Punch)
//   - AttackId::PA049Snorlax (Collapse)
//
// Effect146 - 8 attacks:
//   - AttackId::A2a071ArceusEx (Ultimate Force)
//   - AttackId::A2a086ArceusEx (Ultimate Force)
//   - AttackId::A2a095ArceusEx (Ultimate Force)
//   - AttackId::A2a096ArceusEx (Ultimate Force)
//   - AttackId::A4b299ArceusEx (Ultimate Force)
//   - AttackId::A4b372ArceusEx (Ultimate Force)
//   - AttackId::B1197Altaria (Do the Wave)
//   - AttackId::B1328ArceusEx (Ultimate Force)
//
// Effect147 - 4 attacks:
//   - AttackId::A2b001Weedle (Multiply)
//   - AttackId::A2b097Weedle (Multiply)
//   - AttackId::A4b006Weedle (Multiply)
//   - AttackId::A4b007Weedle (Multiply)
//
// Effect148 - 4 attacks:
//   - AttackId::A2b007Meowscarada (Fighting Claws)
//   - AttackId::A2b073Meowscarada (Fighting Claws)
//   - AttackId::A4b053Meowscarada (Fighting Claws)
//   - AttackId::A4b054Meowscarada (Fighting Claws)
//
// Effect149 - 4 attacks:
//   - AttackId::A2b010CharizardEx (Stoke)
//   - AttackId::A2b080CharizardEx (Stoke)
//   - AttackId::A2b108CharizardEx (Stoke)
//   - AttackId::A4b060CharizardEx (Stoke)
//
// Effect150 - 4 attacks:
//   - AttackId::A2b018Wiglett (Spring Out)
//   - AttackId::A2b101Wiglett (Spring Out)
//   - AttackId::A4b125Wiglett (Spring Out)
//   - AttackId::A4b126Wiglett (Spring Out)
//
// Effect151 - 4 attacks:
//   - AttackId::A2b019WugtrioEx (Pop Out Throughout)
//   - AttackId::A2b081WugtrioEx (Pop Out Throughout)
//   - AttackId::A2b109WugtrioEx (Pop Out Throughout)
//   - AttackId::A4b127WugtrioEx (Pop Out Throughout)
//
// Effect152 - 5 attacks:
//   - AttackId::A2b025Pachirisu (Plasma)
//   - AttackId::A2b103Pachirisu (Plasma)
//   - AttackId::A4b143Pachirisu (Plasma)
//   - AttackId::A4b144Pachirisu (Plasma)
//   - AttackId::PA058Pachirisu (Plasma)
//
// Effect154 - 2 attacks:
//   - AttackId::A2b032MrMime (Juggling)
//   - AttackId::A3116ToxapEx (Spike Cannon)
//
// Effect155 - 3 attacks:
//   - AttackId::A2b039Machamp (Power Press)
//   - AttackId::A3225Machamp (Power Press)
//   - AttackId::PA055Machamp (Power Press)
//
// Effect156 - 5 attacks:
//   - AttackId::A2b048PaldeanClodsireEx (Venoshock)
//   - AttackId::A2b085PaldeanClodsireEx (Venoshock)
//   - AttackId::A2b093PaldeanClodsireEx (Venoshock)
//   - AttackId::A4a104PaldeanClodsireEx (Venoshock)
//   - AttackId::A4b238PaldeanClodsireEx (Venoshock)
//
// Effect158 - 4 attacks:
//   - AttackId::A2b054TinkatonEx (Terrific Thumping)
//   - AttackId::A2b086TinkatonEx (Terrific Thumping)
//   - AttackId::A2b094TinkatonEx (Terrific Thumping)
//   - AttackId::A4b266TinkatonEx (Terrific Thumping)
//
// Effect159 - 2 attacks:
//   - AttackId::A2b057Gholdengo (Scintillating Surfing)
//   - AttackId::A2b077Gholdengo (Scintillating Surfing)
//
// Effect160 - 4 attacks:
//   - AttackId::A2b068Cyclizar (Overacceleration)
//   - AttackId::A4b306Cyclizar (Overacceleration)
//   - AttackId::A4b307Cyclizar (Overacceleration)
//   - AttackId::PA051Cyclizar (Overacceleration)
//
// Effect161 - 5 attacks:
//   - AttackId::A3010Rowlet (Skill Dive)
//   - AttackId::A4b038Rowlet (Skill Dive)
//   - AttackId::A4b039Rowlet (Skill Dive)
//   - AttackId::B1290Rowlet (Skill Dive)
//   - AttackId::PA094Horsea (Water Arrow)
//
// Effect162 - 5 attacks:
//   - AttackId::A3012DecidueyeEx (Pierce the Pain)
//   - AttackId::A3180DecidueyeEx (Pierce the Pain)
//   - AttackId::A3198DecidueyeEx (Pierce the Pain)
//   - AttackId::A4b042DecidueyeEx (Pierce the Pain)
//   - AttackId::B1317DecidueyeEx (Pierce the Pain)
//
// Effect163 - 2 attacks:
//   - AttackId::A3015Lurantis (Petal Blizzard)
//   - AttackId::A4a052Malamar (Mental Surge)
//
// Effect164 - 2 attacks:
//   - AttackId::A3020Tsareena (Three Kick Combo)
//   - AttackId::A3158Tsareena (Three Kick Combo)
//
// Effect166 - 2 attacks:
//   - AttackId::A3024TapuBulu (Stuck-In Tackle)
//   - AttackId::A3159TapuBulu (Stuck-In Tackle)
//
// Effect167 - 2 attacks:
//   - AttackId::A3027AlolanMarowak (Burning Bonemerang)
//   - AttackId::A3160AlolanMarowak (Burning Bonemerang)
//
// Effect171 - 2 attacks:
//   - AttackId::A3040AlolanVulpix (Call Forth Cold)
//   - AttackId::A3162AlolanVulpix (Call Forth Cold)
//
// Effect172 - 4 attacks:
//   - AttackId::A3049CrabominableEx (Insatiable Striking)
//   - AttackId::A3183CrabominableEx (Insatiable Striking)
//   - AttackId::A3201CrabominableEx (Insatiable Striking)
//   - AttackId::A4b121CrabominableEx (Insatiable Striking)
//
// Effect173 - 3 attacks:
//   - AttackId::A3050Wishiwashi (Call for Family)
//   - AttackId::A4b122Wishiwashi (Call for Family)
//   - AttackId::A4b123Wishiwashi (Call for Family)
//
// Effect174 - 4 attacks:
//   - AttackId::A3051WishiwashiEx (School Storm)
//   - AttackId::A3184WishiwashiEx (School Storm)
//   - AttackId::A3202WishiwashiEx (School Storm)
//   - AttackId::A4b124WishiwashiEx (School Storm)
//
// Effect177 - 3 attacks:
//   - AttackId::A3065Vikavolt (Disconnect)
//   - AttackId::B1109Chingling (Jingly Noise)
//   - AttackId::B1192Exploud (Booming Roar)
//
// Effect178 - 2 attacks:
//   - AttackId::A3068TapuKoko (Volt Switch)
//   - AttackId::A3166TapuKoko (Volt Switch)
//
// Effect181 - 2 attacks:
//   - AttackId::A3083Mimikyu (Shadow Hit)
//   - AttackId::PA066Mimikyu (Shadow Hit)
//
// Effect182 - 2 attacks:
//   - AttackId::A3084TapuLele (Energy Arrow)
//   - AttackId::A3170TapuLele (Energy Arrow)
//
// Effect183 - 3 attacks:
//   - AttackId::A3086Cosmoem (Stiffen)
//   - AttackId::A4b182Cosmoem (Stiffen)
//   - AttackId::A4b183Cosmoem (Stiffen)
//
// Effect184 - 3 attacks:
//   - AttackId::A3098Rockruff (Signs of Evolution)
//   - AttackId::A3172Rockruff (Signs of Evolution)
//   - AttackId::B1310Rockruff (Signs of Evolution)
//
// Effect186 - 2 attacks:
//   - AttackId::A3103Mudsdale (High Horsepower)
//   - AttackId::A3173Mudsdale (High Horsepower)
//
// Effect188 - 4 attacks:
//   - AttackId::A3111AlolanMukEx (Chemical Panic)
//   - AttackId::A3188AlolanMukEx (Chemical Panic)
//   - AttackId::A3206AlolanMukEx (Chemical Panic)
//   - AttackId::A4b235AlolanMukEx (Chemical Panic)
//
// Effect190 - 2 attacks:
//   - AttackId::A3118AlolanDugtrio (Iron Head)
//   - AttackId::A4150Bouffalant (Continuous Headbutt)
//
// Effect191 - 4 attacks:
//   - AttackId::A3123Magearna (Silver Cannon)
//   - AttackId::A3175Magearna (Silver Cannon)
//   - AttackId::A4b260Magearna (Silver Cannon)
//   - AttackId::A4b261Magearna (Silver Cannon)
//
// Effect192 - 2 attacks:
//   - AttackId::A3124Drampa (Berserk)
//   - AttackId::A3176Drampa (Berserk)
//
// Effect194 - 3 attacks:
//   - AttackId::A3130Delcatty (Energy Assist)
//   - AttackId::A4b292Delcatty (Energy Assist)
//   - AttackId::A4b293Delcatty (Energy Assist)
//
// Effect196 - 2 attacks:
//   - AttackId::A3139Bewear (Tantrum)
//   - AttackId::A3178Bewear (Tantrum)
//
// Effect198 - 2 attacks:
//   - AttackId::A3a003Rowlet (Fury Attack)
//   - AttackId::A3a070Rowlet (Fury Attack)
//
// Effect200 - 5 attacks:
//   - AttackId::A3a006BuzzwoleEx (Big Beat)
//   - AttackId::A3a076BuzzwoleEx (Big Beat)
//   - AttackId::A3a088BuzzwoleEx (Big Beat)
//   - AttackId::A4b044BuzzwoleEx (Big Beat)
//   - AttackId::A4b360BuzzwoleEx (Big Beat)
//
// Effect201 - 3 attacks:
//   - AttackId::A3a009Blacephalon (Beat Punk)
//   - AttackId::A3a072Blacephalon (Beat Punk)
//   - AttackId::PA076Blacephalon (Beat Punk)
//
// Effect202 - 5 attacks:
//   - AttackId::A3a033LycanrocEx (Lycanfang)
//   - AttackId::A3a078LycanrocEx (Lycanfang)
//   - AttackId::A3a085LycanrocEx (Lycanfang)
//   - AttackId::A4b222LycanrocEx (Lycanfang)
//   - AttackId::B1323LycanrocEx (Lycanfang)
//
// Effect204 - 2 attacks:
//   - AttackId::A3a037AlolanMeowth (Meddle)
//   - AttackId::A3a073AlolanMeowth (Meddle)
//
// Effect206 - 7 attacks:
//   - AttackId::A3a043GuzzlordEx (Grindcore)
//   - AttackId::A3a079GuzzlordEx (Grindcore)
//   - AttackId::A3a086GuzzlordEx (Grindcore)
//   - AttackId::A3b016Vaporeon (Hyper Whirlpool)
//   - AttackId::A3b072Vaporeon (Hyper Whirlpool)
//   - AttackId::A4b248GuzzlordEx (Grindcore)
//   - AttackId::B1132Krookodile (Chomp Chomp Bite)
//
// Effect207 - 5 attacks:
//   - AttackId::A3a047AlolanDugtrioEx (Triplet Headbutt)
//   - AttackId::A3a080AlolanDugtrioEx (Triplet Headbutt)
//   - AttackId::A3a087AlolanDugtrioEx (Triplet Headbutt)
//   - AttackId::A4b251AlolanDugtrioEx (Triplet Headbutt)
//   - AttackId::B1325AlolanDugtrioEx (Triplet Headbutt)
//
// Effect209 - 4 attacks:
//   - AttackId::A3a061Silvally (Brave Buddies)
//   - AttackId::A3a074Silvally (Brave Buddies)
//   - AttackId::A4b302Silvally (Brave Buddies)
//   - AttackId::A4b303Silvally (Brave Buddies)
//
// Effect210 - 2 attacks:
//   - AttackId::A3b002Leafeon (Leaf Blast)
//   - AttackId::A3b070Leafeon (Leaf Blast)
//
// Effect213 - 5 attacks:
//   - AttackId::A3b008Flareon (Assisting Heater)
//   - AttackId::A3b071Flareon (Assisting Heater)
//   - AttackId::A4213Flareon (Assisting Heater)
//   - AttackId::A4b064Flareon (Assisting Heater)
//   - AttackId::A4b065Flareon (Assisting Heater)
//
// Effect214 - 12 attacks:
//   - AttackId::A3b018Vanillite (Sweets Relay)
//   - AttackId::A3b019Vanillish (Sweets Relay)
//   - AttackId::A3b031Swirlix (Sweets Relay)
//   - AttackId::A3b036Milcery (Sweets Relay)
//   - AttackId::A4023Cherubi (Sweets Relay)
//   - AttackId::A4b025Cherubi (Sweets Relay)
//   - AttackId::A4b026Cherubi (Sweets Relay)
//   - AttackId::A4b173Swirlix (Sweets Relay)
//   - AttackId::A4b174Swirlix (Sweets Relay)
//   - AttackId::A4b185Milcery (Sweets Relay)
//   - AttackId::A4b186Milcery (Sweets Relay)
//   - AttackId::PA085Vanillite (Sweets Relay)
//
// Effect215 - 5 attacks:
//   - AttackId::A3b024PrimarinaEx (Hydro Pump)
//   - AttackId::A3b080PrimarinaEx (Hydro Pump)
//   - AttackId::A3b088PrimarinaEx (Hydro Pump)
//   - AttackId::A4b120PrimarinaEx (Hydro Pump)
//   - AttackId::B1320PrimarinaEx (Hydro Pump)
//
// Effect216 - 4 attacks:
//   - AttackId::A3b025Jolteon (Beginning Bolt)
//   - AttackId::A3b074Jolteon (Beginning Bolt)
//   - AttackId::A4219Jolteon (Beginning Bolt)
//   - AttackId::PA086Jolteon (Beginning Bolt)
//
// Effect218 - 2 attacks:
//   - AttackId::A3b028Espeon (Energy Crush)
//   - AttackId::A3b075Espeon (Energy Crush)
//
// Effect219 - 4 attacks:
//   - AttackId::A3b032Slurpuff (Sweets Relay)
//   - AttackId::A4b175Slurpuff (Sweets Relay)
//   - AttackId::A4b176Slurpuff (Sweets Relay)
//   - AttackId::A4b358Slurpuff (Sweets Relay)
//
// Effect220 - 2 attacks:
//   - AttackId::A3b033Sylveon (Evoharmony)
//   - AttackId::A3b076Sylveon (Evoharmony)
//
// Effect221 - 2 attacks:
//   - AttackId::A3b035Mimikyu (Try to Imitate)
//   - AttackId::PA113Mimikyu (Try to Imitate)
//
// Effect222 - 4 attacks:
//   - AttackId::A3b037Alcremie (Sweets Overload)
//   - AttackId::A4b187Alcremie (Sweets Overload)
//   - AttackId::A4b188Alcremie (Sweets Overload)
//   - AttackId::PA087Alcremie (Sweets Overload)
//
// Effect224 - 4 attacks:
//   - AttackId::A3b043Umbreon (Dark Binding)
//   - AttackId::A3b077Umbreon (Dark Binding)
//   - AttackId::A4b239Umbreon (Dark Binding)
//   - AttackId::A4b240Umbreon (Dark Binding)
//
// Effect226 - 2 attacks:
//   - AttackId::A3b048Togedemaru (Bristling Spikes)
//   - AttackId::PA090Togedemaru (Bristling Spikes)
//
// Effect229 - 7 attacks:
//   - AttackId::A3b065Greedent (Enhanced Fang)
//   - AttackId::A4147Stantler (Enhanced Horns)
//   - AttackId::A4183Stantler (Enhanced Horns)
//   - AttackId::B1174MelmetalEx (Metal Arms)
//   - AttackId::B1264MelmetalEx (Metal Arms)
//   - AttackId::B1282MelmetalEx (Metal Arms)
//   - AttackId::PA091Greedent (Enhanced Fang)
//
// Effect230 - 2 attacks:
//   - AttackId::A4003Bellossom (Petal Dance)
//   - AttackId::A4163Bellossom (Petal Dance)
//
// Effect232 - 2 attacks:
//   - AttackId::A4022Heracross (Single Lunge)
//   - AttackId::A4164Heracross (Single Lunge)
//
// Effect234 - 2 attacks:
//   - AttackId::A4032Magby (Toasty Toss)
//   - AttackId::A4166Magby (Toasty Toss)
//
// Effect236 - 6 attacks:
//   - AttackId::A4034HoOhEx (Phoenix Turbo)
//   - AttackId::A4187HoOhEx (Phoenix Turbo)
//   - AttackId::A4210HoOhEx (Phoenix Turbo)
//   - AttackId::A4240HoOhEx (Phoenix Turbo)
//   - AttackId::A4b068HoOhEx (Phoenix Turbo)
//   - AttackId::A4b362HoOhEx (Phoenix Turbo)
//
// Effect238 - 2 attacks:
//   - AttackId::A4045Gyarados (Wild Swing)
//   - AttackId::A4215Gyarados (Wild Swing)
//
// Effect239 - 5 attacks:
//   - AttackId::A4054Corsola (Find a Friend)
//   - AttackId::A4077Cleffa (Twinkly Call)
//   - AttackId::A4134Eevee (Find a Friend)
//   - AttackId::A4231Eevee (Find a Friend)
//   - AttackId::PA093Cleffa (Twinkly Call)
//
// Effect240 - 2 attacks:
//   - AttackId::A4056Octillery (Octazooka)
//   - AttackId::A4169Octillery (Octazooka)
//
// Effect242 - 4 attacks:
//   - AttackId::A4065LanturnEx (Flashing Signal)
//   - AttackId::A4189LanturnEx (Flashing Signal)
//   - AttackId::A4204LanturnEx (Flashing Signal)
//   - AttackId::A4b142LanturnEx (Flashing Signal)
//
// Effect243 - 2 attacks:
//   - AttackId::A4066Pichu (Crackly Toss)
//   - AttackId::A4171Pichu (Crackly Toss)
//
// Effect247 - 2 attacks:
//   - AttackId::A4082Xatu (Life Drain)
//   - AttackId::A4174Xatu (Life Drain)
//
// Effect248 - 2 attacks:
//   - AttackId::A4086Wobbuffet (Reply Strongly)
//   - AttackId::A4175Wobbuffet (Reply Strongly)
//
// Effect251 - 4 attacks:
//   - AttackId::A4100DonphanEx (Gigantic Press)
//   - AttackId::A4191DonphanEx (Gigantic Press)
//   - AttackId::A4206DonphanEx (Gigantic Press)
//   - AttackId::A4b202DonphanEx (Gigantic Press)
//
// Effect252 - 3 attacks:
//   - AttackId::A4118Houndoom (Diving Swipe)
//   - AttackId::B1010Shiftry (Nipping Cyclone)
//   - AttackId::PA096Houndoom (Diving Swipe)
//
// Effect253 - 2 attacks:
//   - AttackId::A4123Scizor (Gale Thrust)
//   - AttackId::A4180Scizor (Gale Thrust)
//
// Effect256 - 2 attacks:
//   - AttackId::A4131Chansey (Scrunch)
//   - AttackId::PA045Nosepass (Iron Defense)
//
// Effect257 - 3 attacks:
//   - AttackId::A4132Blissey (Energy Blow)
//   - AttackId::A4185Blissey (Energy Blow)
//   - AttackId::B1108Drifblim (Balloon Barrage)
//
// Effect258 - 2 attacks:
//   - AttackId::A4133Kangaskhan (Cross-Cut)
//   - AttackId::PA097Kangaskhan (Cross-Cut)
//
// Effect262 - 2 attacks:
//   - AttackId::A4148Smeargle (Splatter Coating)
//   - AttackId::A4184Smeargle (Splatter Coating)
//
// Effect263 - 6 attacks:
//   - AttackId::A4149LugiaEx (Elemental Blast)
//   - AttackId::A4195LugiaEx (Elemental Blast)
//   - AttackId::A4211LugiaEx (Elemental Blast)
//   - AttackId::A4241LugiaEx (Elemental Blast)
//   - AttackId::A4b289LugiaEx (Elemental Blast)
//   - AttackId::A4b371LugiaEx (Elemental Blast)
//
// Effect264 - 3 attacks:
//   - AttackId::A4a003JumpluffEx (Breeze-By Attack)
//   - AttackId::A4a078JumpluffEx (Breeze-By Attack)
//   - AttackId::A4a086JumpluffEx (Breeze-By Attack)
//
// Effect266 - 2 attacks:
//   - AttackId::A4a007Durant (Mountain Munch)
//   - AttackId::B1051Gyarados (Hammering Tail)
//
// Effect267 - 4 attacks:
//   - AttackId::A4a010EnteiEx (Blazing Beatdown)
//   - AttackId::A4a079EnteiEx (Blazing Beatdown)
//   - AttackId::A4a087EnteiEx (Blazing Beatdown)
//   - AttackId::PA110EnteiEx (Blazing Beatdown)
//
// Effect268 - 2 attacks:
//   - AttackId::A4a013Poliwag (Call for Family)
//   - AttackId::B1295Poliwag (Call for Family)
//
// Effect269 - 2 attacks:
//   - AttackId::A4a016Tentacruel (Stun Poison)
//   - AttackId::B1153Drapion (Stun Poison)
//
// Effect271 - 3 attacks:
//   - AttackId::A4a020SuicuneEx (Crystal Waltz)
//   - AttackId::A4a080SuicuneEx (Crystal Waltz)
//   - AttackId::A4a090SuicuneEx (Crystal Waltz)
//
// Effect272 - 2 attacks:
//   - AttackId::A4a023Mantyke (Splashy Toss)
//   - AttackId::A4a105Mantyke (Splashy Toss)
//
// Effect275 - 2 attacks:
//   - AttackId::A4a036Latias (Crossing Flights)
//   - AttackId::PA101Latias (Crossing Flights)
//
// Effect277 - 3 attacks:
//   - AttackId::A4a042PoliwrathEx (Hydro Knuckle)
//   - AttackId::A4a082PoliwrathEx (Hydro Knuckle)
//   - AttackId::A4a089PoliwrathEx (Hydro Knuckle)
//
// Effect278 - 3 attacks:
//   - AttackId::A4a043Phanpy (Flail)
//   - AttackId::A4a076Phanpy (Flail)
//   - AttackId::PA108Phanpy (Flail)
//
// Effect282 - 2 attacks:
//   - AttackId::A4a062Miltank (Rolling Frenzy)
//   - AttackId::PA107Miltank (Rolling Frenzy)
//
// Effect283 - 2 attacks:
//   - AttackId::A4a063Azurill (Squishy Healing)
//   - AttackId::A4a077Azurill (Squishy Healing)
//
// Effect284 - 3 attacks:
//   - AttackId::B1002MegaPinsirEx (Critical Scissors)
//   - AttackId::B1251MegaPinsirEx (Critical Scissors)
//   - AttackId::B1272MegaPinsirEx (Critical Scissors)
//
// Effect285 - 2 attacks:
//   - AttackId::B1004Silcoon (Harden)
//   - AttackId::B1006Cascoon (Harden)
//
// Effect287 - 3 attacks:
//   - AttackId::B1014Simisage (Fury Swipes)
//   - AttackId::B1038Simisear (Fury Swipes)
//   - AttackId::B1062Simipour (Fury Swipes)
//
// Effect288 - 3 attacks:
//   - AttackId::B1016WhimsicottEx (Grass Knot)
//   - AttackId::B1252WhimsicottEx (Grass Knot)
//   - AttackId::B1273WhimsicottEx (Grass Knot)
//
// Effect289 - 5 attacks:
//   - AttackId::B1020Virizion (Sacred Sword)
//   - AttackId::B1070Keldeo (Sacred Sword)
//   - AttackId::B1137Terrakion (Sacred Sword)
//   - AttackId::B1169Cobalion (Sacred Sword)
//   - AttackId::B1235Keldeo (Sacred Sword)
//
// Effect290 - 2 attacks:
//   - AttackId::B1029Arcanine (Fire Fang)
//   - AttackId::PA073Toucannon (Beak Blast)
//
// Effect292 - 3 attacks:
//   - AttackId::B1036MegaBlazikenEx (Mega Burning)
//   - AttackId::B1254MegaBlazikenEx (Mega Burning)
//   - AttackId::B1284MegaBlazikenEx (Mega Burning)
//
// Effect295 - 2 attacks:
//   - AttackId::B1050Magikarp (Waterfall Evolution)
//   - AttackId::B1232Magikarp (Waterfall Evolution)
//
// Effect296 - 3 attacks:
//   - AttackId::B1052MegaGyaradosEx (Mega Blaster)
//   - AttackId::B1255MegaGyaradosEx (Mega Blaster)
//   - AttackId::B1285MegaGyaradosEx (Mega Blaster)
//
// Effect297 - 2 attacks:
//   - AttackId::B1055Ludicolo (Rhythmic Steps)
//   - AttackId::B1233Ludicolo (Rhythmic Steps)
//
// Effect302 - 3 attacks:
//   - AttackId::B1085MegaAmpharosEx (Lightning Lancer)
//   - AttackId::B1258MegaAmpharosEx (Lightning Lancer)
//   - AttackId::B1277MegaAmpharosEx (Lightning Lancer)
//
// Effect303 - 2 attacks:
//   - AttackId::B1088Luxray (Flash Impact)
//   - AttackId::B1237Luxray (Flash Impact)
//
// Effect307 - 3 attacks:
//   - AttackId::B1102MegaAltariaEx (Mega Harmony)
//   - AttackId::B1259MegaAltariaEx (Mega Harmony)
//   - AttackId::B1286MegaAltariaEx (Mega Harmony)
//
// Effect308 - 2 attacks:
//   - AttackId::B1111Cofagrigus (Soul Shot)
//   - AttackId::B1238Cofagrigus (Soul Shot)
//
// Effect310 - 3 attacks:
//   - AttackId::B1124HitmonchanEx (Quick Straight)
//   - AttackId::B1261HitmonchanEx (Quick Straight)
//   - AttackId::B1279HitmonchanEx (Quick Straight)
//
// Effect312 - 2 attacks:
//   - AttackId::B1134Archeops (Wild Spin)
//   - AttackId::B1242Archeops (Wild Spin)
//
// Effect316 - 2 attacks:
//   - AttackId::B1149Honchkrow (Evil Admonition)
//   - AttackId::B1244Honchkrow (Evil Admonition)
//
// Effect318 - 3 attacks:
//   - AttackId::B1151MegaAbsolEx (Darkness Claw)
//   - AttackId::B1262MegaAbsolEx (Darkness Claw)
//   - AttackId::B1280MegaAbsolEx (Darkness Claw)
//
// Effect323 - 2 attacks:
//   - AttackId::B1175Corviknight (Iron Wings)
//   - AttackId::B1246Corviknight (Iron Wings)
//
// Effect327 - 3 attacks:
//   - AttackId::B1183TaurosEx (Wild Tackle)
//   - AttackId::B1265TaurosEx (Wild Tackle)
//   - AttackId::B1283TaurosEx (Wild Tackle)
//
// Effect330 - 2 attacks:
//   - AttackId::B1203Stoutland (Dangerous Bite)
//   - AttackId::B1249Stoutland (Dangerous Bite)
//
// Effect332 - 2 attacks:
//   - AttackId::PA064RayquazaEx (Draco Meteor)
//   - AttackId::PA065RayquazaEx (Draco Meteor)
//
