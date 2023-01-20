use std::ops::RangeInclusive;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
#[derive(Debug)]
pub struct Limits {
	pub(crate) azimuth: RangeInclusive<f64>,
	pub(crate) elevation: RangeInclusive<f64>,
}

#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
#[derive(Debug)]
pub struct Antenna {
	pub(crate) angle_half_sens: f64,
	pub(crate) side_lobes_sensitivity: f64,
}