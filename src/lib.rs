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

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;


#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Debug)]
pub struct Radar {
	pub name: String,
	pub localized: String,
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

		let localized = blk.str("/name").unwrap().to_owned();

		let show_missile_launch_zone = blk.bool("/showMissileLaunchZone").unwrap();

		//													  they misspelled it
		let transceiver_obj = blk.pointer("/transivers").expect("Except to have transceivers");
		let mut transceivers = vec![];

		for (k,v) in transceiver_obj.as_object().unwrap().iter() {
			println!("{}", k);
			transceivers.push(Transceiver::from_value(&WTBlk::new(&v.to_string()).unwrap(), k).unwrap())
		}


		let submode = [
			"searchNarrow",
			"searchMedium",
			"searchWide", "hudLock",
			"verticalLock", "boresightLock",
			"designationLock",
			"track"
		].iter().map(|pattern_name| {
			let format = format!("/scanPatterns/{}", pattern_name);
			let elem = blk.pointer(&format).unwrap();
			Submode::from_value(&WTBlk::new(&elem.to_string()).unwrap(), pattern_name).unwrap()
		}
		).collect();
		
		let scope_range_sets = ScopeRangeSets {
			common: {
				let mut ranges = vec![];
				let mut index = 1;
				while let Ok(range) = blk.float(&format!("/scopeRangeSets/common/range{index}")) {
					ranges.push(range);
					index += 1;
				}
				ranges
			},
			boresight_lock: {
				let mut ranges = vec![];
				let mut index = 1;
				while let Ok(range) = blk.float(&format!("/scopeRangeSets/boresightLock/range{index}")) {
					ranges.push(range);
					index += 1;
				}
				ranges
			},
		};


		Self {
			name,
			localized,
			show_missile_launch_zone,
			transceivers,
			submode,
			signals: vec![],
			scope_range_sets
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

	#[test]
	fn n_019() {
		let file = fs::read_to_string("su_n_019.txt").unwrap();
		let radar = Radar::from_str("n_019".to_owned(), &file);

		dbg!(radar);
	}
}