mod randomizer;

use randomizer::characters::Character;
use randomizer::targets::Target;
use randomizer::pool::Unlocks;
use std::collections::HashSet;

fn main() {
    let mut unlocks = Unlocks::default();
    unlocks.set_unlocked_chars(HashSet::from([
        Character::Isaac,
        Character::Azazel,
        Character::BlueBaby,
    ]));
    unlocks.set_unlocked_targets(HashSet::from([
        Target::Lamb,
        Target::Mother,
        Target::BlueBaby,
    ]));
    unlocks.set_mantle_unlocked(true);
    let unlocks = unlocks;

    match unlocks.get_random_pick() {
        Some((ch, targ)) => println!("{}\n\nVS\n\n{}", ch, targ),
        None => println!("Something went wrong !!"),
    };
}
