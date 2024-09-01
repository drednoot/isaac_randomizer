mod randomizer;
mod toml_parse;

use randomizer::characters::Character;
use randomizer::pool::Unlocks;
use randomizer::targets::Target;
use std::collections::HashSet;
use toml_parse::savefile::Savefile;

fn main() {
    let mut unlocks = Unlocks::default();
    unlocks
        .set_marks(
            Character::Isaac,
            HashSet::from([Target::Heart, Target::Satan, Target::Isaac]),
        )
        .set_mom_beaten(true)
        .set_it_lives_unlocked(true)
        .set_polaroid_unlocked(true);
    // .set_everything_unlocked();
    let unlocks = unlocks;

    let sf: Savefile = unlocks.into();
    match sf.write_to_file("test.toml".to_string()) {
        Err(e) => eprintln!("{:?}", e),
        Ok(_) => {}
    }

    let sf2: Savefile = match Savefile::read_from_file("test.toml".to_string()) {
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
        Ok(s) => s,
    };
    let unlocks: Unlocks = match sf2.try_into() {
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
        Ok(u) => u,
    };

    println!("{:?}", unlocks);

    // match unlocks.get_random_pick() {
    //     Some((ch, targs)) => {
    //         print!("{}\n\nVS\n\n", ch);
    //         let mut sorted: Vec<Target> = targs.into_iter().collect();
    //         sorted.sort();

    //         for targ in sorted {
    //             println!("{}", targ);
    //         }
    //     }
    //     None => println!("Something went wrong !!"),
    // };
}
