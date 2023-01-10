use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Limits {
	pub(crate) azimuth: RangeInclusive<f64>,
	pub(crate) elevation: RangeInclusive<f64>,
}

#[derive(Debug)]
pub struct Antenna {
	pub(crate) angle_half_sens: f64,
	pub(crate) side_lobes_sensitivity: f64,
}