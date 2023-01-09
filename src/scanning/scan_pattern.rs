use crate::scanning::scan_type::ScanType;
use crate::util::Limits;

pub enum ScanPattern {
	// Search
	SearchNarrow,
	SearchMedium,
	SearchWide,

	// I assume these are ACM
	HudLock,
	VerticalLock,
	BoresightLock,

	// Listed under scan patterns?
	Track,
}

pub struct Submode {
	scan_type: ScanType,
	scan_name: ScanPattern, // Not sure about this one
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
