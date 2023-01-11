pub mod scanning;
pub mod transceiver;
pub mod util;
pub mod signals;
pub mod scoped_range_sets;

use std::str::FromStr;
use wt_blk::WTBlk;
use crate::scanning::scan_pattern::{Submode};
use crate::scoped_range_sets::ScopeRangeSets;
use crate::signals::Signal;
use crate::transceiver::Transceiver;

#[derive(Debug)]
pub struct Radar {
	pub name: String,
	pub show_missile_launch_zone: bool,
	pub transceivers: Vec<Transceiver>,
	pub submode: Vec<Submode>,
	pub signals: Vec<Signal>,
	pub scope_range_sets: ScopeRangeSets,
	//fsms is missing as it is very specific logic for radar interaction
}

impl Radar {
	pub fn from_str(name: String, blk_str: &str) -> Self {
		let blk = WTBlk::new(&blk_str).unwrap();

		let show_missile_launch_zone = blk.bool("/showMissileLaunchZone").unwrap();

		let transceivers = [
			"pulse",
			"pulseDoppler"
		].iter().map(|name| {
			// 								Yes they misspelled Transceiver's
			let format = format!("/transivers/{}", name);
			let elem = blk.pointer( &format).unwrap();
			Transceiver::from_value(&WTBlk::new(&elem.to_string()).unwrap(), name).unwrap()
		}).collect();


		let scan_patterns = [
			"searchNarrow",
			"searchMedium",
			"searchWide", "hudLock",
			"verticalLock", "boresightLock",
			"designationLock",
			"track"
		].iter().map(|name| {
			let format = format!("/scanPatterns/{}", name);
			let elem = blk.pointer(&format).unwrap();
			Submode::from_value(&WTBlk::new(&elem.to_string()).unwrap(), name).unwrap()
		}
		).collect();


		Self {
			name,
			show_missile_launch_zone,
			transceivers,
			submode: scan_patterns,
			signals: vec![],
			scope_range_sets: ScopeRangeSets { common: vec![], boresight_lock: vec![] },
		}
	}
}

#[cfg(test)]
mod test {
	use std::fs;
	use crate::Radar;

	#[test]
	fn apg_66() {
		let file = fs::read_to_string("us_an_apg_66.txt").unwrap();
		let radar = Radar::from_str("an_apg_66".to_owned(), &file);

		dbg!(radar);
	}
}