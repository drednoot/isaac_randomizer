use crate::randomizer::characters::Character;
use crate::randomizer::dependency::{
    Dependency, DependencyValue, HasDependency, ItLives, Mantle, Negative, Polaroid,
};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Copy, Clone, EnumIter, EnumCountMacro, Debug, Hash, Eq, PartialEq)]
#[repr(C)]
pub enum Target {
    BlueBaby,
    Lamb,
    MegaSatan,
    Delirium,
    Beast,
    Mother,
    UltraGreed,
    BossRush,
    Hush,

    Satan,
    Isaac,
    Heart,

    Mom,
}

impl Target {
    pub fn get_remaining(completed: &HashSet<Target>) -> HashSet<Target> {
        Target::iter()
            .filter(|item| !completed.contains(item))
            .collect()
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Target::*;
        match self {
            BlueBaby => write!(f, "???"),
            Lamb => write!(f, "The Lamb"),
            MegaSatan => write!(f, "Mega Satan"),
            Delirium => write!(f, "Delirium"),
            Beast => write!(f, "Beast"),
            Mother => write!(f, "Mother"),
            UltraGreed => write!(f, "Ultra Greed"),
            BossRush => write!(f, "Boss Rush"),
            Hush => write!(f, "Hush"),

            Satan => write!(f, "Satan"),
            Isaac => write!(f, "Isaac"),
            Heart => write!(f, "Mom's Heart"),

            Mom => write!(f, "Mom"),
        }
    }
}

impl HasDependency for Target {
    fn depends_on(&self) -> Dependency {
        use crate::randomizer::dependency::Mom as DepMom;
        use Dependency::*;
        use Target::*;
        match self {
            BlueBaby => Singular(DependencyValue::Polaroid(Polaroid)),
            Lamb => Singular(DependencyValue::Negative(Negative)),
            MegaSatan => Sum(vec![
                DependencyValue::Target(Lamb),
                DependencyValue::Target(BlueBaby),
            ]),
            Delirium => Singular(DependencyValue::Target(Hush)),
            Beast => Singular(DependencyValue::Target(Mother)),
            Mother => Product(vec![
                DependencyValue::Target(Hush),
                DependencyValue::Mantle(Mantle),
            ]),
            UltraGreed => None,
            BossRush => None,
            Hush => Singular(DependencyValue::Character(Character::BlueBaby)),

            Satan => Singular(DependencyValue::ItLives(ItLives)),
            Isaac => Singular(DependencyValue::ItLives(ItLives)),
            Heart => Singular(DependencyValue::Mom(DepMom)),

            Mom => None,
        }
    }
}

impl Target {
    pub fn is_significant(&self) -> bool {
        use Target::*;
        match self {
            BlueBaby | Lamb | MegaSatan | Delirium | Beast | Mother | UltraGreed | BossRush
            | Hush => true,
            _ => false,
        }
    }

    pub fn precedence(&self) -> u8 {
        use Target::*;
        match self {
            Delirium => 7,
            MegaSatan => 6,
            BlueBaby => 5,
            Lamb => 4,
            Beast => 4,
            Mother => 4,
            Satan => 4,
            Isaac => 4,
            Hush => 3,
            Heart => 2,
            BossRush => 1,
            Mom => 0,

            UltraGreed => 0,
        }
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Target {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.precedence().cmp(&rhs.precedence())
    }
}

impl FromStr for Target {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Target::*;
        match s {
            "???" => Ok(BlueBaby),
            "The Lamb" => Ok(Lamb),
            "Mega Satan" => Ok(MegaSatan),
            "Delirium" => Ok(Delirium),
            "Beast" => Ok(Beast),
            "Mother" => Ok(Mother),
            "Ultra Greed" => Ok(UltraGreed),
            "Boss Rush" => Ok(BossRush),
            "Hush" => Ok(Hush),
            "Satan" => Ok(Satan),
            "Isaac" => Ok(Isaac),
            "Mom's Heart" => Ok(Heart),
            "Mom" => Ok(Mom),
            _ => Err("Could not convert string to Target"),
        }
    }
}
