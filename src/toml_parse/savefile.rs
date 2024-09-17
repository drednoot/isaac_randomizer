use crate::randomizer::characters::Character;
use crate::randomizer::pool::Unlocks;
use crate::randomizer::targets::Target;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    SerializeError(toml::ser::Error),
    DeserializeError(toml::de::Error),
    ParseError,
    IoError(std::io::Error),
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::SerializeError(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::DeserializeError(e)
    }
}

impl From<&'static str> for Error {
    fn from(_e: &'static str) -> Self {
        Error::ParseError
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Savefile {
    general_config: General,
    marks: HashMap<String, Marks>,
}

impl Savefile {
    pub fn new(general_config: General, marks: HashMap<String, Marks>) -> Self {
        Self {
            general_config,
            marks,
        }
    }

    pub fn write_to_file(&self, path: String) -> Result<(), Error> {
        let toml_str = toml::to_string(&self)?;

        match fs::write(path, toml_str) {
            Err(e) => Err(Error::IoError(e)),
            Ok(_) => Result::Ok(()),
        }
    }

    pub fn read_from_file(path: String) -> Result<Self, Error> {
        let toml_str = fs::read_to_string(path)?;

        match toml::from_str(&toml_str) {
            Err(e) => Err(Error::DeserializeError(e)),
            Ok(s) => Ok(s),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct General {
    unlocked_chars: Vec<String>,
    unlocked_targets: Vec<String>,
    is_mantle_unlocked: bool,
    is_it_lives_unlocked: bool,
    is_polaroid_unlocked: bool,
    is_negative_unlocked: bool,
    is_mom_beaten: bool,
    boss_rush_chance: f32,
    hush_chance: f32,
    roll_boss_rush_on_alt: bool,
}

impl General {
    pub fn new(
        unlocked_chars: Vec<String>,
        unlocked_targets: Vec<String>,
        is_mantle_unlocked: bool,
        is_it_lives_unlocked: bool,
        is_polaroid_unlocked: bool,
        is_negative_unlocked: bool,
        is_mom_beaten: bool,
        boss_rush_chance: f32,
        hush_chance: f32,
        roll_boss_rush_on_alt: bool,
    ) -> Self {
        Self {
            unlocked_chars,
            unlocked_targets,
            is_mantle_unlocked,
            is_it_lives_unlocked,
            is_polaroid_unlocked,
            is_negative_unlocked,
            is_mom_beaten,
            boss_rush_chance,
            hush_chance,
            roll_boss_rush_on_alt,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Marks {
    completed: Vec<String>,
}

impl Marks {
    pub fn new(completed: Vec<String>) -> Self {
        Self { completed }
    }
}

impl TryInto<Unlocks> for Savefile {
    type Error = Error;

    fn try_into(self) -> Result<Unlocks, Self::Error> {
        let mut unl = Unlocks::default();

        let mut chars = HashSet::new();
        for ch_str in self.general_config.unlocked_chars {
            let ch = Character::from_str(ch_str.as_str())?;
            chars.insert(ch);
        }
        unl.set_unlocked_chars(chars);

        let mut targs = HashSet::new();
        for targ_str in self.general_config.unlocked_targets {
            let targ = Target::from_str(targ_str.as_str())?;
            targs.insert(targ);
        }
        unl.set_unlocked_targets(targs);

        for (ch_str, marks) in self.marks {
            let ch = Character::from_str(ch_str.as_str())?;
            let mut marks_set = HashSet::new();
            for targ_str in marks.completed {
                let targ = Target::from_str(targ_str.as_str())?;
                marks_set.insert(targ);
            }
            unl.set_marks(ch, marks_set);
        }

        unl.set_mantle_unlocked(self.general_config.is_mantle_unlocked)
            .set_it_lives_unlocked(self.general_config.is_it_lives_unlocked)
            .set_polaroid_unlocked(self.general_config.is_polaroid_unlocked)
            .set_negative_unlocked(self.general_config.is_negative_unlocked)
            .set_mom_beaten(self.general_config.is_mom_beaten)
            .set_boss_rush_chance(self.general_config.boss_rush_chance)
            .set_hush_chance(self.general_config.hush_chance)
            .set_roll_boss_rush_on_alt(self.general_config.roll_boss_rush_on_alt);

        Ok(unl)
    }
}
