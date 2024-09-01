mod randomizer;

use randomizer::characters::Character;
use randomizer::pool::Unlocks;
use randomizer::targets::Target;
use std::collections::HashSet;

fn main() {
    let mut unlocks = Unlocks::default();
    unlocks
        .set_marks(
            Character::Isaac,
            HashSet::from([Target::Heart, Target::Satan, Target::Isaac]),
        )
        .set_is_mom_beaten(true)
        .set_it_lives_unlocked(true);

    let unlocks = unlocks;

    match unlocks.get_random_pick() {
        Some((ch, targs)) => {
            print!("{}\n\nVS\n\n", ch);
            for targ in targs {
                println!("{}", targ);
            }
        }
        None => println!("Something went wrong !!"),
    };
}
