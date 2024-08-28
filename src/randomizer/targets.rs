use enumflags2::bitflags;
use strum_macros::{EnumIter, EnumCount as EnumCountMacro};
use strum::IntoEnumIterator;
use enumflags2::BitFlags;
use std::fmt;
use std::collections::HashSet;

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
        }
    }
}

