mod commands;
mod cli_structs;

use clap::{Parser, CommandFactory};
use commands::*;
use cli_structs::*;
use std::collections::HashSet;

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

            let SavefileInfo { unlocks: mut file_unlocks, created_new_file } = match read_savefile() {
                Some(val) => val,
                None => return
            };
            let mut savefile_updated = false;

            for item in unlocks {
                match Unlock::from_unlock_arg(item.as_str()) {
                    Some(u) => {
                        match u {
                            Unlock::Character(char) => file_unlocks.add_unlocked_chars(HashSet::from([char])),
                            Unlock::Target(targ) => file_unlocks.add_unlocked_targets(HashSet::from([targ])),
                            Unlock::Unlockable(unl) => match unl {
                                Unlockable::ItLives => file_unlocks.set_it_lives_unlocked(true),
                                Unlockable::Polaroid => file_unlocks.set_polaroid_unlocked(true),
                                Unlockable::Negative => file_unlocks.set_negative_unlocked(true),
                                Unlockable::HolyMantle => file_unlocks.set_mantle_unlocked(true),
                            }
                        };
                        savefile_updated = true;
                    }
                    None => {
                        println!("Skipping unlocking {}, no such item found.", item);
                        Cli::command()
                            .find_subcommand_mut("unlock")
                            .unwrap()
                            .print_help()
                            .unwrap();
                        return;
                    }
                };
            }

            if savefile_updated || created_new_file {
                save_to_savefile(&file_unlocks);
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
