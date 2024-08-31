use crate::randomizer::characters::Character;
use crate::randomizer::dependency::{Dependency, DependencyValue, HasDependency, ItLives, Mantle};
use enumflags2::bitflags;
use enumflags2::BitFlags;
use std::collections::HashSet;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[bitflags]
#[repr(u16)]
#[derive(Copy, Clone, EnumIter, EnumCountMacro, Debug, Hash, Eq, PartialEq)]
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
    pub fn get_remaining(completed: &BitFlags<Target>) -> HashSet<Target> {
        Target::iter()
            .filter(|item| !completed.contains(BitFlags::from_flag(item.to_owned())))
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
        use Dependency::*;
        use Target::*;
        match self {
            BlueBaby => Singular(DependencyValue::Target(Isaac)),
            Lamb => Singular(DependencyValue::Target(Satan)),
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
            Heart => Singular(DependencyValue::Target(Mom)),
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
}
