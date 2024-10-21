use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum GameType {
    Roulette(RouletteVariant),
    CardGame(CardGameVariant),
    Poker(PokerVariant),
    Slots(SlotsVariant),
    Lottery(LotteryVariant),
    Racing(RacingVariant),
    Dice(DiceVariant),
    TableGame(TableGameVariant),
    AsianGame(AsianGameVariant),
    SiciwinGame(SiciwinGameVariant),
    Other(OtherGameVariant),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum RouletteVariant {
    American,
    European,
    French,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum CardGameVariant {
    Blackjack,
    Baccarat,
    PaiGowPoker,
    ThreeCardPoker,
    FourCardPoker,
    LetItRide,
    CasinoWar,
    RedDog,
    Pontoon,
    Spanish21,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum PokerVariant {
    TexasHoldem,
    Omaha,
    SevenCardStud,
    CaribbeanStud,
    VideoPoker,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum SlotsVariant {
    SlotMachines,
    VideoSlots,
    ProgressiveSlots,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum LotteryVariant {
    SiciwinLottery,
    Keno,
    Bingo,
    BingoMatch,
    FastLottery,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum RacingVariant {
    HorseRacing,
    DogRacing,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum DiceVariant {
    Craps,
    ChuckALuck,
    SiciwinDice,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum TableGameVariant {
    BigSixWheel,
    WheelOfFortune,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum AsianGameVariant {
    PaiGowTiles,
    Pachinko,
    Mahjong,
    Fantan,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum SiciwinGameVariant {
    SiciwinGames,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]

pub enum OtherGameVariant {
    SportswinBetting,
    Backgammon,
}

impl GameType {
    pub fn to_db_string(&self) -> &str {
        match self {
            GameType::Roulette(RouletteVariant::American) => "AMERICAN_ROULETTE",
            GameType::Roulette(RouletteVariant::European) => "EUROPEAN_ROULETTE",
            GameType::Roulette(RouletteVariant::French) => "FRENCH_ROULETTE",
            GameType::CardGame(CardGameVariant::Blackjack) => "BLACKJACK",
            GameType::CardGame(CardGameVariant::Baccarat) => "BACCARAT",
            GameType::Poker(PokerVariant::TexasHoldem) => "POKER_TEXAS_HOLDEM",
            GameType::Poker(PokerVariant::Omaha) => "POKER_OMAHA",
            GameType::Poker(PokerVariant::SevenCardStud) => "POKER_SEVEN_CARD_STUD",
            GameType::Poker(PokerVariant::CaribbeanStud) => "POKER_CARIBBEAN_STUD",
            GameType::Poker(PokerVariant::VideoPoker) => "VIDEO_POKER",
            GameType::Dice(DiceVariant::Craps) => "CRAPS",
            GameType::Lottery(LotteryVariant::SiciwinLottery) => "SICIWIN_LOTTERY",
            GameType::Lottery(LotteryVariant::Keno) => "KENO",
            GameType::Lottery(LotteryVariant::Bingo) => "BINGO",
            GameType::Slots(SlotsVariant::SlotMachines) => "SLOT_MACHINES",
            GameType::Slots(SlotsVariant::VideoSlots) => "VIDEO_SLOTS",
            GameType::Slots(SlotsVariant::ProgressiveSlots) => "PROGRESSIVE_SLOTS",
            GameType::CardGame(CardGameVariant::PaiGowPoker) => "PAI_GOW_POKER",
            GameType::AsianGame(AsianGameVariant::PaiGowTiles) => "PAI_GOW_TILES",
            GameType::Other(OtherGameVariant::SportswinBetting) => "SPORTSWIN_BETTING",
            GameType::Racing(RacingVariant::HorseRacing) => "HORSE_RACING",
            GameType::Racing(RacingVariant::DogRacing) => "DOG_RACING",
            GameType::TableGame(TableGameVariant::WheelOfFortune) => "WHEEL_OF_FORTUNE",
            GameType::AsianGame(AsianGameVariant::Pachinko) => "PACHINKO",
            GameType::AsianGame(AsianGameVariant::Mahjong) => "MAHJONG",
            GameType::AsianGame(AsianGameVariant::Fantan) => "FANTAN",
            GameType::SiciwinGame(SiciwinGameVariant::SiciwinGames) => "SICIWIN_GAMES",
            GameType::CardGame(CardGameVariant::ThreeCardPoker) => "THREE_CARD_POKER",
            GameType::CardGame(CardGameVariant::FourCardPoker) => "FOUR_CARD_POKER",
            GameType::CardGame(CardGameVariant::LetItRide) => "LET_IT_RIDE",
            GameType::CardGame(CardGameVariant::CasinoWar) => "CASINO_WAR",
            GameType::CardGame(CardGameVariant::RedDog) => "RED_DOG",
            GameType::CardGame(CardGameVariant::Pontoon) => "PONTOON",
            GameType::CardGame(CardGameVariant::Spanish21) => "SPANISH_21",
            GameType::Dice(DiceVariant::ChuckALuck) => "CHUCK_A_LUCK",
            GameType::TableGame(TableGameVariant::BigSixWheel) => "BIG_SIX_WHEEL",
            GameType::Dice(DiceVariant::SiciwinDice) => "SICIWIN_DICE",
            GameType::Other(OtherGameVariant::Backgammon) => "BACKGAMMON",
            GameType::Lottery(LotteryVariant::BingoMatch) => "BINGO_MATCH",
            GameType::Lottery(LotteryVariant::FastLottery) => "FAST_LOTTERY",
        }
    }

    pub fn from_db_string(s: &str) -> Option<Self> {
        match s {
            "AMERICAN_ROULETTE" => Some(GameType::Roulette(RouletteVariant::American)),
            "EUROPEAN_ROULETTE" => Some(GameType::Roulette(RouletteVariant::European)),
            "FRENCH_ROULETTE" => Some(GameType::Roulette(RouletteVariant::French)),
            "BLACKJACK" => Some(GameType::CardGame(CardGameVariant::Blackjack)),
            "BACCARAT" => Some(GameType::CardGame(CardGameVariant::Baccarat)),
            "POKER_TEXAS_HOLDEM" => Some(GameType::Poker(PokerVariant::TexasHoldem)),
            "POKER_OMAHA" => Some(GameType::Poker(PokerVariant::Omaha)),
            "POKER_SEVEN_CARD_STUD" => Some(GameType::Poker(PokerVariant::SevenCardStud)),
            "POKER_CARIBBEAN_STUD" => Some(GameType::Poker(PokerVariant::CaribbeanStud)),
            "VIDEO_POKER" => Some(GameType::Poker(PokerVariant::VideoPoker)),
            "CRAPS" => Some(GameType::Dice(DiceVariant::Craps)),
            "SICIWIN_LOTTERY" => Some(GameType::Lottery(LotteryVariant::SiciwinLottery)),
            "KENO" => Some(GameType::Lottery(LotteryVariant::Keno)),
            "BINGO" => Some(GameType::Lottery(LotteryVariant::Bingo)),
            "SLOT_MACHINES" => Some(GameType::Slots(SlotsVariant::SlotMachines)),
            "VIDEO_SLOTS" => Some(GameType::Slots(SlotsVariant::VideoSlots)),
            "PROGRESSIVE_SLOTS" => Some(GameType::Slots(SlotsVariant::ProgressiveSlots)),
            "PAI_GOW_POKER" => Some(GameType::CardGame(CardGameVariant::PaiGowPoker)),
            "PAI_GOW_TILES" => Some(GameType::AsianGame(AsianGameVariant::PaiGowTiles)),
            "SPORTSWIN_BETTING" => Some(GameType::Other(OtherGameVariant::SportswinBetting)),
            "HORSE_RACING" => Some(GameType::Racing(RacingVariant::HorseRacing)),
            "DOG_RACING" => Some(GameType::Racing(RacingVariant::DogRacing)),
            "WHEEL_OF_FORTUNE" => Some(GameType::TableGame(TableGameVariant::WheelOfFortune)),
            "PACHINKO" => Some(GameType::AsianGame(AsianGameVariant::Pachinko)),
            "MAHJONG" => Some(GameType::AsianGame(AsianGameVariant::Mahjong)),
            "FANTAN" => Some(GameType::AsianGame(AsianGameVariant::Fantan)),
            "SICIWIN_GAMES" => Some(GameType::SiciwinGame(SiciwinGameVariant::SiciwinGames)),
            "THREE_CARD_POKER" => Some(GameType::CardGame(CardGameVariant::ThreeCardPoker)),
            "FOUR_CARD_POKER" => Some(GameType::CardGame(CardGameVariant::FourCardPoker)),
            "LET_IT_RIDE" => Some(GameType::CardGame(CardGameVariant::LetItRide)),
            "CASINO_WAR" => Some(GameType::CardGame(CardGameVariant::CasinoWar)),
            "RED_DOG" => Some(GameType::CardGame(CardGameVariant::RedDog)),
            "PONTOON" => Some(GameType::CardGame(CardGameVariant::Pontoon)),
            "SPANISH_21" => Some(GameType::CardGame(CardGameVariant::Spanish21)),
            "CHUCK_A_LUCK" => Some(GameType::Dice(DiceVariant::ChuckALuck)),
            "BIG_SIX_WHEEL" => Some(GameType::TableGame(TableGameVariant::BigSixWheel)),
            "SICIWIN_DICE" => Some(GameType::Dice(DiceVariant::SiciwinDice)),
            "BACKGAMMON" => Some(GameType::Other(OtherGameVariant::Backgammon)),
            "BINGO_MATCH" => Some(GameType::Lottery(LotteryVariant::BingoMatch)),
            "FAST_LOTTERY" => Some(GameType::Lottery(LotteryVariant::FastLottery)),
            _ => None,
        }
    }
}
