
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;


#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Debug)]
pub struct ScopeRangeSets {
	pub common: Vec<f64>,
	pub boresight_lock: Vec<f64>,
}