use enumflags2::bitflags;

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Target {
    BlueBaby,
    Lamb,
    MegaSatan,
    Delirium,
    Beast,
    Mother,
    UltraGreed,
}

impl Target {
    pub fn to_string(&self) -> String {
        use Target::*;
        match self {
            BlueBaby => String::from("???"),
            Lamb => String::from("The Lamb"),
            MegaSatan => String::from("Mega Satan"),
            Delirium => String::from("Delirium"),
            Beast => String::from("Beast"),
            Mother => String::from("Mother"),
            UltraGreed => String::from("Ultra Greed"),
        }
    }
}
