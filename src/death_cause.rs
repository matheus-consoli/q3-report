use std::str::FromStr;

use variant_count::VariantCount;

/// Means of death.
// TODO: remove `VariantCount` when `std::mem::variant_count` gets stabilized
// Tracking Issue: https://github.com/rust-lang/rust/issues/73662
#[derive(Debug, Clone, Copy, PartialEq, Eq, VariantCount)]
#[repr(u8)]
pub enum DeathCause {
    Unknown,
    Shotgun,
    Gauntlet,
    Machinegun,
    Grenade,
    GrenadeSplash,
    Rocket,
    RocketSplash,
    Plasma,
    PlasmaSplash,
    Railgun,
    Lightning,
    Bfg,
    BfgSplash,
    Water,
    Slime,
    Lava,
    Crush,
    Telefrag,
    Falling,
    Suicide,
    TargetLaser,
    TriggerHurt,
    Nail,
    Chaingun,
    ProximityMine,
    Kamikaze,
    Juiced,
    Grapple,
}

impl DeathCause {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "MOD_UNKNOWN",
            Self::Shotgun => "MOD_SHOTGUN",
            Self::Gauntlet => "MOD_GAUNTLET",
            Self::Machinegun => "MOD_MACHINEGUN",
            Self::Grenade => "MOD_GRENADE",
            Self::GrenadeSplash => "MOD_GRENADE_SPLASH",
            Self::Rocket => "MOD_ROCKET",
            Self::RocketSplash => "MOD_ROCKET_SPLASH",
            Self::Plasma => "MOD_PLASMA",
            Self::PlasmaSplash => "MOD_PLASMA_SPLASH",
            Self::Railgun => "MOD_RAILGUN",
            Self::Lightning => "MOD_LIGHTNING",
            Self::Bfg => "MOD_BFG",
            Self::BfgSplash => "MOD_BFG_SPLASH",
            Self::Water => "MOD_WATER",
            Self::Slime => "MOD_SLIME",
            Self::Lava => "MOD_LAVA",
            Self::Crush => "MOD_CRUSH",
            Self::Telefrag => "MOD_TELEFRAG",
            Self::Falling => "MOD_FALLING",
            Self::Suicide => "MOD_SUICIDE",
            Self::TargetLaser => "MOD_TARGET_LASER",
            Self::TriggerHurt => "MOD_TRIGGER_HURT",
            Self::Nail => "MOD_NAIL",
            Self::Chaingun => "MOD_CHAINGUN",
            Self::ProximityMine => "MOD_PROXIMITY_MINE",
            Self::Kamikaze => "MOD_KAMIKAZE",
            Self::Juiced => "MOD_JUICED",
            Self::Grapple => "MOD_GRAPPLE",
        }
    }

    #[inline]
    pub fn as_index(self) -> usize {
        self as usize
    }

    #[inline]
    pub unsafe fn from_u8_unchecked(variant: u8) -> Self {
        debug_assert!(variant < Self::VARIANT_COUNT as u8);
        unsafe { core::mem::transmute(variant) }
    }
}

impl FromStr for DeathCause {
    type Err = ();

    fn from_str(cause: &str) -> Result<Self, Self::Err> {
        let cause = match cause {
            "MOD_UNKNOWN" => DeathCause::Unknown,
            "MOD_SHOTGUN" => DeathCause::Shotgun,
            "MOD_GAUNTLET" => DeathCause::Gauntlet,
            "MOD_MACHINEGUN" => DeathCause::Machinegun,
            "MOD_GRENADE" => DeathCause::Grenade,
            "MOD_GRENADE_SPLASH" => DeathCause::GrenadeSplash,
            "MOD_ROCKET" => DeathCause::Rocket,
            "MOD_ROCKET_SPLASH" => DeathCause::RocketSplash,
            "MOD_PLASMA" => DeathCause::Plasma,
            "MOD_PLASMA_SPLASH" => DeathCause::PlasmaSplash,
            "MOD_RAILGUN" => DeathCause::Railgun,
            "MOD_LIGHTNING" => DeathCause::Lightning,
            "MOD_BFG" => DeathCause::Bfg,
            "MOD_BFG_SPLASH" => DeathCause::BfgSplash,
            "MOD_WATER" => DeathCause::Water,
            "MOD_SLIME" => DeathCause::Slime,
            "MOD_LAVA" => DeathCause::Lava,
            "MOD_CRUSH" => DeathCause::Crush,
            "MOD_TELEFRAG" => DeathCause::Telefrag,
            "MOD_FALLING" => DeathCause::Falling,
            "MOD_SUICIDE" => DeathCause::Suicide,
            "MOD_TARGET_LASER" => DeathCause::TargetLaser,
            "MOD_TRIGGER_HURT" => DeathCause::TriggerHurt,
            "MOD_NAIL" => DeathCause::Nail,
            "MOD_CHAINGUN" => DeathCause::Chaingun,
            "MOD_PROXIMITY_MINE" => DeathCause::ProximityMine,
            "MOD_KAMIKAZE" => DeathCause::Kamikaze,
            "MOD_JUICED" => DeathCause::Juiced,
            "MOD_GRAPPLE" => DeathCause::Grapple,
            _ => todo!(),
        };
        Ok(cause)
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct DeathCauseDb([u16; DeathCause::VARIANT_COUNT]);

impl DeathCauseDb {
    #[inline]
    pub fn inc_death(&mut self, death: DeathCause) {
        self.0[death.as_index()] += 1;
    }

    pub fn counted(&self) -> impl Iterator<Item = (u16, DeathCause)> + '_ {
        self.0
            .iter()
            .copied()
            .zip(0u8..) // same as `Iterator::enumerate`, but for u8
            .filter(|(death_counter, _)| *death_counter != 0)
            .map(|(death_counter, cause)| {
                // Safety: we know `cause` is a valid number because it's bounded
                // on `DeathCause::VARIANT_COUNT`
                let cause = unsafe { DeathCause::from_u8_unchecked(cause) };
                (death_counter, cause)
            })
    }
}
