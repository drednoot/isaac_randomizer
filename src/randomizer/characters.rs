use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;
use toml::de::Error;
use std::fmt;
use enumflags2::{BitFlags, make_bitflags};

use crate::randomizer::targets::Target;

const POOL: &str = "resources/pool.toml";
const SETTINGS: &str = "settings.toml";

#[derive(Deserialize)]
struct TomlMain {
    pool: Pool
}

#[derive(Deserialize)]
pub struct Pool {
    #[serde(rename = "characters")]
    chars: Vec<String>
}

impl Pool {
    pub fn new() -> Option<Self> {
        Self::from_toml()
    }

    fn from_toml() -> Option<Pool> {
        let contents = fs::read_to_string(POOL).ok()?;
        let result: Result<TomlMain, Error> = toml::from_str(contents.as_str());
        match result {
            Ok(t) => Some(t.pool),
            Err(_) => None
        }
    }

    pub fn get_random(&self) -> String {
        self.chars.choose(&mut rand::thread_rng()).unwrap().to_string()
    }
}

pub struct Character {
    name: String,
    target: BitFlags<Target>,
}

impl Character {
    pub fn new() -> Self {
        Self {
            name: String::from("asdf"),
            target: make_bitflags!(Target::{MegaSatan}),
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} has targets {}", self.name, self.target.bits())
    }
}
