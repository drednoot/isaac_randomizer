use std::collections::{HashMap, HashSet};
use crate::randomizer::characters::Character;
use crate::randomizer::targets::Target;
use enumflags2::BitFlags;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Unlocks {
    marks: HashMap<Character, BitFlags<Target>>,
    unlocked_chars: HashSet<Character>,
    unlocked_targets: HashSet<Target>,
    is_mantle_unlocked: bool,
}

impl Default for Unlocks {
    fn default() -> Self {
        Self {
            marks: HashMap::new(),
            unlocked_chars: HashSet::from([Character::Isaac]),
            unlocked_targets: HashSet::new(),
            is_mantle_unlocked: false,
        }
    }
}

impl Unlocks {
    pub fn add_marks(&mut self, ch: Character, marks: BitFlags<Target>) {
        if !self.unlocked_chars.contains(&ch) {
            self.unlocked_chars.insert(ch);
        }
        marks
            .iter()
            .for_each(|targ| { 
                if !self.unlocked_targets.contains(&targ) {
                    self.unlocked_targets.insert(targ.clone()); 
                }
            });

        match self.marks.get_mut(&ch) {
            Some(completed) => completed.insert(marks),
            None => { self.marks.insert(ch, marks); },
        };
    }

    pub fn set_unlocked_chars(&mut self, chars: HashSet<Character>) {
        self.unlocked_chars = chars;

        let mut to_remove: Vec<Character> = Vec::new();
        for (ch, _) in &self.marks {
            if !self.unlocked_chars.contains(ch) {
                to_remove.push(*ch);
            }
        }

        for ch_to_remove in to_remove {
            self.unlocked_chars.remove(&ch_to_remove);
        }
    }

    pub fn set_unlocked_targets(&mut self, targets: HashSet<Target>) {
        self.unlocked_targets = targets;

        for (_, targets) in &mut self.marks {
            let mut to_remove: Vec<Target> = Vec::new();

            for targ in targets.iter() {
                if !self.unlocked_targets.contains(&targ) {
                    to_remove.push(targ);
                }
            }

            for targ_to_remove in to_remove {
                targets.remove(targ_to_remove);
            }
        }
    }

    pub fn set_mantle_unlocked(&mut self, is_unlocked: bool) {
        self.is_mantle_unlocked = is_unlocked;
    }

    pub fn get_random_pick(&self) -> Option<(Character, Target)> {
        let mut not_finished: HashSet<Character> = HashSet::new();
        for ch in &self.unlocked_chars {
            match self.marks.get(ch) {
                Some(targ) => {
                    if !targ.is_all() {
                        not_finished.insert(ch.clone());
                    }
                },
                None => {
                    not_finished.insert(ch.clone());
                },
            }
        }
        let not_finished = not_finished;

        let mut pool: HashMap<Character, HashSet<Target>> = HashMap::new();
        for ch in not_finished {
            if (ch == Character::Lost
                || ch == Character::TaintedLost)
                && !self.is_mantle_unlocked {
                    continue;
                }

            let mut valid_targets: HashSet<Target> = match self.marks.get(&ch) {
                Some(completed) => {
                    Target::get_remaining(completed)
                        .iter()
                        .filter(|rem| self.unlocked_targets.contains(rem))
                        .cloned()
                        .collect()

                },
                None => self.unlocked_targets.clone()
            };
            if valid_targets.contains(&Target::Mother) && !self.is_mantle_unlocked {
                valid_targets.remove(&Target::Mother);
                if self.unlocked_targets.contains(&Target::UltraGreed) {
                    valid_targets.insert(Target::UltraGreed);
                }
            }
            let valid_targets = valid_targets;

            if !valid_targets.is_empty() {
                pool.insert(ch, valid_targets);
            }
        }
        let pool = pool;
        
        if pool.is_empty() {
            return None
        }

        self.roll_pool(pool)
    }

    fn roll_pool(&self, pool: HashMap<Character, HashSet<Target>>) -> Option<(Character, Target)> {
        let char_pool: Vec<Character> = pool
            .iter()
            .map(|(ch, _)| -> Character { *ch })
            .collect();

        let rand_char = char_pool.choose(&mut rand::thread_rng())?;
        let target_pool: Vec<Target> = pool.get(rand_char)?
            .iter()
            .map(|targ| *targ)
            .collect();

        let rand_target = target_pool.choose(&mut rand::thread_rng())?;

        Some((*rand_char, *rand_target))
    }
}
