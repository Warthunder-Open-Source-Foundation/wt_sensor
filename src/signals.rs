use std::ops::RangeInclusive;

pub struct Signal {
	dynamic_range: RangeInclusive<f64>,
	ground_clutter: bool,
	aircraft_as_target: bool,
	iff: bool,
	distance: Distance,
	abs_doppler_speed: Option<bool>,
	main_beam_doppler_speed: Option<bool>,
	doppler_speed: Option<DopplerSpeed>,
	track: Option<bool>,
	angular_accuracy: Option<f64>,
	distance_accuracy: Option<f64>,
}

pub struct Distance {
	presents: bool,
	min_value: f64,
	max_value: f64,
	width: f64,
}

pub struct DopplerSpeed {
	presents: bool,
	min_value: f64,
	max_value: f64,
	signal_width_min: f64,
	width: f64,
}

pub enum SignalType {
	PulseSearch,
	PulseDopplerSearch,
	PulseTrack,
	PulseDopplerTrack,
}