// This is code generated from the database.json by card_enum_generator.rs. Do not edit manually.

use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AttackEffect {
    NotImplemented,
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

static EFFECT_TEXT_MAP: LazyLock<HashMap<&'static str, AttackEffect>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("Heal 30 damage from this Pokémon.", AttackEffect::Effect0);
    map.insert("Put 1 random [G] Pokémon from your deck into your hand.", AttackEffect::Effect1);
    map.insert("Your opponent's Active Pokémon is now Asleep.", AttackEffect::Effect2);
    map.insert("Your opponent's Active Pokémon is now Poisoned.", AttackEffect::Effect3);
    map.insert("Flip a coin. If heads, this attack does 30 more damage.", AttackEffect::Effect4);
    map.insert("Flip a coin. If heads, this attack does 40 more damage.", AttackEffect::Effect5);
    map.insert("Heal 10 damage from this Pokémon.", AttackEffect::Effect6);
    map.insert("Flip 2 coins. This attack does 50 damage for each heads.", AttackEffect::Effect7);
    map.insert("Take a [G] Energy from your Energy Zone and attach it to 1 of your Benched [G] Pokémon.", AttackEffect::Effect8);
    map.insert("Flip a coin. If tails, this attack does nothing.", AttackEffect::Effect9);
    map.insert("Discard a [R] Energy from this Pokémon.", AttackEffect::Effect10);
    map.insert("Discard 2 [R] Energy from this Pokémon.", AttackEffect::Effect11);
    map.insert("Flip a coin. If heads, the Defending Pokémon can't attack during your opponent's next turn.", AttackEffect::Effect12);
    map.insert("This Pokémon also does 20 damage to itself.", AttackEffect::Effect13);
    map.insert("Flip 3 coins. Take an amount of [R] Energy from your Energy Zone equal to the number of heads and attach it to your Benched [R] Pokémon in any way you like.", AttackEffect::Effect14);
    map.insert("If this Pokémon has at least 2 extra [W] Energy attached, this attack does 60 more damage.", AttackEffect::Effect15);
    map.insert("Your opponent can't use any Supporter cards from their hand during their next turn.", AttackEffect::Effect16);
    map.insert("Flip 2 coins. If both of them are heads, this attack does 80 more damage.", AttackEffect::Effect17);
    map.insert("This attack does 50 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect18);
    map.insert("Discard a random Energy from your opponent's Active Pokémon.", AttackEffect::Effect19);
    map.insert("If this Pokémon has at least 3 extra [W] Energy attached, this attack does 70 more damage.", AttackEffect::Effect20);
    map.insert("During your opponent's next turn, the Defending Pokémon can't attack.", AttackEffect::Effect21);
    map.insert("Flip a coin. If heads, your opponent's Active Pokémon is now Paralyzed.", AttackEffect::Effect22);
    map.insert("This attack also does 10 damage to each of your opponent's Benched Pokémon.", AttackEffect::Effect23);
    map.insert("If your opponent's Active Pokémon has damage on it, this attack does 60 more damage.", AttackEffect::Effect24);
    map.insert("Discard all Energy from this Pokémon.", AttackEffect::Effect25);
    map.insert("This attack does 30 damage for each of your Benched [L] Pokémon.", AttackEffect::Effect26);
    map.insert("Flip a coin. If heads, this attack does 40 more damage. If tails, this Pokémon also does 20 damage to itself.", AttackEffect::Effect27);
    map.insert("Flip 4 coins. This attack does 40 damage for each heads.", AttackEffect::Effect28);
    map.insert("This attack also does 30 damage to 1 of your Benched Pokémon.", AttackEffect::Effect29);
    map.insert("Flip 4 coins. This attack does 50 damage for each heads.", AttackEffect::Effect30);
    map.insert("This attack does 30 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect31);
    map.insert("Switch this Pokémon with 1 of your Benched Pokémon.", AttackEffect::Effect32);
    map.insert("This attack does 30 more damage for each Energy attached to your opponent's Active Pokémon.", AttackEffect::Effect33);
    map.insert("During your opponent's next turn, this Pokémon takes -20 damage from attacks.", AttackEffect::Effect34);
    map.insert("This attack does 20 more damage for each Energy attached to your opponent's Active Pokémon.", AttackEffect::Effect35);
    map.insert("Discard 2 [P] Energy from this Pokémon.", AttackEffect::Effect36);
    map.insert("Flip 2 coins. This attack does 100 damage for each heads.", AttackEffect::Effect37);
    map.insert("Flip a coin. If heads, during your opponent's next turn, prevent all damage from—and effects of—attacks done to this Pokémon.", AttackEffect::Effect38);
    map.insert("If this Pokémon has damage on it, this attack does 60 more damage.", AttackEffect::Effect39);
    map.insert("This Pokémon also does 50 damage to itself.", AttackEffect::Effect40);
    map.insert("During your opponent's next turn, attacks used by the Defending Pokémon do -20 damage.", AttackEffect::Effect41);
    map.insert("Flip 2 coins. This attack does 80 damage for each heads.", AttackEffect::Effect42);
    map.insert("This attack does 30 damage to 1 of your opponent's Benched Pokémon.", AttackEffect::Effect43);
    map.insert("Heal from this Pokémon the same amount of damage you did to your opponent's Active Pokémon.", AttackEffect::Effect44);
    map.insert("Switch out your opponent's Active Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)", AttackEffect::Effect45);
    map.insert("During your opponent's next turn, the Defending Pokémon can't retreat.", AttackEffect::Effect46);
    map.insert("Put 1 random Nidoran♂ from your deck onto your Bench.", AttackEffect::Effect47);
    map.insert("This attack does 50 more damage for each of your Benched Nidoking.", AttackEffect::Effect48);
    map.insert("If your opponent's Active Pokémon is Poisoned, this attack does 50 more damage.", AttackEffect::Effect49);
    map.insert("Flip a coin. If heads, discard a random Energy from your opponent's Active Pokémon.", AttackEffect::Effect50);
    map.insert("Take a [M] Energy from your Energy Zone and attach it to this Pokémon.", AttackEffect::Effect51);
    map.insert("1 of your opponent's Pokémon is chosen at random 4 times. For each time a Pokémon was chosen, do 50 damage to it.", AttackEffect::Effect52);
    map.insert("Draw a card.", AttackEffect::Effect53);
    map.insert("Flip a coin. If heads, discard a random card from your opponent's hand.", AttackEffect::Effect54);
    map.insert("Flip a coin until you get tails. This attack does 60 damage for each heads.", AttackEffect::Effect55);
    map.insert("Flip 2 coins. This attack does 30 damage for each heads.", AttackEffect::Effect56);
    map.insert("Choose 1 of your opponent's Pokémon's attacks and use it as this attack. If this Pokémon doesn't have the necessary Energy to use that attack, this attack does nothing.", AttackEffect::Effect57);
    map.insert("Flip a coin. If heads, your opponent shuffles their Active Pokémon into their deck.", AttackEffect::Effect58);
    map.insert("This attack does 30 damage for each of your Benched Pokémon.", AttackEffect::Effect59);
    map.insert("Your opponent reveals their hand.", AttackEffect::Effect60);
    map.insert("Take a [G] Energy from your Energy Zone and attach it to this Pokémon.", AttackEffect::Effect61);
    map.insert("Flip a coin for each Energy attached to this Pokémon. This attack does 50 damage for each heads.", AttackEffect::Effect62);
    map.insert("If this Pokémon has at least 3 extra [G] Energy attached, this attack does 70 more damage.", AttackEffect::Effect63);
    map.insert("Flip a coin. If heads, this attack does 60 more damage.", AttackEffect::Effect64);
    map.insert("Discard 2 [R] Energy from this Pokémon. This attack does 80 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect65);
    map.insert("If your opponent's Active Pokémon is Poisoned, this attack does 40 more damage.", AttackEffect::Effect66);
    map.insert("Discard a random Energy from among the Energy attached to all Pokémon (both yours and your opponent's).", AttackEffect::Effect67);
    map.insert("This attack does 50 damage to 1 of your opponent's Benched Pokémon.", AttackEffect::Effect68);
    map.insert("This attack does 10 damage for each of your Benched [L] Pokémon.", AttackEffect::Effect69);
    map.insert("This attack also does 20 damage to each of your opponent's Benched Pokémon.", AttackEffect::Effect70);
    map.insert("This attack does 40 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect71);
    map.insert("Choose 1 of your opponent's Active Pokémon's attacks and use it as this attack.", AttackEffect::Effect72);
    map.insert("This attack does 20 more damage for each of your opponent's Benched Pokémon.", AttackEffect::Effect73);
    map.insert("Heal 20 damage from each of your Pokémon.", AttackEffect::Effect74);
    map.insert("During your opponent's next turn, this Pokémon takes -30 damage from attacks.", AttackEffect::Effect75);
    map.insert("If any of your Pokémon were Knocked Out by damage from an attack during your opponent's last turn, this attack does 60 more damage.", AttackEffect::Effect76);
    map.insert("Put 1 random Koffing from your deck onto your Bench.", AttackEffect::Effect77);
    map.insert("During your opponent's next turn, if the Defending Pokémon tries to use an attack, your opponent flips a coin. If tails, that attack doesn't happen.", AttackEffect::Effect78);
    map.insert("If your opponent's Active Pokémon is a Pokémon ex, this attack does 80 more damage.", AttackEffect::Effect79);
    map.insert("Flip a coin until you get tails. This attack does 20 damage for each heads.", AttackEffect::Effect80);
    map.insert("Shuffle your hand into your deck. Draw a card for each card in your opponent's hand.", AttackEffect::Effect81);
    map.insert("Discard a random Energy from this Pokémon.", AttackEffect::Effect82);
    map.insert("During your next turn, this Pokémon can't use Frenzy Plant.", AttackEffect::Effect83);
    map.insert("Your opponent's Active Pokémon is now Confused.", AttackEffect::Effect84);
    map.insert("Put 1 random Basic Pokémon from your deck onto your Bench.", AttackEffect::Effect85);
    map.insert("Flip a coin. If heads, this attack does 50 more damage.", AttackEffect::Effect86);
    map.insert("During your next turn, this Pokémon can't attack.", AttackEffect::Effect87);
    map.insert("Take a [R] Energy from your Energy Zone and attach it to this Pokémon.", AttackEffect::Effect88);
    map.insert("Your opponent's Active Pokémon is now Burned.", AttackEffect::Effect89);
    map.insert("Discard all [R] Energy from this Pokémon.", AttackEffect::Effect90);
    map.insert("Heal 20 damage from this Pokémon.", AttackEffect::Effect91);
    map.insert("This attack also does 30 damage to 1 of your opponent's Benched Pokémon.", AttackEffect::Effect92);
    map.insert("This attack also does 20 damage to 1 of your opponent's Benched Pokémon.", AttackEffect::Effect93);
    map.insert("If your opponent's Active Pokémon is a [F] Pokémon, this attack does 30 more damage.", AttackEffect::Effect94);
    map.insert("Discard 3 [W] Energy from this Pokémon. This attack also does 20 damage to each of your opponent's Benched Pokémon.", AttackEffect::Effect95);
    map.insert("Choose 2 of your Benched Pokémon. For each of those Pokémon, take a [W] Energy from your Energy Zone and attach it to that Pokémon.", AttackEffect::Effect96);
    map.insert("Discard a [L] Energy from this Pokémon.", AttackEffect::Effect97);
    map.insert("This Pokémon also does 10 damage to itself.", AttackEffect::Effect98);
    map.insert("Take a [L] Energy from your Energy Zone and attach it to this Pokémon.", AttackEffect::Effect99);
    map.insert("If this Pokémon has at least 2 extra [L] Energy attached, this attack does 80 more damage.", AttackEffect::Effect100);
    map.insert("Discard all [L] Energy from this Pokémon. This attack does 120 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect101);
    map.insert("If this Pokémon has a Pokémon Tool attached, this attack does 40 more damage.", AttackEffect::Effect102);
    map.insert("If your opponent's Active Pokémon has a Pokémon Tool attached, this attack does 30 more damage.", AttackEffect::Effect103);
    map.insert("During your next turn, this Pokémon's Overdrive Smash attack does +60 damage.", AttackEffect::Effect104);
    map.insert("Take a [P] Energy from your Energy Zone and attach it to Mesprit or Azelf.", AttackEffect::Effect105);
    map.insert("You can use this attack only if you have Uxie and Azelf on your Bench. Discard all Energy from this Pokémon.", AttackEffect::Effect106);
    map.insert("This attack does 20 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect107);
    map.insert("Discard the top 3 cards of your deck.", AttackEffect::Effect108);
    map.insert("Flip 2 coins. This attack does 20 more damage for each heads.", AttackEffect::Effect109);
    map.insert("If your opponent's Pokémon is Knocked Out by damage from this attack, this Pokémon also does 50 damage to itself.", AttackEffect::Effect110);
    map.insert("Flip 2 coins. This attack does 20 damage for each heads.", AttackEffect::Effect111);
    map.insert("If your opponent's Active Pokémon has damage on it, this attack does 40 more damage.", AttackEffect::Effect112);
    map.insert("This attack does 10 damage to each of your opponent's Pokémon.", AttackEffect::Effect113);
    map.insert("Flip 4 coins. This attack does 40 damage for each heads. If at least 2 of them are heads, your opponent's Active Pokémon is now Poisoned.", AttackEffect::Effect114);
    map.insert("Flip a coin for each Pokémon you have in play. This attack does 20 damage for each heads.", AttackEffect::Effect115);
    map.insert("Flip a coin for each Pokémon you have in play. This attack does 40 damage for each heads.", AttackEffect::Effect116);
    map.insert("If this Pokémon has a Pokémon Tool attached, this attack does 30 more damage.", AttackEffect::Effect117);
    map.insert("Flip a coin until you get tails. This attack does 30 more damage for each heads.", AttackEffect::Effect118);
    map.insert("Flip 3 coins. This attack does 50 more damage for each heads.", AttackEffect::Effect119);
    map.insert("Take 2 [M] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon.", AttackEffect::Effect120);
    map.insert("Flip a coin until you get tails. This attack does 40 more damage for each heads.", AttackEffect::Effect121);
    map.insert("Flip a coin. If heads, this attack does 20 more damage.", AttackEffect::Effect122);
    map.insert("Change the type of the next Energy that will be generated for your opponent to 1 of the following at random: [G], [R], [W], [L], [P], [F], [D], or [M].", AttackEffect::Effect123);
    map.insert("Flip 2 coins. This attack does 40 damage for each heads.", AttackEffect::Effect124);
    map.insert("Before doing damage, discard all Pokémon Tools from your opponent's Active Pokémon.", AttackEffect::Effect125);
    map.insert("Halve your opponent's Active Pokémon's remaining HP, rounded down.", AttackEffect::Effect126);
    map.insert("Your opponent reveals their hand. Choose a card you find there and shuffle it into your opponent's deck.", AttackEffect::Effect127);
    map.insert("Flip 3 coins. This attack does 20 damage for each heads.", AttackEffect::Effect128);
    map.insert("Flip a coin. If heads, put your opponent's Active Pokémon into their hand.", AttackEffect::Effect129);
    map.insert("This attack does more damage equal to the damage this Pokémon has on it.", AttackEffect::Effect130);
    map.insert("Flip 2 coins. If both of them are heads, this attack does 70 more damage.", AttackEffect::Effect131);
    map.insert("This Pokémon also does 30 damage to itself.", AttackEffect::Effect132);
    map.insert("If this Pokémon has damage on it, this attack does 40 more damage.", AttackEffect::Effect133);
    map.insert("Flip a coin. If heads, this attack does 60 more damage. If tails, this Pokémon also does 20 damage to itself.", AttackEffect::Effect134);
    map.insert("This attack also does 10 damage to 1 of your opponent's Benched Pokémon.", AttackEffect::Effect135);
    map.insert("During your opponent's next turn, attacks used by the Defending Pokémon do -30 damage.", AttackEffect::Effect136);
    map.insert("Flip a coin. If heads, your opponent reveals a random card from their hand and shuffles it into their deck.", AttackEffect::Effect137);
    map.insert("This attack does 20 damage to 1 of your opponent's Benched Pokémon.", AttackEffect::Effect138);
    map.insert("If your opponent's Active Pokémon is a Pokémon ex, this attack does 30 more damage.", AttackEffect::Effect139);
    map.insert("During your next turn, this Pokémon's Rolling Spin attack does +60 damage.", AttackEffect::Effect140);
    map.insert("Your opponent's Active Pokémon is now Poisoned. Do 20 damage to this Pokémon instead of the usual amount for this Special Condition.", AttackEffect::Effect141);
    map.insert("If your opponent's Active Pokémon is a [M] Pokémon, this attack does 30 more damage.", AttackEffect::Effect142);
    map.insert("Flip a coin. If tails, during your next turn, this Pokémon can't attack.", AttackEffect::Effect143);
    map.insert("Discard 2 random Energy from this Pokémon.", AttackEffect::Effect144);
    map.insert("This Pokémon is now Asleep.", AttackEffect::Effect145);
    map.insert("This attack does 20 more damage for each of your Benched Pokémon.", AttackEffect::Effect146);
    map.insert("Put 1 random Weedle from your deck onto your Bench.", AttackEffect::Effect147);
    map.insert("If your opponent's Active Pokémon is a Pokémon ex, this attack does 70 more damage.", AttackEffect::Effect148);
    map.insert("Take 3 [R] Energy from your Energy Zone and attach it to this Pokémon.", AttackEffect::Effect149);
    map.insert("1 of your opponent's Pokémon is chosen at random. Do 30 damage to it.", AttackEffect::Effect150);
    map.insert("1 of your opponent's Pokémon is chosen at random 3 times. For each time a Pokémon was chosen, do 50 damage to it.", AttackEffect::Effect151);
    map.insert("Take a [L] Energy from your Energy Zone and attach it to 1 of your Benched [L] Pokémon.", AttackEffect::Effect152);
    map.insert("This attack also does 20 damage to each of your opponent's Benched Pokémon that has any Energy attached.", AttackEffect::Effect153);
    map.insert("Flip 4 coins. This attack does 20 damage for each heads.", AttackEffect::Effect154);
    map.insert("If this Pokémon has at least 2 extra [F] Energy attached, this attack does 50 more damage.", AttackEffect::Effect155);
    map.insert("If your opponent's Active Pokémon is Poisoned, this attack does 60 more damage.", AttackEffect::Effect156);
    map.insert("Flip a coin. If heads, your opponent's Active Pokémon is now Confused.", AttackEffect::Effect157);
    map.insert("Flip a coin. If heads, this attack does 80 more damage.", AttackEffect::Effect158);
    map.insert("Flip a coin for each [M] Energy attached to this Pokémon. This attack does 50 damage for each heads.", AttackEffect::Effect159);
    map.insert("During your next turn, this Pokémon's Overacceleration attack does +20 damage.", AttackEffect::Effect160);
    map.insert("This attack does 10 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect161);
    map.insert("This attack does 100 damage to 1 of your opponent's Pokémon that have damage on them.", AttackEffect::Effect162);
    map.insert("This attack does 20 damage to each of your opponent's Pokémon.", AttackEffect::Effect163);
    map.insert("Flip 3 coins. This attack does 50 damage for each heads.", AttackEffect::Effect164);
    map.insert("If this Pokémon moved from your Bench to the Active Spot this turn, this attack does 60 more damage.", AttackEffect::Effect165);
    map.insert("Flip a coin. If tails, this Pokémon also does 20 damage to itself.", AttackEffect::Effect166);
    map.insert("Flip 2 coins. This attack does 70 damage for each heads. If at least 1 of them is heads, your opponent's Active Pokémon is now Burned.", AttackEffect::Effect167);
    map.insert("Discard a random Energy from both Active Pokémon.", AttackEffect::Effect168);
    map.insert("Your opponent's Active Pokémon is now Poisoned and Burned.", AttackEffect::Effect169);
    map.insert("During your opponent's next turn, if this Pokémon is damaged by an attack, do 40 damage to the Attacking Pokémon.", AttackEffect::Effect170);
    map.insert("Take a [W] Energy from your Energy Zone and attach it to this Pokémon.", AttackEffect::Effect171);
    map.insert("During your next turn, this Pokémon's Insatiable Striking attack does +40 damage.", AttackEffect::Effect172);
    map.insert("Put 1 random Wishiwashi or Wishiwashi ex from your deck onto your Bench.", AttackEffect::Effect173);
    map.insert("This attack does 40 more damage for each of your Benched Wishiwashi and Wishiwashi ex.", AttackEffect::Effect174);
    map.insert("If your opponent's Active Pokémon is a Basic Pokémon, this attack does 60 more damage.", AttackEffect::Effect175);
    map.insert("Discard 2 [L] Energy from this Pokémon.", AttackEffect::Effect176);
    map.insert("During your opponent's next turn, they can't play any Item cards from their hand.", AttackEffect::Effect177);
    map.insert("Switch this Pokémon with 1 of your Benched [L] Pokémon.", AttackEffect::Effect178);
    map.insert("Take a [P] Energy from your Energy Zone and attach it to this Pokémon.", AttackEffect::Effect179);
    map.insert("During your opponent's next turn, they can't take any Energy from their Energy Zone to attach to their Active Pokémon.", AttackEffect::Effect180);
    map.insert("This attack also does 20 damage to 1 of your Pokémon.", AttackEffect::Effect181);
    map.insert("This attack does 20 damage to 1 of your opponent's Pokémon for each Energy attached to that Pokémon.", AttackEffect::Effect182);
    map.insert("During your opponent's next turn, this Pokémon takes -50 damage from attacks.", AttackEffect::Effect183);
    map.insert("Put a random card that evolves from Rockruff from your deck into your hand.", AttackEffect::Effect184);
    map.insert("If your opponent's Active Pokémon has more remaining HP than this Pokémon, this attack does 50 more damage.", AttackEffect::Effect185);
    map.insert("This Pokémon also does 40 damage to itself.", AttackEffect::Effect186);
    map.insert("Discard a random Item card from your opponent's hand.", AttackEffect::Effect187);
    map.insert("1 Special Condition from among Asleep, Burned, Confused, Paralyzed, and Poisoned is chosen at random, and your opponent's Active Pokémon is now affected by that Special Condition. Any Special Conditions already affecting that Pokémon will not be chosen.", AttackEffect::Effect188);
    map.insert("If your opponent's Active Pokémon is affected by a Special Condition, this attack does 60 more damage.", AttackEffect::Effect189);
    map.insert("Flip a coin until you get tails. This attack does 70 damage for each heads.", AttackEffect::Effect190);
    map.insert("If your opponent's Active Pokémon has an Ability, this attack does 40 more damage.", AttackEffect::Effect191);
    map.insert("If any of your Benched Pokémon have damage on them, this attack does 50 more damage.", AttackEffect::Effect192);
    map.insert("During your opponent's next turn, this Pokémon takes +30 damage from attacks.", AttackEffect::Effect193);
    map.insert("Take a [C] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon.", AttackEffect::Effect194);
    map.insert("If your opponent's Active Pokémon is a [D] Pokémon, this attack does 30 more damage.", AttackEffect::Effect195);
    map.insert("This Pokémon is now Confused.", AttackEffect::Effect196);
    map.insert("During your opponent's next turn, attacks used by the Defending Pokémon cost 1 [C] more, and its Retreat Cost is 1 [C] more.", AttackEffect::Effect197);
    map.insert("Flip 3 coins. This attack does 10 damage for each heads.", AttackEffect::Effect198);
    map.insert("This attack does 70 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect199);
    map.insert("During your next turn, this Pokémon can't use Big Beat.", AttackEffect::Effect200);
    map.insert("This Pokémon also does 70 damage to itself.", AttackEffect::Effect201);
    map.insert("Discard a [F] Energy from this Pokémon.", AttackEffect::Effect202);
    map.insert("If Passimian is on your Bench, this attack does 40 more damage.", AttackEffect::Effect203);
    map.insert("Discard a random Pokémon Tool card from your opponent's hand.", AttackEffect::Effect204);
    map.insert("Flip 3 coins. For each heads, a card is chosen at random from your opponent's hand. Your opponent reveals that card and shuffles it into their deck.", AttackEffect::Effect205);
    map.insert("Flip a coin until you get tails. For each heads, discard a random Energy from your opponent's Active Pokémon.", AttackEffect::Effect206);
    map.insert("Flip 3 coins. This attack does 60 damage for each heads.", AttackEffect::Effect207);
    map.insert("Flip 2 coins. If both of them are heads, your opponent's Active Pokémon is Knocked Out.", AttackEffect::Effect208);
    map.insert("If you played a Supporter card from your hand during this turn, this attack does 50 more damage.", AttackEffect::Effect209);
    map.insert("This attack does 20 more damage for each [G] Energy attached to this Pokémon.", AttackEffect::Effect210);
    map.insert("Your opponent reveals a random card from their hand and shuffles it into their deck.", AttackEffect::Effect211);
    map.insert("If 1 of your Pokémon used Sweets Relay during your last turn, this attack does 30 more damage.", AttackEffect::Effect212);
    map.insert("Take a [R] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon.", AttackEffect::Effect213);
    map.insert("If 1 of your Pokémon used Sweets Relay during your last turn, this attack does 20 more damage.", AttackEffect::Effect214);
    map.insert("If this Pokémon has at least 1 extra [W] Energy attached, this attack does 40 more damage.", AttackEffect::Effect215);
    map.insert("If this Pokémon evolved during this turn, this attack does 20 more damage.", AttackEffect::Effect216);
    map.insert("Discard all Energy attached to this Pokémon. Your opponent's Active Pokémon is now Paralyzed.", AttackEffect::Effect217);
    map.insert("This attack does 20 damage for each Energy attached to all of your opponent's Pokémon.", AttackEffect::Effect218);
    map.insert("If 1 of your Pokémon used Sweets Relay during your last turn, this attack does 60 more damage.", AttackEffect::Effect219);
    map.insert("This attack does 30 more damage for each Evolution Pokémon on your Bench.", AttackEffect::Effect220);
    map.insert("Flip a coin. If heads, choose 1 of your opponent's Active Pokémon's attacks and use it as this attack.", AttackEffect::Effect221);
    map.insert("This attack does 40 damage for each time your Pokémon used Sweets Relay during this game.", AttackEffect::Effect222);
    map.insert("This attack also does 10 damage to each of your Benched Pokémon.", AttackEffect::Effect223);
    map.insert("If the Defending Pokémon is a Basic Pokémon, it can't attack during your opponent's next turn.", AttackEffect::Effect224);
    map.insert("Discard all Pokémon Tools from your opponent's Active Pokémon.", AttackEffect::Effect225);
    map.insert("During your opponent's next turn, if this Pokémon is damaged by an attack, do 30 damage to the Attacking Pokémon.", AttackEffect::Effect226);
    map.insert("Flip a coin. If tails, this attack does nothing. If heads, your opponent's Active Pokémon is now Paralyzed.", AttackEffect::Effect227);
    map.insert("This attack does 20 damage for each of your Benched Pokémon.", AttackEffect::Effect228);
    map.insert("If this Pokémon has a Pokémon Tool attached, this attack does 50 more damage.", AttackEffect::Effect229);
    map.insert("Flip 3 coins. This attack does 60 damage for each heads. This Pokémon is now Confused.", AttackEffect::Effect230);
    map.insert("This attack does 40 more damage for each Energy in your opponent's Active Pokémon's Retreat Cost.", AttackEffect::Effect231);
    map.insert("If this Pokémon has no damage on it, this attack does 40 more damage.", AttackEffect::Effect232);
    map.insert("1 other Pokémon (either yours or your opponent's) is chosen at random 3 times. For each time a Pokémon was chosen, do 50 damage to it.", AttackEffect::Effect233);
    map.insert("Take a [R] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon.", AttackEffect::Effect234);
    map.insert("Flip a coin. If tails, discard 2 random Energy from this Pokémon.", AttackEffect::Effect235);
    map.insert("Take a [R], [W], and [L] Energy from your Energy Zone and attach them to your Benched Basic Pokémon in any way you like.", AttackEffect::Effect236);
    map.insert("If your opponent's Active Pokémon is Burned, this attack does 60 more damage.", AttackEffect::Effect237);
    map.insert("You may discard any number of your Benched [W] Pokémon. This attack does 40 more damage for each Benched Pokémon you discarded in this way.", AttackEffect::Effect238);
    map.insert("Put a random Pokémon from your deck into your hand.", AttackEffect::Effect239);
    map.insert("If the Defending Pokémon tries to use an attack, your opponent flips a coin. If tails, that attack doesn't happen. This effect lasts until the Defending Pokémon leaves the Active Spot, and it doesn't stack.", AttackEffect::Effect240);
    map.insert("Move all Energy from this Pokémon to 1 of your Benched Pokémon.", AttackEffect::Effect241);
    map.insert("Flip a coin. If heads, your opponent's Active Pokémon is now Paralyzed. If tails, your opponent's Active Pokémon is now Confused.", AttackEffect::Effect242);
    map.insert("Take a [L] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon.", AttackEffect::Effect243);
    map.insert("This attack also does 10 damage to 1 of your Benched Pokémon.", AttackEffect::Effect244);
    map.insert("This Pokémon is now Asleep. Heal 30 damage from it.", AttackEffect::Effect245);
    map.insert("This attack does 20 damage for each Energy attached to your opponent's Active Pokémon.", AttackEffect::Effect246);
    map.insert("Flip a coin. If heads, your opponent's Active Pokémon's remaining HP is now 10.", AttackEffect::Effect247);
    map.insert("If this Pokémon was damaged by an attack during your opponent's last turn while it was in the Active Spot, this attack does 50 more damage.", AttackEffect::Effect248);
    map.insert("Both Active Pokémon are now Asleep.", AttackEffect::Effect249);
    map.insert("This attack also does 20 damage to each of your Benched Pokémon.", AttackEffect::Effect250);
    map.insert("If this Pokémon has at least 2 extra [F] Energy attached, this attack does 60 more damage.", AttackEffect::Effect251);
    map.insert("Discard a random card from your opponent's hand.", AttackEffect::Effect252);
    map.insert("If this Pokémon moved from your Bench to the Active Spot this turn, this attack does 50 more damage.", AttackEffect::Effect253);
    map.insert("During your next turn, this Pokémon's Gear Spinner attack does +70 damage.", AttackEffect::Effect254);
    map.insert("If your opponent's Active Pokémon is a [G] Pokémon, this attack does 40 more damage.", AttackEffect::Effect255);
    map.insert("Flip a coin. If heads, during your opponent's next turn, prevent all damage done to this Pokémon by attacks.", AttackEffect::Effect256);
    map.insert("This attack does 20 more damage for each Energy attached to this Pokémon.", AttackEffect::Effect257);
    map.insert("If your opponent's Active Pokémon is an Evolution Pokémon, this attack does 40 more damage.", AttackEffect::Effect258);
    map.insert("During your opponent's next turn, attacks used by the Defending Pokémon cost 1 [C] more.", AttackEffect::Effect259);
    map.insert("Flip a coin. If tails, this attack does nothing. If heads, during your opponent's next turn, prevent all damage from—and effects of—attacks done to this Pokémon.", AttackEffect::Effect260);
    map.insert("Draw cards until you have the same number of cards in your hand as your opponent.", AttackEffect::Effect261);
    map.insert("Change the type of a random Energy attached to your opponent's Active Pokémon to 1 of the following at random: [G], [R], [W], [L], [P], [F], [D], or [M].", AttackEffect::Effect262);
    map.insert("Discard a [R], [W], and [L] Energy from this Pokémon.", AttackEffect::Effect263);
    map.insert("You may switch this Pokémon with 1 of your Benched Pokémon.", AttackEffect::Effect264);
    map.insert("If your opponent's Active Pokémon is an evolved Pokémon, devolve it by putting the highest Stage Evolution card on it into your opponent's hand.", AttackEffect::Effect265);
    map.insert("Discard the top card of your opponent's deck.", AttackEffect::Effect266);
    map.insert("If this Pokémon has at least 2 extra [R] Energy attached, this attack does 60 more damage.", AttackEffect::Effect267);
    map.insert("Put 1 random Poliwag from your deck onto your Bench.", AttackEffect::Effect268);
    map.insert("Flip a coin. If heads, your opponent's Active Pokémon is now Poisoned and Paralyzed.", AttackEffect::Effect269);
    map.insert("Discard up to 2 Pokémon Tool cards from your hand. This attack does 50 damage for each card you discarded in this way.", AttackEffect::Effect270);
    map.insert("This attack does 20 damage for each Benched Pokémon (both yours and your opponent's).", AttackEffect::Effect271);
    map.insert("Take a [W] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon.", AttackEffect::Effect272);
    map.insert("If this Pokémon has damage on it, this attack can be used for 1 [L] Energy.", AttackEffect::Effect273);
    map.insert("At the end of your opponent's next turn, do 90 damage to the Defending Pokémon.", AttackEffect::Effect274);
    map.insert("If Latios is on your Bench, this attack does 20 more damage.", AttackEffect::Effect275);
    map.insert("Discard the top card of your deck. If that card is a [F] Pokémon, this attack does 60 more damage.", AttackEffect::Effect276);
    map.insert("If this Pokémon has any [W] Energy attached, this attack does 40 more damage.", AttackEffect::Effect277);
    map.insert("This attack does damage to your opponent's Active Pokémon equal to the damage this Pokémon has on it.", AttackEffect::Effect278);
    map.insert("If your opponent's Active Pokémon is Zangoose, this attack does 40 more damage.", AttackEffect::Effect279);
    map.insert("If this Pokémon has 2 or more different types of Energy attached, this attack does 60 more damage.", AttackEffect::Effect280);
    map.insert("This attack does 60 damage to 1 of your opponent's Pokémon.", AttackEffect::Effect281);
    map.insert("Until this Pokémon leaves the Active Spot, this Pokémon's Rolling Frenzy attack does +30 damage. This effect stacks.", AttackEffect::Effect282);
    map.insert("Heal 50 damage from 1 of your Benched Pokémon.", AttackEffect::Effect283);
    map.insert("Flip a coin. If heads, this attack does 70 more damage.", AttackEffect::Effect284);
    map.insert("During your opponent's next turn, prevent all damage done to this Pokémon by attacks if that damage is 40 or less.", AttackEffect::Effect285);
    map.insert("Choose either Poisoned or Confused. Your opponent's Active Pokémon is now affected by that Special Condition.", AttackEffect::Effect286);
    map.insert("Flip 3 coins. This attack does 40 damage for each heads.", AttackEffect::Effect287);
    map.insert("This attack does 30 more damage for each Energy in your opponent's Active Pokémon's Retreat Cost.", AttackEffect::Effect288);
    map.insert("During your next turn, this Pokémon can't use Sacred Sword.", AttackEffect::Effect289);
    map.insert("Flip a coin. If heads, your opponent's Active Pokémon is now Burned.", AttackEffect::Effect290);
    map.insert("Heal 30 damage from each of your Benched Basic Pokémon.", AttackEffect::Effect291);
    map.insert("Discard Fire[R] Energy from this Pokémon. Your opponent's Active Pokémon is now Burned.", AttackEffect::Effect292);
    map.insert("Flip 2 coins. This attack does 30 more damage for each heads.", AttackEffect::Effect293);
    map.insert("During your opponent's next turn, if this Pokémon is damaged by an attack, do 20 damage to the Attacking Pokémon.", AttackEffect::Effect294);
    map.insert("Put a random card from your deck that evolves from this Pokémon onto this Pokémon to evolve it.", AttackEffect::Effect295);
    map.insert("Discard the top 3 cards of your opponent's deck.", AttackEffect::Effect296);
    map.insert("If you have exactly 1, 3, or 5 cards in your hand, this attack does 60 more damage.", AttackEffect::Effect297);
    map.insert("This attack does 10 more damage for each [W] Energy attached to this Pokémon.", AttackEffect::Effect298);
    map.insert("If you have exactly 2, 4, or 6 cards in your hand, this attack does 30 more damage.", AttackEffect::Effect299);
    map.insert("Prevent all damage done to this Pokémon by attacks from Basic Pokémon during your opponent's next turn.", AttackEffect::Effect300);
    map.insert("1 of your opponent's Benched Pokémon is chosen at random. This attack also does 20 damage to it.", AttackEffect::Effect301);
    map.insert("1 of your opponent's Benched Pokémon is chosen at random 3 times. For each time a Pokémon was chosen, also do 20 damage to it.", AttackEffect::Effect302);
    map.insert("This attack also does 20 damage to 1 of your Benched Pokémon.", AttackEffect::Effect303);
    map.insert("If your opponent's Active Pokémon has damage on it, this attack does 30 more damage.", AttackEffect::Effect304);
    map.insert("Discard a [L] Energy from your opponent's Active Pokémon.", AttackEffect::Effect305);
    map.insert("Discard a card from your hand. If you can't, this attack does nothing.", AttackEffect::Effect306);
    map.insert("This attack does 30 more damage for each of your Benched Pokémon.", AttackEffect::Effect307);
    map.insert("Discard 2 cards from your hand. If you can't discard 2 cards, this attack does nothing.", AttackEffect::Effect308);
    map.insert("During your opponent's next turn, if they attach Energy from their Energy Zone to the Defending Pokémon, that Pokémon will be Asleep.", AttackEffect::Effect309);
    map.insert("This attack's damage isn't affected by Weakness.", AttackEffect::Effect310);
    map.insert("If this Pokémon has damage on it, this attack does 50 more damage.", AttackEffect::Effect311);
    map.insert("This attack does 20 damage to each of your opponent's Pokémon. During your next turn, this Pokémon's Wild Spin attack does +20 damage to each of your opponent's Pokémon.", AttackEffect::Effect312);
    map.insert("Reveal the top 3 cards of your deck. This attack does 60 damage for each Pokémon with a Retreat Cost of 3 or more you find there. Shuffle the revealed cards back into your deck.", AttackEffect::Effect313);
    map.insert("If this Pokémon's remaining HP is 30 or less, this attack does 60 more damage.", AttackEffect::Effect314);
    map.insert("If your opponent's Active Pokémon is a [G] Pokémon, this attack does 50 more damage.", AttackEffect::Effect315);
    map.insert("This attack does 40 more damage for each of your opponent's Pokémon in play that has an Ability.", AttackEffect::Effect316);
    map.insert("Flip a coin. If heads, your opponent reveals their hand. Choose a Supporter card you find there and discard it.", AttackEffect::Effect317);
    map.insert("Your opponent reveals their hand. Choose a Supporter card you find there and discard it.", AttackEffect::Effect318);
    map.insert("During your next turn, this Pokémon's Overdrive Smash attack does +30 damage.", AttackEffect::Effect319);
    map.insert("If your opponent's Active Pokémon is Poisoned, this attack does 70 more damage.", AttackEffect::Effect320);
    map.insert("This attack's damage isn't affected by any effects on your opponent's Active Pokémon.", AttackEffect::Effect321);
    map.insert("If Durant is on your Bench, this attack does 40 more damage.", AttackEffect::Effect322);
    map.insert("Discard 2 [M] Energy from this Pokémon. During your opponent's next turn, this Pokémon takes -50 damage from attacks.", AttackEffect::Effect323);
    map.insert("Flip 2 coins. If both of them are tails, this attack does nothing.", AttackEffect::Effect324);
    map.insert("Heal 40 damage from this Pokémon.", AttackEffect::Effect325);
    map.insert("Flip 2 coins. For each heads, discard a random Energy from your opponent's Active Pokémon. If both of them are tails, this attack does nothing.", AttackEffect::Effect326);
    map.insert("Flip a coin. If tails, this Pokémon also does 30 damage to itself.", AttackEffect::Effect327);
    map.insert("Flip 2 coins. This attack does 30 damage for each heads. If this Pokémon has Lucky Mittens attached, flip 4 coins instead.", AttackEffect::Effect328);
    map.insert("Both Active Pokémon are now Confused.", AttackEffect::Effect329);
    map.insert("If your opponent's Active Pokémon is a Basic Pokémon, this attack does 70 more damage.", AttackEffect::Effect330);
    map.insert("Flip a coin until you get tails. This attack does 40 damage for each heads.", AttackEffect::Effect331);
    map.insert("1 of your opponent's Pokémon is chosen at random 4 times. For each time a Pokémon was chosen, do 40 damage to it.", AttackEffect::Effect332);
    map.insert("Discard a [M] Energy from this Pokémon.", AttackEffect::Effect333);
    map.insert("Discard the top 5 cards of each player's deck.", AttackEffect::Effect334);
    map.insert("Flip a coin. If heads, switch in 1 of your opponent's Benched Pokémon to the Active Spot.", AttackEffect::Effect335);
    map.insert("Flip a coin. If heads, heal 60 damage from this Pokémon.", AttackEffect::Effect336);
    map
});

impl AttackEffect {
    /// Get the AttackEffect for a given effect text
    pub fn from_effect_text(effect_text: &str) -> Self {
        EFFECT_TEXT_MAP.get(effect_text).copied().unwrap_or(AttackEffect::NotImplemented)
    }
}

// Statistics:
// Total unique effect texts: 337

// Effect groupings (attacks with same effect text):
// Effect0 - 18 attacks with this effect:
//   Effect: "Heal 30 damage from this Pokémon."
//   Attacks: Mega Drain, Giant Bloom, Bubble Drain, Giant Bloom, Mega Drain, Mega Drain, Carefree Press, Carefree Press, Carefree Press, Mega Drain, Giant Bloom, Bubble Drain, Giant Bloom, Carefree Press, Draining Kiss, Carefree Press, Mega Drain, Shed Skin
//
// Effect1 - 5 attacks with this effect:
//   Effect: "Put 1 random [G] Pokémon from your deck into your hand."
//   Attacks: Find a Friend, Cry for Help, Cry for Help, Cry for Help, Cry for Help
//
// Effect2 - 20 attacks with this effect:
//   Effect: "Your opponent's Active Pokémon is now Asleep."
//   Attacks: Soothing Scent, Powder Snow, Sleepy Song, Sleepy Song, Sleepy Song, Flickering Spores, Hypnotic Gaze, Dark Void, Water Pulse, Hypnoblast, Sing, Sleepy Song, Water Pulse, Sleep Powder, Sleepy Lullaby, Sleepy Song, Sing, Sleepy Song, Sing, Sing
//
// Effect3 - 34 attacks with this effect:
//   Effect: "Your opponent's Active Pokémon is now Poisoned."
//   Attacks: Poison Powder, Poison Tentacles, Poison Horn, Poison Gas, Poison Horn, Poison Claws, Poison Sting, Poisonous Whip, Poison Gas, Venomous Fang, Poison Jab, Super Poison Breath, New Wave, Electro House, New Wave, Poison Powder, Poison Sting, Venomous Fang, Venomous Fang, Venomous Slash, Poison Sting, Venomous Slash, Venomous Slash, Poison Horn, Poison Sting, Poison Jab, Venomous Slash, Poison Jab, Poison Jab, New Wave, New Wave, Poison Sting, Poison Sting, Poison Gas
//
// Effect4 - 16 attacks with this effect:
//   Effect: "Flip a coin. If heads, this attack does 30 more damage."
//   Attacks: Stomp, Stomp, Leaf Cutter, Heat Breath, Jet Screw, Tumbling Attack, Pummel, Tenacious Hammer, Electrosmash, Trip Over, Quick Attack, Tenacious Hammer, Tenacious Hammer, Ambush, Play Rough, Quick Attack
//
// Effect5 - 6 attacks with this effect:
//   Effect: "Flip a coin. If heads, this attack does 40 more damage."
//   Attacks: Tropical Swing, Quick Attack, Tropical Swing, Tropical Swing, Tropical Swing, Biting Fang
//
// Effect6 - 7 attacks with this effect:
//   Effect: "Heal 10 damage from this Pokémon."
//   Attacks: Absorb, Blot, Leaf Drain, Blot, Blot, Absorb, Drain Slap
//
// Effect7 - 3 attacks with this effect:
//   Effect: "Flip 2 coins. This attack does 50 damage for each heads."
//   Attacks: Double Horn, Double Horn, Double Kick
//
// Effect9 - 25 attacks with this effect:
//   Effect: "Flip a coin. If tails, this attack does nothing."
//   Attacks: Surprise Attack, Sky Attack, Horn Hazard, Focus Fist, Pose, Pose, Kick Shot, Tropical Hammer, Tropical Hammer, Happened to Headbutt, Moombahton, Moombahton, Kick Shot, Tail Smash, Focus Fist, Surprise Attack, Happened to Headbutt, Happened to Headbutt, Moombahton, Moombahton, Tail Smash, Sky Attack, Surprise Attack, Tropical Hammer, Surprise Attack
//
// Effect10 - 15 attacks with this effect:
//   Effect: "Discard a [R] Energy from this Pokémon."
//   Attacks: Ember, Flamethrower, Flamethrower, Fire Blast, Ember, Fire Spin, Fire Spin, Flamethrower, Ember, Ember, Ember, Flamethrower, Flamethrower, Blaze Kick, Ember
//
// Effect11 - 13 attacks with this effect:
//   Effect: "Discard 2 [R] Energy from this Pokémon."
//   Attacks: Fire Spin, Crimson Storm, Crimson Storm, Crimson Storm, Crimson Storm, Fire Blast, Fire Spin, Fire Spin, Fire Spin, Crimson Storm, Fire Spin, Crimson Storm, Fire Blast
//
// Effect12 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, the Defending Pokémon can't attack during your opponent's next turn."
//   Attacks: Tail Whip, Flickering Light
//
// Effect13 - 19 attacks with this effect:
//   Effect: "This Pokémon also does 20 damage to itself."
//   Attacks: Heat Tackle, Inferno Onrush, Inferno Onrush, Steel Tackle, Steel Tackle, Brave Bird, Chaotic Impact, Chaotic Impact, Chaotic Impact, Heat Tackle, Inferno Onrush, Thunder, Wild Tackle, Wild Charge, Inferno Onrush, Chaotic Impact, Chaotic Impact, Double-Edge, Double-Edge
//
// Effect14 - 6 attacks with this effect:
//   Effect: "Flip 3 coins. Take an amount of [R] Energy from your Energy Zone equal to the number of heads and attach it to your Benched [R] Pokémon in any way you like."
//   Attacks: Inferno Dance, Inferno Dance, Inferno Dance, Inferno Dance, Inferno Dance, Inferno Dance
//
// Effect15 - 7 attacks with this effect:
//   Effect: "If this Pokémon has at least 2 extra [W] Energy attached, this attack does 60 more damage."
//   Attacks: Hydro Pump, Hydro Bazooka, Hydro Bazooka, Hydro Pump, Hydro Bazooka, Hydro Bazooka, Hydro Pump
//
// Effect16 - 4 attacks with this effect:
//   Effect: "Your opponent can't use any Supporter cards from their hand during their next turn."
//   Attacks: Headache, Bother, Bother, Headache
//
// Effect17 - 2 attacks with this effect:
//   Effect: "Flip 2 coins. If both of them are heads, this attack does 80 more damage."
//   Attacks: KO Crab, KO Crab
//
// Effect18 - 11 attacks with this effect:
//   Effect: "This attack does 50 damage to 1 of your opponent's Pokémon."
//   Attacks: Water Arrow, Skill Dive, Linear Attack, Linear Attack, Linear Attack, Ice Blade, Ice Blade, Linear Attack, Linear Attack, Skill Dive, Skill Dive
//
// Effect19 - 7 attacks with this effect:
//   Effect: "Discard a random Energy from your opponent's Active Pokémon."
//   Attacks: Hyper Beam, Hyper Beam, Crushing Spear, Crushing Spear, Crushing Spear, Destructive Whirlpool, Crushing Spear
//
// Effect20 - 3 attacks with this effect:
//   Effect: "If this Pokémon has at least 3 extra [W] Energy attached, this attack does 70 more damage."
//   Attacks: Hydro Pump, Hydro Pump, Hydro Pump
//
// Effect22 - 17 attacks with this effect:
//   Effect: "Flip a coin. If heads, your opponent's Active Pokémon is now Paralyzed."
//   Attacks: Ice Beam, Thunder Fang, Thunder Shock, Thunder Shock, Thunder Shock, Ice Beam, String Shot, String Shot, Fake Out, Jolt, Powerful Vise, Attract Smack, Sudden Flash, String Shot, String Shot, Body Slam, Ice Beam
//
// Effect23 - 9 attacks with this effect:
//   Effect: "This attack also does 10 damage to each of your opponent's Benched Pokémon."
//   Attacks: Blizzard, Blizzard, Blizzard, Blizzard, Blizzard, Ocean Cyclone, Ocean Cyclone, Blizzard, Blizzard
//
// Effect24 - 2 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon has damage on it, this attack does 60 more damage."
//   Attacks: Second Strike, Attack the Wound
//
// Effect25 - 10 attacks with this effect:
//   Effect: "Discard all Energy from this Pokémon."
//   Attacks: Thunderbolt, Thunderbolt, Thunderbolt, Thunderbolt, Luster Purge, Luster Purge, Thunderbolt, Hyper Ray, Hyper Ray, Thunderbolt
//
// Effect26 - 7 attacks with this effect:
//   Effect: "This attack does 30 damage for each of your Benched [L] Pokémon."
//   Attacks: Circle Circuit, Circle Circuit, Circle Circuit, Circle Circuit, Circle Circuit, Circle Circuit, Circle Circuit
//
// Effect27 - 3 attacks with this effect:
//   Effect: "Flip a coin. If heads, this attack does 40 more damage. If tails, this Pokémon also does 20 damage to itself."
//   Attacks: Thunder Punch, Thunder Punch, Thunder Punch
//
// Effect29 - 2 attacks with this effect:
//   Effect: "This attack also does 30 damage to 1 of your Benched Pokémon."
//   Attacks: Raging Thunder, Raging Thunder
//
// Effect30 - 6 attacks with this effect:
//   Effect: "Flip 4 coins. This attack does 50 damage for each heads."
//   Attacks: Thundering Hurricane, Thundering Hurricane, Thundering Hurricane, Thundering Hurricane, Thundering Hurricane, Hurricane Punch
//
// Effect31 - 4 attacks with this effect:
//   Effect: "This attack does 30 damage to 1 of your opponent's Pokémon."
//   Attacks: Thunder Spear, Water Arrow, Stretch Tongue, Star Drop
//
// Effect32 - 19 attacks with this effect:
//   Effect: "Switch this Pokémon with 1 of your Benched Pokémon."
//   Attacks: Teleport, Leap Out, Teleport, Teleport, Teleport, Teleport, U-turn, U-turn, Leap Out, Bounce, Leap Out, Leap Out, Leap Out, Teleport, Teleport, Teleport, Teleport, Bounce, Teleport
//
// Effect33 - 10 attacks with this effect:
//   Effect: "This attack does 30 more damage for each Energy attached to your opponent's Active Pokémon."
//   Attacks: Psychic, Psychic, Psychic, Psychic, Psychic, Psychic, Psychic, Psychic, Psychic, Psychic
//
// Effect34 - 18 attacks with this effect:
//   Effect: "During your opponent's next turn, this Pokémon takes -20 damage from attacks."
//   Attacks: Barrier Attack, Expand, Guard Press, Expand, Defensive Unit, Defensive Unit, Defensive Unit, Guard Press, Brass Rock, Steel Wing, Steel Wing, Steel Wing, Steel Wing, Defensive Unit, Frost Barrier, Cotton Guard, Guard Press, Brass Rock
//
// Effect35 - 13 attacks with this effect:
//   Effect: "This attack does 20 more damage for each Energy attached to your opponent's Active Pokémon."
//   Attacks: Psychic, Psychic, Psychic, Energized Blade, Energized Blade, Energized Blade, Psychic, Psychic, Psychic, Energized Blade, Psychic, Psychic, Energized Blade
//
// Effect36 - 9 attacks with this effect:
//   Effect: "Discard 2 [P] Energy from this Pokémon."
//   Attacks: Power Blast, Psydrive, Psydrive, Psydrive, Psydrive, Psydrive, Psydrive, Power Blast, Psydrive
//
// Effect37 - 2 attacks with this effect:
//   Effect: "Flip 2 coins. This attack does 100 damage for each heads."
//   Attacks: Double Lariat, Darkest Lariat
//
// Effect38 - 12 attacks with this effect:
//   Effect: "Flip a coin. If heads, during your opponent's next turn, prevent all damage from—and effects of—attacks done to this Pokémon."
//   Attacks: Dig, Dive, Elegant Swim, Hide, Hide, Chest-ouflage, Spinning Drift, Spinning Drift, Barrier Shove, Hard Roll, Splashing Dodge, Dive
//
// Effect39 - 7 attacks with this effect:
//   Effect: "If this Pokémon has damage on it, this attack does 60 more damage."
//   Attacks: Fight Back, Scar-Charged Smash, Scar-Charged Smash, Scar-Charged Smash, Fight Back, Scar-Charged Smash, Scar-Charged Smash
//
// Effect41 - 9 attacks with this effect:
//   Effect: "During your opponent's next turn, attacks used by the Defending Pokémon do -20 damage."
//   Attacks: Growl, Growl, Growl, Cure Stream, Charm, Charm, Growl, Growl, Growl
//
// Effect42 - 5 attacks with this effect:
//   Effect: "Flip 2 coins. This attack does 80 damage for each heads."
//   Attacks: Bonemerang, Bonemerang, Bonemerang, Double Spin, Bonemerang
//
// Effect43 - 4 attacks with this effect:
//   Effect: "This attack does 30 damage to 1 of your opponent's Benched Pokémon."
//   Attacks: Stretch Kick, Leap Over, Tongue Whip, Leap Over
//
// Effect45 - 3 attacks with this effect:
//   Effect: "Switch out your opponent's Active Pokémon to the Bench. (Your opponent chooses the new Active Pokémon.)"
//   Attacks: Knock Back, Push Out, Roar
//
// Effect46 - 20 attacks with this effect:
//   Effect: "During your opponent's next turn, the Defending Pokémon can't retreat."
//   Attacks: Corner, Electroweb, Clutch, Clutch, Corner, Corner, Anchor Shot, Corner, Anchor Shot, Anchor Shot, Sand Tomb, Corner, Stormy Prison, Bind Down, Stormy Prison, Stormy Prison, Anchor Shot, Stormy Prison, Jaw Lock, Shadow Cage
//
// Effect47 - 2 attacks with this effect:
//   Effect: "Put 1 random Nidoran♂ from your deck onto your Bench."
//   Attacks: Call for Family, Call for Family
//
// Effect48 - 3 attacks with this effect:
//   Effect: "This attack does 50 more damage for each of your Benched Nidoking."
//   Attacks: Lovestrike, Lovestrike, Lovestrike
//
// Effect49 - 2 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon is Poisoned, this attack does 50 more damage."
//   Attacks: Venoshock, Venoshock
//
// Effect50 - 7 attacks with this effect:
//   Effect: "Flip a coin. If heads, discard a random Energy from your opponent's Active Pokémon."
//   Attacks: Crunch, Drill Run, Crunch, Energy Cutoff, Crunch, Crunch, Crunch
//
// Effect52 - 2 attacks with this effect:
//   Effect: "1 of your opponent's Pokémon is chosen at random 4 times. For each time a Pokémon was chosen, do 50 damage to it."
//   Attacks: Draco Meteor, Draco Meteor
//
// Effect53 - 15 attacks with this effect:
//   Effect: "Draw a card."
//   Attacks: Pay Day, Pay Day, Spike Draw, Unlock, Collect, Collect, Stumbling Draw, Stumbling Draw, Unlock, Unlock, Collect, Collect, Collect, Pay Day, Pay Day
//
// Effect54 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, discard a random card from your opponent's hand."
//   Attacks: Shadow Claw, Shadow Claw
//
// Effect56 - 3 attacks with this effect:
//   Effect: "Flip 2 coins. This attack does 30 damage for each heads."
//   Attacks: Dizzy Punch, Double Spin, Dual Chop
//
// Effect57 - 2 attacks with this effect:
//   Effect: "Choose 1 of your opponent's Pokémon's attacks and use it as this attack. If this Pokémon doesn't have the necessary Energy to use that attack, this attack does nothing."
//   Attacks: Copy Anything, Copy Anything
//
// Effect58 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, your opponent shuffles their Active Pokémon into their deck."
//   Attacks: Primal Wingbeat, Primal Wingbeat
//
// Effect59 - 2 attacks with this effect:
//   Effect: "This attack does 30 damage for each of your Benched Pokémon."
//   Attacks: Do the Wave, Do the Wave
//
// Effect60 - 3 attacks with this effect:
//   Effect: "Your opponent reveals their hand."
//   Attacks: Psy Report, Psy Report, Silent Wing
//
// Effect61 - 2 attacks with this effect:
//   Effect: "Take a [G] Energy from your Energy Zone and attach it to this Pokémon."
//   Attacks: Growth Spurt, Growth Spurt
//
// Effect62 - 5 attacks with this effect:
//   Effect: "Flip a coin for each Energy attached to this Pokémon. This attack does 50 damage for each heads."
//   Attacks: Powerful Bloom, Powerful Bloom, Powerful Bloom, Powerful Bloom, Powerful Bloom
//
// Effect64 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, this attack does 60 more damage."
//   Attacks: Rising Lunge, Zone Smash
//
// Effect65 - 2 attacks with this effect:
//   Effect: "Discard 2 [R] Energy from this Pokémon. This attack does 80 damage to 1 of your opponent's Pokémon."
//   Attacks: Volcanic Ash, Volcanic Ash
//
// Effect66 - 2 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon is Poisoned, this attack does 40 more damage."
//   Attacks: Venoshock, Venoshock
//
// Effect67 - 4 attacks with this effect:
//   Effect: "Discard a random Energy from among the Energy attached to all Pokémon (both yours and your opponent's)."
//   Attacks: Rampaging Whirlpool, Rampaging Whirlpool, Rampaging Whirlpool, Rampaging Whirlpool
//
// Effect70 - 3 attacks with this effect:
//   Effect: "This attack also does 20 damage to each of your opponent's Benched Pokémon."
//   Attacks: Gigashock, Blizzard, Blizzard
//
// Effect71 - 4 attacks with this effect:
//   Effect: "This attack does 40 damage to 1 of your opponent's Pokémon."
//   Attacks: Thunder Spear, Star Drop, Star Drop, Ice Blade
//
// Effect72 - 7 attacks with this effect:
//   Effect: "Choose 1 of your opponent's Active Pokémon's attacks and use it as this attack."
//   Attacks: Genome Hacking, Genome Hacking, Genome Hacking, Genome Hacking, Genome Hacking, Genome Hacking, Genome Hacking
//
// Effect73 - 5 attacks with this effect:
//   Effect: "This attack does 20 more damage for each of your opponent's Benched Pokémon."
//   Attacks: Mind Jack, Scattering Cyclone, Scattering Cyclone, Scattering Cyclone, Scattering Cyclone
//
// Effect74 - 2 attacks with this effect:
//   Effect: "Heal 20 damage from each of your Pokémon."
//   Attacks: Bloomshine, Bloomshine
//
// Effect75 - 4 attacks with this effect:
//   Effect: "During your opponent's next turn, this Pokémon takes -30 damage from attacks."
//   Attacks: Guard Press, Guard Press, Guard Press, Guard Press
//
// Effect76 - 7 attacks with this effect:
//   Effect: "If any of your Pokémon were Knocked Out by damage from an attack during your opponent's last turn, this attack does 60 more damage."
//   Attacks: Revenge, Revenge, Spiteful Dance, Spiteful Dance, Spiteful Dance, Revenge, Revenge
//
// Effect78 - 4 attacks with this effect:
//   Effect: "During your opponent's next turn, if the Defending Pokémon tries to use an attack, your opponent flips a coin. If tails, that attack doesn't happen."
//   Attacks: Smokescreen, Flash, Smoke Bomb, Sand Attack
//
// Effect82 - 6 attacks with this effect:
//   Effect: "Discard a random Energy from this Pokémon."
//   Attacks: Air Slash, Air Slash, Air Slash, Power Blast, Air Slash, Air Slash
//
// Effect84 - 17 attacks with this effect:
//   Effect: "Your opponent's Active Pokémon is now Confused."
//   Attacks: Entrancing Melody, Magical Delusion, Magical Delusion, Magical Delusion, Worry Seed, Dazzle Dance, Electronica, Tone-Deaf, Psybeam, Magical Delusion, Dazzle Dance, Magical Delusion, Dazzle Dance, Split Spiral Punch, Confounding Cologne, Confuse Ray, Electronica
//
// Effect85 - 2 attacks with this effect:
//   Effect: "Put 1 random Basic Pokémon from your deck onto your Bench."
//   Attacks: Call for Family, Call for Family
//
// Effect86 - 5 attacks with this effect:
//   Effect: "Flip a coin. If heads, this attack does 50 more damage."
//   Attacks: Flog, Flog, Rising Lunge, Stone Edge, X-Scissor
//
// Effect87 - 9 attacks with this effect:
//   Effect: "During your next turn, this Pokémon can't attack."
//   Attacks: Leafy Cyclone, Prismatic Laser, Giga Impact, Giga Impact, Giga Impact, Giga Impact, Scorching Breath, Giga Impact, Boundless Power
//
// Effect89 - 8 attacks with this effect:
//   Effect: "Your opponent's Active Pokémon is now Burned."
//   Attacks: Bursting Inferno, Searing Flame, Fire Fang, Fire Fang, Fire Fang, Searing Flame, Fire Fang, Fire Fang
//
// Effect90 - 5 attacks with this effect:
//   Effect: "Discard all [R] Energy from this Pokémon."
//   Attacks: Flare Blitz, Flare Blitz, Flare Blitz, Flare Blitz, Flare Blitz
//
// Effect91 - 17 attacks with this effect:
//   Effect: "Heal 20 damage from this Pokémon."
//   Attacks: Nap, Moonlight Gain, Moonlight Gain, Leech Seed, Spiral Drain, Spiral Drain, Sparkling Aria, Sparkling Aria, Sparkling Aria, Honey Snack, Sparkling Aria, Horn Leech, Life Sucker, Sparkling Aria, Bubble Drain, Nap, Leech Seed
//
// Effect92 - 5 attacks with this effect:
//   Effect: "This attack also does 30 damage to 1 of your opponent's Benched Pokémon."
//   Attacks: Aqua Jet, Aura Sphere, Aura Sphere, Aura Sphere, Aura Sphere
//
// Effect93 - 14 attacks with this effect:
//   Effect: "This attack also does 20 damage to 1 of your opponent's Benched Pokémon."
//   Attacks: Muddy Water, Jump Kick, Muddy Water, Spark, Pike, Jump Blues, Jump Blues, Piercing Spin, Jump Blues, Jump Blues, Sprinting Flare, Sprinting Flare, Sprinting Flare, Spark
//
// Effect95 - 7 attacks with this effect:
//   Effect: "Discard 3 [W] Energy from this Pokémon. This attack also does 20 damage to each of your opponent's Benched Pokémon."
//   Attacks: Dimensional Storm, Dimensional Storm, Dimensional Storm, Dimensional Storm, Dimensional Storm, Dimensional Storm, Dimensional Storm
//
// Effect96 - 6 attacks with this effect:
//   Effect: "Choose 2 of your Benched Pokémon. For each of those Pokémon, take a [W] Energy from your Energy Zone and attach it to that Pokémon."
//   Attacks: Oceanic Gift, Oceanic Gift, Oceanic Gift, Oceanic Gift, Oceanic Gift, Oceanic Gift
//
// Effect97 - 3 attacks with this effect:
//   Effect: "Discard a [L] Energy from this Pokémon."
//   Attacks: Thunder Blast, Thunder Blast, Thunder Blast
//
// Effect98 - 23 attacks with this effect:
//   Effect: "This Pokémon also does 10 damage to itself."
//   Attacks: Big Explosion, Blaze Tackle, Heat Tackle, Heat Tackle, Sol Breaker, Sol Breaker, Sol Breaker, Sol Breaker, Big Explosion, Reckless Charge, Take Down, Heat Tackle, Heat Tackle, Heat Tackle, Heat Tackle, Reckless Charge, Reckless Charge, Sol Breaker, Sol Breaker, Reckless Charge, Heat Tackle, Heat Tackle, Reckless Charge
//
// Effect99 - 7 attacks with this effect:
//   Effect: "Take a [L] Energy from your Energy Zone and attach it to this Pokémon."
//   Attacks: Charge, Plasma Hurricane, Plasma Hurricane, Plasma Hurricane, Plasma Hurricane, Plasma Hurricane, Plasma Hurricane
//
// Effect100 - 2 attacks with this effect:
//   Effect: "If this Pokémon has at least 2 extra [L] Energy attached, this attack does 80 more damage."
//   Attacks: Exciting Voltage, Exciting Voltage
//
// Effect102 - 5 attacks with this effect:
//   Effect: "If this Pokémon has a Pokémon Tool attached, this attack does 40 more damage."
//   Attacks: Sparking Gadget, Sparking Gadget, Sparking Gadget, Sparking Gadget, Sparking Gadget
//
// Effect103 - 2 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon has a Pokémon Tool attached, this attack does 30 more damage."
//   Attacks: Assault Laser, Assault Laser
//
// Effect106 - 2 attacks with this effect:
//   Effect: "You can use this attack only if you have Uxie and Azelf on your Bench. Discard all Energy from this Pokémon."
//   Attacks: Supreme Blast, Supreme Blast
//
// Effect107 - 3 attacks with this effect:
//   Effect: "This attack does 20 damage to 1 of your opponent's Pokémon."
//   Attacks: Psychic Arrow, Skill Dive, Skill Dive
//
// Effect108 - 2 attacks with this effect:
//   Effect: "Discard the top 3 cards of your deck."
//   Attacks: Mountain Swing, Mountain Swing
//
// Effect109 - 2 attacks with this effect:
//   Effect: "Flip 2 coins. This attack does 20 more damage for each heads."
//   Attacks: Acrobatics, Swing Around
//
// Effect111 - 7 attacks with this effect:
//   Effect: "Flip 2 coins. This attack does 20 damage for each heads."
//   Attacks: Double Scratch, 2-Step, Double Hit, Double Scratch, Double Scratch, Double Spin, 2-Step
//
// Effect112 - 5 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon has damage on it, this attack does 40 more damage."
//   Attacks: Scratching Nails, Scratching Nails, Scratching Nails, Scratching Nails, Scratching Nails
//
// Effect113 - 2 attacks with this effect:
//   Effect: "This attack does 10 damage to each of your opponent's Pokémon."
//   Attacks: Swirling Disaster, Swirling Disaster
//
// Effect115 - 2 attacks with this effect:
//   Effect: "Flip a coin for each Pokémon you have in play. This attack does 20 damage for each heads."
//   Attacks: Group Beatdown, Group Beatdown
//
// Effect117 - 2 attacks with this effect:
//   Effect: "If this Pokémon has a Pokémon Tool attached, this attack does 30 more damage."
//   Attacks: Metal Arms, Metal Arms
//
// Effect118 - 2 attacks with this effect:
//   Effect: "Flip a coin until you get tails. This attack does 30 more damage for each heads."
//   Attacks: Iron Head, Spiral Rush
//
// Effect120 - 7 attacks with this effect:
//   Effect: "Take 2 [M] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon."
//   Attacks: Metallic Turbo, Metallic Turbo, Metallic Turbo, Metallic Turbo, Metallic Turbo, Metallic Turbo, Metallic Turbo
//
// Effect121 - 7 attacks with this effect:
//   Effect: "Flip a coin until you get tails. This attack does 40 more damage for each heads."
//   Attacks: Licking Fury, Licking Fury, Licking Fury, Guillotine Rush, Guillotine Rush, Licking Fury, Licking Fury
//
// Effect122 - 6 attacks with this effect:
//   Effect: "Flip a coin. If heads, this attack does 20 more damage."
//   Attacks: Quick Attack, Quick Attack, Trip Over, Quick Blow, Quick Blow, Quick Blow
//
// Effect124 - 2 attacks with this effect:
//   Effect: "Flip 2 coins. This attack does 40 damage for each heads."
//   Attacks: Double Hit, Double Smash
//
// Effect125 - 2 attacks with this effect:
//   Effect: "Before doing damage, discard all Pokémon Tools from your opponent's Active Pokémon."
//   Attacks: Pluck, Slapping Knockdown
//
// Effect126 - 3 attacks with this effect:
//   Effect: "Halve your opponent's Active Pokémon's remaining HP, rounded down."
//   Attacks: Super Fang, Super Fang, Super Fang
//
// Effect128 - 5 attacks with this effect:
//   Effect: "Flip 3 coins. This attack does 20 damage for each heads."
//   Attacks: Fury Attack, Triple Slap, Triple Slap, Triple Slap, Triple Slap
//
// Effect130 - 3 attacks with this effect:
//   Effect: "This attack does more damage equal to the damage this Pokémon has on it."
//   Attacks: Raging Hammer, Raging Hammer, Raging Hammer
//
// Effect132 - 3 attacks with this effect:
//   Effect: "This Pokémon also does 30 damage to itself."
//   Attacks: Reckless Charge, Psychobilly, Volt Tackle
//
// Effect133 - 3 attacks with this effect:
//   Effect: "If this Pokémon has damage on it, this attack does 40 more damage."
//   Attacks: Ragin' Mad Strike, Ragin' Mad Strike, Ragin' Mad Strike
//
// Effect135 - 4 attacks with this effect:
//   Effect: "This attack also does 10 damage to 1 of your opponent's Benched Pokémon."
//   Attacks: Spark, Voltaic Bullet, Voltaic Bullet, Voltaic Bullet
//
// Effect137 - 4 attacks with this effect:
//   Effect: "Flip a coin. If heads, your opponent reveals a random card from their hand and shuffles it into their deck."
//   Attacks: Astonish, Astonish, Astonish, Astonish
//
// Effect138 - 3 attacks with this effect:
//   Effect: "This attack does 20 damage to 1 of your opponent's Benched Pokémon."
//   Attacks: Dash Attack, Zappy Shot, Wind Blast
//
// Effect139 - 4 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon is a Pokémon ex, this attack does 30 more damage."
//   Attacks: Fighting Headbutt, Fighting Headbutt, Fighting Headbutt, Fighting Headbutt
//
// Effect143 - 3 attacks with this effect:
//   Effect: "Flip a coin. If tails, during your next turn, this Pokémon can't attack."
//   Attacks: Time Mash, Crashing Fangs, Crashing Fangs
//
// Effect145 - 6 attacks with this effect:
//   Effect: "This Pokémon is now Asleep."
//   Attacks: Collapse, Flop-Down Punch, Flop-Down Punch, Flop-Down Punch, Flop-Down Punch, Collapse
//
// Effect146 - 8 attacks with this effect:
//   Effect: "This attack does 20 more damage for each of your Benched Pokémon."
//   Attacks: Ultimate Force, Ultimate Force, Ultimate Force, Ultimate Force, Ultimate Force, Ultimate Force, Do the Wave, Ultimate Force
//
// Effect147 - 4 attacks with this effect:
//   Effect: "Put 1 random Weedle from your deck onto your Bench."
//   Attacks: Multiply, Multiply, Multiply, Multiply
//
// Effect148 - 4 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon is a Pokémon ex, this attack does 70 more damage."
//   Attacks: Fighting Claws, Fighting Claws, Fighting Claws, Fighting Claws
//
// Effect149 - 4 attacks with this effect:
//   Effect: "Take 3 [R] Energy from your Energy Zone and attach it to this Pokémon."
//   Attacks: Stoke, Stoke, Stoke, Stoke
//
// Effect150 - 4 attacks with this effect:
//   Effect: "1 of your opponent's Pokémon is chosen at random. Do 30 damage to it."
//   Attacks: Spring Out, Spring Out, Spring Out, Spring Out
//
// Effect151 - 4 attacks with this effect:
//   Effect: "1 of your opponent's Pokémon is chosen at random 3 times. For each time a Pokémon was chosen, do 50 damage to it."
//   Attacks: Pop Out Throughout, Pop Out Throughout, Pop Out Throughout, Pop Out Throughout
//
// Effect152 - 5 attacks with this effect:
//   Effect: "Take a [L] Energy from your Energy Zone and attach it to 1 of your Benched [L] Pokémon."
//   Attacks: Plasma, Plasma, Plasma, Plasma, Plasma
//
// Effect154 - 2 attacks with this effect:
//   Effect: "Flip 4 coins. This attack does 20 damage for each heads."
//   Attacks: Juggling, Spike Cannon
//
// Effect155 - 3 attacks with this effect:
//   Effect: "If this Pokémon has at least 2 extra [F] Energy attached, this attack does 50 more damage."
//   Attacks: Power Press, Power Press, Power Press
//
// Effect156 - 5 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon is Poisoned, this attack does 60 more damage."
//   Attacks: Venoshock, Venoshock, Venoshock, Venoshock, Venoshock
//
// Effect158 - 4 attacks with this effect:
//   Effect: "Flip a coin. If heads, this attack does 80 more damage."
//   Attacks: Terrific Thumping, Terrific Thumping, Terrific Thumping, Terrific Thumping
//
// Effect159 - 2 attacks with this effect:
//   Effect: "Flip a coin for each [M] Energy attached to this Pokémon. This attack does 50 damage for each heads."
//   Attacks: Scintillating Surfing, Scintillating Surfing
//
// Effect160 - 4 attacks with this effect:
//   Effect: "During your next turn, this Pokémon's Overacceleration attack does +20 damage."
//   Attacks: Overacceleration, Overacceleration, Overacceleration, Overacceleration
//
// Effect161 - 5 attacks with this effect:
//   Effect: "This attack does 10 damage to 1 of your opponent's Pokémon."
//   Attacks: Skill Dive, Skill Dive, Skill Dive, Skill Dive, Water Arrow
//
// Effect162 - 5 attacks with this effect:
//   Effect: "This attack does 100 damage to 1 of your opponent's Pokémon that have damage on them."
//   Attacks: Pierce the Pain, Pierce the Pain, Pierce the Pain, Pierce the Pain, Pierce the Pain
//
// Effect163 - 2 attacks with this effect:
//   Effect: "This attack does 20 damage to each of your opponent's Pokémon."
//   Attacks: Petal Blizzard, Mental Surge
//
// Effect164 - 2 attacks with this effect:
//   Effect: "Flip 3 coins. This attack does 50 damage for each heads."
//   Attacks: Three Kick Combo, Three Kick Combo
//
// Effect166 - 2 attacks with this effect:
//   Effect: "Flip a coin. If tails, this Pokémon also does 20 damage to itself."
//   Attacks: Stuck-In Tackle, Stuck-In Tackle
//
// Effect167 - 2 attacks with this effect:
//   Effect: "Flip 2 coins. This attack does 70 damage for each heads. If at least 1 of them is heads, your opponent's Active Pokémon is now Burned."
//   Attacks: Burning Bonemerang, Burning Bonemerang
//
// Effect171 - 2 attacks with this effect:
//   Effect: "Take a [W] Energy from your Energy Zone and attach it to this Pokémon."
//   Attacks: Call Forth Cold, Call Forth Cold
//
// Effect172 - 4 attacks with this effect:
//   Effect: "During your next turn, this Pokémon's Insatiable Striking attack does +40 damage."
//   Attacks: Insatiable Striking, Insatiable Striking, Insatiable Striking, Insatiable Striking
//
// Effect173 - 3 attacks with this effect:
//   Effect: "Put 1 random Wishiwashi or Wishiwashi ex from your deck onto your Bench."
//   Attacks: Call for Family, Call for Family, Call for Family
//
// Effect174 - 4 attacks with this effect:
//   Effect: "This attack does 40 more damage for each of your Benched Wishiwashi and Wishiwashi ex."
//   Attacks: School Storm, School Storm, School Storm, School Storm
//
// Effect177 - 3 attacks with this effect:
//   Effect: "During your opponent's next turn, they can't play any Item cards from their hand."
//   Attacks: Disconnect, Jingly Noise, Booming Roar
//
// Effect178 - 2 attacks with this effect:
//   Effect: "Switch this Pokémon with 1 of your Benched [L] Pokémon."
//   Attacks: Volt Switch, Volt Switch
//
// Effect181 - 2 attacks with this effect:
//   Effect: "This attack also does 20 damage to 1 of your Pokémon."
//   Attacks: Shadow Hit, Shadow Hit
//
// Effect182 - 2 attacks with this effect:
//   Effect: "This attack does 20 damage to 1 of your opponent's Pokémon for each Energy attached to that Pokémon."
//   Attacks: Energy Arrow, Energy Arrow
//
// Effect183 - 3 attacks with this effect:
//   Effect: "During your opponent's next turn, this Pokémon takes -50 damage from attacks."
//   Attacks: Stiffen, Stiffen, Stiffen
//
// Effect184 - 3 attacks with this effect:
//   Effect: "Put a random card that evolves from Rockruff from your deck into your hand."
//   Attacks: Signs of Evolution, Signs of Evolution, Signs of Evolution
//
// Effect186 - 2 attacks with this effect:
//   Effect: "This Pokémon also does 40 damage to itself."
//   Attacks: High Horsepower, High Horsepower
//
// Effect188 - 4 attacks with this effect:
//   Effect: "1 Special Condition from among Asleep, Burned, Confused, Paralyzed, and Poisoned is chosen at random, and your opponent's Active Pokémon is now affected by that Special Condition. Any Special Conditions already affecting that Pokémon will not be chosen."
//   Attacks: Chemical Panic, Chemical Panic, Chemical Panic, Chemical Panic
//
// Effect190 - 2 attacks with this effect:
//   Effect: "Flip a coin until you get tails. This attack does 70 damage for each heads."
//   Attacks: Iron Head, Continuous Headbutt
//
// Effect191 - 4 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon has an Ability, this attack does 40 more damage."
//   Attacks: Silver Cannon, Silver Cannon, Silver Cannon, Silver Cannon
//
// Effect192 - 2 attacks with this effect:
//   Effect: "If any of your Benched Pokémon have damage on them, this attack does 50 more damage."
//   Attacks: Berserk, Berserk
//
// Effect194 - 3 attacks with this effect:
//   Effect: "Take a [C] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon."
//   Attacks: Energy Assist, Energy Assist, Energy Assist
//
// Effect196 - 2 attacks with this effect:
//   Effect: "This Pokémon is now Confused."
//   Attacks: Tantrum, Tantrum
//
// Effect198 - 2 attacks with this effect:
//   Effect: "Flip 3 coins. This attack does 10 damage for each heads."
//   Attacks: Fury Attack, Fury Attack
//
// Effect200 - 5 attacks with this effect:
//   Effect: "During your next turn, this Pokémon can't use Big Beat."
//   Attacks: Big Beat, Big Beat, Big Beat, Big Beat, Big Beat
//
// Effect201 - 3 attacks with this effect:
//   Effect: "This Pokémon also does 70 damage to itself."
//   Attacks: Beat Punk, Beat Punk, Beat Punk
//
// Effect202 - 5 attacks with this effect:
//   Effect: "Discard a [F] Energy from this Pokémon."
//   Attacks: Lycanfang, Lycanfang, Lycanfang, Lycanfang, Lycanfang
//
// Effect204 - 2 attacks with this effect:
//   Effect: "Discard a random Pokémon Tool card from your opponent's hand."
//   Attacks: Meddle, Meddle
//
// Effect206 - 7 attacks with this effect:
//   Effect: "Flip a coin until you get tails. For each heads, discard a random Energy from your opponent's Active Pokémon."
//   Attacks: Grindcore, Grindcore, Grindcore, Hyper Whirlpool, Hyper Whirlpool, Grindcore, Chomp Chomp Bite
//
// Effect207 - 5 attacks with this effect:
//   Effect: "Flip 3 coins. This attack does 60 damage for each heads."
//   Attacks: Triplet Headbutt, Triplet Headbutt, Triplet Headbutt, Triplet Headbutt, Triplet Headbutt
//
// Effect209 - 4 attacks with this effect:
//   Effect: "If you played a Supporter card from your hand during this turn, this attack does 50 more damage."
//   Attacks: Brave Buddies, Brave Buddies, Brave Buddies, Brave Buddies
//
// Effect210 - 2 attacks with this effect:
//   Effect: "This attack does 20 more damage for each [G] Energy attached to this Pokémon."
//   Attacks: Leaf Blast, Leaf Blast
//
// Effect213 - 5 attacks with this effect:
//   Effect: "Take a [R] Energy from your Energy Zone and attach it to 1 of your Benched Pokémon."
//   Attacks: Assisting Heater, Assisting Heater, Assisting Heater, Assisting Heater, Assisting Heater
//
// Effect214 - 12 attacks with this effect:
//   Effect: "If 1 of your Pokémon used Sweets Relay during your last turn, this attack does 20 more damage."
//   Attacks: Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay
//
// Effect215 - 5 attacks with this effect:
//   Effect: "If this Pokémon has at least 1 extra [W] Energy attached, this attack does 40 more damage."
//   Attacks: Hydro Pump, Hydro Pump, Hydro Pump, Hydro Pump, Hydro Pump
//
// Effect216 - 4 attacks with this effect:
//   Effect: "If this Pokémon evolved during this turn, this attack does 20 more damage."
//   Attacks: Beginning Bolt, Beginning Bolt, Beginning Bolt, Beginning Bolt
//
// Effect218 - 2 attacks with this effect:
//   Effect: "This attack does 20 damage for each Energy attached to all of your opponent's Pokémon."
//   Attacks: Energy Crush, Energy Crush
//
// Effect219 - 4 attacks with this effect:
//   Effect: "If 1 of your Pokémon used Sweets Relay during your last turn, this attack does 60 more damage."
//   Attacks: Sweets Relay, Sweets Relay, Sweets Relay, Sweets Relay
//
// Effect220 - 2 attacks with this effect:
//   Effect: "This attack does 30 more damage for each Evolution Pokémon on your Bench."
//   Attacks: Evoharmony, Evoharmony
//
// Effect221 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, choose 1 of your opponent's Active Pokémon's attacks and use it as this attack."
//   Attacks: Try to Imitate, Try to Imitate
//
// Effect222 - 4 attacks with this effect:
//   Effect: "This attack does 40 damage for each time your Pokémon used Sweets Relay during this game."
//   Attacks: Sweets Overload, Sweets Overload, Sweets Overload, Sweets Overload
//
// Effect224 - 4 attacks with this effect:
//   Effect: "If the Defending Pokémon is a Basic Pokémon, it can't attack during your opponent's next turn."
//   Attacks: Dark Binding, Dark Binding, Dark Binding, Dark Binding
//
// Effect226 - 2 attacks with this effect:
//   Effect: "During your opponent's next turn, if this Pokémon is damaged by an attack, do 30 damage to the Attacking Pokémon."
//   Attacks: Bristling Spikes, Bristling Spikes
//
// Effect229 - 7 attacks with this effect:
//   Effect: "If this Pokémon has a Pokémon Tool attached, this attack does 50 more damage."
//   Attacks: Enhanced Fang, Enhanced Horns, Enhanced Horns, Metal Arms, Metal Arms, Metal Arms, Enhanced Fang
//
// Effect230 - 2 attacks with this effect:
//   Effect: "Flip 3 coins. This attack does 60 damage for each heads. This Pokémon is now Confused."
//   Attacks: Petal Dance, Petal Dance
//
// Effect232 - 2 attacks with this effect:
//   Effect: "If this Pokémon has no damage on it, this attack does 40 more damage."
//   Attacks: Single Lunge, Single Lunge
//
// Effect234 - 2 attacks with this effect:
//   Effect: "Take a [R] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon."
//   Attacks: Toasty Toss, Toasty Toss
//
// Effect236 - 6 attacks with this effect:
//   Effect: "Take a [R], [W], and [L] Energy from your Energy Zone and attach them to your Benched Basic Pokémon in any way you like."
//   Attacks: Phoenix Turbo, Phoenix Turbo, Phoenix Turbo, Phoenix Turbo, Phoenix Turbo, Phoenix Turbo
//
// Effect238 - 2 attacks with this effect:
//   Effect: "You may discard any number of your Benched [W] Pokémon. This attack does 40 more damage for each Benched Pokémon you discarded in this way."
//   Attacks: Wild Swing, Wild Swing
//
// Effect239 - 5 attacks with this effect:
//   Effect: "Put a random Pokémon from your deck into your hand."
//   Attacks: Find a Friend, Twinkly Call, Find a Friend, Find a Friend, Twinkly Call
//
// Effect240 - 2 attacks with this effect:
//   Effect: "If the Defending Pokémon tries to use an attack, your opponent flips a coin. If tails, that attack doesn't happen. This effect lasts until the Defending Pokémon leaves the Active Spot, and it doesn't stack."
//   Attacks: Octazooka, Octazooka
//
// Effect242 - 4 attacks with this effect:
//   Effect: "Flip a coin. If heads, your opponent's Active Pokémon is now Paralyzed. If tails, your opponent's Active Pokémon is now Confused."
//   Attacks: Flashing Signal, Flashing Signal, Flashing Signal, Flashing Signal
//
// Effect243 - 2 attacks with this effect:
//   Effect: "Take a [L] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon."
//   Attacks: Crackly Toss, Crackly Toss
//
// Effect247 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, your opponent's Active Pokémon's remaining HP is now 10."
//   Attacks: Life Drain, Life Drain
//
// Effect248 - 2 attacks with this effect:
//   Effect: "If this Pokémon was damaged by an attack during your opponent's last turn while it was in the Active Spot, this attack does 50 more damage."
//   Attacks: Reply Strongly, Reply Strongly
//
// Effect251 - 4 attacks with this effect:
//   Effect: "If this Pokémon has at least 2 extra [F] Energy attached, this attack does 60 more damage."
//   Attacks: Gigantic Press, Gigantic Press, Gigantic Press, Gigantic Press
//
// Effect252 - 3 attacks with this effect:
//   Effect: "Discard a random card from your opponent's hand."
//   Attacks: Diving Swipe, Nipping Cyclone, Diving Swipe
//
// Effect253 - 2 attacks with this effect:
//   Effect: "If this Pokémon moved from your Bench to the Active Spot this turn, this attack does 50 more damage."
//   Attacks: Gale Thrust, Gale Thrust
//
// Effect256 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, during your opponent's next turn, prevent all damage done to this Pokémon by attacks."
//   Attacks: Scrunch, Iron Defense
//
// Effect257 - 3 attacks with this effect:
//   Effect: "This attack does 20 more damage for each Energy attached to this Pokémon."
//   Attacks: Energy Blow, Energy Blow, Balloon Barrage
//
// Effect258 - 2 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon is an Evolution Pokémon, this attack does 40 more damage."
//   Attacks: Cross-Cut, Cross-Cut
//
// Effect262 - 2 attacks with this effect:
//   Effect: "Change the type of a random Energy attached to your opponent's Active Pokémon to 1 of the following at random: [G], [R], [W], [L], [P], [F], [D], or [M]."
//   Attacks: Splatter Coating, Splatter Coating
//
// Effect263 - 6 attacks with this effect:
//   Effect: "Discard a [R], [W], and [L] Energy from this Pokémon."
//   Attacks: Elemental Blast, Elemental Blast, Elemental Blast, Elemental Blast, Elemental Blast, Elemental Blast
//
// Effect264 - 3 attacks with this effect:
//   Effect: "You may switch this Pokémon with 1 of your Benched Pokémon."
//   Attacks: Breeze-By Attack, Breeze-By Attack, Breeze-By Attack
//
// Effect266 - 2 attacks with this effect:
//   Effect: "Discard the top card of your opponent's deck."
//   Attacks: Mountain Munch, Hammering Tail
//
// Effect267 - 4 attacks with this effect:
//   Effect: "If this Pokémon has at least 2 extra [R] Energy attached, this attack does 60 more damage."
//   Attacks: Blazing Beatdown, Blazing Beatdown, Blazing Beatdown, Blazing Beatdown
//
// Effect268 - 2 attacks with this effect:
//   Effect: "Put 1 random Poliwag from your deck onto your Bench."
//   Attacks: Call for Family, Call for Family
//
// Effect269 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, your opponent's Active Pokémon is now Poisoned and Paralyzed."
//   Attacks: Stun Poison, Stun Poison
//
// Effect271 - 3 attacks with this effect:
//   Effect: "This attack does 20 damage for each Benched Pokémon (both yours and your opponent's)."
//   Attacks: Crystal Waltz, Crystal Waltz, Crystal Waltz
//
// Effect272 - 2 attacks with this effect:
//   Effect: "Take a [W] Energy from your Energy Zone and attach it to 1 of your Benched Basic Pokémon."
//   Attacks: Splashy Toss, Splashy Toss
//
// Effect275 - 2 attacks with this effect:
//   Effect: "If Latios is on your Bench, this attack does 20 more damage."
//   Attacks: Crossing Flights, Crossing Flights
//
// Effect277 - 3 attacks with this effect:
//   Effect: "If this Pokémon has any [W] Energy attached, this attack does 40 more damage."
//   Attacks: Hydro Knuckle, Hydro Knuckle, Hydro Knuckle
//
// Effect278 - 3 attacks with this effect:
//   Effect: "This attack does damage to your opponent's Active Pokémon equal to the damage this Pokémon has on it."
//   Attacks: Flail, Flail, Flail
//
// Effect282 - 2 attacks with this effect:
//   Effect: "Until this Pokémon leaves the Active Spot, this Pokémon's Rolling Frenzy attack does +30 damage. This effect stacks."
//   Attacks: Rolling Frenzy, Rolling Frenzy
//
// Effect283 - 2 attacks with this effect:
//   Effect: "Heal 50 damage from 1 of your Benched Pokémon."
//   Attacks: Squishy Healing, Squishy Healing
//
// Effect284 - 3 attacks with this effect:
//   Effect: "Flip a coin. If heads, this attack does 70 more damage."
//   Attacks: Critical Scissors, Critical Scissors, Critical Scissors
//
// Effect285 - 2 attacks with this effect:
//   Effect: "During your opponent's next turn, prevent all damage done to this Pokémon by attacks if that damage is 40 or less."
//   Attacks: Harden, Harden
//
// Effect287 - 3 attacks with this effect:
//   Effect: "Flip 3 coins. This attack does 40 damage for each heads."
//   Attacks: Fury Swipes, Fury Swipes, Fury Swipes
//
// Effect288 - 3 attacks with this effect:
//   Effect: "This attack does 30 more damage for each Energy in your opponent's Active Pokémon's Retreat Cost."
//   Attacks: Grass Knot, Grass Knot, Grass Knot
//
// Effect289 - 5 attacks with this effect:
//   Effect: "During your next turn, this Pokémon can't use Sacred Sword."
//   Attacks: Sacred Sword, Sacred Sword, Sacred Sword, Sacred Sword, Sacred Sword
//
// Effect290 - 2 attacks with this effect:
//   Effect: "Flip a coin. If heads, your opponent's Active Pokémon is now Burned."
//   Attacks: Fire Fang, Beak Blast
//
// Effect292 - 3 attacks with this effect:
//   Effect: "Discard Fire[R] Energy from this Pokémon. Your opponent's Active Pokémon is now Burned."
//   Attacks: Mega Burning, Mega Burning, Mega Burning
//
// Effect295 - 2 attacks with this effect:
//   Effect: "Put a random card from your deck that evolves from this Pokémon onto this Pokémon to evolve it."
//   Attacks: Waterfall Evolution, Waterfall Evolution
//
// Effect296 - 3 attacks with this effect:
//   Effect: "Discard the top 3 cards of your opponent's deck."
//   Attacks: Mega Blaster, Mega Blaster, Mega Blaster
//
// Effect297 - 2 attacks with this effect:
//   Effect: "If you have exactly 1, 3, or 5 cards in your hand, this attack does 60 more damage."
//   Attacks: Rhythmic Steps, Rhythmic Steps
//
// Effect302 - 3 attacks with this effect:
//   Effect: "1 of your opponent's Benched Pokémon is chosen at random 3 times. For each time a Pokémon was chosen, also do 20 damage to it."
//   Attacks: Lightning Lancer, Lightning Lancer, Lightning Lancer
//
// Effect303 - 2 attacks with this effect:
//   Effect: "This attack also does 20 damage to 1 of your Benched Pokémon."
//   Attacks: Flash Impact, Flash Impact
//
// Effect307 - 3 attacks with this effect:
//   Effect: "This attack does 30 more damage for each of your Benched Pokémon."
//   Attacks: Mega Harmony, Mega Harmony, Mega Harmony
//
// Effect308 - 2 attacks with this effect:
//   Effect: "Discard 2 cards from your hand. If you can't discard 2 cards, this attack does nothing."
//   Attacks: Soul Shot, Soul Shot
//
// Effect310 - 3 attacks with this effect:
//   Effect: "This attack's damage isn't affected by Weakness."
//   Attacks: Quick Straight, Quick Straight, Quick Straight
//
// Effect312 - 2 attacks with this effect:
//   Effect: "This attack does 20 damage to each of your opponent's Pokémon. During your next turn, this Pokémon's Wild Spin attack does +20 damage to each of your opponent's Pokémon."
//   Attacks: Wild Spin, Wild Spin
//
// Effect316 - 2 attacks with this effect:
//   Effect: "This attack does 40 more damage for each of your opponent's Pokémon in play that has an Ability."
//   Attacks: Evil Admonition, Evil Admonition
//
// Effect318 - 3 attacks with this effect:
//   Effect: "Your opponent reveals their hand. Choose a Supporter card you find there and discard it."
//   Attacks: Darkness Claw, Darkness Claw, Darkness Claw
//
// Effect323 - 2 attacks with this effect:
//   Effect: "Discard 2 [M] Energy from this Pokémon. During your opponent's next turn, this Pokémon takes -50 damage from attacks."
//   Attacks: Iron Wings, Iron Wings
//
// Effect327 - 3 attacks with this effect:
//   Effect: "Flip a coin. If tails, this Pokémon also does 30 damage to itself."
//   Attacks: Wild Tackle, Wild Tackle, Wild Tackle
//
// Effect330 - 2 attacks with this effect:
//   Effect: "If your opponent's Active Pokémon is a Basic Pokémon, this attack does 70 more damage."
//   Attacks: Dangerous Bite, Dangerous Bite
//
// Effect332 - 2 attacks with this effect:
//   Effect: "1 of your opponent's Pokémon is chosen at random 4 times. For each time a Pokémon was chosen, do 40 damage to it."
//   Attacks: Draco Meteor, Draco Meteor
//
