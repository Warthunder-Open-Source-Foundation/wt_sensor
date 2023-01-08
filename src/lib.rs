mod scanning;
mod transceiver;
mod util;
mod signals;

use crate::scanning::scan_pattern::ScanPattern;
use crate::signals::Signal;
use crate::transceiver::Transceiver;

pub struct Radar {
	name: String,
	show_missile_launch_zone: bool,
	transceivers: Vec<Transceiver>,
	scan_patterns: Vec<ScanPattern>,
	signals: Vec<Signal>,
	scope_range_sets: ScopeRangeSets,
	//fsms is missing as it is very specific logic for radar interaction
}

pub struct ScopeRangeSets {
	common: Vec<f64>,
	boresight_lock: Vec<f64>,
}