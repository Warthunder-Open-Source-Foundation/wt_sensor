mod scanning;
mod transceiver;
mod util;
mod signals;
mod scoped_range_sets;

use std::str::FromStr;
use wt_blk::error::WTBlkError;
use wt_blk::WTBlk;
use crate::scanning::scan_pattern::{Submode};
use crate::scoped_range_sets::ScopeRangeSets;
use crate::signals::Signal;
use crate::transceiver::Transceiver;

#[derive(Debug)]
pub struct Radar {
	name: String,
	show_missile_launch_zone: bool,
	transceivers: Vec<Transceiver>,
	scan_patterns: Vec<Submode>,
	signals: Vec<Signal>,
	scope_range_sets: ScopeRangeSets,
	//fsms is missing as it is very specific logic for radar interaction
}

impl Radar {
	pub fn from_str(name: String, blk_str: &str) -> Self {
		let blk = WTBlk::new(&blk_str).unwrap();

		let show_missile_launch_zone = blk.bool("/showMissileLaunchZone").unwrap();

		let transceivers = {
			let mut transceivers = vec![];

			let mut push_if_exists = |name| {
				// 								Yes they misspelled Transceiver's
				if let Ok(elem) = blk.pointer(&format!("/transivers/{}", name)) {
					transceivers.push(Transceiver::from_value(&WTBlk::new(&elem.to_string()).unwrap(), name).unwrap());
				}
			};

			push_if_exists("pulse");
			push_if_exists("pulseDoppler");


			transceivers
		};

		let scan_patterns = {
			let mut scan_patterns = vec![];

			let mut push_if_exists = |name| {
				// 								Yes they misspelled Transceiver's
				if let Ok(elem) = blk.pointer(&format!("/scanPatterns/{}", name)) {
					scan_patterns.push(Submode::from_value(&WTBlk::new(&elem.to_string()).unwrap(), name).unwrap());
				}
			};

			push_if_exists("searchNarrow");
			push_if_exists("searchMedium");
			push_if_exists("searchWide");
			push_if_exists("hudLock");
			push_if_exists("verticalLock");
			push_if_exists("boresightLock");
			push_if_exists("designationLock");
			push_if_exists("track");

			scan_patterns
		};

		Self {
			name,
			show_missile_launch_zone,
			transceivers,
			scan_patterns,
			signals: vec![],
			scope_range_sets: ScopeRangeSets{ common: vec![], boresight_lock: vec![] },
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