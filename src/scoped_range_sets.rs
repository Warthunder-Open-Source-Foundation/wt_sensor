
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;


#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Debug)]
pub struct ScopeRangeSets {
	pub(crate) common: Vec<f64>,
	pub(crate) boresight_lock: Vec<f64>,
}