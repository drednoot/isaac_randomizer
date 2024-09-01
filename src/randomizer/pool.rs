use crate::randomizer::characters::Character;
use crate::randomizer::dependency::{Dependency, DependencyValue, HasDependency, Mantle};
use crate::randomizer::targets::Target;
use enumflags2::{make_bitflags, BitFlags};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Unlocks {
    marks: HashMap<Character, BitFlags<Target>>,
    unlocked_chars: HashSet<Character>,
    unlocked_targets: HashSet<Target>,
    is_mantle_unlocked: bool,
    is_it_lives_unlocked: bool,
    boss_rush_chance: f32,
    hush_chance: f32,
}

impl Default for Unlocks {
    fn default() -> Self {
        Self {
            marks: HashMap::new(),
            unlocked_chars: HashSet::from([Character::Isaac]),
            unlocked_targets: HashSet::new(),
            is_mantle_unlocked: false,
            is_it_lives_unlocked: false,
            boss_rush_chance: 0.5,
            hush_chance: 0.5,
        }
    }
}

impl Unlocks {
    pub fn set_marks(&mut self, ch: Character, marks: HashSet<Target>) -> &mut Self {
        if !self.unlocked_chars.contains(&ch) {
            self.unlocked_chars.insert(ch);
        }
        marks.iter().for_each(|targ| {
            if !self.unlocked_targets.contains(&targ) {
                self.unlocked_targets.insert(targ.clone());
            }
        });

        let mut flags = BitFlags::empty();
        for mark in marks {
            flags.set(mark, true);
        }

        match self.marks.get_mut(&ch) {
            Some(completed) => completed.insert(flags),
            None => {
                self.marks.insert(ch, flags);
            }
        };

        self
    }

    pub fn set_unlocked_chars(&mut self, chars: HashSet<Character>) -> &mut Self {
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

        self
    }

    pub fn set_unlocked_targets(&mut self, targets: HashSet<Target>) -> &mut Self {
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

        self
    }

    pub fn set_mantle_unlocked(&mut self, is_unlocked: bool) -> &mut Self {
        self.is_mantle_unlocked = is_unlocked;
        self
    }

    pub fn set_it_lives_unlocked(&mut self, is_unlocked: bool) -> &mut Self {
        self.is_it_lives_unlocked = is_unlocked;
        self
    }

    pub fn set_boss_rush_chance(&mut self, chance: f32) -> &mut Self {
        self.boss_rush_chance = chance;
        self
    }

    pub fn set_hush_chance(&mut self, chance: f32) -> &mut Self {
        self.hush_chance = chance;
        self
    }

    pub fn set_everything_unlocked(&mut self) {
        for ch in Character::iter() {
            let targs = Target::iter()
                .filter(|targ| targ.is_significant())
                .collect();
            self.set_marks(ch, targs);
        }
        self.set_mantle_unlocked(true).set_it_lives_unlocked(true);
    }

    pub fn get_random_pick(&self) -> Option<(Character, HashSet<Target>)> {
        if self.is_everything_unlocked() {
            return self.roll_pool(Self::everything_pool());
        }

        let mut not_finished: HashSet<Character> = HashSet::new();
        for ch in &self.unlocked_chars {
            match self.marks.get(ch) {
                Some(targ) => {
                    if !targ.is_all() {
                        not_finished.insert(ch.clone());
                    }
                }
                None => {
                    not_finished.insert(ch.clone());
                }
            }
        }
        let not_finished = not_finished;

        let mut pool: HashMap<Character, HashSet<Target>> = HashMap::new();
        for ch in not_finished {
            let valid_targets = self.unlocked_targets_for(&ch);

            if !valid_targets.is_empty() {
                pool.insert(ch, valid_targets);
            }
        }

        if pool.is_empty() {
            pool = self.get_additional_targets();
        }
        let pool = pool;

        self.roll_pool(pool)
    }

    fn is_everything_unlocked(&self) -> bool {
        for ch in Character::iter() {
            if !self.unlocked_chars.contains(&ch) {
                return false;
            }
            match self.marks.get(&ch) {
                Some(marks) => {
                    for targ in Target::iter().filter(|t| t.is_significant()) {
                        if !marks.contains(targ) {
                            return false;
                        }
                    }
                }
                None => return false,
            }
        }

        true
    }

    fn everything_pool() -> HashMap<Character, HashSet<Target>> {
        let mut pool = HashMap::new();

        for ch in Character::iter() {
            let targets: HashSet<Target> = Target::iter()
                .filter(|targ| targ.is_significant())
                .collect();
            pool.insert(ch, targets);
        }

        pool
    }

    fn unlocked_targets_for(&self, ch: &Character) -> HashSet<Target> {
        if (ch == &Character::Lost || ch == &Character::TaintedLost) && !self.is_mantle_unlocked {
            return HashSet::new();
        }

        let mut valid_targets: HashSet<Target> = match self.marks.get(&ch) {
            Some(completed) => Target::get_remaining(completed)
                .iter()
                .filter(|rem| self.unlocked_targets.contains(rem) && rem.is_significant())
                .cloned()
                .collect(),
            None => self
                .unlocked_targets
                .iter()
                .filter(|unlocked| unlocked.is_significant())
                .cloned()
                .collect(),
        };
        if valid_targets.contains(&Target::Mother) && !self.is_mantle_unlocked {
            valid_targets.remove(&Target::Mother);
        }

        valid_targets
    }

    fn get_additional_targets(&self) -> HashMap<Character, HashSet<Target>> {
        let unlockables = Self::get_unlockables();
        let mut targets = HashMap::new();

        for unlockable in unlockables {
            self.resolve_dependency(&unlockable, &mut targets);
        }

        targets
    }

    fn get_unlockables() -> Vec<Dependency> {
        let mut unlockables = vec![
            Dependency::Singular(DependencyValue::Character(Character::Judas)),
            Dependency::Singular(DependencyValue::Character(Character::BlueBaby)),
            Dependency::Singular(DependencyValue::Character(Character::Keeper)),
            Dependency::Singular(DependencyValue::Character(Character::Bethany)),
            Dependency::Singular(DependencyValue::Character(Character::Apollyon)),
            Dependency::Singular(DependencyValue::Mantle(Mantle)),
            Dependency::Singular(DependencyValue::Target(Target::BlueBaby)),
            Dependency::Singular(DependencyValue::Target(Target::Lamb)),
            Dependency::Singular(DependencyValue::Target(Target::MegaSatan)),
            Dependency::Singular(DependencyValue::Target(Target::Delirium)),
            Dependency::Singular(DependencyValue::Target(Target::Beast)),
            Dependency::Singular(DependencyValue::Target(Target::Mother)),
            Dependency::Singular(DependencyValue::Target(Target::Hush)),
        ];

        for ch in Character::iter() {
            if ch.is_tainted() {
                unlockables.push(Dependency::Singular(DependencyValue::Character(ch)));
            }
        }

        unlockables
    }

    fn resolve_dependency(
        &self,
        dep: &Dependency,
        targets: &mut HashMap<Character, HashSet<Target>>,
    ) {
        use Dependency::*;

        if self.is_unlocked_now(dep) {
            return;
        }

        if self.is_unlockable_now(dep) {
            match dep {
                None => {}
                Singular(val) => self.add_target_dependency_val(val, targets),
                Sum(vals) => {
                    for val in vals {
                        if self.is_dependency_val_unlocked(val) {
                            self.add_target_dependency_val(val, targets);
                        }
                    }
                }
                Product(vals) => {
                    for val in vals {
                        self.add_target_dependency_val(val, targets);
                    }
                }
            }
        } else {
            match dep {
                None => {}
                Singular(val) => self.resolve_dependency(&val.depends_on(), targets),
                Sum(vals) | Product(vals) => {
                    for val in vals {
                        self.resolve_dependency(&val.depends_on(), targets);
                    }
                }
            }
        }
    }

    fn add_target_dependency_val(
        &self,
        dep_val: &DependencyValue,
        targets: &mut HashMap<Character, HashSet<Target>>,
    ) {
        use DependencyValue;

        match dep_val {
            DependencyValue::Character(ch) => {
                if ch.is_tainted() {
                    match targets.get_mut(&ch.tainted_to_normal().unwrap()) {
                        None => {
                            targets.insert(
                                ch.tainted_to_normal().unwrap(),
                                HashSet::from([Target::Beast]),
                            );
                        }
                        Some(set) => {
                            set.insert(Target::Beast);
                        }
                    }
                    return;
                }

                if ch == &Character::Bethany {
                    let set = targets.entry(Character::Lazarus).or_insert(HashSet::new());

                    // add targets that have heart in their path (all but The Beast have it)
                    let valid_targets = self.unlocked_targets_for(&ch);

                    if valid_targets.is_empty() {
                        set.insert(Target::Heart);
                    } else {
                        set.extend(valid_targets.iter().filter(|&&targ| targ != Target::Beast));
                    }
                    return;
                }

                // other cases are present in their dependencies, you just need to add targets to
                // all unlocked characters
                self.resolve_dependency(&ch.depends_on(), targets);
            }
            DependencyValue::Target(targ) => {
                self.add_target_to_unlocked_chars(*targ, targets);
            }
            DependencyValue::Mantle(_) => {
                self.add_target_to_unlocked_chars(Target::UltraGreed, targets);
            }
            DependencyValue::ItLives(_) => {
                self.add_target_to_unlocked_chars(Target::Heart, targets);
            }
        }
    }

    fn add_target_to_unlocked_chars(
        &self,
        target: Target,
        targets: &mut HashMap<Character, HashSet<Target>>,
    ) {
        for ch in &self.unlocked_chars {
            let set = targets.entry(*ch).or_insert(HashSet::new());
            set.insert(target);
        }
    }

    fn is_unlockable_now(&self, dep: &Dependency) -> bool {
        use Dependency::*;

        match dep {
            None => true,
            Singular(val) => self.is_dependency_val_unlockable(val),
            Sum(vals) => {
                for val in vals {
                    if self.is_dependency_val_unlockable(val) {
                        return true;
                    }
                }
                false
            }
            Product(vals) => {
                for val in vals {
                    if !self.is_dependency_val_unlockable(val) {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn is_dependency_val_unlockable(&self, dep_val: &DependencyValue) -> bool {
        self.is_unlocked_now(&dep_val.depends_on())
    }

    fn is_unlocked_now(&self, dep: &Dependency) -> bool {
        use Dependency::*;

        match dep {
            None => true,
            Singular(val) => self.is_dependency_val_unlocked(val),
            Sum(vals) => {
                for val in vals {
                    if self.is_dependency_val_unlocked(val) {
                        return true;
                    }
                }
                false
            }
            Product(vals) => {
                for val in vals {
                    if !self.is_dependency_val_unlocked(val) {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn is_dependency_val_unlocked(&self, dep_val: &DependencyValue) -> bool {
        use DependencyValue::*;

        match dep_val {
            Character(ch) => self.unlocked_chars.contains(ch),
            Target(targ) => self.unlocked_targets.contains(targ),
            Mantle(_) => self.is_mantle_unlocked,
            ItLives(_) => self.is_it_lives_unlocked,
        }
    }

    fn roll_pool(
        &self,
        pool: HashMap<Character, HashSet<Target>>,
    ) -> Option<(Character, HashSet<Target>)> {
        let char_pool: Vec<Character> = pool.iter().map(|(ch, _)| -> Character { *ch }).collect();

        let rand_char = char_pool.choose(&mut rand::thread_rng())?;
        let mut special_in_pool: HashSet<Target> = HashSet::new();
        let target_pool: Vec<Target> = pool
            .get(rand_char)?
            .iter()
            .map(|targ| *targ)
            .filter(|targ| match targ {
                Target::BossRush | Target::Hush | Target::MegaSatan => {
                    special_in_pool.insert(*targ);
                    false
                }
                Target::UltraGreed | Target::Beast => {
                    special_in_pool.insert(*targ);
                    true
                }
                _ => true,
            })
            .collect();
        let special_in_pool = special_in_pool;

        let mut targets = HashSet::new();

        let mut rng = rand::thread_rng();

        if !target_pool.is_empty() {
            let rand_target = target_pool.choose(&mut rng)?;

            if rand_target == &Target::UltraGreed {
                targets.insert(*rand_target);
                return Some((*rand_char, targets));
            }

            if matches!(rand_target, Target::Lamb | Target::BlueBaby)
                && special_in_pool.contains(&Target::MegaSatan)
            {
                targets.insert(Target::MegaSatan);
            }

            targets.insert(*rand_target);
        }

        let should_roll_hush =
            special_in_pool.contains(&Target::Hush) && !targets.contains(&Target::Beast);

        if target_pool.len() == 1
            || (target_pool.len() == 2 && special_in_pool.contains(&Target::UltraGreed))
            || target_pool.is_empty()
        {
            if should_roll_hush {
                targets.insert(Target::Hush);
            }
            if special_in_pool.contains(&Target::BossRush) {
                targets.insert(Target::BossRush);
            }
        } else {
            if should_roll_hush && (rng.gen::<f32>() <= self.hush_chance) {
                targets.insert(Target::Hush);
            }
            if special_in_pool.contains(&Target::BossRush)
                && (rng.gen::<f32>() <= self.boss_rush_chance)
            {
                targets.insert(Target::BossRush);
            }
        }

        if targets.is_empty() {
            None
        } else {
            Some((*rand_char, targets))
        }
    }
}
