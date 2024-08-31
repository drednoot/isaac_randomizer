mod randomizer;

use randomizer::characters::Character;
use randomizer::pool::Unlocks;
use randomizer::targets::Target;
use std::collections::HashSet;

fn main() {
    let mut unlocks = Unlocks::default();
    unlocks
        .set_unlocked_chars(HashSet::from([Character::Isaac]))
        // .set_unlocked_targets(HashSet::from([Target::Mom]));
        .add_marks(Character::Isaac, Target::Mom.into())
        .add_marks(Character::Isaac, Target::Heart.into())
        .add_marks(Character::Isaac, Target::UltraGreed.into())
        .set_mantle_unlocked(true);

    let unlocks = unlocks;

    match unlocks.get_random_pick() {
        Some((ch, targ)) => println!("{}\n\nVS\n\n{}", ch, targ),
        None => println!("Something went wrong !!"),
    };
}
