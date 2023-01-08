mod scanning;
mod transceiver;
mod util;

use crate::scanning::scan_pattern::ScanPattern;
use crate::transceiver::Transceiver;

pub struct Radar {
	name: String,
	show_missile_launch_zone: bool,
	transceivers: Vec<Transceiver>,
	scan_patterns: Vec<ScanPattern>,
}