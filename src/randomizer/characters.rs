use crate::randomizer::dependency::{Dependency, DependencyValue, HasDependency, Mantle};
use crate::randomizer::targets::Target;
use std::fmt;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

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
    Bethany,
    JacobAndEsau,
    TaintedIsaac,
    TaintedMagdalene,
    TaintedCain,
    TaintedJudas,
    TaintedBlueBaby,
    TaintedEve,
    TaintedSamson,
    TaintedAzazel,
    TaintedLazarus,
    TaintedEden,
    TaintedLost,
    TaintedLilith,
    TaintedKeeper,
    TaintedApollyon,
    TaintedForgotten,
    TaintedBethany,
    TaintedJacob,
}

impl Character {
    pub fn is_tainted(&self) -> bool {
        use Character::*;
        match self {
            TaintedIsaac | TaintedMagdalene | TaintedCain | TaintedJudas | TaintedBlueBaby
            | TaintedEve | TaintedSamson | TaintedAzazel | TaintedLazarus | TaintedEden
            | TaintedLost | TaintedLilith | TaintedKeeper | TaintedApollyon | TaintedForgotten
            | TaintedBethany | TaintedJacob => true,
            _ => false,
        }
    }

    pub fn tainted_to_normal(&self) -> Option<Character> {
        use Character::*;
        match self {
            TaintedIsaac => Some(Isaac),
            TaintedMagdalene => Some(Magdalene),
            TaintedCain => Some(Cain),
            TaintedJudas => Some(Judas),
            TaintedBlueBaby => Some(BlueBaby),
            TaintedEve => Some(Eve),
            TaintedSamson => Some(Samson),
            TaintedAzazel => Some(Azazel),
            TaintedLazarus => Some(Lazarus),
            TaintedEden => Some(Eden),
            TaintedLost => Some(Lost),
            TaintedLilith => Some(Lilith),
            TaintedKeeper => Some(Keeper),
            TaintedApollyon => Some(Apollyon),
            TaintedForgotten => Some(Forgotten),
            TaintedBethany => Some(Bethany),
            TaintedJacob => Some(JacobAndEsau),
            _ => None,
        }
    }
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
            Bethany => write!(f, "Behtany"),
            JacobAndEsau => write!(f, "Jacob & Esau"),
            TaintedIsaac => write!(f, "Tainted Isaac"),
            TaintedMagdalene => write!(f, "Tainted Magdalene"),
            TaintedCain => write!(f, "Tainted Cain"),
            TaintedJudas => write!(f, "Tainted Judas"),
            TaintedBlueBaby => write!(f, "Tainted ???"),
            TaintedEve => write!(f, "Tainted Eve"),
            TaintedSamson => write!(f, "Tainted Samson"),
            TaintedAzazel => write!(f, "Tainted Azazael"),
            TaintedLazarus => write!(f, "Tainted Lazarus"),
            TaintedEden => write!(f, "Tainted Eden"),
            TaintedLost => write!(f, "Tainted Lost"),
            TaintedLilith => write!(f, "Tainted Lilith"),
            TaintedKeeper => write!(f, "Tainted Keeper"),
            TaintedApollyon => write!(f, "Tainted Apollyon"),
            TaintedForgotten => write!(f, "Tainted Forgotten"),
            TaintedBethany => write!(f, "Tainted Behtany"),
            TaintedJacob => write!(f, "Tainted Jacob"),
        }
    }
}

impl HasDependency for Character {
    fn depends_on(&self) -> Dependency {
        use Character::*;
        use Dependency::*;

        if self.is_tainted() {
            return Product(vec![
                DependencyValue::Target(Target::Beast),
                DependencyValue::Character(self.tainted_to_normal().unwrap()),
            ]);
        }

        match self {
            Isaac => None,
            Magdalene => None,
            Cain => None,
            Judas => Singular(DependencyValue::Target(Target::Satan)),
            BlueBaby => Singular(DependencyValue::Target(Target::Heart)),
            Eve => None,
            Samson => None,
            Azazel => None,
            Lazarus => None,
            Eden => Singular(DependencyValue::Target(Target::Heart)),
            Lost => Singular(DependencyValue::Mantle(Mantle)),
            Lilith => Product(vec![
                DependencyValue::Character(Azazel),
                DependencyValue::Target(Target::UltraGreed),
            ]),
            Keeper => Singular(DependencyValue::Target(Target::UltraGreed)),
            Apollyon => Singular(DependencyValue::Target(Target::MegaSatan)),
            Forgotten => None,
            Bethany => Singular(DependencyValue::Character(Lazarus)),
            JacobAndEsau => Singular(DependencyValue::Target(Target::Mother)),
            _ => None,
        }
    }
}
