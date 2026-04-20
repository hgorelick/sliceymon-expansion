# Undocumented Textmod Guide

# Slice & Dice Undocumented Textmod Guide (v3.2)

#### More Slice & Dice resources can be found at the official Discord server:

https://discord.gg/TqUdVPSWDt

Or linked in-game:

https://tann.fun/games/dice/textmod/

## What is Textmod?

Textmod offers an incredibly diverse range of APIs that can be used to edit existing modifiers, heroes, and items to create almost anything. Many of the available templates in Textmod are fairly intuitive and come with a template and a myriad of randomly generated examples.

I encourage you to look through the API list and experiment with all of the options it provides. This guide will assume you are comfortable with basic Textmod APIs, including the +more option. There are a few more confusing APIs available that seem to lack enough explanation to use them properly, however.

This guide will be split into 7 distinct sections to cover each group of tools: Choosable tags, Phases, Hidden Items & Modifiers, Tog items, Togres and its variants, AbilityData, and TriggerHPData. Each of these is mentioned by the API menu but has advanced usage that is not covered in-game. Within these sections, many different concepts are covered, and each will be accompanied with (hopefully) useful examples.

If you discover any inaccuracies in this guide or have any comments, suggestions, or ideas, feel free to message me on Discord (thun.der). The official Slice & Dice discord server, linked above, is the best place to find me.

To navigate this document, you must switch the tab that is currently selected. On PC, this should show up as a table of contents on the left. Clicking the tab again will show the sections within that tab. On mobile, tabs are switched at the bottom of the screen.

# Choosables & SCPhase

## Choosables and SimpleChoicePhase

The first phase, the SimpleChoicePhase, can be used quite similarly to Choosables. Both are categorized as Modifiers on the API page.  The main difference between the two is that SCPhase allows you to decide whether an option should be given directly or picked by the player from a list, while Choosables will only let you grant rewards directly. SCPhase will also allow lvl.ph, while lvl.ch does not work (by default every phase modifier activates every floor). SCPhase rewards will also pop-up with a screen, while Choosable rewards are directly granted with no pop-up. If possible, Choosables are the more efficient choice, but SCPhase is more flexible.

SCPhase and Choosables both rely on Tags in their syntax to designate which type of reward they are going to give. Some of these are simple, while others have restrictions on how the data is input. The skip tag needs nothing after it.

| Choosable | SCPhase | Tag Type | Syntax |
|---|---|---|---|
| ch.m | ph.!m | Modifier | Standard |
| ch.i | ph.!i | Item | Standard |
| ch.l | ph.!l | Levelup | Standard |
| ch.g | ph.!g | Hero | Standard |
| ch.r | ph.!r | Random | Input |
| ch.q | ph.!q | RandomRange | Input |
| ch.o | ph.!o | Or | Input |
| ch.e | ph.!e | Enu | Three |
| ch.v | ph.!v | Value | Unique |
| ch.p | ph.!p | Replace | Unique |
| ch.s | ph.!s | Skip | None |

Standard Tags simply need an entity from their type listed afterwards to grant that reward.

Input Tags manage how the reward is granted and are therefore reliant on the other tags to classify what type of reward is given.

Enu has three valid inputs and grants a random item based on which one is given.

Value has unique syntax and increases an initially useless hidden variable of the creator's choice. Replace will replace a chosen modifier with a different reward.

Skip has no syntax.

### Standard Syntax Tags

Modifiers, Items, Levelups, and Hero additions are all extremely similar in Choosables and the SCPhase. All one needs to do is use the tag letter as an indicator of the type of reward, then type its name. A few examples:

ch.mBone Math: Adds the Bone Math Modifier.

ph.!iMana Jelly: Grants the player a Mana Jelly.

ch.lDabbler: Levels up an eligible hero to Dabbler.

ph.!gRuffian: Adds a Ruffian to the party.

The difference between the l and g tags is that the l Levelup tag levels an existing hero while the g Hero tag adds a new one. Additionally, ph.!gRuffian is functionally identical to ph.!madd.Ruffian. If the Levelup tag has no eligible target, it will default to the topmost hero.

These Standard tags can also be used to add rewards that have been created with Textmod. A few more examples:

ch.m4.fight.Bramble+Rat

2.ph.!ileft2.Longsword.tier.3.n.Medium Sword

ch.lHerbalist.hp.12.i.hat.Mystic.img.Druid.hue.-10.n.Herbufflist

8.ph.!gMimic.i.self.Mortal^3#bot.Wand of Stun.hue.30.n.Might Mimic

ch.mch.mch.mch.mWurst

While these modifiers, particularly the choosables shown here, might not seem much more useful than simply starting with the reward, they become more interesting combined with the other Tags to create randomly generated rewards or checking a Value to see if a player has "earned" a reward.

### Random and RandomRange Tags

Unlike the Tags in the previous section, the Random tags will only pick from Designed entities in the base game unless they have no remaining ones to choose from. These tags instead rely on other tags to decide which pool the reward is being picked from.  ch.q, the RandomRange tag Choosable, is shown in the API-2 menu. As the number of rewards granted can be chosen, ensure you don't make the number too large and crash your game.

The syntax for the Random tag is as follows: ch.r1~2~m, where 1 is the tier of the reward generated, 2 is the number generated, and m is the tag generated. This Choosable in particular would grant 2 tier 1 modifiers.

A few examples:

ph.!r-20~2~m : Phase where 2 random tier -20 Modifiers are granted.

ch.r2~1~l  : Grants 1 random Levelup for your lowest tier hero. The amount and tier cannot be changed for random Levelups.

ph.!r10~1~g : Grants 1 random tier 10 Hero. The amount cannot be changed for random Heroes either.

ch.r13~2~i : Grants 2 tier 13 Items.

The syntax for the RandomRange tag is very similar: in ch.q1~2~3~m, there are 3 rewards generated in either tier 1 or 2, and the tag type generated is m. In this case, 3 tier 1 or 2 modifiers are granted to the player. A tier is selected in the range and every reward granted will be in the selected tier.

ph.!q-20~20~3~m : Grants 3 modifiers of one tier between -20 and 20.

ch.q2~4~1~l : Grants 1 random Levelup for your lowest tier hero. The amount and tier cannot be changed for random Levelups.

ph.!q1~3~4~g : grants 1 random hero of the same tier between 1 and 3 and the same herocol.

ch.q4~8~6~i : grants 6 items of a random tier between 4 and 8.

### Or Tag

This tag is probably the most useful one for creating Custom modes where you would like randomness between runs. The Or Tag is a bit different from the Random tags; rather than choosing a random reward from a tier or range of tiers, it will allow you to create a list and allow the game to choose a random reward from that list. You can even alternate tags in the middle of the list.

Between each entry in an Or tag and other similar APIs, there's a unique delimiter to avoid collisions. In this case, it's @4, although in other scenarios it may be @1, @2, @3, a ; , or something else entirely.

Here are a few examples:

ch.omadd.Bones@4mWurst will choose between add.bones and Wurst.

ph.!olDabbler@4iAnchor will either level a hero to Dabbler or grant the player an Anchor.

ch.om4.fight.Bramble+Rat.mn.Bramble@4m4.fight.Troll.mn.Troll@4m4.fight.Wolf+Alpha.mn.Alpha@4m(4.fight.Sarcophagus.hp.5.mn.Lucky!) Randomizes the Floor 4 fight between 4 choices.

2.ph.!olKnight@4lMonk@4lArmorer grants one of the listed tier 2 Grays as a levelup on floor 2.

ch.om(2.ph.!lKnight)@4m(2.ph.!lMonk)@4m(2.ph.!lArmorer) also grants one of the listed tier 2 Grays as a levelup on floor 2.

As is shown by the above two modifiers, it's possible to replicate SCPhase's ability to be placed on any floor by simply making an Or Choosable grant a random SCPhase with one choice. Using parentheses () to bracket the modifiers can be useful to avoid collisions in situations like these. The main difference between these options is that the player will know in advance which reward they'll be getting later if you use the Choosable, and also that just using SCPhase is going to be shorter and simpler.

### Enu Tag

The Enu Tag is used as a reward type but has only three specific inputs that it will accept: RandoKeywordT1Item, RandoKeywordT5Item, and RandoKeywordT7Item. This is basically just the backend for the "random keyword on X sides" items that may be randomly offered in item rewards.

RandoKeywordT1Item offers a random keyword item for the rightmost side, for example, rightmost.k.engage.

RandoKeywordT5Item offers a random keyword item for either the left side, top and bottom sides, or the right 3 sides, for example, left.k.pair, topbot.k.fizz, or right3.k.focus.

RandoKeywordT7Item offers a random keyword item for all sides, for example, all.k.ego.

A few examples of Enu Tag:

ch.eRandoKeywordT7Item will start the run with a random keyword item for all sides.

e4.ph.!eRandoKeywordT5Item will grant a T5 random keyword item on every 4th floor.

### Value Tag

The Value Tag seems useless at first but is granted versatility by the Boolean Phases, which will be explained in greater depth in their section later in the guide. You may recognize its use either as a hidden Seed value or as a Gold mechanic, but in actuality you can name the variable almost anything and use it as a requirement for any phase later. The syntax is v(variable name)V(amount added). The second V is not a true Tag and actually just separates the value name from the amount being added, but I will color it as a tag for visual purposes.

Here are a few examples of the Value Tag being used to add or subtract from a variable:

ch.vGoldV50 will add 50 Gold.

1-20.ph.!vLevelV1 will add 1 to Level every floor.

ch.ovSeedV1@4vSeedV2@4vSeedV3@4vSeedV4@4vSeedV5 uses an Or Tag to choose a random Seed value at the start between 1 and 5.

The Value can also be viewed later through Phases by using [val(variable name)] in any phase where the text would be visible. MessagePhase makes this simplest with ph.4[valgold] sending a message every floor that states only the amount of gold for that run.

You can also put the gold value in a doc. for an item or modifier. Example: void.doc.[valgold].n.Gold Counter

### Replace Tag

This tag works strangely for many inputs. It serves as the backend for "upgrading" curses in Cursed modes, for example picking Heavy Dice^3 after Heavy Dice^4. As  3.0, the old curse is simply replaced instead of being crossed out on the list. Therefore, the Replace Tag will only work for replacing Modifiers, but can be used to grant any Standard tag reward as a result.

The syntax for this tag is ph.!pm(modifier)~(reward), where the reward is being added and the (modifier) will be removed. Heroes, Items, and others cannot be replaced, only modifiers. It also doesn't check if you actually have the modifier you're trying to replace, but will remove it if you have it. In that sense, if the player starts with a powerful modifier, you can give them the option to replace it with another strong reward later instead of simply giving them both.

A few examples:

 ph.!pmWurst~gPaladin will remove the Wurst modifier and add a Paladin to the party.

Note that in the case where the player does not have Wurst, ph.!gPaladin would have the same effect.

ph.!pmFizzing~iCharged Skull will grant Charged Skull in exchange for losing Fizzing.

3.ph.!pm4.Skip Rewards~mBossilisk removes 4.Skip Rewards, allowing the player to get the reward on that floor, in exchange for making the fight more difficult.

### Skip Tag & SCPhase Choices

The Skip Tag exists for creating an option in a choice or random reward where nothing is granted. As such, it will probably find the most use in SCPhases or other ChoicePhases where the player has control over which options they are selecting.

SCPhase has the ability to create a phase where the player has the choice to select a reward rather than picking one randomly. ChoicePhase can also do this with several templates that will be reviewed on its page. The delimiter used here is @3.

ph.!lSoldier@3lDruid@3r2~1~l@3s : This one might look familiar. Hero levelups, simple Modifier selections, and Item reward screens are all created with the SCPhase syntax, so it's easy to replicate and make custom ones. They can also be combined, but more complicated reward screens like PointBuy will need ChoicePhase instead.

ph.!iSapphire@3iWandify@3r2~1~i grants an item reward screen.

ph.!mWurst@3mTraining@3r-1~1~m grants a curse choice screen.

ph.!lTrapper@3mFizzing@3r2~1~i@3s grants a unique choice screen.

ph.!mWurst@3iAnchor@3lPilgrim@3go0.294@3s@3eRandoKeywordT1Item@3q0~2~1~i@3r2~1~l@3vgoldV50@3vThunderScoreV1@3pmTraining~mDouble Monsters@3oiAmnesia@4oiEye of Horus : This is a phase that contains every Tag in the game available as a choice. Note that or uses @4 as a delimiter while ph.! uses @3.

The phase icon chosen as well as the format of the choice screen will be entirely decided by the Tag of the first reward listed.

Placing a semicolon in the syntax can allow for a "title" to be given to an SCPhase. Example: ph.!Example Title;iCorset@3iBallet Shoes@3s

# Phases

## Phases

The Phase Textmod API  ph.phase can be similarly difficult to approach. There are several different cases for phase, and the formatting on them can be confusing without prior knowledge. Below is a comprehensive list containing each phase.

| Code | Phase |
|---|---|
| ph.! | SimpleChoicePhase |
| ph.0 | PlayerRollingPhase |
| ph.1 | TargetingPhase |
| ph.2 | LevelEndPhase |
| ph.3 | EnemyRollingPhase |
| ph.4 | MessagePhase |
| ph.5 | HeroChangePhase |
| ph.6 | ResetPhase |
| ph.7 | ItemCombinePhase |
| ph.8 | PositionSwapPhase |
| ph.9 | ChallengePhase |
| ph.b | BooleanPhase |
| ph.c | ChoicePhase |
| ph.d | DamagePhase |
| ph.e | RunEndPhase |
| ph.l | LinkedPhase |
| ph.r | RandomRevealPhase |
| ph.s | SeqPhase |
| ph.t | TradePhase |
| ph.g | PhaseGeneratorTransformPhase |
| ph.z | BooleanPhase2 |

### phi. & phmp.

Under A and B in the modifiers section are phi.# and phmp.+-. These two TextMod APIs are short for "Phase Indexed" and "Phase Mod Pick" and can be used to simulate phases without any syntax.

phmp.+- uses an integer in place of +- to create a modifier selection screen where the total you need to reach is +-. It's very similar to the screen at the start of some difficulty modes. phmp.-10 is much like an unfair modifier pick screen.

phi.# will instead accept a number 1-9 and generate a phase every floor based on that number. It can be capped to certain levels with lvl.phi.# or lvl-lvl.phi.#.

| Code | Phase |
|---|---|
| phi.0 | Levelup Phase |
| phi.1 | Standard Loot Phase |
| phi.2 | Reroll Phase |
| phi.3 | Reroll Phase |
| phi.4 | Optional Tweak phase |
| phi.5 | Hero Position Swap phase |
| phi.6 | Standard Challenge phase |
| phi.7 | Easy Challenge phase |
| phi.8 | Hero Position Swap phase |
| phi.9 | Trade phase (cursed chest) |

Some examples:

e2.phi.1 will include an item reward before every second level.

8.phi.7 will generate an easy challenge on level 8.

phi.9 will put a Trade phase cursed chest on every floor.

e2.1.phi.0 will put a Levelup phase before every odd floor.

### PlayerRollingPhase, TargetingPhase, DamagePhase and EnemyRollingPhase

These four Phases represent the common action phases taken during enemy combat. All four will accept two numbers afterwards separated by a semicolon, for example ph.01;2. As far as I can tell this does nothing.

They aren't very useful as modifiers, but there can be some strange results such as events occuring in the wrong order depending on what you start with. Let me know if you do anything cool with these!

ph.0 indicates the PlayerRollingPhase, which is when the player is rerolling their dice. Strangely, If you simply use ph.0, the dice will roll, but if you use ph.01;2 for example they will already have random sides facing up. If used as the start, you will have 0 rerolls, and EnemyRollingPhase and TargetingPhase will be next.

ph.1 is for the TargetingPhase, which is when the player uses hero dice and abilities against enemies. If used as the start, random dice sides will be greyed-out and face-up for use against the enemies. If you kill any enemies, the game will crash after selecting End Turn. EnemyRollingPhase and DamagePhase will be next.

ph.3 is for the EnemyRollingPhase. This one is already at the start, so using it as a modifier has no discernable effect.

ph.d is for the DamagePhase. This is where the enemies attack their targets. At the start, the enemies have no targets at all, so this one is skipped and does nothing. Note that the pseudo-phase SurrenderPhase also takes place here. If heroes have sufficient HP, using ph.d will make enemies flee immediately.

### MessagePhase

The MessagePhase, ph.4, simply sends a message with custom contents. It can be used to relay instructions for a custom mode, information about modifiers, show current Values, or even load images. There is a button that reads "ok" by default to dismiss the message, but the button's text can be changed. The syntax is simply  ph.4(message);(buttontext).

ph.4Hello World sends a message reading "Hello World".

ph.4You currently have [valgold] gold. will, if the Value gold has previously been set, tell the player how much gold they have.

ph.4Never gonna give you up;Never gonna let you down is a MessagePhase with custom text for the button.

Different colors can be used for the text, with a number of different colors in brackets changing the color. Images for entities or custom images will also work, for example [Thief].

ph.4[orange]Thief[Thief][n][yellow]Fighter[Fighter][cu][cu]The Heroes.

1.add.(vase.((2.x3.add.rat)&(ph.4The [rat][rat][rat]s are coming...;oh no))).i.t.Goblin.n.Rat Vase.hsv.0:-99:-40 adds a "Rat Vase" which, on death, grants a modifier and a MessagePhase containing flavor text about the modifier's effect.

ph.4[3c29ff2a992b9f2caa2dvp2fd92dcc2sf03tjb63ub3m3mnb25xb7ib72Y4567XYYnopDDEdda4wb8wgb8hMxttkb4wkb4Mlub3mmob1ob0wpbqbr9esssfMvoM9ssferbqbpMb0owb0ob2mnuub4kwb4kb6tjxwgwb8gddc] creates a MessagePhase with a custom image inserted.

### HeroChangePhase

HeroChangePhase, ph.5, is the phase that presents you with the option to reroll a hero into either a different class or into a generated one. The syntax for this phase is quite simple.

I'll use ph.501 as an example: the first number, 5, shows that this is a HeroChangePhase. The second, 0, selects the hero starting from the top. This list starts from 0 and will typically end with 4 being the 5th hero. The last one, 1, chooses the type of phase. 0 will indicate "random class" and 1 will be "generated hero".

ph.520: reroll the class of the third hero

ph.501: replace the first hero with a generated one

ch.om4.ph.500@4m4.ph.510@4m4.ph.520

@4m4.ph.530@4m4.ph.540: grants a modifier that will reroll the class of one of the heroes in positions 0-4 on level 4.

### ResetPhase

ResetPhase, ph.6, will cause the Cursed mode reset screen to pop up, de-leveling all heroes to level 1 and removing all of your non-modifier items. It can work to reset a team after every level but might be useful in other applications, for example, in combination with vase. to create a unique loss condition after a specific enemy dies.

In version 3.1, custom Cursed modes were added, allowing ResetPhase to be used as intended in Cursed mode. Utilize this with the Add Fight modifiers and Cursemode Loopdiff.

ResetPhase has no syntax. Examples:

ph.6

4.add.warchief.t.(vase.ph.6).t.goblin

e20.19.ph.6 will spawn a ResetPhase on the first floor of every loop, as in every 20th floor starting with the 21st.

(x9.Add 100 Fights)&(x7.Add 10 Fights)&(x9.Add Fight)&(Cursemode Loopdiff)&(e20.19.ph.6)&hidden is a base that can be used for a custom Cursed mode; it increases the level cap to 999, adds Cursemode Loopdiff, hides this modifier, and adds a ResetPhase after every loop.

### ItemCombinePhase

The ItemCombinePhase, ph.7, will either smith multiple tier 0-3 items together to create a higher tier item, or it will smash the second highest tier item you have to create a number of tier 3 items based on its tier. There are two hard-coded cases for this phase: SecondHighestToTierThrees and ZeroToThreeToSingle, each doing as its name suggests.

Examples:

ph.7SecondHighestToTierThrees: Chooses an item from the second highest tier of items you possess. Generates x random tier 3 items where x = (itemtier + 2) / 3, rounding down. SecondHighestToTierThrees will ignore tier 0 and lower items.

ph.7ZeroToThreeToSingle: Gives the option to turn all tier 0 to 3 items into a tier x item, where x = sum(itemtier + 1) / 2. Note that tier 0 items contribute 1/2 of a tier to the result. Tier 1 items add 1 tier, tier 2s add 1 1/2 tiers, and tier 3s add 2 tiers. The result will round down.

### PositionSwapPhase

PositionSwapPhase, ph.8, uses hero position numbered similarly to HeroChangePhase. The syntax for this is simply ph.8(firsthero)(secondhero), where the option is given to swap (firsthero) and (secondhero)'s positions.

Note that if the hero position selected (usually 5 or higher) does not exist, the game will throw an error and the phase will do nothing.

ph.801: swap the top hero and the one below it.

ph.824: swap the third and fifth heroes.

ph.896: swap the 10th and 7th heroes.

### ChallengePhase

As of v3.1, ChallengePhase cannot be recreated as a modifier. It can, however, still be created as a phase in Paste mode. phi.6 and phi.7 can also be used to generate challenges.

ChallengePhase, ph.9, gives the player the option to fight additional enemies on the current floor in exchange for additional rewards beforehand. In the standard game, it will always grant items for taking challenges, but the reward can actually be changed to any of the Choosable Tags.

There is only one Challenge type: extraMonsters. The syntax for the ChallengePhase is similar to how phases used to be stored prior to Textmod. It's a bit much to type out every time, so I would recommend copy and pasting it and changing the monsters and rewards when creating one.

The syntax is ph.9{\"reward\":{\"data\":\"iItem@3mModifier@3lLevelup@3gAddHero...\"},\"type\":{\"extraMonsters\":[\"Enemy1\",\"Enemy2\",...]}} The rewards are separated by @3 while the enemies are placed inside multiple layers of quotations, backslashes, and brackets.

Examples:

ph.9{\"reward\":{\"data\":\"iMonocle",\"iSeedling"},\"type\":{\"extraMonsters\":[\"Militia\",\"Militia\"]}} grants the player a Monocle and Seedling in exchange for adding two Militias.

ph.9{\"reward\":{\"data\":\"mBoss Bones\"},\"type\":{\"extraMonsters\":[\"Fountain\",\"Barrel\"]}} is a backwards version of a normal challenge where the reward is a curse but the enemies make the fight easier.

8.ph.9{\"reward\":{\"data\":\"m8.Stone Rain@3iLearn Hack@3m8.Leyline^1\"},\"type\":{\"extraMonsters\":[\"Log\"]}} creates a challenge on level 8 reliant entirely on the rewards and adds a useless enemy.

<!-- parse-artifact: the ChallengePhase example in the source uses mixed smart/straight quotes which produce a malformed JSON-like string; preserved as-is from the guide -->

### BooleanPhase

BooleanPhase, ph.b, checks a previously-set Value and chooses one of two different phases depending on if the value is above or below a selected number. Note that while it can trigger a second BooleanPhase if you place it at the end, it cannot be nested in the middle due to collision. Therefore, if you want to chain BooleanPhases, you must start from the highest value and go lower.

The syntax for BooleanPhase is as follows:

ph.b(value);##;phaseA@2phaseB, where (value) is the set value the phase is checking and ## the number the value is being compared to. If you have at least ## of (value), the output is phaseA, otherwise, the output is phaseB.

The delimiters for BooleanPhase are ; and @2. Here are some examples:

ph.bDoubloon;5;4Excellent you have [yellow][valdoubloon] Doubloons[cu]!@24You need more [yellow]Doubloons[cu]! is a BooleanPhase that shows one of two MessagePhases based on how many Doubloons you have; at least 5 are needed for the first message.

1.ph.bSeed;3;!m1.fight.Boar@2bSeed;2;!m1.fight.Goblin@2bSeed;1;!m1.fight.Illusion@2!m1.fight.Log chooses between multiple SCPhases to change the floor 1 fight based on the value of Seed. If seed is 3 or higher, the first phase is an SCPhase that adds  1.fight.Boar. If it's lower, the next phase is a second BooleanPhase which will check if the seed is 2 or higher.

1.ph.bDragon;4;tmPoison Tendrils@3mHero Regen^2@2bDragon;3;tmSlow Spells^2@3ik.fizz@2bDragon;2;tmSandstorm^1@3it.gnoll@2!mReliable is a chained BooleanPhase that offers either one of three TradePhases or the Reliable modifier on floor 1 depending on the value of Dragon.

### BooleanPhase2

BooleanPhase2, ph.z, is identical to BooleanPhase, except it uses the delimiters @6 and @7. Because of this, it can be used alongside SeqPhase or BooleanPhase itself to chain longer conditionals or choice trees. BooleanPhase2 may be less intuitive to use, as the semicolon from BooleanPhase has been replaced by @6.

Here are a few examples of BooleanPhase2 in use:

=ph.cUpToNumber#2;vLevelV1@3vItemV1,ph.bLevel;1;zItem@61@6lgh@1gi@7gh@2bItem;1;gi@2!s combines BooleanPhase and BooleanPhase2 to give an item, a levelup, or both, depending on the values of Item and Level. PhaseGeneratorTransformPhase is used to generate the rewards.

=ph.!vgoldV400@3vgoldV2000@3vgoldV0,ph.zgold@62000@6sBuy Something! You have [valgold] Gold.@1Tier 1 Item [400 gold]@2!vgoldV-400@2!r1~1~i@1Random Level up [2000 gold]@2!vgoldV-2000@2!r1~1~l@7zgold@6400@6sBuy Something! You have [valgold] Gold.@1Tier 1 Item [400 gold]@2!vgoldV-400@2!r1~1~i@1[red]Random Level up [2000 gold] [Too Expensive][cu]@24You can't afford that!@7l4You can't even afford to go to the shop. Maybe you will find some gold in the trash.@1!vgoldV200 creates a shop by combining BooleanPhase2 with SeqPhase. The shop will not allow the player to buy something unless they have enough Gold.

### ChoicePhase

ChoicePhase, ph.c, is similar to SimpleChoicePhase but accepts 4 unique inputs for a "type" of choice, then continues with the rewards listed normally. Similar to SCPhase, the delimiters are ; and @3. The inputs are as follows:

ph.cPointBuy#(number);, which functions similar to a curse selection screen in some difficulties. PointBuy uses modifier tier, item tier, and hero tier to add to the total.

ph.cNumber#(number);, which lets you pick exactly (number) rewards. This functions similar to a Hard curse choice without Complex Hard.

ph.cUpToNumber#(number);, which lets you pick up to (number) rewards.

ph.cOptional#(number);, where (number) is irrelevant and you can either take everything or nothing. This is used in-game for optional tweak additions.

A few examples:

ph.cPointBuy#-20;mSandstorm^3@3mPoison Tendrils@3mHunt^2@3lMonk@3omDeep Pockets@4mDouble Loot@3mBot Caltrops: note that the Or Tag uses the tier of the first reward listed.

ph.cNumber#3;gb0.271@3lDruid@3mTower^12@3iWooden Bracelet

ph.cOptional#3;mSticky Blanks@3gVeteran

ph.cUpToNumber#2;iPowdered Mana@3mi.Amnesia.part.1@3iCan

### RunEndPhase

The RunEndPhase, ph.e, will end your run when it's used. Custom mode now has a proper level end screen as of v3.1, which will show your team and equipped items.

ph.e: ends the run at the start of every floor…

7.ph.e: ends the run at the start of floor 7.

ph.bLives;1;4You have [valLives] lives left.@2e is a BooleanPhase that ends the player's run if they don't have at least 1 Lives.

### LinkedPhase

LinkedPhase, ph.l, is a special phase which will cause two different phases to take place one after another. It uses the unique delimiter @1 to separate them. You can also chain LinkedPhases to combine an indefinite number of them. LinkedPhase can be used to ensure that players go through phases in the correct order to avoid breaking logic in some instances. Let me know any unique uses you find for it!

The syntax is ph.l(phase1)@1(phase2). Here are a few examples of it:

ph.l!lMonk@3lWarden@3r2~1~l@3s@1521 links a hero upgrade SCPhase and a HeroChangePhase, forcing the player to decide whether or not to reroll the gray hero after it's been upgraded, rather than rerolling and then making the choice meaningless by upgrading afterwards.

ph.l4Message1@1l4Message2@1l4Message3@14Message4 links four MessagePhases to be read in order. Note that the last phase doesn't have an l for LinkedPhase as that would cause the game to expect another phase afterwards.

1.ph.l!mdelevel@1l!m1.phi.0@1l!m1.phi.0@1l!m1.phi.0@1l!m1.phi.0@1!m1.phi.0 creates a LinkedPhase that de-levels all heroes, and then adds 5 levelup screens for them to upgrade again. If LinkedPhase is not used, the levelups are generated at the same time and will collide. In the case where it is used, each phi.0 is generated sequentially instead and they do not collide with each other.

### RandomRevealPhase

The RandomRevealPhase, ph.r, typically shows which item or hero you were given after choosing a random choice, but it can be used for custom pop-ups to show hidden rewards granted by Choosables or possibly for flavor. Note that the popup will say "Gained" but the reward will not actually be added.

Any Choosable can be used for RandomRevealPhase, including the basic hero, item, levelup, and modifier choosables, but also including RandomRange, Or, Enu, and Replace choosables.

The syntax for RandomRevealPhase is ph.r(reward). A few examples:

ph.riDiamond Skull

ph.lrilearn.slice.n.Slice@1rlBuckle.i.hat.Ludus.img.Dice.n.Dice: A LinkedPhase revealing Slice & Dice.

ph.romWurst@4lDabbler

ph.rr1~1~i shows the player that they have obtained a random tier 1 item.

ph.rpm2 less max mana~m3 less max mana is a popup which tells the player that 2 less max mana was replaced by 3 less max mana.

ph.rpldabble~ldabbler is a popup showing that Dabble has been replaced with Dabbler. The text in the box seems a bit confused, though.

ph.reRandoKeywordT1Item displays a popup stating that a RANDOM keyword item (rightmost) has been obtained.

ph.rs displays Gained: skip.

### SeqPhase

SeqPhase, ph.s, is one of the more difficult phases to format properly. It begins with a message and a set number of options afterwards which each lead to different phases. To separate the different options and the phases afterwards, SeqPhase has two separate delimiters: @1 to separate the different options and @2 to separate the phases that come afterwards.

An example phase to show the syntax:

ph.sInitialMessage@1Button1@24Option1@24Option1Cont.@1Button2@24Option2@24Option2Cont.@24Option2More@24Option2IsReallyLong

If you have more than two buttons or have long option sequences, it can be helpful to test the phase multiple times to make sure everything is working properly.

ph.sChoose a Party@1[Scoundrel][Ruffian][Buckle][Splint][Cultist]@2!mparty.Scoundrel+Ruffian+Buckle+Splint+Cultist@1[Dabble][Fighter][Alloy][Gardener][Prodigy]@2!mparty.Dabble@2!madd.Fighter@2!madd.Alloy@2!madd.Gardener@2!madd.Prodigy showcases a party choice at the start of a run. One option has just one item in its sequence, while the other is longer.

2.add.(vase.3.ph.sThe dead are rising...@1Uh oh...@2!m(4.fight.bones+zombie+zombie.mn.Undead Attack)@2!iSapphire Skull@1No that was just a goblin.@24Or was it?@2!m(4.fight.goblin+ogre.mn.Surprise!)@2!iPolearm.mn.Grave? Danger).n.Odd Grave.t.grave.t.goblin.img.grave.hsv.0:-10:-15.draw.rain of arrows.hsv.20:-10:-20:2:8.rect.07130304:222.mn.Undeadly Encounter is a unique event created through vase. that gives the player the option between two different level 4 fights and item rewards.

ph.sChoose@1KnightOption@2!mfight.rat@24Just kidding it's a rat@2ri(void.n.you can.doc.put any phase here)@1RatOption@2!gKnight@24Just kidding you get a knight instead@26@24Did you really think you could have a knight for free is a SeqPhase created to show that any phase can be placed in a SeqPhase, including ResetPhase and RandomRevealPhase. If the player chooses KnightOption, they fight a rat instead. If the player chooses RatOption, they are given a Knight and their party is reset afterwards.

### TradePhase

TradePhase, ph.t, is the phase used for "cursed chests" that appear in the game. In normal gameplay, they will usually offer random items in exchange for also accepting a random curse, but can also contain a tier -# curse combined with tier # blessing.

The syntax for TradePhase is quite simple, and it's functionally identical to the ChoicePhase input Optional. ph.t(reward1)@3(reward2) will offer a trade where both reward1 and reward2 are either accepted or declined together.

A few examples:

1.ph.tr1~4~i@3r-1~1~m is a common cursed chest one might see on level 1. It'll offer 4 random tier 1 items and 1 random tier -1 curse. The player won't be able to view the items or curse until after accepting.

4.ph.tmBoss Bones@3q2~3~1~i will give a modifier that makes the current level and all future bosses more difficult in exchange for an additional tier 2-3 item.

ph.tiLead Weight@3iWorn Arms@3iBarrel Hoops@3m5th selfShield offers two cursed items and two rewards that may help counter them.

### LevelEndPhase

As of v3.1, LevelEndPhase cannot contain more than 1 Phase when used as a modifier in Custom mode. Multiple phases can still be used for Paste mode.

LevelEndPhase, ph.2, is listed last because it is the phase where the rest of them are contained between levels. The syntax almost entirely consists of other phases. Because of the way it works, if you use ph.2{} in Custom mode, it'll create a reward box like other phases but clicking it overlays a second level screen. Therefore, it's probably more useful for Paste.

The syntax for LevelEndPhase is as follows: ph.2{ps:[(phase),(phase)]}.

ph.2{ps:[tr-1~1~m@3r1~4~i,!lPriestess@3lSparky]} creates a LevelEndPhase containing a cursed chest and a levelup screen.

ph.2{ps:[4The run is going to end now;what,e]} is a LevelEndPhase with a funny message and a RunEndPhase.

### PhaseGeneratorTransformPhase

PhaseGeneratorTransformPhase, or ph.g, is the only new phase added in version 3.1. It's not very complicated, as it only has two inputs. Both are easily recreated with phi.0 and phi.1, although PGTP can also be used to input a randomly generated reward screen into another phase, such as SeqPhase, LinkedPhase, BooleanPhase, or LevelEndPhase. In this aspect, it is unique and may be useful.

ph.gh creates a hero levelup phase, similarly to phi.0

ph.gi creates an item reward phase, similarly to phi.1

ph.sChoose one@1Levelup@2gh@1Item@2gi creates a SeqPhase which allows the player to choose whether they would like a randomly generated levelup screen or item screen.

# Hidden Items & Modifiers

## Hidden Items

Many applications of AbilityData and TriggerHPData can use the new hidden items to create unique effects, so they'll have a dedicated section here. As of v3.2, there are 23 hidden items in the game, although the majority of these are tog items, which have their own sections. Instead, this mini-section will focus on the remaining 7: rgreen, clearicon, cleardesc, Idol of Chrzktx, Idol of Aiiu, Idol of Pythagoras, and False Idol.

Items in Slice & Dice that must be equipped to a certain color hero were reworked in v3.1 to instead be equipable by anyone but only have bonuses on a certain hero color. At base, rgreen adds +1 HP if equipped to a green hero, but this conditional can be added to other items using the splice. API to create similar custom items.

Examples:

rgreen.splice.twisted bar

rgreen.splice.natural (can never do anything)

cleardesc and clearicon are items that allow you to clear the descriptions of an item or the helpful icons that appear for many effects. After doing this, you can add in a custom description or icon image using doc. and [image text] if desired.

Examples:

(Anchor#cleardesc.doc.Start of turn 1[n]Self-shield 1 if the current day ends in a y.n.Anchor)

(Golden D6#clearicon.doc.[petrify-diagram].n.Trollden D6)

The remaining four items, the Idols, grant max HP depending on the letters or numbers in the equipped hero's name. These have been all but removed from the game to accommodate translation, but their effects still exist:

Idol of Chrzktx (tier 6): +1 Max HP per consonant (including y)

Idol of Aiiu (tier 5): +1 Max HP per vowel (including y)

Idol of Pythagoras (tier 0): +1 Max HP per number

False Idol (tier 0): +1 Max HP per z

## Hidden Modifiers

In version 3.1, some hidden modifiers were added to the game. I'll cover them here, although they may not be present in examples for all sections. They are: Skip, Wish, Clear Party, Missing, Temporary, Hidden, Skip All, Add Fight, Add 10 Fights, Add 100 Fights, Minus Fight, and Cursemode Loopdiff.

Skip is a modifier which has no effect. Clear Party and Missing are identical; both simply remove the entire party of heroes.

Wish grants access to all of Wish mode's abilities. This allows players to customize items, heroes, and modifiers within a run, as well as skip fights if they desire.

Temporary and Hidden are excellent modifiers for keeping Custom Mode modifier lists clean. Hidden modifiers will be invisible to the player unless a box on the modifier menu is checked, and Temporary modifiers will remove themselves after a single combat to clear up the list and keep pastes small. These can be used to make sure a player can easily see modifiers they picked for difficulty or obtained through events.

Skip All is similar to Skip Rewards but additionally skips all events. This can be useful to avoid collision with certain events or just to remove them from the game and possibly replace them with your own. Skip All&e2.1.phi.1&e2.phi.0 will simply remove events and set the normal upgrades and item rewards.

The Add and Minus fight modifiers can be used to change the length of your custom mode (or custom paste). Difficulty ramps up normally except with Cursemode Loopdiff which causes level 21 and level 1 to have the same enemy balance. This allows the possibility of creating your own custom cursed modes or simply having a shorter custom run such as a boss rush or similar.

# Tog items

## Tog items

Below is a table listing each tog item and what it toggles on a hero's dice. All except two of these, togtime and togfri, are reliant on the left side to grant its effects to the other sides. These items are all compatible with side restrictions such as topbot., but they will still take their changes from the left side. They can also be used on both heroes and monsters.

Since the left side will need to be used for most tog items, it may be easiest to create a custom side on a statue using them and create a hat. of that statue for your custom hero. A particularly useful example would be Thief.i.top.hat.(Fighter.i.(Ice Cube)#(togkey)), which creates a 1 damage stasis side with no visible keyword icon. It would be impossible to change Thief's left side after doing this, so using a hat. is more convenient.

Togres received a few new additions in v3.2, including the addition of 6 new variants: togresa, togresm, togresn, togreso, togress, and togresx. Togres and these variants will be described in a separate section.

| Tog Item | Toggles what? | Description |
|---|---|---|
| togtime | Buff Duration Time | buff duration toggles inf/1 to all sides |
| togtarg | Side Targeting | take targeting from left side to all sides |
| togfri | Friendliness Targeting | toggle friendliness type friend/foe to all sides |
| togvis | Side Visual Animations | take visual from left side to all sides |
| togeft | Side effect | take eft from left side to all sides |
| togpip | Pips | take pips from left side to all sides |
| togkey | Keywords | take keywords from left side to all sides |
| togorf | Or Friendly Target | take left side as friendly targeting effect to all sides |
| togunt | Untargeted Effect | take untargeted effect from left side to all sides |

### Togtime

Togtime changes the buff duration by toggling it between being a 1 turn buff and being an indefinite one. The uses for togtime include any sides that say "this fight" or "this turn" in their base description. These sides include the dodge side, the undying side, inflict keyword sides, and Textmod sticker. sides. Any keywords such as Boost or Decay will unfortunately not be affected.

The most interesting uses for togtime will likely be using the new sticker. API. Normally, it applies an item effect to a hero for one turn, but you can use togtime to make it a permanent effect for that combat.

Here are a few examples:

Pilgrim.i.togtime: Pilgrim's keyword sides now add the keywords for the entire fight. Pilgrim's undying side also lasts for the entire fight now.

Berserker.i.sticker.Shortsword#togtime: Berserker gains a sticker side which grants an ally the effects of Shortsword for the entire combat.

Seer.i.sticker.k.cantrip#togtime: Seer gains a sticker side which grants an ally the cantrip keyword to all sides for the entire combat. Seer's dodge side also lasts for the entire fight now.

### Togtarg

Togtarg borrows the targeting from the left side and applies it to other sides. This applies to the side's default targeting. It won't apply to keywords, but a different tog item, togres, will. It will also not cause shields to target enemies or similar, but togfri can be used for that. Togtarg can be used to convert other sides into sides that target "all targets", "ALL", "top and bottom targets", single targets, or even turn targeted sides into untargeted sides and vice versa.

One good reason to use togtarg is to use a side's animation for a different targeting type. Some side animations, including Boar tusks and Ogre's slam are limited to certain targeting types normally but their targeting can be changed using togtarg. Fair warning, many enemy side animations were not intended to be used by the player and may function a bit weirdly. Dragon breath and many others are animated backwards.

Another good reason to use togtarg is because it can cause sticker. sides to target all allies instead of only targeting one. It can also be used to change the targeting of a cast. side.

Here are a few examples of togtarg in use:

Statue.i.(seedling)#(right2.hat.boar)#(togtarg)#(right2.facade.dan18:0) is a Statue that uses togtarg to create a damage side using Boar's tusk attack animation.

Whirl.i.(mid.hat.Tarantus)#(mid.togtarg) gives Whirl a middle side that deals 12 damage to all enemies. Unfortunately, Tarantus' bite side is incompatible with this targeting, so the damage is instant and has no animation.

Mage.i.(Seedling)#(togtarg)#(Peaked Cap)#(k.engage) creates a Mage with functional Mana Engage sides which target an enemy and use their health as a bonus for the Engage restriction.

Statue.sd.72-2.i.sticker.k.undergrowth#togtarg is a Statue with a sticker. side that grants all allies the undergrowth keyword to all sides this turn.

### Togfri

Togfri toggles the "friendliness" of all of a hero or monster's sides. It works essentially the same as the possessed keyword, but has a few more applications, especially since togfri can be used as part of custom abilities while possessed cannot. Sides that previously targeted allies will target enemies after equipping togfri, and vice versa.

Here are a few examples using togfri:

orb.sThief.abilitydata.Statue.i.left.cast.slice#togfri is an Orb that has the passive of dealing 1 damage to all of its allies upon death. Togfri changes the friendliness of cast.slice to target all of orb's allies instead of its enemies.

blind.i.togfri is an enemy that targets other enemies.

statue.i.(all.sticker.k.halveengage)#togfri is a hero that can add HalveEngage to an enemy's sides using a sticker. side with toggled friendliness.

### Togvis

Togvis causes a hero's dice sides to borrow the animation used for the left side. In that sense, many of its use cases could be shared with togtarg, but one or the other may be easier depending on the situation. Togvis also sometimes copies the animation sound of a side, which can also be favorable. Unique effects given by sticker. or cast. can also be reskinned using togvis.

The tables below include every possible visual effect with one source listed for each. Enemy effects may be repeated for different enemy sizes. Also note that many enemy effects may behave strangely; the game doesn't expect heroes to use them.

| Visual effect | Example Side | Visual Effect | Example Side |
|---|---|---|---|
| Sword | sd.15 (damage) | Slice | sd.137 (damage rampage) |
| Punch | sd.174 (damage defy) | Kriss | sd.30 (damage cruel) |
| Fork | sd.36 (damage cleave) | Hammer | sd.39 (damage heavy) |
| SwordQuartz | sd.40 (quartz damage) | Poison | sd.91 (poison wand) |
| Arrow | sd.46 (damage ranged) | Shield Bash | sd.41 (damage steel) |
| Heal | sd.92 (singleuse selfheal) | Lightning | sd.88 (singleuse charged) |
| Flame | sd.90 (cruel singleuse) | Frost | sd.95 (weaken singleuse) |
| Big Zap | sd.101 (chaos wand) | Undying | sd.117 (undying) |
| Taunt | sd.118 (redirect) | HealBasic | sd.103 (heal) |
| Fang | sd.169 (snake damage) | Wolf Bite | sd.170 (wolf damage) |
| Claw | sd.171 (wolf cleave) | BoostShield | sd.146 (add selfshield) |
| BoostHeal | sd.147 (add selfheal) | Beam | sd.181 (target enemy) |
| Boost | sd.150 (add engage) | Anvil | left.cast.drop |
| Ellipse | left.cast.slay | Crush | left.cast.crush |
| MultiBlade | left.cast.blades | Singularity | left.cast.harvest |
| Freeze | left.cast.tick | Cross | left.cast.hex |

Enemy sides will assume you want the side to be on the left side for easy use with togvis.

| Visual | Example Side | Visual | Example Side |
|---|---|---|---|
| Gaze | left.top.hat.illusion | Bee Sting | left.hat.bee |
| Bone | left.hat.bones | Rat Bite | left.hat.rat |
| Poison Bite | left.right.hat.imp | Slime | left.hat.slimelet |

| Visual | Example Side | Visual | Example Side |
|---|---|---|---|
| Cross | left.hat.ghost | Troll Club | left.hat.troll |
| Broom | left.hat.gytha | Bat Swarm | left.hat.agnes |
| Gaze | left.right.hat.gytha | Stomp | left.hat.ogre |
| Rocks | left.right.hat.slate | Spikes | left.right.hat.spiker |
| Rock Punch | left.hat.slate | Spike Punch | left.hat.spiker |
| Beak | left.hat.caw | Curse | left.hat.magrat |
| Slime | left.hat.slimer | Big Claw | left.hat.alpha |
| Alpha Bite | left.hat.bramble | CleaveSword | left.top.hat.ogre |
| Boar Bite | left.hat.boar | Boar Tusks | left.right.hat.boar |

| Visual | Example Side | Visual | Example Side |
|---|---|---|---|
| Slam | left.top.hat.troll king | Dragon Bite | left.hat.rotten |
| Tarantus Bite | left.hat.tarantus | Gaze | left.right.hat.lich |
| Fire Breath | left.hat.dragon | PoisonBreath | left.right.hat.dragon |
| Frost  Flank | left.hat.basalt | Red Beam | left.top.hat.basalt |
| Slime | left.mid.hat.slime queen | | |

Here are some examples using togvis:

disciple.i.togvis is a hero that plays the "heal" sound effect when using the mana sides.

healer.i.(left.hat.ogre)#(togvis) is a healer that uses the ogre "slam" animation every time she uses a side. It's a bit buggy.

Militia.i.(left.hat.Caldera)#(togvis)#(Peaked Cap) uses a flame damage animation to reflect the torch in its sprite.

Wallop.i.(left.top.hat.Monk)#(sticker.Shortsword)#(togvis) causes the Monk redirect hand to show over targets of the sticker. side, stun side, and damage side. Unfortunately, the sound effect is not copied.

Fighter.i.(left.cast.drop)#(right2.togvis)#(left.right.hat.Slate)#(col.togvis)#(Peaked Cap) is a Fighter that throws anvils at teammates when he shields them and throws rocks at enemies to damage them. The anvil effect is borrowed from left.cast.drop, while the rocks are from left.right.hat.slate.

(rightmost.hat.(Buckle.i.(right5.Cloak)#(togtarg)#(left.right.hat.Spiker)#(togvis)#(right5.facade.Lem66:0))).n.Ninja Training.tier.3.img.Bal13 is an item granted a targeted dodge side. Togtarg makes the dodge side target an ally, while togvis grants the side a spiker attack visual, which the item repurposes as a thrown kunai.

### Togeft

Togeft copies the base effect from the left side and gives it to the other 5 sides. It doesn't change targeting or keywords however, so equipping a base Venom with togeft will replace their middle side with "3 damage to an ally, cleanse".

Given that it's often more convenient to begin a side with the correct base effect and add everything else afterwards, togeft is often not as useful as other tog items. One of the best potential uses for togeft, cast.burst or cast.vine, will unfortunately replace the other sides with an error blank instead of working as expected. However, it can still be useful depending on the way you're ordering things.

Here are a few examples of the uses of togeft:

tainted.i.togeft replaces all of Tainted's sides with blanks, however, the visuals, pips, and keywords all stay the same, so you have to click on the sides to notice.

Fighter.i.(left.right.hat.Alpha)#(togeft)#Wrench#(Peaked Cap).n.Fighter This fighter looks the same as a normal one, but all of his sides spawn Wolves instead of attacking or shielding.

Ludus.i.(left.hat.(Poet.i.Seedling#togeft#Pendulum))#togeft#k.damage#k.doubdiff#k.plus#k.squared#Peaked Cap#Ocular Amulet#all.facade.bas15:0.n.Ludus is a particularly evil character that looks like a normal Ludus, but will deal heavy damage and potentially kill your entire party if you try to use them.

### Togpip

Togpip will copy the pips from the left side and give them to the others. This can mainly be used to remove pips from pipped sides or to give pips to pipless sides, the latter case being the more useful one. A pipless side technically counts as having -999 pips, so pipped sides may behave strangely if they are converted to pipless ones.

Pipless sides, on the other hand, have been fine as far as I've seen, and you can use their new pips to give them pip-based keywords. After being converted, they are also automatically eligible for all keywords that pipped sides are able to have, like Growth, ManaGain, etc.

A few examples:

Valkyrie.i.(topbot.togpip)#(topbot.k.pain) is a Valkyrie that has been nerfed by adding 4 pain to the top and bottom Undying sides. However, this also opens the opportunity to add a positive keyword to those sides, such as ManaGain or SelfShield.

Reflection.i.(rightmost.togpip)#(rightmost.Face of Horus) turns Reflection's rightmost blank into a blank (3 pips), which is eligible to be used as a side for its tactic, Thrike.

Chronos.i.togpip turns Chronos' pipped sides into pipless sides, making them display -999 and become pretty much useless.

### Togkey

Togkey adds the keywords from a hero's left side to the rest of their sides. Notably, it does not display the keyword icon on the corner of their dice.

Here's a good example of that use:

Thief.i.top.hat.(Fighter.i.(Ice Cube)#(togkey)) allows a custom Stasis side to be created without having the keyword icon on the bottom left. To use a facade, simply give the Fighter a facade before applying togkey.

However, togkey is a bit more useful than it may seem at first glance. By copying a keyword from the left side that is already present on other sides, keywords may be duplicated and their effects can stack.

Not every keyword will work like this, but from my testing, most of the additive and multiplicative ones work. Unfortunately, Growth does not work, but most others do. There's definitely a lot of unique things you can do with this tog item.

Here are a few examples using togkey to duplicate keywords:

Gladiator.i.(mid.togkey) has duplicated engage to the middle side, which now deals x4 damage if the target is on Full HP.

Barbarian.i.(left.Blindfold)#k.lucky#togkey#togkey#togkey#togkey#togkey#togkey#togkey is likely to have all of his sides crippled by Lucky's stacking negative bonuses.

Priestess.i.togkey has a middle side which now grants double the empty max HP.

Stalwart.i.(left.blindfold)#(left.k.first)#(rightmost.togkey)#(rightmost.togkey)#(memory) will greatly reward you for playing Stalwart's rightmost side first. Memory is included to remove the changes to the left side.

### Togorf

Togorf allows you to grant the left side's effect as an optional effect for the other sides. In practice it's a bit buggy; only a friendly-targeting side such as "Shield 2" can be applied to other sides, and they can only be applied to sides which pick an enemy target. When using togorf, it's best to modify it using sidepos. to avoid other sides turning into bugged blanks.

Notably, sides which have been altered by togorf have their pips visually removed, as the two effects may have different pip values.

Examples of togorf in use:

Thief.abilitydata.(Statue.sd.110-1:137-1:0-0:0-0:76-1:0-0.i.(mid.Blindfold)#(mid.k.poison)#(mid.togorf)#(Peaked Cap)#(Rusty Plate).n.Vine Plus.img.Sprout) has a custom Vine+ ability which either deals 1 damage poison or heals an ally for 1 regen. Currently the spell is bugged to not display the "orf" friendly side's regen keyword. This can be fixed using sidesc.

Knight.i.topbot.togorf can either shield 3 or deal 3 damage with his top and bot sides. Unfortunately, the Exert keyword applies to both effects.

Brawler.sd.56-3:24-2:24-2:24-2:24-2:24-2.i.togorf#Peaked Cap is a custom hero with all sides replaced by "2 damage or shield 3 doubleuse". Note that doubleuse applies to both effects despite only originally being on the damage sides.

Leader.sd.left:56-2.i.(mid.togorf)#(left.Second Chance) inspires his allies to be more flexible than base-game Leader and allows them to either deal 2 damage or shield 2 with his duplicate side. Regardless of which effect is chosen first, the togorf option will be duplicated to all.

### Togunt

Togunt allows an untargeted effect from the left side to be added to another side as a bonus. Any side or spell that doesn't require the player to pick a target when using it counts as an untargeted effect. Commonly, this includes mana, revives, and sides that affect the group of allies or enemies. There are also more niche untargeted effects such as enchant.

Technically, any effect can become untargeted through the use of togtarg, but an untargeted shield 2 for example will simply do nothing. What could possibly be more interesting is using togtarg to apply unique targeting such as from cast.crush or cast.drop. Below is a quick list of useful untargeted effects and side examples, let me know if any are missing:

| Effect | Example |
|---|---|
| Reroll | sd.125 |
| Summon | sd.172, any hat.egg.entity |
| Revive | sd.35, sd.136, sd.166 |
| enchant. | any enchant.modifier API |
| Mana | sd.76, many more. |
| Damage all or self | sd.34, sd.128 |
| Shield all or self | sd.72, sd.73 |
| Heal all or self | sd.107 |
| Damage ALL | sd.54, sd.158, sd.160 |
| cast.crush | cast.crush |
| cast.drop | cast.drop |
| sticker. all or ALL or self | sticker. API with togtarg/togfri |

Prodigy.i.left.mid.hat.(Spade.i.Sapphire#(mid.Pharaoh Curse)#(togunt)).i.left.facade.bas35:0.n.Prodigy is an alternative Prodigy that may use his side regardless of whether there are any defeated allies: the revive is added as a togunt untargeted effect.

topbot.(mid.hat.Statue.sd.187-1.i.(Viscera)#(left.k.shield)#(togunt)) is an item using togunt to apply a "shield self 1" effect to the Viscera item sides, allowing a hero to always survive at 1 HP. topbot.mid.hat is used to copy the middle side of Statue to the top and bottom sides of the hero equipping the item.

x4.Fighter.i.(left.enchant.Sandstorm^1)#(togunt)#(Peaked Cap) is a powerful boosted Fighter that activates a stacking Sandstorm^1 modifier after every use of his dice. Togunt is used to apply the enchant side as an untargeted effect to all sides, and Peaked Cap is used to override the left side.

Sorcerer.i.(left.hat.Spade)#(togunt)#(Pendulum)#(Silk Cape) is a modified Sorcerer that uses togunt to apply a revive side to his cantrip reroll sides. Note that a revive cantrip side causes considerable bugs during the rolling phase.

Stoic.i.(left.mid.hat.Defender.i.(left.cast.crush)#(togtarg))#(row.togunt)#(Memory) is a custom Stoic who shields the top and bottom heroes for 2 when he uses the redirect sides. Togtarg applies the targeting from left.cast.crush to Defender's shield 2 side, which is then copied onto Stoic using left.mid.hat. row.togunt then applies the shield 2 to topbot as an untargeted effect to the row, and memory restores Stoic's stun side.

# Togres & Variants

## Togres & Togres Variants

Togres is likely the most complicated tog item. It allows for the conditional bonus of a keyword or side, for example the full HP requirement for the engage bonus, to be used instead as a restriction for using the side. If togres is used with an engage side, the other sides would only be able to be used on full HP targets. There are over 50 unique restrictions that can be accessed in this manner.

As of version 3.2, several different variants of togres were created for the purpose of combining and changing targeting restrictions, further adding to the possibilities. In addition, togresm was added, which allows for a targeting restriction to be turned into a x2 multiplier, similar to a custom keyword. Working with these new togres variants can be difficult due to issues with description generation and accuracy, so any creations should be tested thoroughly.

If you are familiar with logic gates, many of these will be more intuitive; togresa, togreso, togresx, and togresn are based on AND, OR, XOR, and NOT respectively.

| Tog Item | Last Letter | Description |
|---|---|---|
| togres | | Take restrictions from left side |
| togresm | Multiplier | Transform 'targeting restrictions' into 'x2 conditionals' |
| togresa | AND | Merge restrictions from left side using AND |
| togreso | OR | Merge restrictions from left side using OR |
| togresx | XOR | Merge restrictions from left side using XOR |
| togress | SWAP | Swap 'I' and 'target' in restrictions |
| togresn | NOT | Invert targeting restrictions |

### Togres

Togres uses the left side to apply targeting restrictions to the other sides. It's one of the most versatile toggle items for creating new side effects. Many of the targeting restrictions that togres uses come from keywords, however there are some that are obtained from sides.

Most targeting restrictions have misleading or completely missing descriptions. The table will describe their effects more accurately. Sidesc may be useful when creating sides using togres. Keywords listed with an asterisk * will not update the description at all, but still change the targeting restrictions of sides when used with togres.

| Keyword | Targeting Restriction | Keyword | Targeting Restriction |
|---|---|---|---|
| pristine | I must have Full HP | engage | Target must have FullHP |
| swapcruel | I must have half or less HP | cruel | Target must have half or less HP |
| swapterminal | I must have exactly 1 HP | terminal | Target must have exactly 1 HP |
| deathwish | I must be Dying | swapdeathwish | Target must be Dying |
| bully | I must have the most HP of all | uppercut | Target must have the most HP of all |
| moxie | I must have the least HP of all | squish | Target must have the least HP of all |
| armoured | I must be shielded | wham | Target must be shielded |
| antideathwish | I must NOT be Dying | eliminate | Target must have the least HP |
| priswish | I must be FullHP AND Dying | heavy | Target must have the most HP |
| antipristine | I must NOT have full HP | reborn | Target must be dead (?) |
| serrated | Target must not have gained Shields | century | Target must have 100 or more HP |
| engine | Target and I must both have full HP | scared | Target must have N or less HP |
| generous | Target must NOT be me | picky | Target must have exactly N HP |
| ego* | Must target self | duel* | Must target attacker |
| first* | Must be the 1st dice used | sixth* | Must be the 6th dice used |
| step* | Previous sides must be a run of 2 with me | run* | Previous sides must be a run of 3 with me |
| sprint* | Previous sides must be a run of 5 with me | chain* | Must share a keyword with previous side |
| inspired* | Previous side must be higher pips | antipair* | Previous side must NOT have same pips |
| pair* | Previous side must have same pips | trio* | Previous 2 sides must have same pips |
| quin* | Previous 4 sides must have same pips | sept* | Previous 6 sides must have same pips |
| dog* | Target must have same HP as me | overdog* | Target must have less HP than me |
| antidog* | Target must NOT have same HP as me | underdog* | Target must have more HP than me |
| focus* | Must target same target as last dice | tall* | Must choose the top target |
| sloth* | Target must have fewer Blank sides than me | patient* | I must not have used a dice side last turn |
| paxin* | Previous side must have same pips XOR previous side must have same keyword | underocus* | Target must have more HP than me AND Must target same target as last dice |
| hyena* | Target's HP must be coprime with mine | | |

Some sides are also able to influence togres, but not as many of them:

| Side | Targeting Restriction |
|---|---|
| 122 (kill an enemy ranged) | Target must have N or less HP |
| 43 (stun an enemy with equal or less HP) | Target must have equal or less HP than me |

Two more restrictions can be accessed in a roundabout way using cast. sides:

| cast. side | Targeting Restriction |
|---|---|
| cast.scald | Target must be Damaged |
| cast.dsspotless | Target must be Undamaged |

Even without the other togres variants, there's a lot of room for creating new and unique sides here. Here are a few examples of using it:

Statue.col.r.hp.7.sd.76-2:107-1:103-5:103-5:52-2:52-2.i.(mid.hat.Statue.sd.107-1:103-1.i.(left.k.generous)#(togtarg)#(togres)).i.mid.k.vitality#k.pain.n.Vampire.i.(mid.facade.alp5:0)#(mid.sidesc.All other heroes heal [pips]).img.Vampire.abilitydata.(Statue.sd.107-2:0-0:0-0:0-0:76-3:0-0.img.infuse.n.Infuse) is a Vampire edit that replaces her middle side with the old "Heal 1 to all other allies, Vitality Pain". Generous is used with togres to apply the NotMe targeting restriction.

Thief.abilitydata.Statue.sd.17-1:34-2.i.togres#togtarg#Origami#Ballet Shoes.img.Flick.n.Flick uses togres to grant the FullHP restriction to Flick, reverting it to the overpowered version from v1.0.

rightmost.hat.statue.sd.15-2.i.(rightmost.cast.infinity)#(left.k.century)#(left.k.cruel)#(togres).tier.0.n.Giant Slayer.img.sling.hsv.0:-99:-25 is a joke item that replaces the rightmost side with a nearly impossible to use "kill an enemy with half or less HP with 100 or more HP" using century, cruel, and togres.

Statue.sd.left:186-0.i.(mid.sticker.Eye of Horus)#(mid.togtarg)#(left.cast.scald)#(mid.togres)#togtime is a Statue with a sticker. that grants +1 to all sides. Togtarg makes it only target itself, togres grants it the restriction of must be Damaged, and togtime makes it a permanent buff for the combat.

### Togresm

Togresm, or Togres Mult, turns targeting restrictions placed onto sides by togres into x2 conditional bonuses with the same requirement. For example, if the "Target must have full HP" condition is applied to sides by using togres on engage, togresm will add a conditional "x2 if the target has full HP" to the sides. In this case, it is similar to the engage keyword, but it does not count as a keyword for interactions such as the rainbow keyword or Blindfold item.

Note that turning a restriction into a x2 conditional removes the restriction; a side may be used as normal without the initial restriction afterwards.

By combining togresm with restrictions that do not originate from x2 keywords, such as the cast.scald restriction or the picky restriction, new x2 keyword-like effects can be created. This can be further expanded with togresa, togreso, togresx, togress, and togresn to change the initial restriction and therefore change the condition of the multiplier. This will be explained more in the upcoming sections.

Some basic examples of togresm:

Medic.i.(left.k.picky)#(topbot.togres)#(topbot.togresm)#(Memory) uses togresm to add a conditional: "x2 if the target has N HP" to the top and bottom sides. The description is a bit misleading and may need to be changed. left.k.picky adds a restriction for topbot.togres to copy to the top and bottom sides, topbot.togresm turns the restriction into a x2 conditional multiplier, and Memory reverts the left side to remove the picky keyword.

Fighter.i.(left.k.trio)#(mid.togres)#(mid.togresm)#(Peaked Cap) creates a fighter with the left 2 sides: 2 damage, x2 if this has the same pips as the previous 2 sides this turn. The effect is the same as trio, but the multiplier is x2 rather than x3. Note that, because the restriction is from a keyword that does not update the description, the side description states "x2 if null". This is not accurate.

Lost.i.(left.cast.scald)#(col.blindfold)#(col.togres)#(col.togresm)#(Memory) changes Lost to only require targets to be damaged rather than half or less HP for their column sides to be x2. The condition is from cast.scald.

### Togresa

Togresa, or Togres AND, combines targeting restrictions from the left side to other sides. On its own, this is the same as togres; adding a second targeting restriction to a side is the same as requiring both.

By combining togresa with the other togres variants, a restriction or conditional  such as "A and (B or C)" or "NOT (A and B)" can be created. Additionally, togresa creates a x2 conditional "x2 if A and B", whereas a second togres restriction would create two separate x2 conditionals with togresm.

Examples:

Whirl.i.(left.k.underdog)#(mid.togres)#(left.k.engage)#(mid.togresa)#(mid.togresm)#(Memory) is a modified Whirl that has been granted a powerful middle side against strong single targets: 3 damage, x2 if the target has more HP than me AND is full HP. Togres first adds the underdog restriction to the middle side, and togresa adds the engage restriction after engage has also been added to the left side. Mid.togresm turns the AND restriction into an AND conditional x2 and Memory reverts the left side to default.

(topbot.hat.Buckle.i.(left.hat.Ace)#(togres)#(togresn)#(left.mid.hat.Ace)#(togresa)#(togresm)#(topbot.facade.Can25:0)#(sidesc.Shield [pips] [light]p[cu][blue]anti[cu][light]trio[cu][nh][light]p[cu][blue]anti[cu][light]trio[cu] - x2 if the conditions of both [light]pair[cu] and [blue]anti[cu][light]trio[cu] are met).n.Pair of Jacks).tier.5.img.Pair of Kings is a more complicated example that chains togresn for an "antitrio" restriction with togresa, which also requires the pair restriction. In this case, the shield 2 sides are doubled if the last side has the same pips, but NOT if the last 2 sides have the same pips. A custom side description is added with sidesc to clarify the effect; the original description is insufficient to convey this information to the player. (left.hat.Ace) and (left.mid.hat.Ace) are used as the sources for the trio and pair keywords for togres.

### Togreso

Togreso, or Togres OR, combines targeting restrictions using an inclusive OR operation. This allows either restriction A or B, or both, to be met in order to use a hero's side or ability. While togresa can be used to make a restriction more difficult to reach, togreso can be used to create restrictions or conditionals that are easier to meet.

Some examples using togreso:

(bot.hat.(Veteran.i.(left.k.first)#(bot.togres)#(Memory)#(left.k.sixth)#(bot.togreso)#(bot.facade.Ale8:0))).tier.4.n.Katana.img.Ale8 is an item that adds a katana to a hero's bottom side. The katana can only be used if this is the first or sixth dice side this turn. A side description should be added with sidesc to clarify this to the player. Note how the first restriction is added to the side with togres, while the second one is added with togreso.

(rightmost.mid.hat.fighter.i.(left.k.first)#(mid.togres)#(mid.togresn)#(memory)#(left.k.engage)#(mid.togresa)#(memory)#(left.k.trio)#(mid.togreso)#(mid.facade.Bal15:0)#(sidesc.[pips] damage if this has the same pips as the 2 previous dice this turn OR (if the target is FullHP AND this is not the first dice this turn))).tier.3.n.Wayward Pike.img.Bal15 is an item that adds a restricting attack to the rightmost side. This side can only be used if the conditions of just trio OR (engage AND NOT first) are met. Since this is an inclusive OR, the side will still work if both conditions are met.

Scalpel.splice.(mid.hat.Surgeon.i.(left.k.ego)#(mid.togres)#(Memory)#(left.k.terminal)#(mid.togreso)#(Memory)#(left.k.engage)#(mid.togreso)#(mid.facade.Lem219:0)).tier.4.n.Fickle Heart.img.Lem219 is a manaGain healing side that must meet one of three conditions in order to be used: the restrictions of ego, terminal, or engage. The description shows terminal and engage's conditions properly, but not ego's.

### Togresx

Togresx, or Togres XOR, combines targeting restrictions using an exclusive OR operation. This means that, when combining restrictions A and B, the side will be usable if either A or B is true, but not both. This can also be reversed using togresn to create an XNOR restriction: the side will be usable if neither condition is true or if both are true.

This can be used to create more unique sides, for example, a shield side with the restriction cruel xor ego can be used on any ally below half HP or on the user itself, but only if the user is above half HP. This can result in the side having no valid targets.

Examples using togresx:

(top.mid.hat.(Fighter.i.(left.k.duel)#(togres)#(Memory)#(left.k.pristine)#(togresx)#(togresm)#(facade.Can5:0)#(sidesc.[pips] damage [blue]dux[cu][light]tine[cu][nh][blue]dux[cu][light]tine[cu]- x2 if the condition [blue]duel[cu] xor [light]pristine[cu] is met))).n.Imperfect Weapon.img.Can5 is an item that adds a 2 damage duxtine side to the hero's top side. Duxtine is a custom keyword with the condition duel xor pristine. Togres adds the duel condition to the side, togresx adds the xor restriction with pristine, and togresm creates the custom keyword.

learn.sAssassin.abilitydata.((Brigand.i.(left.k.cruel)#(togres)#(Memory)#(left.k.squish)#(togresx)#(togresn)#(togresm)#(left.hat.Quartz)#(togvis)#(Ballet Shoes)#(Rusty Plate)#(sidesc.[pips] damage [orange]crux[cu][yellow]nish[cu][nh][orange]crux[cu][yellow]nish[cu]- x2 if the condition [orange]cruel[cu] xnor [yellow]squish[cu] is met)).n.Infused Slash.img.spe12.hue.10) is a custom spell item that teaches the spell Infused Slash, which costs 1 mana and deals 1 damage. Togres, togresx, togresn, and togresm are used to grant this spell a custom cruxnish keyword which doubles the damage if the condition cruel xnor squish is met; either both conditions or neither must be met for the multiplier. Togvis grants the spell the visuals from the quartz damage side.

### Togress

Togress, or Togres Swap, is used to swap "I" with "the target" in togres restrictions or conditionals. It works in the same way as the swap operator used on some keywords in-game: terminal is x2 against 1 HP targets, swapterminal is x2 if I am 1 HP. Like the other togres variants, it will take this effect directly from the side if possible, but since it does not take the effect from a specific side, it will only work on sides that have togres-compatible keywords.

Togress crashes the game more frequently than most other tog items; more frequent trial and error may be necessary when using it. In addition, many combinations such as swapcruel or swapengage already exist.

Unique effects you can create with togress include swapped serrated and century. Many others such as swapped priswish or picky simply crash the game. Since century is very niche, togress has even fewer uses.

Some examples:

all.mid.hat.Fighter.i.(left.k.bully)#(togres)#(togress)#(togresm)#(mid.sidesc.[pips] damage [light]swap[cu][orange]bully[cu][nh][light]swap[cu][orange]bully[cu]- x2 vs targets with the most HP) is an item that replaces all sides with "2 damage swapbully", which deals x2 damage to targets with the most HP. This is essentially a recreated version of the uppercut keyword.

x9.Priestess.i.(left.k.century)#(topbot.togres)#(topbot.togress) is a unique Priestess hero that needs at least 100 HP to be able to use her top and bottom sides. Togres copies the "target must have 100 or more HP" restriction from the left side's century keyword, while togress swaps this to require the hero itself to have 100 or more HP.

(mid.hat.Soldier.i.(left.k.serrated)#(togres)#(togress)#(left.top.hat.Scrapper)#(togvis)#(k.steel)#(togresm)#(facade.bas41:0)).tier.6.n.Shield Gambit.img.Tower Shield is an item that grants the side "3 damage steel, x2 if I have gained no shields". This presents the opportunity to deal 6 damage with no shields or to gain at least 4 shields to do more damage.

### Togresn

Togresn, or Togres NOT, allows you to invert any restriction or conditional multiplier, changing the side to require the opposite of the initial condition. It works the same way as the anti operator used on some keywords in the game. Unlike togress, this nearly doubles the number of available togres effects and additionally allows the AND, OR, and XOR combinations from other togres variants to be flipped into NAND, NOR, and XNOR. Also unlike togress, the NOT restriction works on every keyword that a togres restriction can be generated for.

Here are some examples featuring togresn:

(right2.hat.Ludus.i.(Origami)#(left.k.sloth)#(togres)#(togresn)#(self.rightmost.Jammed)#(right.facade.Ale7:0)#(right.sidesc.[pips] damage to an enemy with equal or more blank sides).n.Spot Weakness).tier.3.img.void.draw.Lem224:1:-1 replaces the 2 right sides on the equipped hero with "6 damage to an enemy with equal or more blank sides" and "blank stasis". This item uses the togres sloth restriction and inverts it using togresn.

(topbot.hat.(Fighter.i.(left.k.cruel)#(topbot.togres)#(Memory)#(left.k.engage)#(topbot.togreso)#(togresn)#(togresm)#(topbot.facade.Mut9:0))).n.Saber.img.Mut9 is an item that replaces the top and bottom sides with 1 damage, x2 if the target is not at half or less HP or full HP. This item uses togreso to grant an OR restriction which togresn then inverts to a NOR restriction. Neither condition can be met for this side to give x2.

(rightmost.mid.hat.Fighter.i.(left.k.first)#(togres)#(Memory)#(left.k.inspired)#(togreso)#(togresn)#(Memory)#(left.k.antipair)#(togresa)#(togresm)#(sidesc.[pips] damage [grey]uninspired[cu][nh][grey]uninspired[cu]- x2 if the previous dice this turn had fewer pips)#(facade.Bal8:0)).tier.5.n.Dull Blade.img.Bal8 is an item that replaces the rightmost side with "2 damage uninspired", where uninspired is a custom keyword that grants x2 if the previous dice used was lower. This item uses a NOR restriction from togresn negating a "first OR inspired" condition, requiring that neither are true. Togresa is then used to force both the NOR condition and the condition antipair to be met; this ensures that the previous dice was not the same value.

# AbilityData

## AbilityData

AbilityData allows you to create your own Spells and Tactics with unique abilities and costs. It was already in the game in v3.0, but received a huge update, allowing players to create custom tactics in addition to the already-available custom spells. This is a bit more complicated, so it warrants a bit of explanation.

For a custom ability, the game uses the sides, name, and image of a target hero to create a new spell from that data.

The Example's image, Jester, is used for the image for the custom ability. This can be borrowed from other spells or items and hue-shifted, or you can create a custom one.

Example's name is used for the name of the ability. In this case, the spell would be named Example.

Side 1 is used for the primary effect of the ability. Many different effects and keywords can be used, but not all of them.

Side 2 is used for a secondary effect. This one has to be non-targeted, such as "damage to all" or "gain mana"

Side 5 is used for the mana cost of the ability. If there's a side here, the ability will be automatically counted as a spell instead of a tactic. The pips of that side are used for the cost, the side itself doesn't matter.

Sides 3, 4, and 6 are used for the side cost of a custom tactic. As a result, a custom tactic can have up to 3 separate side costs. Unfortunately, this means tactics with more than 3 costs such as Unite (Prince) and Oof (tier 0 item) cannot be recreated.

Below are all of the possible side costs for a custom tactic. Some of these are unused in base-game tactics.

### Custom Spells

To create a custom spell, all you need to do is to create the desired effect on the left side, with the option of adding a second untargeted effect on the middle side. Effects on either side must not "target self", as there is no source for the spell to target.

 The mana cost on the right side can be set by the pips, between 0 and 999. Many keywords such as Engage or Cruel can also be applied. In general, if a keyword has an effect based on the hero using it, like Deathwish, Ego, or Exert, it's not going to be applicable to spells.

Here are a few examples of custom spells created using abilitydata:

Statue.col.b.hp.8.i.hat.Evoker.i.Sickle.n.Evoker.img.Evoker.abilitydata.(Statue.sd.181-3:46-3.i.togvis#Origami#Ballet Shoes.img.Beam.n.Beam) is an Evoker using togvis to gain the custom spell Beam: Deal 3 damage, ranged for 3 mana cost. Togvis gives the ranged side the Beam visual effect.

Thief.abilitydata.Statue.i.(right.sticker.Compass)#(left.InfiniHeal)#(left.Twisted Bar)#(togtarg)#(Origami).n.Roll has a custom spell that uses togtarg to cause a sticker. to target all allies with the Compass effect this turn, similar to the base-game spell Flip. After Infiniheal grants the targeting restriction, it is flipped with Origami to become the cost. The final .n.Roll designates the name of the spell.

Thief.abilitydata.Statue.i.(Big Shield)#(Cloak)#(togtarg)#(Pendulum)#(Rusty Plate)#(Shortsword) has a spell that uses togtarg to allow a normally impossible side (dodge) to be used on a spell and target an ally. This uses Shortsword as the 2 mana cost.

learn.sThief.abilitydata.(Statue.sd.103-3.i.Shortsword.img.infuse.n.Blood).n.Blood Spellbook.tier.4 is a tier 4 item using the spell from an abilitydata to grant a spell that heals 3 for 2 mana. learn.sThief indicates "learn the spell of Thief". Note that learn.tThief also works.

### Custom Tactics

Custom Tactics are a bit more complicated due to the way their costs are set. Basic costs, such as Damage, Shield, Heal, or Mana pips are quite simple, however the other costs are unintuitive. Below I will list the cost type, the rule for obtaining it, and one example for a side that would grant that cost. The number of pips for the first 5, Damage, Shield, Heal, Mana, and Any, will also decide the number of pips required for the tactic.

| Tactic cost | What to put on sides 3, 4, or 6 | Example |
|---|---|---|
| Damage pip | Any damage side | sd.15-2 (Basic Damage) |
| Shield pip | Any shield side | sd.56-2 (Basic Shield) |
| Heal pip | Any heal side | sd.103-2 (Basic Heal) |
| Mana pip | Any mana side | sd.76-2 (Basic Mana) |
| Any pip | Revive side | sd.136-2 (Revive) |
| Blank side | Any blank side EXCEPT the default | sd.8-0 (Exerted Blank) |
| Sides below this point must NOT be damage/shield/heal/mana/revive/blanks. | | |
| 1-keyword side | Qualifying side with 1 Keyword | sd.13 (I Die Cantrip) |
| 2-keyword side | Qualifying side with 2 Keywords | .i.(top.cast.DSVarhest) |
| 1-pip side | 1-pip side with 0, 3, or >4 keywords | sd.177-1 (Target Ally) |
| 2-pip side | 2-pip side with 0, 3, or >4 keywords | sd.177-2 (Target Ally) |
| 3-pip side | 3-pip side with 0, 3, or >4 keywords | sd.177-3 (Target Ally) |
| 4-pip side | 4-pip side with 0, 3, or >4 keywords | sd.177-4 (Target Ally) |
| 4-keyword side | Qualifying side with 4 Keywords | .i.(top.cast.DSVarhest#Fly) |

Keep in mind that placing a side in the right position on a dice will result in the game processing the ability as a spell instead.

Some examples involving custom tactics:

Thief.abilitydata.(Statue.sd.15-2:0:177-3:177-2:0:177-1.i.left.cast.abyss) has a difficult tactic to use, costing a 3-pip, 2-pip, and 1-pip side, however it uses cast.abyss to give the tactic a strong payoff.

Roulette.abilitydata.(Statue.sd.15-2:0-0:177-1.i.(left.togfri)#top.k.engage#top.k.flesh#top.k.pristine#top.k.deathwish.n.Fold.img.Pair of Kings.hsv.0:-50:0) grants Roulette a "Fold" tactic that can use a 4-keyword side to deal 2 damage to an ally. By adding a keyword to their left side, you could use this to dodge the death. Togfri is used to cause damage to an ally.

(Lazy.abilitydata.(Statue.sd.56-2:0:5:5.n.Snooze)).n.Lazy is an alternative version of Lazy that has a custom tactic which costs 2 blanks and shields an ally for 2. The curse blank side is used on Statue's top and bottom to designate the cost, since normal blanks don't count.

Statue.abilitydata.(Statue.i.(left.InfiniHeal)#(rightmost.sticker.(Abacus.part.0))#(togtarg)#(Ballet Shoes)#(rightmost.Trowel).n.Skim)) has a custom tactic, Skim, which costs 1 of any pip and applies the Abacus effect to all heroes using a togtarg-modified sticker. All heroes' rows will be shifted by 1 upon use.

learn.tStatue.abilitydata.(Statue.sd.181-1:105-1:1-0:0-0:0-0:0-0.i.togvis#Pendulum#Rusty Plate.n.Drink).n.Learn Drink.tier.2 is a tier 2 item which grants the holder the custom tactic "Drink": heal 1 vitality at the cost of 1 blank side. Togvis is used to grant the ability the "target enemy" side's visual effect. learn.tStatue indicates "learn the tactic of Statue". Note that learn.sStatue also works for tactics.

# TriggerHPData

## TriggerHPData

TriggerHPData is a Textmod API that generates an item which grants a custom HP pip to an entity. Like AbilityData, it is reliant on a target hero to provide data for the item to use. The left side of the hero is used for the active effect when the pip is damaged, the hero's HP is used to determine where the pip will be on the entity's health bar, and the hero color is used to determine the color of the health pip. Unfortunately, the shape of the TriggerHPData pip cannot currently be changed.

Any side that doesn't require the player to select a target can be used for TriggerHPData. This includes mana, damage to all, shield to all, revives, and sides that target self. Togtarg can be used with this to cause normally unusable sides to target self and be used with TriggerHPData.

Below is the table for the first 21 hero HP values and the pips a TriggerHPData with that HP will influence. Beyond 21 HP, TriggerHPDatas will affect the Nth HP, where N = (HP-20). For example, 22 HP will affect the 2nd HP, 25HP will affect the 5th HP, 31 HP will affect the 11th HP, and so on.

| HP | Pips affected | HP | Pips affected | HP | Pips affected | HP | Pips affected |
|---|---|---|---|---|---|---|---|
| 1 | All HP | 7 | Every 10, starting with 5 | 13 | Inner 5 | 19 | 2 Evenly Spaced HP |
| 2 | Every 2 | 8 | Every 2, starting with 1 | 14 | Outer 1 | 20 | 3 Evenly Spaced HP |
| 3 | Every 3 | 9 | Every 3, starting with 1 | 15 | Outer 2 | 21 | 4 Evenly Spaced HP |
| 4 | Every 4 | 10 | Inner 1 | 16 | Outer 3 | ... | |
| 5 | Every 5 | 11 | Inner 2 | 17 | Outer 5 | ... | |
| 6 | Every 10 | 12 | Inner 3 | 18 | Middle HP | N | The N-20th |

Examples using TriggerHPData:

(slimer.i.triggerhpdata.Housecat.hp.22.i.hat.Chest#Origami).n.Slimer grants Slimer a second HP pip that summons a slimelet.

(rmon.67f.hp.8.i.triggerhpdata.(Mage.hp.24.col.w.sd.186.i.(cast.bind)#(togtarg)#(Pendulum)).i.(left2.facade.bas199:68).sd.11-1:11-1:8-6:8-6:8-6:8-6.n.Gytha.img.Gytha) recreates Gytha from an older version by giving her a damage immunity pip similar to Ghost on her 4th HP. Togtarg is also used to change the targeting of cast.bind to allow for this.

dragon.i.(triggerhpdata.Whirl.hp.31.i.hat.(Dragon.sd.10-1))#(triggerhpdata.Whirl.hp.41.i.hat.(Dragon.sd.10-1))#(triggerhpdata.Whirl.hp.51.i.hat.(Dragon.sd.10-1)).n.Dragon is a Dragon that damages all heroes for 1 after losing its 11th, 21st, and 31st HP. Unfortunately, the breath animation does not play, but defeated heroes will still "burn" as expected.

Tracked.part.0#triggerhpdata.(Statue.hp.5.col.w.i.left.right.hat.Alpha).img.Tracked.n.Tracked.tier.-1 is an alternative version of the curse item Tracked that summons a wolf upon losing every 5th HP. It may be equipped on lower HP heroes to lessen the downside. col.w is used to change the color of the Tracked pips.

---

# Reference Index

This index lists codes, prefixes, and patterns that appear in the guide with brief descriptions traceable to the guide's own text. Codes used only in examples without explicit definition are marked as such.

## Phase codes (`ph.X`)

| Code | Name | Notes |
|---|---|---|
| `ph.!` | SimpleChoicePhase | Used with tag letters (e.g. `ph.!m`, `ph.!i`, `ph.!l`, `ph.!g`, `ph.!r`, `ph.!q`, `ph.!o`, `ph.!e`, `ph.!v`, `ph.!p`, `ph.!s`). Delimiter `@3`. Title supported via semicolon. |
| `ph.0` | PlayerRollingPhase | Accepts two numbers separated by semicolon (e.g. `ph.01;2`), but "As far as I can tell this does nothing." |
| `ph.1` | TargetingPhase | As above. |
| `ph.2` | LevelEndPhase | Syntax `ph.2{ps:[(phase),(phase)]}`. "cannot contain more than 1 Phase when used as a modifier in Custom mode." |
| `ph.3` | EnemyRollingPhase | "already at the start, so using it as a modifier has no discernable effect." |
| `ph.4` | MessagePhase | Syntax `ph.4(message);(buttontext)`. Supports color brackets and entity image tags. |
| `ph.5` | HeroChangePhase | Syntax `ph.5XY` where X is hero position 0-4 and Y is 0 (random class) or 1 (generated hero). |
| `ph.6` | ResetPhase | "no syntax". |
| `ph.7` | ItemCombinePhase | Inputs `SecondHighestToTierThrees` and `ZeroToThreeToSingle`. |
| `ph.8` | PositionSwapPhase | Syntax `ph.8(firsthero)(secondhero)`. |
| `ph.9` | ChallengePhase | JSON-like syntax with `extraMonsters`. "cannot be recreated as a modifier" as of v3.1. |
| `ph.b` | BooleanPhase | Syntax `ph.b(value);##;phaseA@2phaseB`. |
| `ph.c` | ChoicePhase | Inputs `PointBuy`, `Number`, `UpToNumber`, `Optional`. Delimiters `;` and `@3`. |
| `ph.d` | DamagePhase | "enemies have no targets at all, so this one is skipped and does nothing." |
| `ph.e` | RunEndPhase | Ends the run. |
| `ph.g` | PhaseGeneratorTransformPhase | Inputs `h` (levelup) and `i` (item reward). |
| `ph.l` | LinkedPhase | Syntax `ph.l(phase1)@1(phase2)`. |
| `ph.r` | RandomRevealPhase | Syntax `ph.r(reward)`. |
| `ph.s` | SeqPhase | Delimiters `@1` (options) and `@2` (phases). |
| `ph.t` | TradePhase | Syntax `ph.t(reward1)@3(reward2)`. |
| `ph.z` | BooleanPhase2 | Delimiters `@6` and `@7`. |

## Phase-related shorthand prefixes

| Code | Description |
|---|---|
| `phi.#` | "Phase Indexed" — accepts number 1-9 to generate a phase every floor. `phi.0` Levelup; `phi.1` Standard Loot; `phi.2` Reroll; `phi.3` Reroll; `phi.4` Optional Tweak; `phi.5` Hero Position Swap; `phi.6` Standard Challenge; `phi.7` Easy Challenge; `phi.8` Hero Position Swap; `phi.9` Trade (cursed chest). |
| `phmp.+-` | "Phase Mod Pick" — uses integer in place of `+-` to create a modifier selection screen where the total to reach is that integer. |

## Choosable / SCPhase tag letters

| Choosable | SCPhase | Tag | Notes |
|---|---|---|---|
| `ch.m` / `ph.!m` | | Modifier | Standard tag. |
| `ch.i` / `ph.!i` | | Item | Standard. |
| `ch.l` / `ph.!l` | | Levelup | Standard; defaults to topmost hero if no eligible target. |
| `ch.g` / `ph.!g` | | Hero | Standard; `ph.!gRuffian` is functionally identical to `ph.!madd.Ruffian`. |
| `ch.r` / `ph.!r` | | Random | Input; syntax `ch.r(tier)~(number)~(tagletter)`. |
| `ch.q` / `ph.!q` | | RandomRange | Input; syntax `ch.q(tier1)~(tier2)~(number)~(tagletter)`. |
| `ch.o` / `ph.!o` | | Or | Input; uses delimiter `@4`. |
| `ch.e` / `ph.!e` | | Enu | Three inputs: `RandoKeywordT1Item`, `RandoKeywordT5Item`, `RandoKeywordT7Item`. |
| `ch.v` / `ph.!v` | | Value | Syntax `v(variable name)V(amount added)`. |
| `ch.p` / `ph.!p` | | Replace | Syntax `ph.!pm(modifier)~(reward)`. Only replaces modifiers. |
| `ch.s` / `ph.!s` | | Skip | No syntax. |

## Value / display helpers

| Code | Description |
|---|---|
| `[val(variable name)]` | Displays the stored Value in any phase where text is visible (e.g. `ph.4[valgold]`). |
| `[valgold]`, `[valdoubloon]`, `[valLives]`, `[valItem]` etc. | Examples of value display in guide. |
| `[orange]`, `[yellow]`, `[red]`, `[light]`, `[blue]`, `[grey]`, `[cu]`, `[nh]`, `[n]` | Color and formatting brackets used within phase/side text (appears in examples; explicit definitions not in guide). |
| `[Thief]`, `[Fighter]`, `[rat]`, `[petrify-diagram]` | Entity/image bracket references in MessagePhase and doc. text. |

## Tog items

| Code | Description |
|---|---|
| `togtime` | Buff duration toggles inf/1 to all sides. |
| `togtarg` | Take targeting from left side to all sides. |
| `togfri` | Toggle friendliness type friend/foe to all sides. |
| `togvis` | Take visual from left side to all sides. |
| `togeft` | Take eft from left side to all sides. |
| `togpip` | Take pips from left side to all sides. |
| `togkey` | Take keywords from left side to all sides. |
| `togorf` | Take left side as friendly targeting effect to all sides. |
| `togunt` | Take untargeted effect from left side to all sides. |
| `togres` | Take restrictions from left side. |
| `togresm` | Transform 'targeting restrictions' into 'x2 conditionals'. |
| `togresa` | Merge restrictions from left side using AND. |
| `togreso` | Merge restrictions from left side using OR. |
| `togresx` | Merge restrictions from left side using XOR. |
| `togress` | Swap 'I' and 'target' in restrictions. |
| `togresn` | Invert targeting restrictions. |

## Hidden items (non-tog subset)

| Item | Description |
|---|---|
| `rgreen` | "+1 HP if equipped to a green hero"; usable via `splice.`. |
| `clearicon` | Clears the helpful icons that appear for many effects. |
| `cleardesc` | Clears the description of an item. |
| `Idol of Chrzktx` (tier 6) | +1 Max HP per consonant (including y). |
| `Idol of Aiiu` (tier 5) | +1 Max HP per vowel (including y). |
| `Idol of Pythagoras` (tier 0) | +1 Max HP per number. |
| `False Idol` (tier 0) | +1 Max HP per z. |

## Hidden modifiers

| Modifier | Description |
|---|---|
| `Skip` | "a modifier which has no effect". |
| `Wish` | Grants access to all of Wish mode's abilities. |
| `Clear Party` | Removes the entire party of heroes. |
| `Missing` | Identical to Clear Party. |
| `Temporary` | Removes itself after a single combat. |
| `Hidden` | Invisible to the player unless a box on the modifier menu is checked. |
| `Skip All` | Similar to Skip Rewards but additionally skips all events. |
| `Add Fight` / `Add 10 Fights` / `Add 100 Fights` | Change the length of a custom mode. |
| `Minus Fight` | Change the length of a custom mode. |
| `Cursemode Loopdiff` | "causes level 21 and level 1 to have the same enemy balance". |

## Property codes appearing in examples

These appear in the guide's examples. Only those with explicit definitions or table entries in the guide are annotated; others are marked "appears in examples, no explicit definition".

| Code | Description from guide |
|---|---|
| `.sd.` | Side data. Used with `FaceID-Pips` (e.g. `sd.15-2`, `sd.76-2:107-1:103-5:103-5:52-2:52-2`). Specific face IDs documented in togvis visual-effect table (e.g. `sd.15` damage, `sd.30` damage cruel, `sd.36` damage cleave, `sd.39` damage heavy, `sd.40` quartz damage, `sd.41` damage steel, `sd.46` damage ranged, `sd.54`/`sd.158`/`sd.160` damage ALL, `sd.56-2` Basic Shield, `sd.72`/`sd.73` Shield all or self, `sd.76` Mana, `sd.88` singleuse charged, `sd.90` cruel singleuse, `sd.91` poison wand, `sd.92` singleuse selfheal, `sd.95` weaken singleuse, `sd.101` chaos wand, `sd.103` heal / `sd.103-2` Basic Heal, `sd.107` Heal all or self, `sd.117` undying, `sd.118` redirect, `sd.125` Reroll, `sd.128` Damage all or self, `sd.136`/`sd.166`/`sd.35` Revive, `sd.137` damage rampage, `sd.146` add selfshield, `sd.147` add selfheal, `sd.150` add engage, `sd.169` snake damage, `sd.170` wolf damage, `sd.171` wolf cleave, `sd.172` Summon, `sd.174` damage defy, `sd.181` target enemy / Beam, `sd.186` / `sd.187`, `sd.8-0` Exerted Blank, `sd.13` I Die Cantrip, `sd.177-1..4` Target Ally, `sd.43` stun an enemy with equal or less HP, `sd.122` kill an enemy ranged, `sd.5`, `sd.11-1`, `sd.10-1`, `sd.24-2`, `sd.56-3`, `sd.17-1`, `sd.110-1`, `sd.105-1`). |
| `.i.` | Item/equipment attachment (e.g. `.i.hat.Mystic`, `.i.togtime`, `.i.(Seedling)`). Appears in examples, no explicit definition in guide. |
| `.hp.` | Hit points. Used for heroes (e.g. `.hp.12`, `.hp.8`) and in TriggerHPData (e.g. `hp.22`, `hp.24`, `hp.31`). |
| `.col.` | Color/hero-color (e.g. `.col.b`, `.col.r`, `.col.w`). Appears in examples; `col.w` noted as "used to change the color of the Tracked pips". |
| `.tier.` | Tier of an item/modifier/reward (e.g. `.tier.3`, `.tier.-1`, `.tier.0`, `.tier.4`, `.tier.5`, `.tier.6`). Appears in examples, no explicit definition. |
| `.img.` | Image (e.g. `.img.Druid`, `.img.Beam`, `.img.spark`, `.img.Tracked`). Appears in examples, no explicit definition. |
| `.n.` | Name (e.g. `.n.Medium Sword`, `.n.Slice`, `.n.Example Title`). Appears in examples, no explicit definition. |
| `.mn.` | Used in fight names and menu names (e.g. `.mn.Bramble`, `.mn.Undead Attack`, `.mn.Grave? Danger`). Appears in examples, no explicit definition. |
| `.hue.` | Hue shift (e.g. `.hue.30`, `.hue.-10`, `.hue.10`). Appears in examples, no explicit definition. |
| `.hsv.` | HSV adjustments (e.g. `.hsv.0:-99:-40`, `.hsv.0:-10:-15`, `.hsv.0:-99:-25`, `.hsv.0:-50:0`, `.hsv.20:-10:-20:2:8`). Appears in examples, no explicit definition. |
| `.rect.` | e.g. `.rect.07130304:222`. Appears in examples, no explicit definition. |
| `.draw.` | e.g. `.draw.rain of arrows`, `.draw.Lem224:1:-1`. Appears in examples, no explicit definition. |
| `.part.` | Item-part selector (e.g. `.part.0`, `.part.1` in `mi.Amnesia.part.1`, `Abacus.part.0`, `Tracked.part.0`). Appears in examples, no explicit definition. |
| `.abilitydata.` | Custom ability data (spells and tactics). Dedicated section in guide. |
| `.triggerhpdata.` | Custom HP pip. Dedicated section in guide. |
| `.splice.` | API to add conditionals like rgreen to other items (described under Hidden Items). |
| `.doc.` | Used for custom description text (e.g. `void.doc.[valgold].n.Gold Counter`, and under `cleardesc`). |
| `.facade.` | e.g. `.facade.dan18:0`, `.facade.Lem66:0`, `.facade.bas15:0`, `.facade.bas199:68`, `.facade.Can25:0`. Appears in examples, no explicit definition. |
| `.sidesc.` | Custom side description (e.g. `mid.sidesc.All other heroes heal [pips]`). Appears in examples; described only implicitly as "a custom side description is added with sidesc to clarify the effect". |
| `.sidepos.` | Mentioned under Togorf: "it's best to modify it using sidepos. to avoid other sides turning into bugged blanks." |
| `.sticker.` | API that "applies an item effect to a hero for one turn" (described under Togtime). Examples include `sticker.Shortsword`, `sticker.k.cantrip`, `sticker.Compass`, `sticker.(Abacus.part.0)`, `sticker.Eye of Horus`, `sticker.k.halveengage`, `sticker.k.undergrowth`. |
| `.cast.` | Cast API producing unique effects. Examples include `cast.drop`, `cast.slay`, `cast.crush`, `cast.blades`, `cast.harvest`, `cast.tick`, `cast.hex`, `cast.slice`, `cast.abyss`, `cast.infinity`, `cast.scald`, `cast.dsspotless`, `cast.bind`, `cast.DSVarhest`, `cast.burst`, `cast.vine`. `cast.scald`: "Target must be Damaged"; `cast.dsspotless`: "Target must be Undamaged". |
| `.enchant.` | Untargeted effect type. `enchant.modifier` noted in untargeted-effects table. Example `left.enchant.Sandstorm^1`. |
| `.hat.` | Hat (borrow sides/visuals from another entity). Used heavily throughout examples (e.g. `hat.Mystic`, `hat.boar`, `hat.Spade`, `hat.egg.entity`, `left.hat.ghost`, etc.). Appears in examples, no explicit definition. |
| `.add.` | Add entity (e.g. `add.Ruffian`, `add.bones`, `add.warchief`, `add.rat`). Appears in examples, no explicit definition. |
| `.fight.` | Fight specifier (e.g. `4.fight.Bramble+Rat`, `4.fight.Troll`, `fight.rat`, `fight.goblin+ogre`). Appears in examples, no explicit definition. |
| `.learn.` | Learn a spell/tactic (e.g. `learn.sThief`, `learn.tThief`, `learn.sAssassin`, `learn.tStatue`, `learn.slice`, `learn.hack`). `learn.sX` = learn spell of X; `learn.tX` = learn tactic of X. |
| `.k.` | Keyword application (e.g. `k.engage`, `k.cantrip`, `k.pair`, `k.fizz`, `k.focus`, `k.ego`, `k.cruel`, `k.first`, `k.sixth`, `k.picky`, `k.trio`, `k.scald` (via cast), `k.century`, `k.generous`, `k.trio`, `k.underdog`, `k.pristine`, `k.duel`, `k.bully`, `k.serrated`, `k.sloth`, `k.antipair`, `k.inspired`, `k.squish`, `k.terminal`, `k.lucky`, `k.pain`, `k.steel`, `k.vitality`, `k.shield`, `k.flesh`, `k.deathwish`, `k.damage`, `k.doubdiff`, `k.plus`, `k.squared`, `k.halveengage`, `k.undergrowth`, `k.poison`). Appears in examples, no explicit definition. |
| `.t.` | Type tag (e.g. `.t.Goblin`, `.t.grave`, `.t.goblin`, `.t.gnoll`). Appears in examples, no explicit definition. |
| `.vase.` | Event wrapper used with other phases (e.g. `(vase.ph.6)`, `2.add.(vase.3.ph.s...)`). Appears in examples, no explicit definition. |
| `.lvl.` | Level-scoping prefix (e.g. `lvl.phi.#`, `lvl-lvl.phi.#`, `lvl.ph` which works for SCPhase but not `lvl.ch`). |
| `.self.` | Equipped-self restriction (e.g. `i.self.Mortal^3`, `self.rightmost.Jammed`). Appears in examples, no explicit definition. |
| `.left.` / `.mid.` / `.right.` / `.top.` / `.bot.` / `.topbot.` / `.row.` / `.col.` (position) / `.all.` / `.rightmost.` / `.left2.` / `.right2.` / `.right3.` / `.right5.` / `.col.` | Side-position restrictions used as prefixes (e.g. `topbot.`, `right3.k.focus`, `left.k.pair`, `rightmost.k.engage`, `rightmost.hat.statue`, `right5.Cloak`, `col.blindfold`, `col.togvis`). |
| `.facade.` | See above. |
| `.memory` / `Memory` | Item used to "revert the left side to default/remove the changes to the left side" in examples. |
| `.hidden` | Hidden modifier flag used in custom mode constructions. |
| `.temporary` | Temporary modifier flag. |
| `.party.` | e.g. `mparty.Scoundrel+...`, `mparty.Dabble`. Appears in examples, no explicit definition. |
| `.delevel` | e.g. `mdelevel`. Appears in examples, no explicit definition. |

## Entity/pool prefixes appearing in examples

These prefixes appear in the guide (and/or in the game's template system referenced by the guide). Descriptions from the guide are provided where available; otherwise marked as "appears in examples, no explicit definition".

| Prefix | Description |
|---|---|
| `rmon.` | Replica monster, e.g. `rmon.67f.hp.8.i.triggerhpdata...`. Appears in examples, no explicit definition. |
| `replica.` | Not explicitly referenced in guide text examined — no guide entry. |
| `orb.` | e.g. `orb.sThief.abilitydata...`. Appears in examples, no explicit definition. |
| `dragon.` | Used as entity prefix in TriggerHPData example. |
| `slimer.` / `blind.` / `statue.` / `tainted.` / `healer.` / `mage.` / `militia.` / `fighter.` / `thief.` / `whirl.` / `prodigy.` / `medic.` / `reflection.` / `chronos.` / `valkyrie.` / `priestess.` / `gladiator.` / `barbarian.` / `stalwart.` / `brawler.` / `leader.` / `sorcerer.` / `stoic.` / `roulette.` / `lazy.` / `disciple.` / `pilgrim.` / `berserker.` / `seer.` / `evoker.` / `wallop.` / `soldier.` / `veteran.` / `surgeon.` / `brigand.` / `lost.` / `ludus.` / `defender.` / `scrapper.` / `buckle.` / `ace.` / `poet.` / `spade.` / `knight.` / `housecat.` / `tarantus.` / `caldera.` / `basalt.` / `quartz.` / `slate.` / `spiker.` / `bramble.` / `boar.` / `ogre.` / `troll king.` / `rotten.` / `lich.` / `dragon.` / `slime queen.` / `slimelet.` / `magrat.` / `gytha.` / `agnes.` | These hero/enemy names appear in the guide's examples as prefixes before `.i.`, `.abilitydata.`, etc. The guide does not document them as a closed list; they are instances of the in-game template API. |

## Delimiters used in phase/choosable syntax

| Delimiter | Used by |
|---|---|
| `;` | SimpleChoicePhase title / ChoicePhase type / BooleanPhase (between value, number, and phases) / PlayerRollingPhase numbers / MessagePhase button text. |
| `@1` | LinkedPhase separator; SeqPhase option separator; BooleanPhase2 (with `@7` wrapping). |
| `@2` | BooleanPhase branch separator; SeqPhase phase-after separator. |
| `@3` | SCPhase / ChoicePhase / TradePhase reward separator. |
| `@4` | Or tag separator. |
| `@6` | BooleanPhase2 (replaces semicolon). |
| `@7` | BooleanPhase2 end delimiter. |
| `~` | Random / RandomRange internal separator; Replace tag separator (`modifier~reward`). |

## Special notes

- `Peaked Cap` appears frequently in examples to "override the left side".
- `Memory` / `memory` appears frequently to "remove the changes to the left side" or "revert the left side to default".
- `Origami`, `Pendulum`, `Rusty Plate`, `Ballet Shoes`, `Shortsword`, `Silk Cape` are commonly combined with abilitydata to set mana costs or tactic costs.
- `Blindfold` appears in togres examples and is noted to "stack negative bonuses" with Lucky and to not count x2 conditionals from togresm as a keyword.
