use std::ops::RangeInclusive;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;


#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
#[derive(Debug)]
pub struct Signal {
	signal_type: SignalType,
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


#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
#[derive(Debug)]
pub struct Distance {
	presents: bool,
	min_value: f64,
	max_value: f64,
	width: f64,
}


#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
#[derive(Debug)]
pub struct DopplerSpeed {
	presents: bool,
	min_value: f64,
	max_value: f64,
	signal_width_min: f64,
	width: f64,
}



#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect, bevy_reflect::FromReflect))]
#[derive(Debug, strum_macros::ToString)]
pub enum SignalType {
	PulseSearch,
	PulseDopplerSearch,
	PulseTrack,
	PulseDopplerTrack,
}