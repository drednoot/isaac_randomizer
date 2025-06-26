use crate::randomizer::pool::Unlocks;
use crate::randomizer::targets::Target;
use crate::toml_parse::savefile::Savefile;
use crate::toml_parse::savefile::Error;

use std::env;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "srati", version = "0.1", about = "Smart Randomized Adventures of Tormented Isaac")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        long_about = "unlock character/target/unlock\n\
            Available characters to unlock:\n\
            \tIsaac\n\
            \tMagdalene\n\
            \tCain\n\
            \tJudas\n\
            \tBlue_Baby\n\
            \tEve\n\
            \tSamson\n\
            \tAzazel\n\
            \tLazarus\n\
            \tEden\n\
            \tLost\n\
            \tLilith\n\
            \tKeeper\n\
            \tApollyon\n\
            \tForgotten\n\
            \tBethany\n\
            \tJacob_&_Esau\n\
            \tTainted_Isaac\n\
            \tTainted_Magdalene\n\
            \tTainted_Cain\n\
            \tTainted_Judas\n\
            \tTainted_???\n\
            \tTainted_Eve\n\
            \tTainted_Samson\n\
            \tTainted_Azazel\n\
            \tTainted_Lazarus\n\
            \tTainted_Eden\n\
            \tTainted_Lost\n\
            \tTainted_Lilith\n\
            \tTainted_Keeper\n\
            \tTainted_Apollyon\n\
            \tTainted_Forgotten\n\
            \tTainted_Bethany\n\
            \tTainted_Jacob\n\
            Available Targets to unlock:\n\
            \tBlue_Baby_Boss\n\
            \tThe_Lamb\n\
            \tMega_Satan\n\
            \tDelirium\n\
            \tBeast\n\
            \tMother\n\
            \tHush\n\
            \tSatan\n\
            \tIsaac_Boss\n\
            \tMoms_Heart\n\
            Available Unlocks to set unlocked:\n\
            \tIt_Lives\n\
            \tPolaroid\n\
            \tNegative\n\
            \tHoly_Mantle"
    )]
    Unlock {
        #[arg(value_name = "characters/targets/unlocks")]
        unlocks: Vec<String>,
    },

    #[command(
        long_about = "lock back characters/targets/unlocks\n\
        For a full list of available unlocks use `srati unlock --help`",
    )]
    Ununlock {
        #[arg(value_name = "characters/targets/unlocks")]
        unlocks: Vec<String>,
    },

    #[command(
        long_about = "set completion marks completed for a character\n\
            Available characters to specify:\n\
            \tIsaac\n\
            \tMagdalene\n\
            \tCain\n\
            \tJudas\n\
            \tBlue_Baby\n\
            \tEve\n\
            \tSamson\n\
            \tAzazel\n\
            \tLazarus\n\
            \tEden\n\
            \tLost\n\
            \tLilith\n\
            \tKeeper\n\
            \tApollyon\n\
            \tForgotten\n\
            \tBethany\n\
            \tJacob_And_Esau\n\
            \tTainted_Isaac\n\
            \tTainted_Magdalene\n\
            \tTainted_Cain\n\
            \tTainted_Judas\n\
            \tTainted_blue_baby\n\
            \tTainted_Eve\n\
            \tTainted_Samson\n\
            \tTainted_Azazael\n\
            \tTainted_Lazarus\n\
            \tTainted_Eden\n\
            \tTainted_Lost\n\
            \tTainted_Lilith\n\
            \tTainted_Keeper\n\
            \tTainted_Apollyon\n\
            \tTainted_Forgotten\n\
            \tTainted_Behtany\n\
            \tTainted_Jacob\n\
            Available Completion Marks to set:\n\
            \tBlue_Baby\n\
            \tThe_Lamb\n\
            \tMega_Satan\n\
            \tDelirium\n\
            \tBeast\n\
            \tMother\n\
            \tUltra_Greed\n\
            \tHush\n\
            \tSatan\n\
            \tIsaac\n\
            \tMoms_Heart\n\
            \tIt_Lives"
    )]
    Mark {
        #[arg(value_name = "character")]
        character: String,
        #[arg(value_name = "marks")]
        marks: Vec<String>,
    },

    #[command(
        long_about = "remove completion marks from a character (don't type marks argument for all marks)\n\
        For a full list of available characters use `srati mark --help`",
    )]
    Unmark {
        #[arg(value_name = "character")]
        character: String,

        #[arg(value_name = "marks")]
        marks: Option<Vec<String>>
    },

    #[command(
        long_about = "set certain options to configure srati\n\
            Available options:\n\
            \tbossrush <chance from 0.0 to 1.0> -- chance to roll boss rush, 1.0 for 100%\n\
            \thush <chance from 0.0 to 1.0> -- chance to roll hush, 1.0 for 100%\n\
            \tbossrushalt <true/false> -- roll boss rush on alt path (Mother/Beast); no true/false defaults to true"
    )]
    Set {
        #[arg(value_name = "key")]
        key: String,
        #[arg(value_name = "value")]
        value: Option<String>,
    },
}

pub struct SavefileInfo {
    pub unlocks: Unlocks,
    pub created_new_file: bool,
}

pub fn read_savefile() -> Option<SavefileInfo> {
    let mut created_new_file = false;
    let unlocks: Unlocks = match Savefile::read_from_file(get_savefile_path().to_string()) {
        Err(Error::IoError(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            created_new_file = true;
            Unlocks::default()
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return None;
        }
        Ok(s) => {
            match s.try_into() {
                Err(e) => {
                    eprintln!("{:?}", e);
                    return None;
                }
                Ok(u) => u,
            }
        }
    };

    Some(SavefileInfo {
        unlocks: unlocks,
        created_new_file,
    })
}

pub fn get_random_pick(unlocks: &Unlocks) {
    match unlocks.get_random_pick() {
        Some((ch, targs_set)) => {
            print!("{}\n\nVS\n\n", ch);
            let mut targs: Vec<&Target> = targs_set.iter().collect();
            targs.sort();
            for targ in targs {
                println!("{}", targ);
            }
        }
        None => {
            println!("couldn't roll ):");
        }
    }
}

pub fn save_to_savefile(unlocks: &Unlocks) {
    let savefile: Savefile = Into::into(unlocks);
    match savefile.write_to_file(get_savefile_path().to_string()) {
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
        Ok(_) => {},
    }
}

fn get_savefile_path() -> String {
    match env::var("SRATI_SF") {
        Ok(val) => val,
        Err(_) => "sf.toml".to_string(),
    }
}
