use crate::randomizer::characters::Character;
use crate::randomizer::dependency::{Dependency, DependencyValue, HasDependency, Mantle};
use crate::randomizer::targets::Target;
use crate::toml_parse::savefile::{General, Marks, Savefile};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Unlocks {
    marks: HashMap<Character, HashSet<Target>>,
    unlocked_chars: HashSet<Character>,
    unlocked_targets: HashSet<Target>,
    is_mantle_unlocked: bool,
    is_it_lives_unlocked: bool,
    is_polaroid_unlocked: bool,
    is_negative_unlocked: bool,
    boss_rush_chance: f32,
    hush_chance: f32,
    roll_boss_rush_on_alt: bool,
}

impl Default for Unlocks {
    fn default() -> Self {
        Self {
            marks: HashMap::new(),
            unlocked_chars: HashSet::from([Character::Isaac]),
            unlocked_targets: HashSet::from([Target::Mom, Target::UltraGreed, Target::BossRush]),
            is_mantle_unlocked: false,
            is_it_lives_unlocked: false,
            is_polaroid_unlocked: false,
            is_negative_unlocked: false,
            boss_rush_chance: 1.0,
            hush_chance: 1.0,
            roll_boss_rush_on_alt: true,
        }
    }
}

impl Unlocks {
    pub fn new(
        marks: HashMap<Character, HashSet<Target>>,
        unlocked_chars: HashSet<Character>,
        unlocked_targets: HashSet<Target>,
        is_mantle_unlocked: bool,
        is_it_lives_unlocked: bool,
        is_polaroid_unlocked: bool,
        is_negative_unlocked: bool,
        boss_rush_chance: f32,
        hush_chance: f32,
        roll_boss_rush_on_alt: bool,
    ) -> Self {
        Self {
            marks,
            unlocked_chars,
            unlocked_targets,
            is_mantle_unlocked,
            is_it_lives_unlocked,
            is_polaroid_unlocked,
            is_negative_unlocked,
            boss_rush_chance,
            hush_chance,
            roll_boss_rush_on_alt,
        }
    }

    pub fn set_marks(&mut self, ch: Character, marks: HashSet<Target>) -> &mut Self {
        if !self.unlocked_chars.contains(&ch) {
            self.unlocked_chars.insert(ch);
        }
        marks.iter().for_each(|targ| {
            if !self.unlocked_targets.contains(&targ) {
                self.unlocked_targets.insert(targ.clone());
            }
        });

        self.marks.insert(ch, marks);
        self
    }

    pub fn add_marks(&mut self, ch: Character, marks: HashSet<Target>) -> &mut Self {
        match self.marks.get_mut(&ch) {
            Some(targs) => {
                for targ in marks {
                    targs.insert(targ);
                }
            }
            None => {
                self.marks.insert(ch, marks);
            }
        }

        self
    }

    pub fn remove_marks(&mut self, ch: &Character, marks: &HashSet<Target>) -> &mut Self {
        if let Some(targs) = self.marks.get_mut(&ch) {
            for targ in marks {
                targs.remove(targ);
            }

            if self.marks.get(&ch).unwrap().is_empty() {
                self.marks.remove(&ch);
            }
        } else {
            return self
        }

        self
    }

    pub fn remove_all_marks(&mut self, ch: &Character) -> &mut Self {
        self.marks.remove(&ch);
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

    pub fn add_unlocked_chars(&mut self, chars: HashSet<Character>) -> &mut Self {
        for ch in chars {
            self.unlocked_chars.insert(ch);
        }

        self
    }

    pub fn remove_unlocked_chars(&mut self, chars: &HashSet<Character>) -> &mut Self {
        for ch in chars {
            if self.unlocked_chars.remove(&ch) {
                self.marks.remove(&ch);
            }
        }

        self
    }

    pub fn set_unlocked_targets(&mut self, targets: HashSet<Target>) -> &mut Self {
        self.unlocked_targets = targets;

        for (_, targets) in &mut self.marks {
            let mut to_remove: Vec<Target> = Vec::new();

            for targ in targets.iter() {
                if !self.unlocked_targets.contains(&targ) {
                    to_remove.push(*targ);
                }
            }

            for targ_to_remove in to_remove {
                targets.remove(&targ_to_remove);
            }
        }

        self
    }

    pub fn add_unlocked_targets(&mut self, targs: HashSet<Target>) -> &mut Self {
        for targ in targs {
            self.unlocked_targets.insert(targ);
        }

        self
    }

    pub fn remove_unlocked_targets(&mut self, targs: &HashSet<Target>) -> &mut Self {
        for targ in targs {
            if self.unlocked_targets.remove(&targ) {
                for (_, marked_targets) in &mut self.marks {
                    marked_targets.remove(&targ);
                }
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

    pub fn set_polaroid_unlocked(&mut self, is_unlocked: bool) -> &mut Self {
        self.is_polaroid_unlocked = is_unlocked;
        self
    }

    pub fn set_negative_unlocked(&mut self, is_unlocked: bool) -> &mut Self {
        self.is_negative_unlocked = is_unlocked;
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

    pub fn set_roll_boss_rush_on_alt(&mut self, is_roll: bool) -> &mut Self {
        self.roll_boss_rush_on_alt = is_roll;
        self
    }

    pub fn set_everything_unlocked(&mut self) {
        for ch in Character::iter() {
            let targs = Target::iter()
                .filter(|targ| self.is_target_significant(targ))
                .collect();
            self.set_marks(ch, targs);
        }
        self.set_mantle_unlocked(true)
            .set_it_lives_unlocked(true)
            .set_polaroid_unlocked(true)
            .set_negative_unlocked(true);
    }

    pub fn get_random_pick(&self) -> Option<(Character, HashSet<Target>)> {
        if self.is_everything_unlocked() {
            return self.roll_pool(Self::everything_pool());
        }

        let mut not_finished: HashSet<Character> = HashSet::new();
        for ch in &self.unlocked_chars {
            if !self.is_char_completed(&ch) {
                not_finished.insert(ch.clone());
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
            if !self.is_char_completed(&ch) {
                return false;
            }
        }

        true
    }

    fn is_char_completed(&self, ch: &Character) -> bool {
        match self.marks.get(&ch) {
            Some(marks) => {
                for targ in Target::iter().filter(|t| self.is_target_significant(t)) {
                    if !marks.contains(&targ) {
                        return false;
                    }
                }
            }
            None => return false,
        };
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
                .filter(|rem| self.unlocked_targets.contains(rem) && self.is_target_significant(rem))
                .cloned()
                .collect(),
            None => self
                .unlocked_targets
                .iter()
                .filter(|unlocked| self.is_target_significant(unlocked))
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
            DependencyValue::Mom(_) => {
                self.add_target_to_unlocked_chars(Target::Mom, targets);
            }
            DependencyValue::Polaroid(_) => {
                self.add_target_to_unlocked_chars(Target::Isaac, targets);
            }
            DependencyValue::Negative(_) => {
                self.add_target_to_unlocked_chars(Target::Satan, targets);
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
            && !self.is_unlocked_now(&Dependency::Singular(dep_val.clone()))
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
        use crate::randomizer::targets::Target::Heart;

        match dep_val {
            Character(ch) => self.unlocked_chars.contains(ch),
            Target(targ) => self.unlocked_targets.contains(targ),
            Mantle(_) => self.is_mantle_unlocked,
            ItLives(_) => self.is_it_lives_unlocked,
            Mom(_) => self.unlocked_targets.contains(&Heart),
            Polaroid(_) => self.is_polaroid_unlocked,
            Negative(_) => self.is_negative_unlocked,
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
                Target::BossRush | Target::Hush | Target::MegaSatan | Target::Delirium => {
                    special_in_pool.insert(*targ);
                    false
                }
                Target::UltraGreed | Target::Beast => {
                    special_in_pool.insert(*targ);
                    true
                }
                _ => self.is_target_significant(&targ),
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
        } else if special_in_pool.contains(&Target::Delirium) {
            targets.insert(Target::Delirium);
            targets.insert(Target::Hush);
            return Some((*rand_char, targets));
        }

        let should_roll_boss_rush = special_in_pool.contains(&Target::BossRush)
            && if targets.contains(&Target::Beast) || targets.contains(&Target::Mother) {
                self.roll_boss_rush_on_alt
            } else {
                true
            }
            && self.unlocked_targets.contains(&Target::Heart);

        let should_roll_hush = special_in_pool.contains(&Target::Hush)
            && !targets.contains(&Target::Beast)
            && !targets.contains(&Target::Mother);

        if target_pool.len() == 1
            || (target_pool.len() == 2 && special_in_pool.contains(&Target::UltraGreed))
            || target_pool.is_empty()
        {
            if should_roll_hush {
                targets.insert(Target::Hush);
            }
            if should_roll_boss_rush {
                targets.insert(Target::BossRush);
            }
        } else {
            if should_roll_hush && (rng.gen::<f32>() <= self.hush_chance) {
                targets.insert(Target::Hush);
            }
            if should_roll_boss_rush && (rng.gen::<f32>() <= self.boss_rush_chance) {
                targets.insert(Target::BossRush);
            }
        }

        if targets.is_empty() {
            None
        } else {
            Some((*rand_char, targets))
        }
    }

    pub fn is_target_significant(&self, targ: &Target) -> bool {
        use Target::*;
        use crate::randomizer::dependency::{Polaroid, Negative, ItLives, Mom as MomDep};
        match targ {
            BlueBaby | Lamb | MegaSatan | Delirium | Beast | Mother | UltraGreed | BossRush
            | Hush => true,
            Isaac if self.is_dependency_val_unlockable(&DependencyValue::Polaroid(Polaroid)) => true,
            Satan if self.is_dependency_val_unlockable(&DependencyValue::Negative(Negative)) => true,
            Heart if self.is_dependency_val_unlockable(&DependencyValue::ItLives(ItLives)) => true,
            Mom if self.is_dependency_val_unlockable(&DependencyValue::Mom(MomDep)) => true,
            _ => false,
        }
    }

}

impl Into<Savefile> for &Unlocks {
    fn into(self) -> Savefile {
        Savefile::new(
            General::new(
                self.unlocked_chars
                    .iter()
                    .map(|ch| -> String { format!("{}", ch) })
                    .collect(),
                self.unlocked_targets
                    .iter()
                    .map(|targ| -> String { format!("{}", targ) })
                    .collect(),
                self.is_mantle_unlocked,
                self.is_it_lives_unlocked,
                self.is_polaroid_unlocked,
                self.is_negative_unlocked,
                self.boss_rush_chance,
                self.hush_chance,
                self.roll_boss_rush_on_alt,
            ),
            HashMap::from_iter(self.marks.iter().map(|(ch, targs)| -> (String, Marks) {
                (
                    format!("{}", ch),
                    Marks::new(
                        targs
                            .iter()
                            .map(|targ| -> String { format!("{}", targ) })
                            .collect(),
                    ),
                )
            })),
        )
    }
}
