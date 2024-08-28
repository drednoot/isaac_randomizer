use std::fmt;
use strum_macros::{EnumIter, EnumCount as EnumCountMacro};

// const SETTINGS: &str = "settings.toml";

#[derive(EnumIter, EnumCountMacro, Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Character {
	Isaac,
	Magdalene,
	Cain,
	Judas,
	BlueBaby,
	Eve,
	Samson,
	Azazel,
	Lazarus,
	Eden,
	Lost,
	Lilith,
	Keeper,
	Apollyon,
	Forgotten,
	Behtany,
	JacobAndEsau,
	TaintedIsaac,
	TaintedMagdalene,
	TaintedCain,
	TaintedJudas,
	TaintedBlueBaby,
	TaintedEve,
	TaintedSamson,
	TaintedAzazael,
	TaintedLazarus,
	TaintedEden,
	TaintedLost,
	TaintedLilith,
	TaintedKeeper,
	TaintedApollyon,
	TaintedForgotten,
	TaintedBehtany,
	TaintedJacob,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Character::*;
        match self {
            Isaac => write!(f, "Isaac"),
            Magdalene => write!(f, "Magdalene"),
            Cain => write!(f, "Cain"),
            Judas => write!(f, "Judas"),
            BlueBaby => write!(f, "???"),
            Eve => write!(f, "Eve"),
            Samson => write!(f, "Samson"),
            Azazel => write!(f, "Azazel"),
            Lazarus => write!(f, "Lazarus"),
            Eden => write!(f, "Eden"),
            Lost => write!(f, "Lost"),
            Lilith => write!(f, "Lilith"),
            Keeper => write!(f, "Keeper"),
            Apollyon => write!(f, "Apollyon"),
            Forgotten => write!(f, "Forgotten"),
            Behtany => write!(f, "Behtany"),
            JacobAndEsau => write!(f, "Jacob & Esau"),
            TaintedIsaac => write!(f, "Tainted Isaac"),
            TaintedMagdalene => write!(f, "Tainted Magdalene"),
            TaintedCain => write!(f, "Tainted Cain"),
            TaintedJudas => write!(f, "Tainted Judas"),
            TaintedBlueBaby => write!(f, "Tainted ???"),
            TaintedEve => write!(f, "Tainted Eve"),
            TaintedSamson => write!(f, "Tainted Samson"),
            TaintedAzazael => write!(f, "Tainted Azazael"),
            TaintedLazarus => write!(f, "Tainted Lazarus"),
            TaintedEden => write!(f, "Tainted Eden"),
            TaintedLost => write!(f, "Tainted Lost"),
            TaintedLilith => write!(f, "Tainted Lilith"),
            TaintedKeeper => write!(f, "Tainted Keeper"),
            TaintedApollyon => write!(f, "Tainted Apollyon"),
            TaintedForgotten => write!(f, "Tainted Forgotten"),
            TaintedBehtany => write!(f, "Tainted Behtany"),
            TaintedJacob => write!(f, "Tainted Jacob"),
        }
    }
}
