use crate::randomizer::characters::Character;
use crate::randomizer::targets::Target;

pub enum Dependency {
    None,
    Singular(DependencyValue),
    Sum(Vec<DependencyValue>),
    Product(Vec<DependencyValue>),
}

#[derive(Clone)]
pub enum DependencyValue {
    Character(Character),
    Target(Target),
    Mantle(Mantle),
    ItLives(ItLives),
    Mom(Mom),
    Polaroid(Polaroid),
    Negative(Negative),
}

impl HasDependency for DependencyValue {
    fn depends_on(&self) -> Dependency {
        use DependencyValue::*;

        match self {
            Character(ch) => ch.depends_on(),
            Target(targ) => targ.depends_on(),
            Mantle(mantle) => mantle.depends_on(),
            ItLives(itlives) => itlives.depends_on(),
            Mom(mom) => mom.depends_on(),
            Polaroid(polaroid) => polaroid.depends_on(),
            Negative(negative) => negative.depends_on(),
        }
    }
}

#[derive(Clone)]
pub struct Mantle;

impl HasDependency for Mantle {
    fn depends_on(&self) -> Dependency {
        Dependency::Singular(DependencyValue::Target(Target::UltraGreed))
    }
}

#[derive(Clone)]
pub struct ItLives;

impl HasDependency for ItLives {
    fn depends_on(&self) -> Dependency {
        Dependency::Singular(DependencyValue::Target(Target::Heart))
    }
}

#[derive(Clone)]
pub struct Mom;

impl HasDependency for Mom {
    fn depends_on(&self) -> Dependency {
        Dependency::None
    }
}

#[derive(Clone)]
pub struct Polaroid;

impl HasDependency for Polaroid {
    fn depends_on(&self) -> Dependency {
        Dependency::Singular(DependencyValue::Target(Target::Isaac))
    }
}

#[derive(Clone)]
pub struct Negative;

impl HasDependency for Negative {
    fn depends_on(&self) -> Dependency {
        Dependency::Singular(DependencyValue::Target(Target::Satan))
    }
}

pub trait HasDependency {
    fn depends_on(&self) -> Dependency;
}
