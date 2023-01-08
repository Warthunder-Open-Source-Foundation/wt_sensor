use std::ops::RangeInclusive;

pub struct Limits {
	azimuth: RangeInclusive<f64>,
	elevation: RangeInclusive<f64>,
}

pub struct Antenna {
	angle_half_sens: f64,
	side_lobes_sensitivity: f64,
}