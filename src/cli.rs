use crate::randomizer::pool::Unlocks;
use crate::randomizer::targets::Target;
use crate::toml_parse::savefile::Savefile;
use crate::toml_parse::savefile::Error;

use clap::{Parser, Subcommand, CommandFactory};

#[derive(Parser, Debug)]
#[command(name = "srati", version = "0.1", about = "Smart Randomized Adventures of Tormented Isaac")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        long_about = "unlock character/target/unlock\n\
            Available characters to unlock:\n\
            \tMagdalene\n\
            \tCain\n\
            \tJudas\n\
            \t???\n\
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
            Available Targets to unlock:\n\
            \t???_Boss\n\
            \tThe_Lamb\n\
            \tMega_Satan\n\
            \tDelirium\n\
            \tBeast\n\
            \tMother\n\
            \tHush\n\
            \tSatan\n\
            \tIsaac\n\
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
            \t???\n\
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
            \t???\n\
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
        long_about = "remove completion marks from a character\n\
        For a full list of available characters use `srati mark --help`",
    )]
    Unmark {
        #[arg(value_name = "character")]
        character: String,
    },

    #[command(
        long_about = "set certain options to configure srati\n\
            Available options:\n\
            \tbossrush <chance from 0.0 to 1.0> -- chance to roll boss rush, 1.0 for 100%\n\
            \thush <chance from 0.0 to 1.0> -- chance to roll hush, 1.0 for 100%\n\
            \tbossrushalt <true/false> -- roll boss rush on alt path (Mother/Beast)"
    )]
    Set {
        #[arg(value_name = "key")]
        key: String,
        #[arg(value_name = "value")]
        value: Option<String>,
    },
}

pub fn parse_cmd() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Unlock { unlocks }) => {
            if unlocks.is_empty() {
                Cli::command()
                    .find_subcommand_mut("unlock")
                    .unwrap()
                    .print_help()
                    .unwrap();
                println!();
                std::process::exit(1);
            }
            for item in unlocks {
                println!("Unlocking: {}", item);
            }
        }

        Some(Commands::Ununlock { unlocks }) => {
            if unlocks.is_empty() {
                Cli::command()
                    .find_subcommand_mut("ununlock")
                    .unwrap()
                    .print_help()
                    .unwrap();
                println!();
                std::process::exit(1);
            }
            for item in unlocks {
                println!("Locking: {}", item);
            }
        }

        Some(Commands::Mark { character, marks }) => {
            if character.is_empty() || marks.is_empty() {
                Cli::command()
                    .find_subcommand_mut("mark")
                    .unwrap()
                    .print_help()
                    .unwrap();
                println!();
                std::process::exit(1);
            }
            for mark in marks {
                println!("Marking for {}: {}", character, mark);
            }
        }

        Some(Commands::Unmark { character }) => {
            if character.is_empty() {
                Cli::command()
                    .find_subcommand_mut("unmark")
                    .unwrap()
                    .print_help()
                    .unwrap();
                println!();
                std::process::exit(1);
            }
            println!("Removing makrs for {}", character);
        }

        Some(Commands::Set { key, value }) => {
            let val = value.unwrap();
            if let Ok(b) = val.parse::<bool>() {
                println!("Setting {} = {}", key, b);
            } else if let Ok(f) = val.parse::<f64>() {
                println!("Setting {} = {}", key, f);
            }
        }

        None => {
            let SavefileInfo { unlocks, created_new_file } = match read_savefile() {
                Some(val) => val,
                None => return
            };
            get_random_pick(&unlocks);
            if created_new_file {
                save_to_savefile(&unlocks);
            }
        }
    }
}

struct SavefileInfo {
    unlocks: Unlocks,
    created_new_file: bool,
}

fn read_savefile() -> Option<SavefileInfo> {
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

fn get_random_pick(unlocks: &Unlocks) {
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

fn save_to_savefile(unlocks: &Unlocks) {
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
    "sf.toml".to_string()
}
