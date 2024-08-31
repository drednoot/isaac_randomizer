use crate::randomizer::characters::Character;
use crate::randomizer::targets::Target;

pub enum Dependency {
    None,
    Singular(DependencyValue),
    Sum(Vec<DependencyValue>),
    Product(Vec<DependencyValue>),
}

pub enum DependencyValue {
    Character(Character),
    Target(Target),
    Mantle(Mantle),
    ItLives(ItLives),
}

impl HasDependency for DependencyValue {
    fn depends_on(&self) -> Dependency {
        use DependencyValue::*;

        match self {
            Character(ch) => ch.depends_on(),
            Target(targ) => targ.depends_on(),
            Mantle(mantle) => mantle.depends_on(),
            ItLives(itlives) => itlives.depends_on(),
        }
    }
}

// holy mantle struct that has a dependency
pub struct Mantle;

impl HasDependency for Mantle {
    fn depends_on(&self) -> Dependency {
        Dependency::Singular(DependencyValue::Target(Target::UltraGreed))
    }
}

// it lives struct that has a dependency
pub struct ItLives;

impl HasDependency for ItLives {
    fn depends_on(&self) -> Dependency {
        Dependency::Singular(DependencyValue::Target(Target::Heart))
    }
}

pub trait HasDependency {
    fn depends_on(&self) -> Dependency;
}
