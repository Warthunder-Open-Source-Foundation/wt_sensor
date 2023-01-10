use std::str::FromStr;
use wt_blk::WTBlk;
use crate::scanning::scan_type::Pattern;
use crate::util::Limits;

#[derive(Debug)]
pub enum SubmodeCategory {
	// Search
	SearchNarrow,
	SearchMedium,
	SearchWide,

	// I assume these are ACM
	HudLock,
	VerticalLock,
	BoresightLock,

	DesignationLock, // Transfer state from Search to Track?

	Track,
}

#[derive(Debug)]
pub struct Submode {
	pattern: Pattern,
	category: SubmodeCategory, // Not sure about this one
	limits: Limits,
	roll_stab_limit: Option<f64>, // Stabilizes search on the horizon against roll
	pitch_stab_limit: Option<f64>, // Stabilizes radar on the horizon against elevation
	period: f64, // Round trip scan time across any bars including reset
	width: f64, // Width of scanned area in degrees, from radar normal to one side, meaning the total FOV from the normal is double of this

	// Pyramid scanning uses bars to segment, with an amount and dimension
	// Pyramids appear to start at the lowest elevation in the leftmost scanning offset
	bar_height: Option<f64>,
	bars_count: Option<u8>,

	row_major: Option<bool>, // True means horizontal bars, false means vertical bars, (where false means inverting the width and height parameters)
	center_elevation: Option<f64>, // Offsets entire search on the elevation
	indicate: Option<bool>, // Probably whether or not to show up on the screen outside of the radar indicator
}

impl Submode {
	pub fn from_value(value: &WTBlk, scan_pattern: &str) -> Option<Self> {
		let scan_name = SubmodeCategory::from_str(scan_pattern).ok()?;

		let scan_type = Pattern::from_str(value.str("/type").ok()?).ok()?;


		Some(Self {
			pattern: scan_type,
			category: SubmodeCategory::SearchNarrow,
			limits: Limits {
				azimuth: 0.0..=0.0,
				elevation: 0.0..=0.0,
			},
			roll_stab_limit: None,
			pitch_stab_limit: None,
			period: 0.0,
			width: 0.0,
			bar_height: None,
			bars_count: None,
			row_major: None,
			center_elevation: None,
			indicate: None,
		})
	}
}

impl FromStr for SubmodeCategory {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"searchNarrow" => Ok(Self::SearchNarrow),
			"searchMedium" => Ok(Self::SearchNarrow),
			"searchWide" => Ok(Self::SearchNarrow),
			"hudLock" => Ok(Self::SearchNarrow),
			"verticalLock" => Ok(Self::SearchNarrow),
			"boresightLock" => Ok(Self::SearchNarrow),
			"designationLock" => Ok(Self::DesignationLock),
			"track" => Ok(Self::Track),
			_ => Err(())
		}
	}
}
