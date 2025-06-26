mod commands;
mod cli_structs;

use clap::{Parser, CommandFactory};
use commands::*;
use cli_structs::*;
use std::collections::HashSet;

fn print_help(subcommand: &str) -> ! {
    Cli::command()
        .find_subcommand_mut(subcommand)
        .unwrap()
        .print_help()
        .unwrap();
    std::process::exit(1)
}

macro_rules! print_help_msg {
    ($subcmd:expr, $fmt:expr $(, $arg:expr )* $(,)? ) => {{
        Cli::command()
            .find_subcommand_mut($subcmd)
            .expect(concat!("unknown subcommand: ", $subcmd))
            .print_help()
            .unwrap();
        eprintln!("\n{}", format!($fmt $(, $arg )*));
        std::process::exit(1);
    }};
}

pub fn parse_cmd() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Unlock { unlocks }) => {
            if unlocks.is_empty() {
                print_help("unlock");
            }

            let SavefileInfo { unlocks: mut file_unlocks, created_new_file } = match read_savefile() {
                Some(val) => val,
                None => std::process::exit(1)
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
                        println!("Skipping unlocking {}: no such item found.", item);
                    }
                };
            }

            if savefile_updated || created_new_file {
                save_to_savefile(&file_unlocks);
            }
        }

        Some(Commands::Ununlock { unlocks }) => {
            if unlocks.is_empty() {
                print_help("ununlock");
            }

            let SavefileInfo { unlocks: mut file_unlocks, created_new_file } = match read_savefile() {
                Some(val) => val,
                None => std::process::exit(1)
            };
            let mut savefile_updated = false;

            for item in unlocks {
                match Unlock::from_unlock_arg(item.as_str()) {
                    Some(u) => {
                        match u {
                            Unlock::Character(char) => file_unlocks.remove_unlocked_chars(&HashSet::from([char])),
                            Unlock::Target(targ) => file_unlocks.remove_unlocked_targets(&HashSet::from([targ])),
                            Unlock::Unlockable(unl) => match unl {
                                Unlockable::ItLives => file_unlocks.set_it_lives_unlocked(false),
                                Unlockable::Polaroid => file_unlocks.set_polaroid_unlocked(false),
                                Unlockable::Negative => file_unlocks.set_negative_unlocked(false),
                                Unlockable::HolyMantle => file_unlocks.set_mantle_unlocked(false),
                            }
                        };
                        savefile_updated = true;
                    }
                    None => {
                        println!("Skipping locking {}: no such item found.", item);
                    }
                };
            }

            if savefile_updated || created_new_file {
                save_to_savefile(&file_unlocks);
            }
        }

        Some(Commands::Mark { character: char_str, marks: marks_strs }) => {
            if char_str.is_empty() || marks_strs.is_empty() {
                print_help("mark");
            }

            let char = match Unlock::try_str_to_character(char_str.as_str()) {
                Some(c) => c,
                None => print_help_msg!("mark", "No such character: {}", char_str),
            };

            let SavefileInfo { unlocks: mut file_unlocks, created_new_file } = match read_savefile() {
                Some(val) => val,
                None => std::process::exit(1)
            };
            let mut savefile_updated = false;

            for mark_str in marks_strs {
                match Unlock::try_str_to_target(mark_str.as_str(), false) {
                    Some(targ) => {
                        file_unlocks.add_marks(char, HashSet::from([targ]));
                        savefile_updated = true;
                    }
                    None => {
                        println!("Skipping adding mark {} to character {}: no such target found.", mark_str, char_str);
                    }
                }
            }

            if savefile_updated || created_new_file {
                save_to_savefile(&file_unlocks);
            }
        }

        Some(Commands::Unmark { character: char_str, marks: marks_strs_opt }) => {
            if char_str.is_empty() {
                print_help("unmark");
            }

            let char = match Unlock::try_str_to_character(char_str.as_str()) {
                Some(c) => c,
                None => print_help_msg!("unmark", "No such character: {}", char_str),
            };

            let SavefileInfo { unlocks: mut file_unlocks, created_new_file } = match read_savefile() {
                Some(val) => val,
                None => std::process::exit(1)
            };
            let mut savefile_updated = false;

            match marks_strs_opt {
                Some(marks_strs) => {
                    for mark_str in marks_strs {
                        match Unlock::try_str_to_target(mark_str.as_str(), false) {
                            Some(targ) => {
                                file_unlocks.remove_marks(&char, &HashSet::from([targ]));
                                savefile_updated = true;
                            }
                            None => {
                                println!("Skipping removing mark {} from character {}: no such target found.", mark_str, char_str);
                            }
                        }
                    }
                }
                None => {
                    file_unlocks.remove_all_marks(&char);
                    savefile_updated = true;
                }
            }

            if savefile_updated || created_new_file {
                save_to_savefile(&file_unlocks);
            }
        }

        Some(Commands::Set { key, value }) => {
            if key.is_empty() {
                print_help("set");
            }

            let SavefileInfo { unlocks: mut file_unlocks, created_new_file } = match read_savefile() {
                Some(val) => val,
                None => std::process::exit(1)
            };
            let savefile_updated;

            match key.as_str() {
                "bossrush" => {
                    match value {
                        Some(val) => {
                            match val.parse::<f32>() {
                                Ok(chance) => {
                                    if chance >= 0. && chance <= 1. {
                                        file_unlocks.set_boss_rush_chance(chance);
                                        savefile_updated = true;
                                    } else {
                                        print_help_msg!("set", "Chance {} is not between 0.0 and 1.0", chance);
                                    }
                                }
                                Err(_) => {
                                    print_help_msg!("set", "Chance must be a value between 0.0 and 1.0 (your input was {})", val);
                                }
                            } 
                        }
                        None => {
                            print_help_msg!("set", "Boss rush chance was not provided");
                        }
                    }
                }
                "hush" => {
                    match value {
                        Some(val) => {
                            match val.parse::<f32>() {
                                Ok(chance) => {
                                    if chance >= 0. && chance <= 1. {
                                        file_unlocks.set_hush_chance(chance);
                                        savefile_updated = true;
                                    } else {
                                        print_help_msg!("set", "Chance {} is not between 0.0 and 1.0", chance);
                                    }
                                }
                                Err(_) => {
                                    print_help_msg!("set", "Chance must be a value between 0.0 and 1.0 (your input was {})", val);
                                }
                            } 
                        }
                        None => {
                            print_help_msg!("set", "Hush chance was not provided");
                        }
                    }
                }
                "bossrushalt" => {
                    match value {
                        Some(val) => {
                            match val.parse::<bool>() {
                                Ok(b) => {
                                    file_unlocks.set_roll_boss_rush_on_alt(b);
                                    savefile_updated = true;
                                }
                                Err(_) => {
                                    print_help_msg!("set", "Must input true/false value (your input was {})", val);
                                }
                            } 
                        }
                        None => {
                            file_unlocks.set_roll_boss_rush_on_alt(true);
                            savefile_updated = true;
                        }
                    }
                }
                _ => {
                    print_help("set");
                }
            }

            if savefile_updated || created_new_file {
                save_to_savefile(&file_unlocks);
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
