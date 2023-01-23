use std::ops::RangeInclusive;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
#[derive(Debug)]
pub struct Limits {
	pub azimuth: RangeInclusive<f64>,
	pub elevation: RangeInclusive<f64>,
}

#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
#[derive(Debug)]
pub struct Antenna {
	pub angle_half_sens: f64,
	pub side_lobes_sensitivity: f64,
}