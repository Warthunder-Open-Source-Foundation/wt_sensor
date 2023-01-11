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
	pub pattern: Pattern,
	pub category: SubmodeCategory, // Not sure about this one
	pub limits: Limits,
	pub roll_stab_limit: Option<f64>, // Stabilizes search on the horizon against roll
	pub pitch_stab_limit: Option<f64>, // Stabilizes radar on the horizon against elevation
	pub period: Option<f64>, // Round trip scan time across any bars including reset
	pub width: Option<f64>, // Width of scanned area in degrees, from radar normal to one side, meaning the total FOV from the normal is double of this

	// Pyramid scanning uses bars to segment, with an amount and dimension
	// Pyramids appear to start at the lowest elevation in the leftmost scanning offset
	pub bar_height: Option<f64>,
	pub bars_count: Option<u8>,

	pub row_major: Option<bool>, // True means horizontal bars, false means vertical bars, (where false means inverting the width and height parameters)
	pub center_elevation: Option<f64>, // Offsets entire search on the elevation
	pub indicate: Option<bool>, // Probably whether or not to show up on the screen outside of the radar indicator
}

impl Submode {
	pub fn from_value(value: &WTBlk, scan_pattern: &str) -> Option<Self> {
		let category = SubmodeCategory::from_str(scan_pattern).ok()?;
		let pattern = Pattern::from_str(value.str("/type").ok()?).ok()?;

		let limits = Limits {
			azimuth: {
				let lhs = value.float("/azimuthLimits/0").ok()?;
				let rhs= value.float("/azimuthLimits/1").ok()?;
				lhs..=rhs
			},
			elevation: {
				let lhs = value.float("/elevationLimits/0").ok()?;
				let rhs= value.float("/elevationLimits/1").ok()?;
				lhs..=rhs
			},
		};

		let roll_stab_limit = value.float("/rollStabLimit").ok();
		let pitch_stab_limit = value.float("/pitchStabLimit").ok();
		let period = value.float("/period").ok();
		let width = value.float("/width").ok();
		let bar_height = value.float("/barHeight").ok();
		let bars_count = value.int("/barsCount").ok().map(|x|x as u8);
		let row_major = value.bool("/rowMajor").ok();
		let center_elevation = value.float("/centerElevation").ok();
		let indicate = value.bool("/indicate").ok();

		Some(Self {
			pattern,
			category,
			limits,
			roll_stab_limit,
			pitch_stab_limit,
			period,
			width,
			bar_height,
			bars_count,
			row_major,
			center_elevation,
			indicate,
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
