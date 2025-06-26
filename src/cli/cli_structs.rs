use crate::randomizer::{targets, characters};

pub enum Unlockable {
    ItLives,
    Polaroid,
    Negative,
    HolyMantle,
}

pub enum Unlock {
    Character(characters::Character),
    Target(targets::Target),
    Unlockable(Unlockable),
}

impl Unlock {
    pub fn from_unlock_arg(arg: &str) -> Option<Self> {
        let lower_arg_binding = arg.to_lowercase();
        let lower_arg = lower_arg_binding.as_str();

        if let Some(char) = Self::try_str_to_character(lower_arg) {
            return Some(Self::Character(char));
        }

        if let Some(targ) = Self::try_str_to_target(lower_arg, true) {
            return Some(Self::Target(targ));
        }

        match Self::try_str_to_unlockable(lower_arg) {
            Some(u) => Some(Self::Unlockable(u)),
            None => None,
        }
    }

    pub fn try_str_to_character(str: &str) -> Option<characters::Character> {
        use characters::Character::*;

        match str {
            "isaac" => Some(Isaac),
            "magdalene" => Some(Magdalene),
            "cain" => Some(Cain),
            "judas" => Some(Judas),
            "blue_baby" => Some(BlueBaby),
            "eve" => Some(Eve),
            "samson" => Some(Samson),
            "azazel" => Some(Azazel),
            "lazarus" => Some(Lazarus),
            "eden" => Some(Eden),
            "lost" => Some(Lost),
            "lilith" => Some(Lilith),
            "keeper" => Some(Keeper),
            "apollyon" => Some(Apollyon),
            "forgotten" => Some(Forgotten),
            "bethany" => Some(Bethany),
            "jacob_And_esau" => Some(JacobAndEsau),
            "tainted_isaac" => Some(TaintedIsaac),
            "tainted_magdalene" => Some(TaintedMagdalene),
            "tainted_cain" => Some(TaintedCain),
            "tainted_judas" => Some(TaintedJudas),
            "tainted_blue_baby" => Some(TaintedBlueBaby),
            "tainted_eve" => Some(TaintedEve),
            "tainted_samson" => Some(TaintedSamson),
            "tainted_azazel" => Some(TaintedAzazel),
            "tainted_lazarus" => Some(TaintedLazarus),
            "tainted_eden" => Some(TaintedEden),
            "tainted_lost" => Some(TaintedLost),
            "tainted_lilith" => Some(TaintedLilith),
            "tainted_keeper" => Some(TaintedKeeper),
            "tainted_apollyon" => Some(TaintedApollyon),
            "tainted_forgotten" => Some(TaintedForgotten),
            "tainted_bethany" => Some(TaintedBethany),
            "tainted_jacob" => Some(TaintedJacob),
            _ => None,
        }
    }

    pub fn try_str_to_target(str: &str, require_disambiguation: bool) -> Option<targets::Target> {
        use targets::Target::*;

        match str {
            "blue_baby" if !require_disambiguation => Some(BlueBaby),
            "blue_baby_boss" => Some(BlueBaby),
            "the_lamb" => Some(Lamb),
            "mega_satan" => Some(MegaSatan),
            "delirium" => Some(Delirium),
            "beast" => Some(Beast),
            "mother" => Some(Mother),
            "ultra_greed" => Some(UltraGreed),
            "hush" => Some(Hush),
            "satan" => Some(Satan),
            "isaac" if !require_disambiguation => Some(Isaac),
            "isaac_boss" => Some(Isaac),
            "moms_heart" => Some(Heart),
            "it_lives" if !require_disambiguation => Some(Heart),
            "it_lives_boss" if !require_disambiguation => Some(Heart),
            _ => None
        }
    }

    pub fn try_str_to_unlockable(str: &str) -> Option<Unlockable> {
        match str {
            "it_lives" => Some( Unlockable::ItLives ),
            "polaroid" => Some( Unlockable::Polaroid ),
            "negative" => Some( Unlockable::Negative ),
            "holy_mantle" => Some( Unlockable::HolyMantle ),
            _ => None
        }
    }
}
