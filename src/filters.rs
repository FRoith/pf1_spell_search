use filter_derive::FilterReprMacro;
use filter_repr::{FilterRepr, FilterState};

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Spellschool {
    Abjuration(FilterState),
    Conjuration(FilterState),
    Divination(FilterState),
    Enchantment(FilterState),
    Evocation(FilterState),
    Illusion(FilterState),
    Necromancy(FilterState),
    Transmutation(FilterState),
    Universal(FilterState),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Subschool {
    Calling(FilterState),
    Creation(FilterState),
    Healing(FilterState),
    Summoning(FilterState),
    Teleportation(FilterState),
    Scrying(FilterState),
    Charm(FilterState),
    Compulsion(FilterState),
    Figment(FilterState),
    Glamer(FilterState),
    Pattern(FilterState),
    Phantasm(FilterState),
    Shadow(FilterState),
    Polymorph(FilterState),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Level {
    _0(FilterState),
    _1(FilterState),
    _2(FilterState),
    _3(FilterState),
    _4(FilterState),
    _5(FilterState),
    _6(FilterState),
    _7(FilterState),
    _8(FilterState),
    _9(FilterState),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Domain {
    Air(FilterState),
    Animal(FilterState),
    Artifice(FilterState),
    Chaos(FilterState),
    Charm(FilterState),
    Community(FilterState),
    Darkness(FilterState),
    Death(FilterState),
    Destruction(FilterState),
    Earth(FilterState),
    Erosion(FilterState),
    Evil(FilterState),
    Fire(FilterState),
    Glory(FilterState),
    Good(FilterState),
    Healing(FilterState),
    Knowledge(FilterState),
    Law(FilterState),
    Liberation(FilterState),
    Luck(FilterState),
    Madness(FilterState),
    Magic(FilterState),
    Nobility(FilterState),
    Plant(FilterState),
    Protection(FilterState),
    Repose(FilterState),
    Ruins(FilterState),
    Rune(FilterState),
    Scalykind(FilterState),
    Strength(FilterState),
    Sun(FilterState),
    Travel(FilterState),
    Trickery(FilterState),
    Vermin(FilterState),
    Void(FilterState),
    War(FilterState),
    Water(FilterState),
    Weather(FilterState),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum Save {
    None(FilterState),
    Will(FilterState),
    Reflex(FilterState),
    Fortitude(FilterState),
    Text(FilterState),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellResistance {
    Yes(FilterState),
    No(FilterState),
    Text(FilterState),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellComponent {
    Verbal(FilterState),
    Somatic(FilterState),
    Material(FilterState),
}

impl SpellComponent {
    pub fn special_test(&self, spell: &crate::spell::Spell) -> bool {
        match self {
            Self::Verbal(FilterState::None) => true,
            Self::Somatic(FilterState::None) => true,
            Self::Material(FilterState::None) => true,
            Self::Verbal(FilterState::Positive) => spell.verbal,
            Self::Somatic(FilterState::Positive) => spell.somatic,
            Self::Material(FilterState::Positive) => spell.material,
            Self::Verbal(FilterState::Negative) => !spell.verbal,
            Self::Somatic(FilterState::Negative) => !spell.somatic,
            Self::Material(FilterState::Negative) => !spell.material,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellDescriptor {
    Acid(FilterState),
    Air(FilterState),
    Chaotic(FilterState),
    Cold(FilterState),
    Curse(FilterState),
    Darkness(FilterState),
    Death(FilterState),
    Disease(FilterState),
    Draconic(FilterState),
    Earth(FilterState),
    Electricity(FilterState),
    Emotion(FilterState),
    Evil(FilterState),
    Fear(FilterState),
    Fire(FilterState),
    Force(FilterState),
    Good(FilterState),
    #[name = "Language-Dependent"]
    LanguageDependent(FilterState),
    Lawful(FilterState),
    Light(FilterState),
    Meditative(FilterState),
    #[name = "Mind-Affecting"]
    MindAffecting(FilterState),
    Pain(FilterState),
    Poison(FilterState),
    Ruse(FilterState),
    Shadow(FilterState),
    Sonic(FilterState),
    Water(FilterState),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellRange {
    Personal(FilterState),
    Touch(FilterState),
    Close(FilterState),
    Medium(FilterState),
    Long(FilterState),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, FilterReprMacro)]
pub enum SpellSource {
    #[name = "PFRPG Core"]
    PfrpgCore(FilterState),
    #[name = "Monster Codex"]
    MonsterCodex(FilterState),
    #[name = "Advanced Race Guide"]
    AdvancedRaceGuide(FilterState),
    #[name = "Advanced Class Guide"]
    AdvancedClassGuide(FilterState),
    #[name = "Advanced Class Origins"]
    AdvancedClassOrigins(FilterState),

    #[name = "Ultimate Magic"]
    UltimateMagic(FilterState),
    #[name = "Ultimate Combat"]
    UltimateCombat(FilterState),
    #[name = "Ultimate Intrigue"]
    UltimateIntrigue(FilterState),
    #[name = "Ultimate Wilderness"]
    UltimateWilderness(FilterState),

    #[name = "Armor Masters Handbook"]
    ArmorMastersHandbook(FilterState),
    #[name = "Demon Hunter's Handbook"]
    DemonHuntersHandbook(FilterState),
    #[name = "Dragonslayer's Handbook"]
    DragonslayersHandbook(FilterState),
    #[name = "Dungeoneers Handbook"]
    DungeoneersHandbook(FilterState),
    #[name = "Giant Hunters Handbook"]
    GiantHuntersHandbook(FilterState),
    #[name = "Haunted Heroes Handbook"]
    HauntedHeroesHandbook(FilterState),
    #[name = "Healer's Handbook"]
    HealersHandbook(FilterState),
    #[name = "Monster Hunter's Handbook"]
    MonsterHuntersHandbook(FilterState),
    #[name = "Monster Summoner's Handbook"]
    MonsterSummonersHandbook(FilterState),
    #[name = "Plane-Hopper's Handbook"]
    PlaneHoppersHandbook(FilterState),
    #[name = "Spymaster's Handbook"]
    SpymastersHandbook(FilterState),
    #[name = "Undead Slayer's Handbook"]
    UndeadSlayerHandbook(FilterState),
    #[name = "The HarrowHandbook"]
    TheHarrowHandbook(FilterState),

    #[name = "Dungeons Of Golarion"]
    DungeonsOfGolarion(FilterState),
    #[name = "Dwarves of Golarion"]
    DwarvesOfGolarion(FilterState),
    #[name = "Gnomes Of Golarion"]
    GnomesOfGolarion(FilterState),
    #[name = "Goblins Of Golarion"]
    GoblinsOfGolarion(FilterState),
    #[name = "Humans Of Golarion"]
    HumansOfGolarion(FilterState),
    #[name = "Kobolds Of Golarion"]
    KoboldsOfGolarion(FilterState),
    #[name = "Orcs of Golarion"]
    OrcsOfGolarion(FilterState),

    #[name = "Champions Of Balance"]
    ChampionsOfBalance(FilterState),
    #[name = "Champions Of Corruption"]
    ChampionsOfCorruption(FilterState),
    #[name = "Champions Of Purity"]
    ChampionsOfPurity(FilterState),
    #[name = "Faiths Of Corruption"]
    FaithsOfCorruption(FilterState),
    #[name = "Faiths Of Purity"]
    FaithsOfPurity(FilterState),

    #[name = "Pathfinder Society Primer"]
    PathfinderSocietyPrimer(FilterState),
    #[name = "Pathfinder Society Field Guide"]
    PathfinderSocietyFieldGuide(FilterState),
    #[name = "PFS S3-09"]
    PfsS309(FilterState),

    #[name = "Heroes Of The Darklands"]
    HeroesOfTheDarklands(FilterState),
    #[name = "Heroes Of The High Court"]
    HeroesOfTheHighCourt(FilterState),
    #[name = "Heroes Of The Streets"]
    HeroesOfTheStreets(FilterState),
    #[name = "Heroes Of The Wild"]
    HeroesOfTheWild(FilterState),

    #[name = "Inner Sea Magic"]
    InnerSeaMagic(FilterState),
    #[name = "Inner Sea World Guide"]
    InnerSeaWorldGuide(FilterState),
    #[name = "Inner Sea Gods"]
    InnerSeaGods(FilterState),
    #[name = "Inner Sea Races"]
    InnerSeaRaces(FilterState),
    #[name = "Inner Sea Intrigue"]
    InnerSeaIntrigue(FilterState),
    #[name = "Inner Sea Temples"]
    InnerSeaTemples(FilterState),
    #[name = "Inner Sea Monster Codex"]
    InnerSeaMonsterCodex(FilterState),
    #[name = "Pirates Of The Inner Sea"]
    PiratesOfTheInnerSea(FilterState),
    #[name = "Knights Of The Inner Sea"]
    KnightsOfTheInnerSea(FilterState),

    #[name = "Occult Adventures"]
    OccultAdventures(FilterState),
    #[name = "Occult Mysteries"]
    OccultMysteries(FilterState),
    #[name = "Occult Origins"]
    OccultOrigins(FilterState),
    #[name = "Occult Realms"]
    OccultRealms(FilterState),

    #[name = "Mythic Adventures"]
    MythicAdventures(FilterState),
    #[name = "Mythic Origins"]
    MythicOrigins(FilterState),

    #[name = "Classic Treasures"]
    ClassicTreasures(FilterState),
    #[name = "Faction Guide"]
    FactionGuide(FilterState),
    #[name = "Cheliax Empire Of Devils"]
    CheliaxEmpireOfDevils(FilterState),
    #[name = "Horsemen Of The Apocalypse"]
    HorsemenOfTheApocalypse(FilterState),
    Sargava(FilterState),
    Apg(FilterState),
    #[name = "Rival Guide"]
    RivalGuide(FilterState),
    #[name = "Paizo Blog"]
    PaizoBlog(FilterState),

    #[name = "Dragon Empires Primer"]
    DragonEmpiresPrimer(FilterState),
    #[name = "Lost Kingdoms"]
    LostKingdoms(FilterState),
    #[name = "RotRL-AE-Appendix"]
    RotRlAeAppendix(FilterState),
    #[name = "Blood Of The Night"]
    BloodOfTheNight(FilterState),
    #[name = "People Of The North"]
    PeopleOfTheNorth(FilterState),
    #[name = "Animal Archive"]
    AnimalArchive(FilterState),
    #[name = "Condition Cards"]
    ConditionCards(FilterState),
    #[name = "Chronicle Of The Righteous"]
    ChronicleOfTheRighteous(FilterState),
    #[name = "Quests and Campaigns"]
    QuestsAndCampaigns(FilterState),
    #[name = "The Dragon's Demand"]
    TheDragonsDemand(FilterState),
    #[name = "Faiths & Philosophies"]
    FaithsPhilosophies(FilterState),
    #[name = "Demons Revisited"]
    DemonsRevisited(FilterState),
    #[name = "Blood Of The Moon"]
    BloodOfTheMoon(FilterState),
    #[name = "Magical Marketplace"]
    MagicalMarketplace(FilterState),
    #[name = "Osirion, Legacy Of Pharaohs"]
    OsirionLegacyOfPharaos(FilterState),
    #[name = "People Of The Sands"]
    PeopleOfTheSands(FilterState),
    #[name = "Blood Of The Elements"]
    BloodOfTheElements(FilterState),
    #[name = "People Of The River"]
    PeopleOfTheRiver(FilterState),
    #[name = "Technology Guide"]
    TechnologyGuide(FilterState),
    #[name = "People Of The Stars"]
    PeopleOfTheStars(FilterState),
    #[name = "Ranged Tactics Toolbox"]
    RangedTacticsToolbox(FilterState),
    #[name = "Familiar Folio"]
    FamiliarFolio(FilterState),
    #[name = "Melee Tactics Toolbox"]
    MeleeTacticsToolbox(FilterState),
    #[name = "Cohorts & Companions"]
    CohortsAndCompanions(FilterState),
    #[name = "Dirty Tactics Toolbox"]
    DirtyTacticsToolbox(FilterState),
    #[name = "Black Markets"]
    BlackMarkets(FilterState),
    #[name = "Agents Of Evil"]
    AgentsOfEvil(FilterState),
    #[name = "Arcane Anthology"]
    ArcaneAnthology(FilterState),
    #[name = "Blood Of Shadows"]
    BloodOfShadows(FilterState),
    #[name = "Magic Tactics Toolbox"]
    MagicTacticsToolbox(FilterState),
    #[name = "Legacy Of Dragons"]
    LegacyOfDragons(FilterState),
    #[name = "Horror Adventures"]
    HorrorAdventures(FilterState),
    #[name = "Planes Of Power"]
    PlanesOfPower(FilterState),
    #[name = "Divine Anthology"]
    DivineAnthology(FilterState),
    #[name = "Curse Of The Crimson Throne Chapter Appendix"]
    CurseOfTheCrimsonThroneChapterAppendix(FilterState),
    #[name = "Blood Of The Beast"]
    BloodOfTheBeast(FilterState),
    #[name = "Paths Of The Righteous"]
    PathsOfTheRighteous(FilterState),
    #[name = "The First World Realm Of The Fey"]
    TheFirstWorldRealmOfTheFey(FilterState),
    #[name = "Seekers of Secrets"]
    SeekersOfSecrets(FilterState),
    #[name = "Villain Codex"]
    VillainCodex(FilterState),
    #[name = "Qadira Jewel Of The East"]
    QadiraJewelOfTheEast(FilterState),
    #[name = "Psychic Anthology"]
    PsychicAnthology(FilterState),
    #[name = "Legacy Of The First World"]
    LegacyOfTheFirstWorld(FilterState),
    #[name = "Adventurer's Guide"]
    AdventurersGuide(FilterState),
    #[name = "Adventurer's Armory 2"]
    AdventurersArmory2(FilterState),
    #[name = "Aquatic Adventures"]
    AquaticAdventures(FilterState),
    #[name = "Book of the Damned"]
    BookOfTheDamned(FilterState),
    #[name = "Planar Adventures"]
    PlanarAdventures(FilterState),
    #[name = "Sword of Air"]
    SwordOfAir(FilterState),
    #[name = "Rappan Athuk"]
    RappanAthuk(FilterState),
    #[name = "Distant Realms"]
    DistantRealms(FilterState),

    #[name = "AP 29"]
    AP29(FilterState),
    #[name = "AP 30"]
    AP30(FilterState),
    #[name = "AP 35"]
    AP35(FilterState),
    #[name = "AP 42"]
    AP42(FilterState),
    #[name = "AP 50"]
    AP50(FilterState),
    #[name = "AP 55"]
    AP55(FilterState),
    #[name = "AP 56"]
    AP56(FilterState),
    #[name = "AP 62"]
    AP62(FilterState),
    #[name = "AP 64"]
    AP64(FilterState),
    #[name = "AP 65"]
    AP65(FilterState),
    #[name = "AP 67"]
    AP67(FilterState),
    #[name = "AP 68"]
    AP68(FilterState),
    #[name = "AP 69"]
    AP69(FilterState),
    #[name = "AP 71"]
    AP71(FilterState),
    #[name = "AP 74"]
    AP74(FilterState),
    #[name = "AP 77"]
    AP77(FilterState),
    #[name = "AP 78"]
    AP78(FilterState),
    #[name = "AP 80"]
    AP80(FilterState),
    #[name = "AP 81"]
    AP81(FilterState),
    #[name = "AP 82"]
    AP82(FilterState),
    #[name = "AP 84"]
    AP84(FilterState),
    #[name = "AP 86"]
    AP86(FilterState),
    #[name = "AP 89"]
    AP89(FilterState),
    #[name = "AP 91"]
    AP91(FilterState),
    #[name = "AP 93"]
    AP93(FilterState),
    #[name = "AP 95"]
    AP95(FilterState),
    #[name = "AP 102"]
    AP102(FilterState),
    #[name = "AP 107"]
    AP107(FilterState),
    #[name = "AP 110"]
    AP110(FilterState),
    #[name = "AP 113"]
    AP113(FilterState),
    #[name = "AP 115"]
    AP115(FilterState),
    #[name = "AP 116"]
    AP116(FilterState),
    #[name = "AP 117"]
    AP117(FilterState),
    #[name = "AP 119"]
    AP119(FilterState),
    #[name = "AP 131"]
    AP131(FilterState),
    #[name = "AP 134"]
    AP134(FilterState),
    #[name = "AP 135"]
    AP135(FilterState),
    #[name = "AP 140"]
    AP140(FilterState),
    #[name = "AP 143"]
    AP143(FilterState),
}
